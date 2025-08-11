use std::sync::Arc;

use async_channel::{Receiver, Sender};
use papaya::HashMap;
use serde::Deserialize;
use sqlx::SqlitePool;

use adapter_types::dependencies::{
    ActiveStepWorkerDependencyProvider, CompletedInstanceWorkerDependencyProvider,
    CompletedStepWorkerDependencyProvider, ControlServerDependencyProvider, DependencyManager,
    FailedInstanceWorkerDependencyProvider, FailedStepWorkerDependencyProvider,
    NewEventWorkerDependencyProvider, NewInstanceWorkerDependencyProvider,
    NextStepWorkerDependencyProvider, active_step_worker::ActiveStepWorkerDependencies,
    completed_instance_worker::CompletedInstanceWorkerDependencies,
    completed_step_worker::CompletedStepWorkerDependencies,
    control_server::ControlServerDependencies,
    failed_instance_worker::FailedInstanceWorkerDependencies,
    failed_step_worker::FailedStepWorkerDependencies, new_event_worker::NewEventWorkerDependencies,
    new_instance_worker::NewInstanceWorkerDependencies,
    next_step_worker::NextStepWorkerDependencies,
};
use surgeflow_types::{
    FullyQualifiedStep, InstanceEvent, Project, WorkflowInstance, WorkflowInstanceId,
};

