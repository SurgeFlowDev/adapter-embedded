use async_channel::Receiver;
use surgeflow_types::{FullyQualifiedStep, InstanceEvent, Project, WorkflowInstance};

use adapter_types::receivers::{
    ActiveStepReceiver, CompletedInstanceReceiver, CompletedStepReceiver, EventReceiver,
    FailedInstanceReceiver, FailedStepReceiver, NewInstanceReceiver, NextStepReceiver,
};
use std::marker::PhantomData;

use crate::EmbeddedAdapterError;

#[derive(Debug, Clone)]
pub struct EmbeddedCompletedInstanceReceiver<P: Project> {
    receiver: Receiver<WorkflowInstance<P>>,
    _marker: PhantomData<P>,
}

impl<P: Project> CompletedInstanceReceiver<P> for EmbeddedCompletedInstanceReceiver<P> {
    type Handle = ();
    type Error = EmbeddedAdapterError<P>;
    async fn receive(&mut self) -> Result<(WorkflowInstance<P>, Self::Handle), Self::Error> {
        let workflow_instance = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;

        Ok((workflow_instance, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedCompletedInstanceReceiver<P> {
    pub fn new(receiver: Receiver<WorkflowInstance<P>>) -> Self {
        Self {
            receiver,
            _marker: PhantomData,
        }
    }
}

///////////////////////////////////////
///////////////////////////////////////
///////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedFailedInstanceReceiver<P: Project> {
    receiver: Receiver<WorkflowInstance<P>>,
    _marker: PhantomData<P>,
}

impl<P: Project> FailedInstanceReceiver<P> for EmbeddedFailedInstanceReceiver<P> {
    type Handle = ();
    type Error = EmbeddedAdapterError<P>;
    async fn receive(&mut self) -> Result<(WorkflowInstance<P>, Self::Handle), Self::Error> {
        let workflow_instance = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;

        Ok((workflow_instance, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedFailedInstanceReceiver<P> {
    pub fn new(receiver: Receiver<WorkflowInstance<P>>) -> Self {
        Self {
            receiver,
            _marker: PhantomData,
        }
    }
}

///////////////////////////////////////
///////////////////////////////////////
///////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedNewInstanceReceiver<P: Project> {
    receiver: Receiver<WorkflowInstance<P>>,
    _marker: PhantomData<P>,
}

impl<P: Project> NewInstanceReceiver<P> for EmbeddedNewInstanceReceiver<P> {
    type Handle = ();
    type Error = EmbeddedAdapterError<P>;
    async fn receive(&mut self) -> Result<(WorkflowInstance<P>, Self::Handle), Self::Error> {
        let workflow_instance = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;
        Ok((workflow_instance, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedNewInstanceReceiver<P> {
    pub fn new(receiver: Receiver<WorkflowInstance<P>>) -> Self {
        Self {
            receiver,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedEventReceiver<P: Project> {
    receiver: Receiver<InstanceEvent<P>>,
}

impl<P: Project> EventReceiver<P> for EmbeddedEventReceiver<P> {
    type Error = EmbeddedAdapterError<P>;
    type Handle = ();
    async fn receive(&mut self) -> Result<(InstanceEvent<P>, Self::Handle), Self::Error> {
        let event = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;
        Ok((event, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedEventReceiver<P> {
    pub fn new(receiver: Receiver<InstanceEvent<P>>) -> Self {
        Self { receiver }
    }
}
#[derive(Debug, Clone)]
pub struct EmbeddedNextStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> NextStepReceiver<P> for EmbeddedNextStepReceiver<P> {
    type Error = EmbeddedAdapterError<P>;
    type Handle = ();
    async fn receive(&mut self) -> Result<(FullyQualifiedStep<P>, Self::Handle), Self::Error> {
        let step = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;
        Ok((step, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedNextStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedCompletedStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> CompletedStepReceiver<P> for EmbeddedCompletedStepReceiver<P> {
    type Error = EmbeddedAdapterError<P>;
    type Handle = ();
    async fn receive(&mut self) -> Result<(FullyQualifiedStep<P>, Self::Handle), Self::Error> {
        let step = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;
        Ok((step, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedCompletedStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedFailedStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> FailedStepReceiver<P> for EmbeddedFailedStepReceiver<P> {
    type Error = EmbeddedAdapterError<P>;
    type Handle = ();
    async fn receive(&mut self) -> Result<(FullyQualifiedStep<P>, Self::Handle), Self::Error> {
        let step = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;
        Ok((step, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedFailedStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedActiveStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> ActiveStepReceiver<P> for EmbeddedActiveStepReceiver<P> {
    type Error = EmbeddedAdapterError<P>;
    type Handle = ();
    async fn receive(&mut self) -> Result<(FullyQualifiedStep<P>, Self::Handle), Self::Error> {
        let step = self
            .receiver
            .recv()
            .await
            .map_err(EmbeddedAdapterError::ReceiveMessageError)?;
        Ok((step, ()))
    }

    async fn accept(&mut self, _handle: Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: Project> EmbeddedActiveStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}
