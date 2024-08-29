use std::path::PathBuf;

use itertools::Itertools;
use re_chunk::{TimeInt, Timeline};
use re_chunk_store::{ChunkStore, ChunkStoreConfig, LatestAtQueryExpression, VersionPolicy};
use re_dataframe::{EntityPathRule, QueryEngine};
use re_log_types::StoreKind;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect_vec();

    let Some(path_to_rrd) = args.get(1) else {
        eprintln!(
            "Usage: {} <path_to_rrd>",
            args.get(0).map(|s| s.as_str()).unwrap_or("$BIN")
        );
        std::process::exit(1);
    };

    let stores = ChunkStore::from_rrd_filepath(
        &ChunkStoreConfig::DEFAULT,
        path_to_rrd,
        VersionPolicy::Warn,
    )?;

    if false {
        dbg!(stores.keys());
        for (store_id, store) in &stores {
            if store_id.kind != StoreKind::Recording {
                continue;
            }

            let schema = store.schema();
            eprintln!("=== Store {store_id} === {schema:#?}");
        }
    }

    for (store_id, store) in &stores {
        if store_id.kind != StoreKind::Recording {
            continue;
        }

        // TODO: need ChunkStoreHandle and QueryCacheHandle asap
        let cache = re_dataframe::external::re_query::Caches::new(store);
        let engine = QueryEngine {
            store,
            cache: &cache,
        };

        {
            let query = LatestAtQueryExpression {
                // TODO: this thing needs to impl FromStr or smth
                entity_path_expr: EntityPathRule::including_subtree("/".into()),
                timeline: Timeline::log_time(),
                time_value: TimeInt::MAX,
            };

            let query_handle = engine.latest_at(&query, None /* columns */);
            let batch = query_handle.get();

            eprintln!("{query}:\n{batch}");
        }

        eprintln!("---");

        {
            let query = LatestAtQueryExpression {
                // TODO: this thing needs to impl FromStr or smth
                entity_path_expr: EntityPathRule::including_subtree(
                    "/helix/structure/scaffolding".into(),
                ),
                timeline: Timeline::log_time(),
                time_value: TimeInt::MAX,
            };

            let query_handle = engine.latest_at(&query, None /* columns */);
            let batch = query_handle.get();

            eprintln!("{query}:\n{batch}");
        }
    }

    Ok(())
}
