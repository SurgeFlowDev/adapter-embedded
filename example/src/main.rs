use adapter_embedded::MIGRATOR;
use adapter_embedded::dependencies::{EmbeddedAdapterConfig, EmbeddedDependencyManager};
use sqlx::query;
use surgeflow::main_handler;
use tracing::Level;

use crate::workflows::workflow_2::MyProject;
use crate::workflows::workflow_2::MyWorkflow;

mod workflows;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let config = EmbeddedAdapterConfig {};

    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to SQLite database");

    MIGRATOR.run(&pool).await?;
    query("INSERT OR IGNORE INTO workflows (name) VALUES ('MyWorkflow')")
        .execute(&pool)
        .await?;

    let dependency_manager = EmbeddedDependencyManager::new(config, pool);

    let project = MyProject {
        // workflow_1: Workflow1 {},
        workflow: MyWorkflow {},
    };

    main_handler(project, dependency_manager).await?;
    Ok(())
}
