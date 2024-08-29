//! All the APIs used specifically for `re_dataframe`.

use std::{
    collections::BTreeSet,
    sync::{atomic::Ordering, Arc},
};

use ahash::HashSet;
use arrow2::{
    array::{
        Array as ArrowArray, ListArray as ArrowListArray, PrimitiveArray as ArrowPrimitiveArray,
        StructArray as ArrowStructArray,
    },
    chunk::Chunk as ArrowChunk,
    datatypes::DataType as ArrowDataType,
    Either,
};

use itertools::Itertools;
use nohash_hasher::IntSet;
use re_chunk::{Chunk, LatestAtQuery, RangeQuery};
use re_log_types::{EntityPath, TimeInt, Timeline};
use re_log_types::{EntityPathRule, ResolvedTimeRange};
use re_types_core::{ArchetypeName, ComponentName, ComponentNameSet, Loggable as _};

use crate::{store::ChunkIdSetPerTime, ChunkStore};

// Used all over in docstrings.
#[allow(unused_imports)]
use crate::RowId;

// --- Descriptors ---

// TODO: it'd be nice if these descriptors were themselves defined in arrow... you'd like to be
// able to get a dataframe of the schema itself.

// TODO
// TODO: Eeeeh, I dunno. But we need _something_.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlColumnDescriptor {
    // TODO
    pub component_name: ComponentName,

    /// The Arrow datatype of the column.
    pub datatype: ArrowDataType,
}

impl PartialOrd for ControlColumnDescriptor {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ControlColumnDescriptor {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Self {
            component_name,
            datatype: _,
        } = self;
        component_name.cmp(&other.component_name)
    }
}

// TODO
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeColumnDescriptor {
    /// The timeline this column is associated with.
    pub timeline: Timeline,

    /// The Arrow datatype of the column.
    pub datatype: ArrowDataType,
}

impl PartialOrd for TimeColumnDescriptor {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeColumnDescriptor {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Self {
            timeline,
            datatype: _,
        } = self;
        timeline.cmp(&other.timeline)
    }
}

// TODO
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentColumnDescriptor {
    /// The path of the entity.
    pub entity_path: EntityPath,

    /// Optional name of the `Archetype` associated with this data.
    ///
    /// `None` if the data wasn't logged through an archetype.
    ///
    /// Example: `rerun.archetypes.Points3D`.
    pub archetype_name: Option<ArchetypeName>,

    /// Optional name of the field within `Archetype` associated with this data.
    ///
    /// `None` if the data wasn't logged through an archetype.
    ///
    /// Example: `positions`.
    //
    // TODO: interning?
    pub archetype_field_name: Option<String>,

    /// Semantic name associated with this data.
    ///
    /// This is fully implied by `archetype_name` and `archetype_field`, but
    /// included for semantic convenience.
    ///
    /// Example: `rerun.components.Position3D`.
    pub component_name: ComponentName,

    /// The Arrow datatype of the column.
    pub datatype: ArrowDataType,
    // TODO: this is unsurprisingly problematic: a column can exist as both static and temporal at
    // the same time since all of this business is cross entity.
    // /// Whether this column represents static data.
    // //
    // // TODO: probably could use more explanation?
    // pub is_static: bool,
}

impl PartialOrd for ComponentColumnDescriptor {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComponentColumnDescriptor {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Self {
            entity_path,
            archetype_name,
            archetype_field_name,
            component_name,
            datatype: _,
        } = self;

        entity_path
            .cmp(&other.entity_path)
            .then_with(|| component_name.cmp(&other.component_name))
            .then_with(|| archetype_name.cmp(&other.archetype_name))
            .then_with(|| archetype_field_name.cmp(&other.archetype_field_name))
    }
}

// TODO
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ColumnDescriptor {
    Control(ControlColumnDescriptor),
    Time(TimeColumnDescriptor),
    Component(ComponentColumnDescriptor),
}

impl ColumnDescriptor {
    #[inline]
    pub fn entity_path(&self) -> Option<&EntityPath> {
        match self {
            Self::Component(descr) => Some(&descr.entity_path),
            Self::Control(_) | Self::Time(_) => None,
        }
    }
}

// --- Queries ---

// TODO: queries should be hashable, but EntityPathRule is problematic on that front

// TODO
// TODO: multi-PoVs
// TODO: join policies
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatestAtQueryExpression {
    /// The entity path expression to query.
    ///
    /// Example: `world/camera/**`
    pub entity_path_expr: EntityPathRule,

    /// The timeline to query.
    ///
    /// Example: `frame`.
    pub timeline: Timeline,

    /// The time at which to query.
    ///
    /// Example: `18`.
    pub time_value: TimeInt,
}

