use derive_more::{From, TryInto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surgeflow::{
    BareEvent, ConvertingProjectEventToWorkflowEventError,
    ConvertingProjectStepToWorkflowStepError, ConvertingWorkflowEventToEventError,
    ConvertingWorkflowStepToStepError, Event, Immediate, Project, ProjectStep, Step, StepSettings,
    SurgeflowWorkflowStepError, TryFromRef, Workflow, WorkflowEvent, WorkflowStep,
    WorkflowStepWithSettings,
};

use crate::workflows::{MyProject, MyProjectEvent};

///////////////

#[derive(Clone, Debug)]
pub struct Workflow2 {}

impl Workflow for Workflow2 {
    type Project = MyProject;

    type Event = Workflow2Event;

    type Step = Workflow2Step;

    const NAME: &'static str = "workflow_2";

    fn entrypoint() -> WorkflowStepWithSettings<Self> {
        WorkflowStepWithSettings {
            step: Workflow2Step::Step0(Step0),
            settings: StepSettings { max_retries: 3 },
        }
    }
}

// pub trait Event:
//     Serialize + for<'a> Deserialize<'a> + Clone + fmt::Debug + Send + JsonSchema + 'static
// {
//     fn value_is<T: Event + 'static>(&self) -> bool;
// }

impl Event for Workflow2Event {
    fn value_is<T: Event + 'static>(&self) -> bool {
        match self {
            Workflow2Event::Event0(ev) => ev.value_is::<T>(),
            Workflow2Event::Immediate(ev) => ev.value_is::<T>(),
        }
    }
}

impl From<SurgeflowWorkflowStepError<Workflow2StepError>> for Workflow2StepError {
    fn from(error: SurgeflowWorkflowStepError<Workflow2StepError>) -> Self {
        todo!();
    }
}

impl From<Workflow2StepError> for SurgeflowWorkflowStepError<Workflow2StepError> {
    fn from(error: Workflow2StepError) -> Self {
        SurgeflowWorkflowStepError::StepError(error)
    }
}

impl Step<Workflow2> for Workflow2Step {
    type Event = Workflow2Event;
    // type Workflow = Workflow2;
    type Error = SurgeflowWorkflowStepError<Workflow2StepError>;

    // Workflow2StepError;

    async fn run(
        &self,
        wf: Workflow2,
        event: Self::Event,
    ) -> Result<Option<WorkflowStepWithSettings<Workflow2>>, <Self as Step<Workflow2>>::Error> {
        let res = match self {
            Workflow2Step::Step0(step) => step
                .run(wf, event.try_into()?)
                .await
                .map_err(Workflow2StepError::Step0)?,
            Workflow2Step::Step1(step) => step
                .run(wf, event.try_into()?)
                .await
                .map_err(Workflow2StepError::Step1)?,
        };
        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum Workflow2Step {
    Step0(Step0),
    Step1(Step1),
}

#[derive(thiserror::Error, Debug)]
pub enum Workflow2StepError {
    #[error("Step0 error: {0}")]
    Step0(<Step0 as Step<Workflow2>>::Error),
    #[error("Step1 error: {0}")]
    Step1(<Step1 as Step<Workflow2>>::Error),
}

//////////// ProjectStep::Error <-> WorkflowStep::Error conversions

impl From<<<Workflow2 as Workflow>::Step as WorkflowStep>::Error>
    for <<<Workflow2 as Workflow>::Project as Project>::Step as ProjectStep>::Error
{
    fn from(error: <<Workflow2 as Workflow>::Step as WorkflowStep>::Error) -> Self {
        <<<Workflow2 as Workflow>::Project as Project>::Step as ProjectStep>::Error::Workflow2(
            error,
        )
    }
}

impl TryFrom<<<MyProject as Project>::Step as ProjectStep>::Error>
    for <<Workflow2 as Workflow>::Step as WorkflowStep>::Error
{
    type Error = ConvertingProjectStepToWorkflowStepError;

    fn try_from(
        error: <<<Workflow2 as Workflow>::Project as Project>::Step as ProjectStep>::Error,
    ) -> Result<Self, Self::Error> {
        type Error = <<MyProject as Project>::Step as ProjectStep>::Error;
        match error {
            Error::Workflow2(e) => Ok(e),
            _ => Err(ConvertingProjectStepToWorkflowStepError),
        }
    }
}

//////////// WorkflowStep::Error <-> Step::Error conversions

impl From<<Step0 as Step<Workflow2>>::Error>
    for <<Workflow2 as Workflow>::Step as WorkflowStep>::Error
{
    fn from(error: <Step0 as Step<Workflow2>>::Error) -> Self {
        SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step0(error))
    }
}

impl From<<Step1 as Step<Workflow2>>::Error>
    for <<Workflow2 as Workflow>::Step as WorkflowStep>::Error
{
    fn from(error: <Step1 as Step<Workflow2>>::Error) -> Self {
        SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step1(error))
    }
}

impl TryFrom<<<Workflow2 as Workflow>::Step as WorkflowStep>::Error>
    for <Step1 as Step<Workflow2>>::Error
{
    type Error = ConvertingWorkflowStepToStepError;

    fn try_from(
        error: <<Workflow2 as Workflow>::Step as WorkflowStep>::Error,
    ) -> Result<Self, Self::Error> {
        match error {
            SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step1(e)) => Ok(e),
            _ => Err(ConvertingWorkflowStepToStepError),
        }
    }
}

