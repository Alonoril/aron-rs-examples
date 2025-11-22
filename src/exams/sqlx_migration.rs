use std::fmt::Debug;
use crate::error::DBErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use sea_orm::Database as SeaDatabase;
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;
use tracing::info;
pub trait DbCfgTrait: Default + Debug + Send + Sync {
    fn db_url(&self) -> String;
    fn debug_db_url(&self) -> String;
    fn max_conns(&self) -> u32;
    fn min_conns(&self) -> u32;
    fn conn_timeout_secs(&self) -> u64;
    fn idle_timeout_secs(&self) -> u64;
    fn max_lifetime_secs(&self) -> u64;
}


#[async_trait::async_trait]
pub trait SqlxMigrateTrait {
    async fn migrate(&self, conn: &DatabaseConnection) -> AppResult<()>;
}

#[async_trait::async_trait]
pub trait DatabaseTrait<T, Cfg, Mg>
where
    Cfg: DbCfgTrait + Sync + Send,
    Mg: SqlxMigrateTrait + Sync + Send,
{
    async fn setup(cfg: &Cfg, migrate: &Mg) -> AppResult<T>;

    async fn connect(cfg: &Cfg) -> AppResult<DatabaseConnection> {
        let mut opt = ConnectOptions::new(cfg.db_url());
        opt.max_connections(cfg.max_conns())
            .min_connections(cfg.min_conns())
            .connect_timeout(Duration::from_secs(cfg.conn_timeout_secs()))
            .idle_timeout(Duration::from_secs(cfg.idle_timeout_secs()))
            .max_lifetime(Duration::from_secs(cfg.max_lifetime_secs()));

        let pool = SeaDatabase::connect(opt)
            .await
            .map_err(map_err!(&DBErr::InitDbPoolErr, cfg.debug_db_url()))?;

        info!("connected to database，url: {}", cfg.debug_db_url());
        Ok(pool)
    }
}


/// Database Connection
#[derive(Debug)]
pub struct DatabaseConn {
    pub pool: DatabaseConnection,
}

#[async_trait::async_trait]
impl<DbCfg, Mg> DatabaseTrait<DatabaseConn, DbCfg, Mg> for DatabaseConn
where
    DbCfg: DbCfgTrait,
    Mg: SqlxMigrateTrait + Sync + Send,
{
    async fn setup(cfg: &DbCfg, migrate: &Mg) -> AppResult<DatabaseConn> {
        // let db = Self::connect(cfg).await?;
        let db = <Self as DatabaseTrait<DatabaseConn, DbCfg, Mg>>::connect(cfg).await?;
        migrate.migrate(&db).await?;
        Ok(Self { pool: db })
    }
}