impl std::fmt::Display for LatestAtQueryExpression {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            entity_path_expr,
            timeline,
            time_value,
        } = self;

        f.write_fmt(format_args!(
            "latest state for '{entity_path_expr}' at {} on {:?}",
            timeline.typ().format_utc(*time_value),
            timeline.name(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeQueryExpression {
    /// The entity path expression to query.
    ///
    /// Example: `world/camera/**`
    pub entity_path_expr: EntityPathRule,

    /// The timeline to query.
    ///
    /// Example `frame`
    pub timeline: Timeline,

    /// The time range to query.
    pub time_range: ResolvedTimeRange,

    /// A point of view is specified by a component column descriptor.
    ///
    /// In a range query results, each non-null value of the component column
    /// will generate a row in the result.
    ///
    /// Note that a component can still be logged multiple
    pub point_of_views: Vec<ComponentColumnDescriptor>,
    //
    // TODO
    // /// Which joining policy to use. See `RowJoinPolicy`
    // row_join_policy: RowJoinPolicy,
}

impl std::fmt::Display for RangeQueryExpression {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            entity_path_expr,
            timeline,
            time_range,
            point_of_views,
        } = self;

        f.write_fmt(format_args!(
            "{entity_path_expr} ranging {}..={} on {:?} as seen from {point_of_views:?}",
            timeline.typ().format_utc(time_range.min()),
            timeline.typ().format_utc(time_range.max()),
            timeline.name(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryExpression {
    LatestAt(LatestAtQueryExpression),
    Range(RangeQueryExpression),
}

impl QueryExpression {
    #[inline]
    pub fn entity_path_expr(&self) -> &EntityPathRule {
        match self {
            Self::LatestAt(query) => &query.entity_path_expr,
            Self::Range(query) => &query.entity_path_expr,
        }
    }
}

impl std::fmt::Display for QueryExpression {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LatestAt(query) => query.fmt(f),
            Self::Range(query) => query.fmt(f),
        }
    }
}

// ---

// TODO: at some point all of this mess is going to require async since we will want to hit the
// remote schema instead?

impl ChunkStore {
    /// Returns the full schema of the store.
    ///
    /// This will include a column descriptor for every timeline and every component on every
    /// entity that has been written to the store so far.
    ///
    /// The order of the columns is guaranteed to be in a specific order:
    /// * first, the control columns in lexical order (`RowId`);
    /// * second, the time columns in lexical order (`frame_nr`, `log_time`, ...);
    /// * third, the component columns in lexical order (`Color`, `Radius, ...`).
    pub fn schema(&self) -> Vec<ColumnDescriptor> {
        re_tracing::profile_function!();

        let controls = std::iter::once(ColumnDescriptor::Control(ControlColumnDescriptor {
            component_name: RowId::name(),
            datatype: RowId::arrow_datatype(),
        }));

        let timelines = self.all_timelines().into_iter().map(|timeline| {
            ColumnDescriptor::Time(TimeColumnDescriptor {
                timeline,
                datatype: timeline.datatype(),
            })
        });

        // TODO: think about what happens once there's disk/network IO on the way.
        // Can we parallelize this? surely
        let components = self
            .temporal_chunk_ids_per_entity_per_component
            .iter()
            .flat_map(|(entity_path, per_timeline)| {
                per_timeline
                    .iter()
                    .map(move |(timeline, per_component)| (entity_path, timeline, per_component))
            })
            .flat_map(|(entity_path, _timeline, per_component)| {
                // TODO: this is a disaster but not sure we can do much better at the moment.
                per_component.keys().filter_map(|component_name| {
                    self.lookup_datatype(component_name).map(|datatype| {
                        ColumnDescriptor::Component(ComponentColumnDescriptor {
                            entity_path: entity_path.clone(),
                            archetype_name: None, // TODO: hmm... can we fake this for now somehow?
                            archetype_field_name: None, // TODO: hmm... can we fake this for now somehow?
                            component_name: *component_name,
                            datatype: datatype.clone(),
                        })
                    })
                })
            })
            .collect::<BTreeSet<_>>();

        controls.chain(timelines).chain(components).collect()
    }

    // TODO: should this look at what components exist at all, or actually run a full blown query?
    // probably the former -- you don't want your schema changing depending on the timestamp, do you?
    //
    // TODO: in the end all that matter is that the schema is fixed for the duration of the handle.
    //
    // TODO
    /// Returns the filtered schema for the given query expression.
    ///
    /// This should only include columns which will contain non-empty values from the perspective of
    /// the query semantics.
    ///
    /// Note that this result has the potential to include both multiple `TimeColumnDescriptor` and
    /// `ComponentColumnDescriptor`. Any timeline that is referenced in the same row as as the point-of-view
    /// column will be included in the result.
    pub fn schema_for_query(&self, query: &QueryExpression) -> Vec<ColumnDescriptor> {
        // TODO: showing the query as a oneliner would be nice (need display impl)
        re_tracing::profile_function!();

        // TODO: i still think not applying time is the right move? or maybe a meet-me-halfway
        // kinda thing: just look at metadata (so chunk headers only).

        let schema = self
            .schema()
            .into_iter()
            .filter(|descr| {
                descr.entity_path().map_or(true, |entity_path| {
                    query.entity_path_expr().matches(entity_path)
                })
            })
            .collect_vec();

        // TODO: think about what happens once there's disk/network IO on the way.
        // TODO: can we parallelize this? surely
        let mut filtered_out = HashSet::default();
        for column_descr in &schema {
            let ColumnDescriptor::Component(descr) = column_descr else {
                continue;
            };

            match query {
                QueryExpression::LatestAt(query) => {
                    let q = LatestAtQuery::new(query.timeline, query.time_value);
                    // TODO: i guess doing a full blown query here sucks -- we want any()
                    // semantics, pretty much
                    if self
                        .latest_at_relevant_chunks(&q, &descr.entity_path, descr.component_name)
                        .is_empty()
                    {
                        filtered_out.insert(column_descr.clone());
                    }
                }

                // TODO: hmmm wait, does that make sense? if the secondary components all have
                // latestat semantics, then that is what they should be checked for.
                QueryExpression::Range(query) => {
                    let q = RangeQuery::new(query.timeline, query.time_range);
                    // TODO: i guess doing a full blown query here sucks -- we want any()
                    // semantics, pretty much
                    if self
                        .range_relevant_chunks(&q, &descr.entity_path, descr.component_name)
                        .is_empty()
                    {
                        filtered_out.insert(column_descr.clone());
                    }
                }
            }
        }

        schema
            .into_iter()
            .filter(|descr| !filtered_out.contains(descr))
            .collect()
    }
}
