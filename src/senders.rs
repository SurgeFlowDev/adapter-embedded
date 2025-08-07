use async_channel::Sender;
use std::marker::PhantomData;

use surgeflow_types::{FullyQualifiedStep, InstanceEvent, Project, WorkflowInstance};

use adapter_types::senders::{
    ActiveStepSender, CompletedInstanceSender, CompletedStepSender, EventSender,
    FailedInstanceSender, FailedStepSender, NewInstanceSender, NextStepSender,
};

use crate::EmbeddedAdapterError;

#[derive(Debug, Clone)]
pub struct EmbeddedSqsNextStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> NextStepSender<P> for EmbeddedSqsNextStepSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedSqsNextStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedSqsActiveStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> ActiveStepSender<P> for EmbeddedSqsActiveStepSender<P> {
    type Error = EmbeddedAdapterError<P>;
    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedSqsActiveStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedSqsFailedStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> FailedStepSender<P> for EmbeddedSqsFailedStepSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedSqsFailedStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedSqsEventSender<P: Project> {
    sender: Sender<InstanceEvent<P>>,
}

impl<P: Project> EventSender<P> for EmbeddedSqsEventSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: InstanceEvent<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedSqsEventSender<P> {
    pub fn new(sender: Sender<InstanceEvent<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedSqsNewInstanceSender<P: Project> {
    sender: Sender<WorkflowInstance>,
    _marker: PhantomData<P>,
}

impl<P: Project> NewInstanceSender<P> for EmbeddedSqsNewInstanceSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: WorkflowInstance) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendWorkflowInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedSqsNewInstanceSender<P> {
    pub fn new(sender: Sender<WorkflowInstance>) -> Self {
        Self {
            sender,
            _marker: PhantomData,
        }
    }
}

////////////////////////////////////////
////////////////////////////////////////
////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedSqsFailedInstanceSender<P: Project> {
    sender: Sender<WorkflowInstance>,
    _marker: PhantomData<P>,
}

impl<P: Project> FailedInstanceSender<P> for EmbeddedSqsFailedInstanceSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: WorkflowInstance) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendWorkflowInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedSqsFailedInstanceSender<P> {
    pub fn new(sender: Sender<WorkflowInstance>) -> Self {
        Self {
            sender,
            _marker: PhantomData,
        }
    }
}

////////////////////////////////////////
////////////////////////////////////////
////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedSqsCompletedInstanceSender<P: Project> {
    sender: Sender<WorkflowInstance>,
    _marker: PhantomData<P>,
}

impl<P: Project> CompletedInstanceSender<P> for EmbeddedSqsCompletedInstanceSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: WorkflowInstance) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendWorkflowInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedSqsCompletedInstanceSender<P> {
    pub fn new(sender: Sender<WorkflowInstance>) -> Self {
        Self {
            sender,
            _marker: PhantomData,
        }
    }
}

////////////////////////////////////////
////////////////////////////////////////
////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedSqsCompletedStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> CompletedStepSender<P> for EmbeddedSqsCompletedStepSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedSqsCompletedStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}
