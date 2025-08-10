use derive_more::{From, TryInto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surgeflow::{
    ConvertingProjectEventToWorkflowEventError, ConvertingProjectStepToWorkflowStepError,
    ConvertingWorkflowEventToEventError, ConvertingWorkflowStepToStepError, Event, Immediate,
    Project, ProjectStep, Step, StepResult, SurgeflowWorkflowStepError, TryFromRef, Workflow,
    WorkflowEvent, WorkflowStep, WorkflowStepWithSettings, next_step, workflow,
};

use crate::workflows::{MyProject, MyProjectEvent};

///////////////

#[derive(Clone, Debug)]
pub struct Workflow1 {}

#[workflow]
impl Workflow for Workflow1 {
    type Project = MyProject;
    type Step = step!(Step0, Step1);
    type Event = event!(Event0);
    const NAME: &'static str = "workflow_1";

    fn entrypoint() -> WorkflowStepWithSettings<Self> {
        next_step(Step0).max_retries(3).call()
    }
}

//////////// ProjectStep::Error <-> WorkflowStep::Error conversions

impl From<<<Workflow1 as Workflow>::Step as WorkflowStep>::Error>
    for <<<Workflow1 as Workflow>::Project as Project>::Step as ProjectStep>::Error
{
    fn from(error: <<Workflow1 as Workflow>::Step as WorkflowStep>::Error) -> Self {
        <<<Workflow1 as Workflow>::Project as Project>::Step as ProjectStep>::Error::Workflow1(
            error,
        )
    }
}

impl TryFrom<<<MyProject as Project>::Step as ProjectStep>::Error>
    for <<Workflow1 as Workflow>::Step as WorkflowStep>::Error
{
    type Error = ConvertingProjectStepToWorkflowStepError;

    fn try_from(
        error: <<<Workflow1 as Workflow>::Project as Project>::Step as ProjectStep>::Error,
    ) -> Result<Self, Self::Error> {
        type Error = <<MyProject as Project>::Step as ProjectStep>::Error;
        match error {
            Error::Workflow1(e) => Ok(e),
            _ => Err(ConvertingProjectStepToWorkflowStepError),
        }
    }
}

////////////////////////////////////////////////////////////////////

impl WorkflowStep for Workflow1Step {
    type Workflow = Workflow1;
    type Error = Workflow1StepError;

    async fn run(
        &self,
        wf: Self::Workflow,
        event: <Self::Workflow as Workflow>::Event,
    ) -> Result<
        Option<WorkflowStepWithSettings<Self::Workflow>>,
        SurgeflowWorkflowStepError<<Self as WorkflowStep>::Error>,
    > {
        let res = match self {
            Workflow1Step::A(step) => step
                .run(wf, event.try_into()?)
                .await
                .map_err(|e| SurgeflowWorkflowStepError::StepError(Workflow1StepError::A(e)))?,
            Workflow1Step::B(step) => step
                .run(wf, event.try_into()?)
                .await
                .map_err(|e| SurgeflowWorkflowStepError::StepError(Workflow1StepError::B(e)))?,
        };
        Ok(res)
    }

    fn is_event<T: Event + 'static>(&self) -> bool {
        match self {
            Workflow1Step::A(step) => step.value_has_event::<T>(),
            Workflow1Step::B(step) => step.value_has_event::<T>(),
        }
    }

    fn is_workflow_event(&self, event: &<Self::Workflow as Workflow>::Event) -> bool {
        match self {
            Workflow1Step::A(step) => step.value_has_event::<<Step0 as Step>::Event>(),

            Workflow1Step::B(_) => event.is_event::<<Step1 as Step>::Event>(),
        }
    }
}

impl Event for Workflow1Event {}

impl Step for Workflow1Step {
    type Event = Workflow1Event;
    type Workflow = Workflow1;
    type Error = Workflow1StepError;

