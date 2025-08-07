use papaya::HashMap;
use std::{marker::PhantomData, sync::Arc};

use super::EmbeddedAdapterError;
use adapter_types::managers::StepsAwaitingEventManager;
use surgeflow_types::{FullyQualifiedStep, Project, WorkflowInstanceId};

#[derive(Clone)]
pub struct EmbeddedSqsStepsAwaitingEventManager<P: Project> {
    map: Arc<HashMap<WorkflowInstanceId, FullyQualifiedStep<P>>>,
    _phantom: PhantomData<P>,
}

impl<P: Project> EmbeddedSqsStepsAwaitingEventManager<P> {
    // TODO: should be private
    pub fn new(map: Arc<HashMap<WorkflowInstanceId, FullyQualifiedStep<P>>>) -> Self {
        Self {
            map,
            _phantom: PhantomData,
        }
    }
}

impl<P: Project> StepsAwaitingEventManager<P> for EmbeddedSqsStepsAwaitingEventManager<P> {
    type Error = EmbeddedAdapterError<P>;
    async fn get_step(
        &mut self,
        instance_id: WorkflowInstanceId,
    ) -> Result<Option<FullyQualifiedStep<P>>, Self::Error> {
        let map = self.map.pin();
        match map.get(&instance_id) {
            Some(step) => Ok(Some(step.clone())),
            None => Ok(None),
        }
    }

    async fn delete_step(&mut self, instance_id: WorkflowInstanceId) -> Result<(), Self::Error> {
        let map = self.map.pin();
        map.remove(&instance_id);
        Ok(())
    }

    async fn put_step(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        let map = self.map.pin();
        map.insert(step.instance.external_id, step);
        Ok(())
    }
}

mod persistence_manager {
    use sqlx::{SqlitePool, query};

    use adapter_types::managers::PersistenceManager;
    use surgeflow_types::{Project, StepId, WorkflowInstance, WorkflowInstanceId};

    use crate::EmbeddedAdapterError;

    #[derive(Clone)]
    pub struct EmbeddedPersistenceManager {
        sqlx_pool: SqlitePool,
    }
    impl EmbeddedPersistenceManager {
        pub fn new(sqlx_pool: SqlitePool) -> Self {
            Self { sqlx_pool }
        }
    }
    impl<P: Project> PersistenceManager<P> for EmbeddedPersistenceManager {
        type Error = EmbeddedAdapterError<P>;
        async fn set_step_status(&self, step_id: StepId, status: i32) -> Result<(), Self::Error> {
            let step_id = step_id.to_string();
            query!(
                r#"
                UPDATE workflow_steps SET "status" = $1
                WHERE "external_id" = $2
                "#,
                status,
                step_id
            )
            .execute(&self.sqlx_pool)
            .await?;

            Ok(())
        }

        async fn insert_step(
            &self,
            workflow_instance_id: WorkflowInstanceId,
            step_id: StepId,
            step: &P::Step,
        ) -> Result<(), Self::Error> {
            let json_step = serde_json::to_value(step).map_err(EmbeddedAdapterError::SerializeError)?;
            let workflow_instance_id = workflow_instance_id.to_string();
            let step_id = step_id.to_string();
            query!(
                r#"
                INSERT INTO workflow_steps ("workflow_instance_external_id", "external_id", "step")
                VALUES ($1, $2, $3)
                "#,
                workflow_instance_id,
                step_id,
                json_step
            )
            .execute(&self.sqlx_pool)
            .await?;

            Ok(())
        }

        async fn insert_step_output(
            &self,
            step_id: StepId,
            output: Option<&P::Step>,
        ) -> Result<(), EmbeddedAdapterError<P>> {
            let output = serde_json::to_value(output).expect("TODO: handle serialization error");
            let step_id = step_id.to_string();
            query!(
                r#"
            INSERT INTO workflow_step_outputs ("workflow_step_id", "output")
            VALUES ((SELECT id FROM workflow_steps WHERE external_id = $1), $2)
            "#,
                step_id,
                output
            )
            .execute(&self.sqlx_pool)
            .await?;

            Ok(())
        }

        async fn insert_instance(
            &self,
            workflow_instance: WorkflowInstance,
        ) -> Result<WorkflowInstanceId, Self::Error> {
            let external_id = workflow_instance.external_id.to_string();
            let workflow_name = String::from(workflow_instance.workflow_name);
            query!(
                r#"
                INSERT INTO workflow_instances ("workflow_id", "external_id")
                SELECT "id", $1
                FROM workflows
                WHERE "name" = $2
                RETURNING "external_id"
                "#,
                external_id,
                workflow_name                
            )
            .fetch_one(&self.sqlx_pool)
            .await?;

            Ok(workflow_instance.external_id)
        }
    }
}
pub use persistence_manager::EmbeddedPersistenceManager;
