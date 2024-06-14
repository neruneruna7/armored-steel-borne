use sqlx::PgPool;
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::{self, Postgres};

/// コンテナの生存期間を，呼び出し元にゆだねるために，コンテナの変数を返す
pub async fn setup_postgres_testcontainer() -> (ContainerAsync<Postgres>, PgPool) {
    let container = postgres::Postgres::default().start().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string =
        &format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",);
    let pool = PgPool::connect(connection_string).await.unwrap();
    // スキーマをセットアップする
    // マイグレーション
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    (container, pool)
}