    async fn run(
        &self,
        wf: Self::Workflow,
        event: Self::Event,
    ) -> Result<Option<WorkflowStepWithSettings<Self::Workflow>>, <Self as Step>::Error> {
        match self {
            Workflow1Step::A(step) => step
                .run(wf, event.try_into().unwrap())
                .await
                .map_err(Workflow1StepError::A),
            Workflow1Step::B(step) => step
                .run(wf, event.try_into().unwrap())
                .await
                .map_err(Workflow1StepError::B),
        }
    }
}

///// Steps

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Step0;

#[derive(thiserror::Error, Debug)]
pub enum Step0Error {
    // TODO
    #[error("Step0 error")]
    Unknown,
}

impl Step for Step0 {
    type Event = Event0;
    type Workflow = Workflow1;
    type Error = Step0Error;

    async fn run(&self, wf: Self::Workflow, event: Self::Event) -> StepResult<Self> {
        tracing::info!("Running Step0 in Workflow1");
        Ok(Some(next_step(Step1).max_retries(3).call()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Step1;

#[derive(thiserror::Error, Debug)]
pub enum Step1Error {
    // TODO
    #[error("Step1 error")]
    Unknown,
}

impl Step for Step1 {
    type Event = Immediate;
    type Workflow = Workflow1;
    type Error = Step1Error;

    async fn run(
        &self,
        wf: Self::Workflow,
        event: Self::Event,
    ) -> Result<Option<WorkflowStepWithSettings<Self::Workflow>>, <Self as Step>::Error> {
        tracing::info!("Running Step1 in Workflow1");
        Ok(None)
    }
}

// events

impl WorkflowEvent for Workflow1Event {
    type Workflow = Workflow1;

    fn is_event<T: Event + 'static>(&self) -> bool {
        match self {
            Workflow1Event::A(_) => Event0::is::<T>(),
            Workflow1Event::Immediate(_) => Immediate::is::<T>(),
        }
    }
}

impl TryFrom<Workflow1Event> for Immediate {
    type Error = ConvertingWorkflowEventToEventError;

    fn try_from(event: Workflow1Event) -> Result<Self, Self::Error> {
        match event {
            Workflow1Event::Immediate(immediate) => Ok(immediate),
            _ => Err(ConvertingWorkflowEventToEventError),
        }
    }
}

impl From<Immediate> for Workflow1Event {
    fn from(immediate: Immediate) -> Self {
        Workflow1Event::Immediate(immediate)
    }
}

impl TryFrom<Workflow1Event> for Event0 {
    type Error = ConvertingWorkflowEventToEventError;

    fn try_from(event: Workflow1Event) -> Result<Self, Self::Error> {
        if let Workflow1Event::A(event0) = event {
            Ok(event0)
        } else {
            Err(ConvertingWorkflowEventToEventError)
        }
    }
}

impl TryFrom<MyProjectEvent> for Workflow1Event {
    type Error = ConvertingProjectEventToWorkflowEventError;

    fn try_from(event: MyProjectEvent) -> Result<Self, Self::Error> {
        match event {
            MyProjectEvent::Workflow1(workflow_event) => Ok(workflow_event),
            MyProjectEvent::Immediate(immediate) => Ok(Workflow1Event::Immediate(immediate)),
            _ => Err(ConvertingProjectEventToWorkflowEventError),
        }
    }
}

impl TryFromRef<MyProjectEvent> for Workflow1Event {
    type Error = ConvertingProjectEventToWorkflowEventError;

    fn try_from_ref(event: &MyProjectEvent) -> Result<&Self, Self::Error> {
        match event {
            MyProjectEvent::Workflow1(workflow_event) => Ok(workflow_event),
            // TODO: why is this an error?
            MyProjectEvent::Immediate(_) => Err(ConvertingProjectEventToWorkflowEventError),
            _ => Err(ConvertingProjectEventToWorkflowEventError),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Event0 {}

impl From<Event0> for Workflow1Event {
    fn from(event: Event0) -> Self {
        Workflow1Event::A(event)
    }
}

impl Event for Event0 {}
