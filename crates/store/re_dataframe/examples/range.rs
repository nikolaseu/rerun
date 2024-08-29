use std::path::PathBuf;

use itertools::Itertools;
use re_chunk_store::{ChunkStore, ChunkStoreConfig, VersionPolicy};
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

    dbg!(stores.keys());

    for (store_id, store) in stores {
        if store_id.kind != StoreKind::Recording {
            continue;
        }

        let schema = store.schema();
        eprintln!("=== Store {store_id} === {schema:#?}");
    }

    Ok(())
}
