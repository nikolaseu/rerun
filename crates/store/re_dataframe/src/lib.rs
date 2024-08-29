//! The Rerun public data APIs. Get dataframes back from your Rerun datastore.

#[doc(no_inline)]
pub use re_log_types::EntityPathRule;

pub mod external {
    pub use re_chunk;
    pub use re_chunk_store;
    pub use re_query;
}

// ---

use ahash::{HashMap, HashSet};
use arrow2::{
    array::{
        Array as ArrowArray, ListArray as ArrowListArray, PrimitiveArray as ArrowPrimitiveArray,
        StructArray as ArrowStructArray,
    },
    chunk::Chunk as ArrowChunk,
    datatypes::{
        DataType as ArrowDatatype, Field as ArrowField, Metadata as ArrowMetadata,
        Schema as ArrowSchema, TimeUnit as ArrowTimeUnit,
    },
    Either,
};

use re_chunk::{
    EntityPath, LatestAtQuery, RowId, TimeInt, Timeline, TransportChunk, UnitChunkShared,
};
use re_chunk_store::{
    ChunkStore, ColumnDescriptor, ComponentColumnDescriptor, ControlColumnDescriptor,
    LatestAtQueryExpression, QueryExpression, RangeQueryExpression, TimeColumnDescriptor,
};
use re_log_types::ResolvedTimeRange;
use re_query::Caches;
use re_types_core::ArchetypeName;

// ---

// TODO: at some point all of this mess is going to require async... unless we just turn
// QueryHandle into a promise?

// TODO: arrow-rs migration -> arrow::RecordBatch
pub type RecordBatch = TransportChunk;

// --- Queries ---

// TODO: comment on the dense-ness of the results?
// TODO: put a ChunkStoreHandle and a QueryCachesHandle in here
// TODO: decimation
// TODO: at some point we will need some pagination of the indices themselves

// TODO: let's start with range only and see if we can retrofit latest-at

pub type ChunkStoreHandle = ();
pub type QueryCacheHandle = ();

pub struct QueryEngine<'a> {
    // store: ChunkStoreHandle,
    // cache: QueryCacheHandle,
    pub store: &'a ChunkStore,
    pub cache: &'a Caches,
}

impl QueryEngine<'_> {
    /// Returns the full schema of the store.
    ///
    /// This will include a column descriptor for every timeline and every component on every
    /// entity that has been written to the store so far.
    ///
    /// The order of the columns to guaranteed to be in a specific order:
    /// * first, the control columns in lexical order (`RowId`);
    /// * second, the time columns in lexical order (`frame_nr`, `log_time`, ...);
    /// * third, the component columns in lexical order (`Color`, `Radius, ...`).
    pub fn schema(&self) -> Vec<ColumnDescriptor> {
        self.store.schema()
    }

    // TODO: do we need latest_at vs. range or do we need just expose a
    // QueryHandle::new(QueryExpression) and call it a day?

    // TODO: outdated, likely
    //
    /// Executes a `LatestAt` query.
    ///
    /// Returns a single RecordBatch containing a single row.
    ///
    /// If `columns` is provided, the `LatestAtResult` will only include those columns.
    ///
    /// Because data is often logged concurrently across multiple timelines, the non-primary timelines
    /// are still valid data-columns to include in the result. So a user could, for example, query
    /// for a range of data on the `frame` timeline, but still include the `log_time` timeline in
    /// the result.
    ///
    /// Alternatively, `columns` will be determined by the result of `schema_for_query`.
    ///
    /// Any provided `ColumnDescriptors` that don't match a column in the result will still be included, but the
    /// data will be null for the entire column.
    pub fn latest_at(
        &self,
        query: &LatestAtQueryExpression,
        // TODO: hmmm... it is a bit weird to pass a full blown descriptor here... shouldnt it be a
        // hash or a handle of some kind?
        columns: Option<Vec<ColumnDescriptor>>,
    ) -> LatestAtQueryHandle<'_> {
        LatestAtQueryHandle {
            engine: self,
            columns: columns.unwrap_or_else(|| {
                self.store
                    .schema_for_query(&QueryExpression::LatestAt(query.clone()))
            }),
            query: query.clone(),
        }
    }

    #[cfg(TODO)]
    // TODO: range
    /// Executes a `Range` query.
    ///
    /// Returns a single RecordBatch containing a single row.
    ///
    /// If `columns` is provided, the `RangeQueryResult` will only include those columns.
    ///
    /// Because data is often logged concurrently across multiple timelines, the non-primary timelines
    /// are still valid data-columns to include in the result. So a user could, for example, query
    /// for a range of data on the `frame` timeline, but still include the `log_time` timeline in
    /// the result.
    ///
    /// Alternatively, `columns` will be determined by the result of `schema_for_query`.
    ///
    /// Any provided `ColumnDescriptors` that don't match a column in the result will still be included, but the
    /// data will be null for the entire column.
    pub fn range(
        &self,
        query: &RangeQueryExpression,
        columns: Option<Vec<ColumnDescriptor>>,
    ) -> LatestAtQueryHandle {
        let RangeQueryExpression {
            entity_path_expr,
            timeline,
            time_range,
            point_of_views,
        } = query;
        todo!()
        // QueryHandle {
        //     query: QueryExpression::Range(query),
        //     columns: (),
        //     indices: (),
        // }
    }
}