use super::{
    EmbeddedAdapterError,
    managers::{EmbeddedPersistenceManager, EmbeddedStepsAwaitingEventManager},
    receivers::{
        EmbeddedActiveStepReceiver, EmbeddedCompletedInstanceReceiver,
        EmbeddedCompletedStepReceiver, EmbeddedEventReceiver, EmbeddedFailedInstanceReceiver,
        EmbeddedFailedStepReceiver, EmbeddedNewInstanceReceiver, EmbeddedNextStepReceiver,
    },
    senders::{
        EmbeddedActiveStepSender, EmbeddedCompletedStepSender, EmbeddedEventSender,
        EmbeddedFailedInstanceSender, EmbeddedFailedStepSender, EmbeddedNewInstanceSender,
        EmbeddedNextStepSender,
    },
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbeddedAdapterConfig {}

#[derive(derive_more::Debug)]
pub struct EmbeddedDependencyManager<P: Project> {
    steps_awaiting_event_map: Arc<HashMap<WorkflowInstanceId, FullyQualifiedStep<P>>>,
    next_step_channel: Option<(
        Sender<FullyQualifiedStep<P>>,
        Receiver<FullyQualifiedStep<P>>,
    )>,
    active_step_channel: Option<(
        Sender<FullyQualifiedStep<P>>,
        Receiver<FullyQualifiedStep<P>>,
    )>,
    failed_step_channel: Option<(
        Sender<FullyQualifiedStep<P>>,
        Receiver<FullyQualifiedStep<P>>,
    )>,
    completed_step_channel: Option<(
        Sender<FullyQualifiedStep<P>>,
        Receiver<FullyQualifiedStep<P>>,
    )>,
    //
    new_instance_channel: Option<(Sender<WorkflowInstance<P>>, Receiver<WorkflowInstance<P>>)>,
    completed_instance_channel:
        Option<(Sender<WorkflowInstance<P>>, Receiver<WorkflowInstance<P>>)>,
    failed_instance_channel: Option<(Sender<WorkflowInstance<P>>, Receiver<WorkflowInstance<P>>)>,
    //
    new_event_channel: Option<(Sender<InstanceEvent<P>>, Receiver<InstanceEvent<P>>)>,
    //
    sqlx_pool: SqlitePool,
    config: EmbeddedAdapterConfig,
}

impl<P: Project> EmbeddedDependencyManager<P> {
    pub fn new(config: EmbeddedAdapterConfig, sqlx_pool: SqlitePool) -> Self {
        Self {
            steps_awaiting_event_map: Arc::new(HashMap::new()),
            sqlx_pool,
            config,
            next_step_channel: None,
            active_step_channel: None,
            failed_step_channel: None,
            completed_step_channel: None,
            new_instance_channel: None,
            completed_instance_channel: None,
            failed_instance_channel: None,
            new_event_channel: None,
        }
    }
}

impl<P: Project> EmbeddedDependencyManager<P> {
    fn steps_awaiting_event_map(&self) -> Arc<HashMap<WorkflowInstanceId, FullyQualifiedStep<P>>> {
        self.steps_awaiting_event_map.clone()
    }
    fn next_step_sender(&mut self) -> &Sender<FullyQualifiedStep<P>> {
        if self.next_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.next_step_channel = Some((sender, receiver));
        }
        &self.next_step_channel.as_ref().unwrap().0
    }
    fn next_step_receiver(&mut self) -> &Receiver<FullyQualifiedStep<P>> {
        if self.next_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.next_step_channel = Some((sender, receiver));
        }
        &self.next_step_channel.as_ref().unwrap().1
    }
    fn active_step_sender(&mut self) -> &Sender<FullyQualifiedStep<P>> {
        if self.active_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.active_step_channel = Some((sender, receiver));
        }
        &self.active_step_channel.as_ref().unwrap().0
    }
    fn active_step_receiver(&mut self) -> &Receiver<FullyQualifiedStep<P>> {
        if self.active_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.active_step_channel = Some((sender, receiver));
        }
        &self.active_step_channel.as_ref().unwrap().1
    }
    fn failed_step_sender(&mut self) -> &Sender<FullyQualifiedStep<P>> {
        if self.failed_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.failed_step_channel = Some((sender, receiver));
        }
        &self.failed_step_channel.as_ref().unwrap().0
    }
    fn failed_step_receiver(&mut self) -> &Receiver<FullyQualifiedStep<P>> {
        if self.failed_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.failed_step_channel = Some((sender, receiver));
        }
        &self.failed_step_channel.as_ref().unwrap().1
    }
    fn completed_step_sender(&mut self) -> &Sender<FullyQualifiedStep<P>> {
        if self.completed_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.completed_step_channel = Some((sender, receiver));
        }
        &self.completed_step_channel.as_ref().unwrap().0
    }
    fn completed_step_receiver(&mut self) -> &Receiver<FullyQualifiedStep<P>> {
        if self.completed_step_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.completed_step_channel = Some((sender, receiver));
        }
        &self.completed_step_channel.as_ref().unwrap().1
    }
    fn new_instance_sender(&mut self) -> &Sender<WorkflowInstance<P>> {
        if self.new_instance_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.new_instance_channel = Some((sender, receiver));
        }
        &self.new_instance_channel.as_ref().unwrap().0
    }
    fn new_instance_receiver(&mut self) -> &Receiver<WorkflowInstance<P>> {
        if self.new_instance_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.new_instance_channel = Some((sender, receiver));
        }
        &self.new_instance_channel.as_ref().unwrap().1
    }
    // fn completed_instance_sender(&mut self) -> &Sender<WorkflowInstance> {
    //     if self.completed_instance_channel.is_none() {
    //         let (sender, receiver) = async_channel::unbounded();
    //         self.completed_instance_channel = Some((sender, receiver));
    //     }
    //     &self.completed_instance_channel.as_ref().unwrap().0
    // }
    fn completed_instance_receiver(&mut self) -> &Receiver<WorkflowInstance<P>> {
        if self.completed_instance_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.completed_instance_channel = Some((sender, receiver));
        }
        &self.completed_instance_channel.as_ref().unwrap().1
    }
    fn failed_instance_sender(&mut self) -> &Sender<WorkflowInstance<P>> {
        if self.failed_instance_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.failed_instance_channel = Some((sender, receiver));
        }
        &self.failed_instance_channel.as_ref().unwrap().0
    }
    fn failed_instance_receiver(&mut self) -> &Receiver<WorkflowInstance<P>> {
        if self.failed_instance_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.failed_instance_channel = Some((sender, receiver));
        }
        &self.failed_instance_channel.as_ref().unwrap().1
    }
    fn new_event_sender(&mut self) -> &Sender<InstanceEvent<P>> {
        if self.new_event_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.new_event_channel = Some((sender, receiver));
        }
        &self.new_event_channel.as_ref().unwrap().0
    }
    fn new_event_receiver(&mut self) -> &Receiver<InstanceEvent<P>> {
        if self.new_event_channel.is_none() {
            let (sender, receiver) = async_channel::unbounded();
            self.new_event_channel = Some((sender, receiver));
        }
        &self.new_event_channel.as_ref().unwrap().1
    }

    async fn sqlx_pool(&mut self) -> &SqlitePool {
        &self.sqlx_pool
    }
}

impl<P: Project> CompletedInstanceWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type CompletedInstanceReceiver = EmbeddedCompletedInstanceReceiver<P>;
    type Error = EmbeddedAdapterError<P>;

    async fn completed_instance_worker_dependencies(
        &mut self,
    ) -> Result<CompletedInstanceWorkerDependencies<P, Self::CompletedInstanceReceiver>, Self::Error>
    {
        let completed_instance_receiver =
            EmbeddedCompletedInstanceReceiver::new(self.completed_instance_receiver().clone());

        Ok(CompletedInstanceWorkerDependencies::new(
            completed_instance_receiver,
        ))
    }
}

