use async_channel::Sender;
use std::marker::PhantomData;

use surgeflow_types::{FullyQualifiedStep, InstanceEvent, Project, WorkflowInstance};

use adapter_types::senders::{
    ActiveStepSender, CompletedInstanceSender, CompletedStepSender, EventSender,
    FailedInstanceSender, FailedStepSender, NewInstanceSender, NextStepSender,
};

use crate::EmbeddedAdapterError;

#[derive(Debug, Clone)]
pub struct EmbeddedNextStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> NextStepSender<P> for EmbeddedNextStepSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedNextStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedActiveStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> ActiveStepSender<P> for EmbeddedActiveStepSender<P> {
    type Error = EmbeddedAdapterError<P>;
    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedActiveStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedFailedStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> FailedStepSender<P> for EmbeddedFailedStepSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedFailedStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedEventSender<P: Project> {
    sender: Sender<InstanceEvent<P>>,
}

impl<P: Project> EventSender<P> for EmbeddedEventSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: InstanceEvent<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedEventSender<P> {
    pub fn new(sender: Sender<InstanceEvent<P>>) -> Self {
        Self { sender }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedNewInstanceSender<P: Project> {
    sender: Sender<WorkflowInstance<P>>,
    _marker: PhantomData<P>,
}

impl<P: Project> NewInstanceSender<P> for EmbeddedNewInstanceSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: WorkflowInstance<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendWorkflowInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedNewInstanceSender<P> {
    pub fn new(sender: Sender<WorkflowInstance<P>>) -> Self {
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
pub struct EmbeddedFailedInstanceSender<P: Project> {
    sender: Sender<WorkflowInstance<P>>,
    _marker: PhantomData<P>,
}

impl<P: Project> FailedInstanceSender<P> for EmbeddedFailedInstanceSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: WorkflowInstance<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendWorkflowInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedFailedInstanceSender<P> {
    pub fn new(sender: Sender<WorkflowInstance<P>>) -> Self {
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
pub struct EmbeddedCompletedInstanceSender<P: Project> {
    sender: Sender<WorkflowInstance<P>>,
    _marker: PhantomData<P>,
}

impl<P: Project> CompletedInstanceSender<P> for EmbeddedCompletedInstanceSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&self, step: WorkflowInstance<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendWorkflowInstanceEventError)?;

        Ok(())
    }
}

impl<P: Project> EmbeddedCompletedInstanceSender<P> {
    pub fn new(sender: Sender<WorkflowInstance<P>>) -> Self {
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
pub struct EmbeddedCompletedStepSender<P: Project> {
    sender: Sender<FullyQualifiedStep<P>>,
}

impl<P: Project> CompletedStepSender<P> for EmbeddedCompletedStepSender<P> {
    type Error = EmbeddedAdapterError<P>;

    async fn send(&mut self, step: FullyQualifiedStep<P>) -> Result<(), Self::Error> {
        self.sender
            .send(step)
            .await
            .map_err(EmbeddedAdapterError::SendStepError)?;
        Ok(())
    }
}

impl<P: Project> EmbeddedCompletedStepSender<P> {
    pub fn new(sender: Sender<FullyQualifiedStep<P>>) -> Self {
        Self { sender }
    }
}