impl TryFrom<<<Workflow2 as Workflow>::Step as WorkflowStep>::Error>
    for <Step0 as Step<Workflow2>>::Error
{
    type Error = ConvertingWorkflowStepToStepError;

    fn try_from(
        error: <<Workflow2 as Workflow>::Step as WorkflowStep>::Error,
    ) -> Result<Self, Self::Error> {
        match error {
            SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step0(e)) => Ok(e),
            _ => Err(ConvertingWorkflowStepToStepError),
        }
    }
}

////////////

impl WorkflowStep for Workflow2Step {
    type Workflow = Workflow2;
    type Error = <Self as Step<Workflow2>>::Error;

    // async fn run(
    //     &self,
    //     wf: Self::Workflow,
    //     event: <Self::Workflow as Workflow>::Event,
    //     // TODO: WorkflowStep should not be hardcoded here, but rather there should be a "Workflow" associated type,
    //     // where we can get the WorkflowStep type from
    // ) -> Result<
    //     Option<WorkflowStepWithSettings<Self::Workflow>>,
    //     SurgeflowWorkflowStepError<<Self as WorkflowStep>::Error>,
    // > {
    //     let res = match self {
    //         Workflow2Step::Step0(step) => step
    //             .run(wf, event.try_into()?)
    //             .await
    //             .map_err(|e| SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step0(e)))?,
    //         Workflow2Step::Step1(step) => step
    //             .run(wf, event.try_into()?)
    //             .await
    //             .map_err(|e| SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step1(e)))?,
    //     };
    //     Ok(res)
    // }

    // fn is_event<T: Event + 'static>(&self) -> bool {
    //     match self {
    //         Workflow2Step::Step0(_) => <Step0 as Step<Workflow2>>::Event::is::<T>(),
    //         Workflow2Step::Step1(_) => <Step1 as Step<Workflow2>>::Event::is::<T>(),
    //     }
    // }

    // fn is_workflow_event(&self, event: &<Self::Workflow as Workflow>::Event) -> bool {
    //     match self {
    //         Workflow2Step::Step0(_) => event.is_event::<<Step0 as Step<Workflow2>>::Event>(),
    //         Workflow2Step::Step1(_) => event.is_event::<<Step1 as Step<Workflow2>>::Event>(),
    //     }
    // }
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

impl Step<Workflow2> for Step0 {
    type Event = Event0;
    type Error = Step0Error;

    async fn run(
        &self,
        wf: Workflow2,
        event: Self::Event,
    ) -> Result<Option<WorkflowStepWithSettings<Workflow2>>, <Self as Step<Workflow2>>::Error> {
        tracing::info!("Running Step0 in Workflow2");
        Ok(Some(WorkflowStepWithSettings {
            step: Workflow2Step::Step1(Step1),
            settings: StepSettings { max_retries: 3 },
        }))
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

impl Step<Workflow2> for Step1 {
    type Event = Immediate;
    type Error = Step1Error;

    async fn run(
        &self,
        wf: Workflow2,
        event: Self::Event,
    ) -> Result<Option<WorkflowStepWithSettings<Workflow2>>, <Self as Step<Workflow2>>::Error> {
        tracing::info!("Running Step1 in Workflow2");
        Ok(None)
    }
}

// events

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Workflow2Event {
    Event0(Event0),
    #[serde(skip)]
    Immediate(Immediate),
}

impl WorkflowEvent for Workflow2Event {
    type Workflow = Workflow2;

    // fn is_event<T: Event + 'static>(&self) -> bool {
    //     match self {
    //         Workflow2Event::Event0(_) => Event0::is::<T>(),
    //         Workflow2Event::Immediate(_) => Immediate::is::<T>(),
    //     }
    // }
}

impl TryFrom<Workflow2Event> for Immediate {
    type Error = ConvertingWorkflowEventToEventError;

    fn try_from(event: Workflow2Event) -> Result<Self, Self::Error> {
        match event {
            Workflow2Event::Immediate(immediate) => Ok(immediate),
            _ => Err(ConvertingWorkflowEventToEventError),
        }
    }
}

impl From<Immediate> for Workflow2Event {
    fn from(immediate: Immediate) -> Self {
        Workflow2Event::Immediate(immediate)
    }
}

impl TryFrom<Workflow2Event> for Event0 {
    type Error = ConvertingWorkflowEventToEventError;

    fn try_from(event: Workflow2Event) -> Result<Self, Self::Error> {
        if let Workflow2Event::Event0(event0) = event {
            Ok(event0)
        } else {
            Err(ConvertingWorkflowEventToEventError)
        }
    }
}

impl TryFrom<MyProjectEvent> for Workflow2Event {
    type Error = ConvertingProjectEventToWorkflowEventError;

    fn try_from(event: MyProjectEvent) -> Result<Self, Self::Error> {
        match event {
            MyProjectEvent::Workflow2(workflow_event) => Ok(workflow_event),
            MyProjectEvent::Immediate(immediate) => Ok(Workflow2Event::Immediate(immediate)),
            _ => Err(ConvertingProjectEventToWorkflowEventError),
        }
    }
}

impl TryFromRef<MyProjectEvent> for Workflow2Event {
    type Error = ConvertingProjectEventToWorkflowEventError;

    fn try_from_ref(event: &MyProjectEvent) -> Result<&Self, Self::Error> {
        match event {
            MyProjectEvent::Workflow2(workflow_event) => Ok(workflow_event),
            // TODO: why is this an error?
            MyProjectEvent::Immediate(_) => Err(ConvertingProjectEventToWorkflowEventError),
            _ => Err(ConvertingProjectEventToWorkflowEventError),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Event0 {}

impl From<Event0> for Workflow2Event {
    fn from(event: Event0) -> Self {
        Workflow2Event::Event0(event)
    }
}

impl BareEvent for Event0 {}
