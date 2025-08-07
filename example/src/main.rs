use embedded_adapter::MIGRATOR;
use embedded_adapter::dependencies::{AwsAdapterConfig, AwsDependencyManager};
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

    let config = AwsAdapterConfig {
        pg_connection_string: "sqlite::memory:".into(),
    };

    let mut dependency_manager = AwsDependencyManager::new(config);

    let pool = dependency_manager.sqlx_pool().await;

    MIGRATOR.run(pool).await?;
    query("INSERT INTO workflows (name) VALUES ('workflow_1'), ('workflow_2')")
        .execute(pool)
        .await?;

    let project = MyProject {
        workflow_1: Workflow1 {},
        workflow_2: Workflow2 {},
    };

    main_handler(project, dependency_manager).await?;
    Ok(())
}
