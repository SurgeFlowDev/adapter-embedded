use adapter_embedded::MIGRATOR;
use adapter_embedded::dependencies::{EmbeddedAdapterConfig, EmbeddedDependencyManager};
use sqlx::query;
use surgeflow::main_handler;
use tracing::Level;
use workflows::MyProject;
use workflows::workflow_1::Workflow1;
use workflows::workflow_2::Workflow2;

mod workflows;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let config = EmbeddedAdapterConfig {
        pg_connection_string: "sqlite::memory:".into(),
    };

    let pool = sqlx::SqlitePool::connect(&config.pg_connection_string)
        .await
        .expect("Failed to connect to SQLite database");

    MIGRATOR.run(&pool).await?;
    query("INSERT INTO workflows (name) VALUES ('workflow_1'), ('workflow_2')")
        .execute(&pool)
        .await?;

    let dependency_manager = EmbeddedDependencyManager::new(config);

    let project = MyProject {
        workflow_1: Workflow1 {},
        workflow_2: Workflow2 {},
    };

    main_handler(project, dependency_manager).await?;
    Ok(())
}