impl<P: Project> CompletedStepWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type CompletedStepReceiver = EmbeddedCompletedStepReceiver<P>;
    type NextStepSender = EmbeddedNextStepSender<P>;
    type PersistenceManager = EmbeddedPersistenceManager;
    type Error = EmbeddedAdapterError<P>;

    async fn completed_step_worker_dependencies(
        &mut self,
    ) -> Result<
        CompletedStepWorkerDependencies<
            P,
            Self::CompletedStepReceiver,
            Self::NextStepSender,
            Self::PersistenceManager,
        >,
        Self::Error,
    > {
        let completed_step_receiver =
            EmbeddedCompletedStepReceiver::<P>::new(self.completed_step_receiver().clone());

        let next_step_sender = EmbeddedNextStepSender::<P>::new(self.next_step_sender().clone());

        let persistence_manager = EmbeddedPersistenceManager::new(self.sqlx_pool().await.clone());

        Ok(CompletedStepWorkerDependencies::new(
            completed_step_receiver,
            next_step_sender,
            persistence_manager,
        ))
    }
}

impl<P: Project> ActiveStepWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type ActiveStepReceiver = EmbeddedActiveStepReceiver<P>;
    type ActiveStepSender = EmbeddedActiveStepSender<P>;
    type FailedStepSender = EmbeddedFailedStepSender<P>;
    type CompletedStepSender = EmbeddedCompletedStepSender<P>;
    type PersistenceManager = EmbeddedPersistenceManager;
    type Error = EmbeddedAdapterError<P>;

    async fn active_step_worker_dependencies(
        &mut self,
    ) -> Result<
        ActiveStepWorkerDependencies<
            P,
            Self::ActiveStepReceiver,
            Self::ActiveStepSender,
            Self::FailedStepSender,
            Self::CompletedStepSender,
            Self::PersistenceManager,
        >,
        Self::Error,
    > {
        let active_step_receiver =
            EmbeddedActiveStepReceiver::<P>::new(self.active_step_receiver().clone());

        let active_step_sender =
            EmbeddedActiveStepSender::<P>::new(self.active_step_sender().clone());

        let failed_step_sender =
            EmbeddedFailedStepSender::<P>::new(self.failed_step_sender().clone());

        let completed_step_sender =
            EmbeddedCompletedStepSender::<P>::new(self.completed_step_sender().clone());

        let persistence_manager = EmbeddedPersistenceManager::new(self.sqlx_pool().await.clone());

        Ok(ActiveStepWorkerDependencies::new(
            active_step_receiver,
            active_step_sender,
            failed_step_sender,
            completed_step_sender,
            persistence_manager,
        ))
    }
}

impl<P: Project> FailedInstanceWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type FailedInstanceReceiver = EmbeddedFailedInstanceReceiver<P>;
    type Error = EmbeddedAdapterError<P>;

    async fn failed_instance_worker_dependencies(
        &mut self,
    ) -> Result<FailedInstanceWorkerDependencies<P, Self::FailedInstanceReceiver>, Self::Error>
    {
        let failed_instance_receiver =
            EmbeddedFailedInstanceReceiver::<P>::new(self.failed_instance_receiver().clone());

        Ok(FailedInstanceWorkerDependencies::new(
            failed_instance_receiver,
        ))
    }
}

impl<P: Project> FailedStepWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type FailedStepReceiver = EmbeddedFailedStepReceiver<P>;
    type FailedInstanceSender = EmbeddedFailedInstanceSender<P>;
    type PersistenceManager = EmbeddedPersistenceManager;
    type Error = EmbeddedAdapterError<P>;

    async fn failed_step_worker_dependencies(
        &mut self,
    ) -> Result<
        FailedStepWorkerDependencies<
            P,
            Self::FailedStepReceiver,
            Self::FailedInstanceSender,
            Self::PersistenceManager,
        >,
        Self::Error,
    > {
        let failed_step_receiver =
            EmbeddedFailedStepReceiver::<P>::new(self.failed_step_receiver().clone());

        let failed_instance_sender =
            EmbeddedFailedInstanceSender::<P>::new(self.failed_instance_sender().clone());

        let persistence_manager = EmbeddedPersistenceManager::new(self.sqlx_pool().await.clone());

        Ok(FailedStepWorkerDependencies::new(
            failed_step_receiver,
            failed_instance_sender,
            persistence_manager,
        ))
    }
}