pub struct LatestAtQueryHandle<'a> {
    // TODO: refs are hell, we need ChunkStoreHandle and QueryCacheHandle.
    engine: &'a QueryEngine<'a>,

    /// The original query expression used to instantiate this handle.
    query: LatestAtQueryExpression,

    /// The schema that describes any data returned through this handle.
    columns: Vec<ColumnDescriptor>,
}

impl LatestAtQueryHandle<'_> {
    // TODO
    // /// The actual Arrow RecordBatch containing the data.
    // ///
    // /// Note that this RecordBatch contains redundant raw arrow schema representation of
    // /// the columns in the `columns` field. The two must match for this to be a valid result.
    pub fn get(&self) -> RecordBatch {
        // TODO: async / parallelization / etc

        // TODO: no matter what, the thing we output here must have the shape described by `self.columns`.

        // let schema = self.columns.iter().map(|descr| ArrowField::new());

        // TODO: sorbetization
        // TODO: ColumnDescriptor.to_field pls
        let schema = ArrowSchema {
            fields: self
                .columns
                .iter()
                .map(|descr| match descr {
                    ColumnDescriptor::Control(descr) => {
                        let ControlColumnDescriptor {
                            component_name,
                            datatype,
                        } = descr;
                        ArrowField::new(
                            component_name.to_string(),
                            datatype.clone(),
                            false, /* nullable */
                        )
                    }

                    ColumnDescriptor::Time(descr) => {
                        let TimeColumnDescriptor { timeline, datatype } = descr;
                        ArrowField::new(
                            timeline.name().to_string(),
                            datatype.clone(),
                            false, /* nullable */
                        )
                    }

                    ColumnDescriptor::Component(descr) => {
                        let ComponentColumnDescriptor {
                            entity_path,
                            archetype_name,
                            archetype_field_name,
                            component_name,
                            datatype,
                        } = descr;
                        ArrowField::new(
                            component_name.short_name().to_owned(),
                            datatype.clone(),
                            false, /* nullable */
                        )
                        .with_metadata(
                            [
                                Some(("sorbet.path".to_owned(), entity_path.to_string())),
                                Some((
                                    "sorbet.semantic_type".to_owned(),
                                    component_name.short_name().to_owned(),
                                )),
                                archetype_name.map(|name| {
                                    (
                                        "sorbet.semantic_family".to_owned(),
                                        name.short_name().to_owned(),
                                    )
                                }),
                                archetype_field_name.as_ref().map(|name| {
                                    ("sorbet.logical_type".to_owned(), name.to_owned())
                                }),
                            ]
                            .into_iter()
                            .flatten()
                            .collect(),
                        )
                    }
                })
                .collect(),

            metadata: Default::default(), // TODO,
        };

        // for descr in columns {}

        // TODO: parallelize / async / etc
        let query = LatestAtQuery::new(self.query.timeline, self.query.time_value);
        let chunks: HashMap<&ComponentColumnDescriptor, UnitChunkShared> = self
            .columns
            .iter()
            .filter_map(|descr| match descr {
                ColumnDescriptor::Control(_) => None,

                ColumnDescriptor::Time(_) => None,

                ColumnDescriptor::Component(descr) => {
                    let ComponentColumnDescriptor {
                        entity_path,
                        archetype_name,
                        archetype_field_name,
                        component_name,
                        datatype,
                    } = descr;

                    let results = self.engine.cache.latest_at(
                        self.engine.store,
                        &query,
                        entity_path,
                        [*component_name],
                    );

                    results
                        .components
                        .get(component_name)
                        .cloned()
                        .map(|chunk| (descr, chunk))
                }
            })
            .collect();

        // Find max RowId.

        // Find max time for each timeline.
        // TODO: needs to deal with the edge case where the winning chunk doesn't actually contain
        // all timelines.
        let mut max_index = (TimeInt::STATIC, RowId::ZERO);
        let mut winner = None;
        for descr in self.columns.iter().filter_map(|descr| match descr {
            ColumnDescriptor::Time(descr) => Some(descr),
            _ => None,
        }) {
            for chunk in chunks.values() {
                if let Some(index) = chunk.index(&self.query.timeline) {
                    if index > max_index {
                        winner = Some(chunk.clone());
                        max_index = index;
                    }
                }
            }
            //
        }

        // TODO: (for range) there are more efficient ways to do this: namely a bootstrap latestat
        // followed by a range query with all components as PoV -- but one thing at a time.
        let arrays = self
            .columns
            .iter()
            .map(|descr| match descr {
                ColumnDescriptor::Control(descr) => {
                    let ControlColumnDescriptor { datatype, .. } = descr;
                    winner.as_ref().map_or_else(
                        // TODO: I guess this is nullable then, all of it
                        || arrow2::array::new_null_array(datatype.clone(), 1),
                        |chunk| chunk.row_ids_raw_for_real().to_boxed(),
                    )
                }

                ColumnDescriptor::Time(descr) => {
                    let TimeColumnDescriptor { datatype, .. } = descr;
                    winner
                        .as_ref()
                        .and_then(|chunk| chunk.timelines().get(&descr.timeline))
                        .map_or_else(
                            // TODO: I guess this is nullable then, all of it
                            || arrow2::array::new_null_array(datatype.clone(), 1),
                            |time_column| time_column.times_raw_for_real().to_boxed(),
                        )
                }

                ColumnDescriptor::Component(descr) => {
                    let ComponentColumnDescriptor { datatype, .. } = descr;

                    chunks
                        .get(descr)
                        .and_then(|chunk| chunk.components().get(&descr.component_name))
                        .map_or_else(
                            || arrow2::array::new_null_array(datatype.clone(), 1),
                            |list_array| list_array.to_boxed(),
                        )
                }
            })
            .collect();

        RecordBatch {
            schema,
            data: ArrowChunk::new(arrays),
        }
    }

    // // TODO: streams all results -- no intermediate state
    // pub fn iter(&self) -> impl Iterator<Item = RecordBatch> {
    //     std::iter::empty()
    // }
    //
    // // TODO: returns paginated results
    // pub fn get(row_range: std::ops::RangeInclusive<u64>) -> impl Iterator<Item = RecordBatch> {
    //     std::iter::empty()
    // }
}

// TODO: mention prefetching possibilities in range impl

// struct LatestAtResult {
//     /// The original query expression used to generate this result.
//     query: LatestAtQueryExpression,
//
//     /// The columns in the result, in the same order as the data.
//     columns: Vec<ColumnDescriptor>,
//
//     /// The actual Arrow RecordBatch containing the data.
//     ///
//     /// Note that this RecordBatch contains redundant raw arrow schema representation of
//     /// the columns in the `columns` field. The two must match for this to be a valid result.
//     data: RecordBatch,
// }
//
// struct RangeQueryResult {
//     /// The original query expression used to generate this result.
//     query: RangeQueryExpression,
//
//     /// The columns in the result, in the same order as the data.
//     columns: Vec<ColumnDescriptor>,
//
//     /// The actual arrow RecordBatch containing the data.
//     ///
//     /// Note that this RecordBatch contains redundant raw arrow schema representation of
//     /// the columns in the `columns` field. The two must match for this to be a valid result.
//     data: Iter<Item = RecordBatch>,
// }
