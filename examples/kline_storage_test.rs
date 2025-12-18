use aron_rs_examples::kline_rksdb::kline_storage::KlineRdb;
use aron_rs_examples::kline_rksdb::{KLineHisQuery, Resolution};
use base_infra::result::AppResult;
use chain_types::endless::eds_addr_ext::ToEdsAddr;
use chrono::NaiveDateTime;
use rksdb_cfg::RksdbConfig;
use rksdb_infra::OpenRocksDB;
use std::path::PathBuf;
use tracing::info;

#[tokio::main]
async fn main() -> AppResult<()> {
    // rocksdb
    let kline_rdb = setup()?;

    let token = "ENDLESSsssssssssssssssssssssssssssssssssssss".to_eds_addr()?;
    let query = KLineHisQuery::new(Resolution::Min1, 8);
    let res = kline_rdb.list_kline_history(&query, token)?;
    info!("RDB#list_kline_history:res.len:{}", res.len());

    for (key, item) in res {
        let time = NaiveDateTime::from_timestamp(key.2 as i64, 0);
        info!("RDB#list_kline_history:time[{time}] key[{key:?}] item:{item:?}");
    }
    Ok(())
}

fn setup() -> AppResult<KlineRdb> {
    let mut rdb_cfg = RksdbConfig::default();
    rdb_cfg.set_data_dir(PathBuf::from("/tmp/test_rdb"));
    let kline_db = KlineRdb::new(
        KlineRdb::get_db_path(rdb_cfg.get_dir_paths()),
        "kline_db",
        &rdb_cfg.rocksdb_configs.rks_db_config,
        false,
        false,
    )?;

    Ok(kline_db)
}