impl<P: Project> NewEventWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type ActiveStepSender = EmbeddedActiveStepSender<P>;
    type EventReceiver = EmbeddedEventReceiver<P>;
    type StepsAwaitingEventManager = EmbeddedStepsAwaitingEventManager<P>;
    type Error = EmbeddedAdapterError<P>;

    async fn new_event_worker_dependencies(
        &mut self,
    ) -> Result<
        NewEventWorkerDependencies<
            P,
            Self::ActiveStepSender,
            Self::EventReceiver,
            Self::StepsAwaitingEventManager,
        >,
        Self::Error,
    > {
        let steps_awaiting_event_map = self.steps_awaiting_event_map().clone();
        let new_event_receiver = EmbeddedEventReceiver::<P>::new(self.new_event_receiver().clone());
        let active_step_sender =
            EmbeddedActiveStepSender::<P>::new(self.active_step_sender().clone());

        let steps_awaiting_event_manager =
            EmbeddedStepsAwaitingEventManager::<P>::new(steps_awaiting_event_map);

        Ok(NewEventWorkerDependencies::new(
            active_step_sender,
            new_event_receiver,
            steps_awaiting_event_manager,
        ))
    }
}

impl<P: Project> NewInstanceWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type NextStepSender = EmbeddedNextStepSender<P>;
    type NewInstanceReceiver = EmbeddedNewInstanceReceiver<P>;
    type PersistenceManager = EmbeddedPersistenceManager;
    type Error = EmbeddedAdapterError<P>;

    async fn new_instance_worker_dependencies(
        &mut self,
    ) -> Result<
        NewInstanceWorkerDependencies<
            P,
            Self::NextStepSender,
            Self::NewInstanceReceiver,
            Self::PersistenceManager,
        >,
        Self::Error,
    > {
        let new_instance_receiver =
            EmbeddedNewInstanceReceiver::<P>::new(self.new_instance_receiver().clone());

        let next_step_sender = EmbeddedNextStepSender::<P>::new(self.next_step_sender().clone());

        let persistence_manager = EmbeddedPersistenceManager::new(self.sqlx_pool().await.clone());

        Ok(NewInstanceWorkerDependencies::new(
            next_step_sender,
            new_instance_receiver,
            persistence_manager,
        ))
    }
}

impl<P: Project> NextStepWorkerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type NextStepReceiver = EmbeddedNextStepReceiver<P>;
    type ActiveStepSender = EmbeddedActiveStepSender<P>;
    type StepsAwaitingEventManager = EmbeddedStepsAwaitingEventManager<P>;
    type PersistenceManager = EmbeddedPersistenceManager;
    type Error = EmbeddedAdapterError<P>;

    async fn next_step_worker_dependencies(
        &mut self,
    ) -> Result<
        NextStepWorkerDependencies<
            P,
            Self::NextStepReceiver,
            Self::ActiveStepSender,
            Self::StepsAwaitingEventManager,
            Self::PersistenceManager,
        >,
        Self::Error,
    > {
        let steps_awaiting_event_map = self.steps_awaiting_event_map().clone();

        let next_step_receiver =
            EmbeddedNextStepReceiver::<P>::new(self.next_step_receiver().clone());

        let active_step_sender =
            EmbeddedActiveStepSender::<P>::new(self.active_step_sender().clone());

        let steps_awaiting_event_manager =
            EmbeddedStepsAwaitingEventManager::<P>::new(steps_awaiting_event_map);

        let persistence_manager = EmbeddedPersistenceManager::new(self.sqlx_pool().await.clone());

        Ok(NextStepWorkerDependencies::new(
            next_step_receiver,
            active_step_sender,
            steps_awaiting_event_manager,
            persistence_manager,
        ))
    }
}

impl<P: Project> ControlServerDependencyProvider<P> for EmbeddedDependencyManager<P> {
    type EventSender = EmbeddedEventSender<P>;
    type NewInstanceSender = EmbeddedNewInstanceSender<P>;
    type Error = EmbeddedAdapterError<P>;

    async fn control_server_dependencies(
        &mut self,
    ) -> Result<ControlServerDependencies<P, Self::EventSender, Self::NewInstanceSender>, Self::Error>
    {
        let new_event_sender = EmbeddedEventSender::<P>::new(self.new_event_sender().clone());

        let new_instance_sender =
            EmbeddedNewInstanceSender::<P>::new(self.new_instance_sender().clone());

        Ok(ControlServerDependencies::new(
            new_event_sender,
            new_instance_sender,
        ))
    }
}

impl<P: Project> DependencyManager<P> for EmbeddedDependencyManager<P> {
    type Error = EmbeddedAdapterError<P>;
}
