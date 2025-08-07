use async_channel::Receiver;
use surgeflow_types::{FullyQualifiedStep, InstanceEvent, Project, WorkflowInstance};

use adapter_types::receivers::{
    ActiveStepReceiver, CompletedInstanceReceiver, CompletedStepReceiver, EventReceiver,
    FailedInstanceReceiver, FailedStepReceiver, NewInstanceReceiver, NextStepReceiver,
};
use std::marker::PhantomData;

use crate::EmbeddedAdapterError;

#[derive(Debug, Clone)]
pub struct EmbeddedSqsCompletedInstanceReceiver<P: Project> {
    receiver: Receiver<WorkflowInstance>,
    _marker: PhantomData<P>,
}

impl<P: Project> CompletedInstanceReceiver<P> for EmbeddedSqsCompletedInstanceReceiver<P> {
    type Handle = ();
    type Error = EmbeddedAdapterError<P>;
    async fn receive(&mut self) -> Result<(WorkflowInstance, Self::Handle), Self::Error> {
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

impl<P: Project> EmbeddedSqsCompletedInstanceReceiver<P> {
    pub fn new(receiver: Receiver<WorkflowInstance>) -> Self {
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
pub struct EmbeddedSqsFailedInstanceReceiver<P: Project> {
    receiver: Receiver<WorkflowInstance>,
    _marker: PhantomData<P>,
}

impl<P: Project> FailedInstanceReceiver<P> for EmbeddedSqsFailedInstanceReceiver<P> {
    type Handle = ();
    type Error = EmbeddedAdapterError<P>;
    async fn receive(&mut self) -> Result<(WorkflowInstance, Self::Handle), Self::Error> {
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

impl<P: Project> EmbeddedSqsFailedInstanceReceiver<P> {
    pub fn new(receiver: Receiver<WorkflowInstance>) -> Self {
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
pub struct EmbeddedSqsNewInstanceReceiver<P: Project> {
    receiver: Receiver<WorkflowInstance>,
    _marker: PhantomData<P>,
}

impl<P: Project> NewInstanceReceiver<P> for EmbeddedSqsNewInstanceReceiver<P> {
    type Handle = ();
    type Error = EmbeddedAdapterError<P>;
    async fn receive(&mut self) -> Result<(WorkflowInstance, Self::Handle), Self::Error> {
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

impl<P: Project> EmbeddedSqsNewInstanceReceiver<P> {
    pub fn new(receiver: Receiver<WorkflowInstance>) -> Self {
        Self {
            receiver,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedSqsEventReceiver<P: Project> {
    receiver: Receiver<InstanceEvent<P>>,
}

impl<P: Project> EventReceiver<P> for EmbeddedSqsEventReceiver<P> {
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

impl<P: Project> EmbeddedSqsEventReceiver<P> {
    pub fn new(receiver: Receiver<InstanceEvent<P>>) -> Self {
        Self { receiver }
    }
}
#[derive(Debug, Clone)]
pub struct EmbeddedSqsNextStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> NextStepReceiver<P> for EmbeddedSqsNextStepReceiver<P> {
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

impl<P: Project> EmbeddedSqsNextStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedSqsCompletedStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> CompletedStepReceiver<P> for EmbeddedSqsCompletedStepReceiver<P> {
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

impl<P: Project> EmbeddedSqsCompletedStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedSqsFailedStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> FailedStepReceiver<P> for EmbeddedSqsFailedStepReceiver<P> {
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

impl<P: Project> EmbeddedSqsFailedStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct EmbeddedSqsActiveStepReceiver<P: Project> {
    receiver: Receiver<FullyQualifiedStep<P>>,
}

impl<P: Project> ActiveStepReceiver<P> for EmbeddedSqsActiveStepReceiver<P> {
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

impl<P: Project> EmbeddedSqsActiveStepReceiver<P> {
    pub fn new(receiver: Receiver<FullyQualifiedStep<P>>) -> Self {
        Self { receiver }
    }
}
