// use derive_more::{From, TryInto};
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
// use surgeflow::{
//     __Event, __Step, __Workflow, ConvertingProjectEventToWorkflowEventError,
//     ConvertingProjectStepToWorkflowStepError, ConvertingWorkflowEventToEventError,
//     ConvertingWorkflowStepToStepError, Event, Immediate, Project, StepSettings, StepWithSettings,
//     SurgeflowWorkflowStepError, TryFromRef, Workflow,
// };

// use crate::workflows::{MyProject, MyProjectEvent};

// ///////////////

// #[derive(Clone, Debug)]
// pub struct Workflow2 {}

// impl Workflow<MyProject> for Workflow2 {
//     type Event = Workflow2Event;

//     type Step = Workflow2Step;

//     type Error = TempError;

//     const NAME: &'static str = "workflow_2";

//     fn entrypoint() -> StepWithSettings<MyProject> {
//         StepWithSettings {
//             step: Workflow2Step::Step0(Step0),
//             settings: StepSettings { max_retries: 3 },
//         }
//     }
// }

// // pub trait Event:
// //     Serialize + for<'a> Deserialize<'a> + Clone + fmt::Debug + Send + JsonSchema + 'static
// // {
// //     fn value_is<T: Event + 'static>(&self) -> bool;
// // }

// impl __Event<MyProject, Workflow2> for Workflow2Event {
//     fn value_is<T: __Event<MyProject, Workflow2> + 'static>(&self) -> bool {
//         match self {
//             Workflow2Event::Event0(ev) => ev.value_is::<T>(),
//             Workflow2Event::Immediate(ev) => ev.value_is::<T>(),
//         }
//     }
// }

// impl From<SurgeflowWorkflowStepError<Workflow2StepError>> for Workflow2StepError {
//     fn from(error: SurgeflowWorkflowStepError<Workflow2StepError>) -> Self {
//         todo!();
//     }
// }

// impl From<Workflow2StepError> for SurgeflowWorkflowStepError<Workflow2StepError> {
//     fn from(error: Workflow2StepError) -> Self {
//         SurgeflowWorkflowStepError::StepError(error)
//     }
// }

// impl __Step<MyProject, Workflow2> for Workflow2Step {
//     type Event = Workflow2Event;
//     // type Workflow = Workflow2;
//     type Error = SurgeflowWorkflowStepError<Workflow2StepError>;

//     // Workflow2StepError;

//     async fn run(
//         &self,
//         wf: Workflow2,
//         event: Self::Event,
//     ) -> Result<Option<StepWithSettings<MyProject>>, <Self as __Step<MyProject, Workflow2>>::Error>
//     {
//         let res = match self {
//             Workflow2Step::Step0(step) => step
//                 .run(wf, event.try_into()?)
//                 .await
//                 .map_err(Workflow2StepError::Step0)?,
//             Workflow2Step::Step1(step) => step
//                 .run(wf, event.try_into()?)
//                 .await
//                 .map_err(Workflow2StepError::Step1)?,
//         };
//         Ok(res)
//         }

//     fn value_has_event_value<T: __Event<MyProject, Workflow2> + 'static>(&self, e: &T) -> bool {
//         e.value_is::<Self::Event>()
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
// pub enum Workflow2Step {
//     Step0(Step0),
//     Step1(Step1),
// }

// #[derive(thiserror::Error, Debug)]
// pub enum Workflow2StepError {
//     #[error("Step0 error: {0}")]
//     Step0(<Step0 as Step<Workflow2>>::Error),
//     #[error("Step1 error: {0}")]
//     Step1(<Step1 as Step<Workflow2>>::Error),
// }

// //////////// ProjectStep::Error <-> WorkflowStep::Error conversions

// impl From<<<Workflow2 as Workflow>::Step as WorkflowStep>::Error>
//     for <<<Workflow2 as Workflow>::Project as Project>::Step as ProjectStep>::Error
// {
//     fn from(error: <<Workflow2 as Workflow>::Step as WorkflowStep>::Error) -> Self {
//         <<<Workflow2 as Workflow>::Project as Project>::Step as ProjectStep>::Error::Workflow2(
//             error,
//         )
//     }
// }

// impl TryFrom<<<MyProject as Project>::Step as ProjectStep>::Error>
//     for <<Workflow2 as Workflow>::Step as WorkflowStep>::Error
// {
//     type Error = ConvertingProjectStepToWorkflowStepError;

//     fn try_from(
//         error: <<<Workflow2 as Workflow>::Project as Project>::Step as ProjectStep>::Error,
//     ) -> Result<Self, Self::Error> {
//         type Error = <<MyProject as Project>::Step as ProjectStep>::Error;
//         match error {
//             Error::Workflow2(e) => Ok(e),
//             _ => Err(ConvertingProjectStepToWorkflowStepError),
//         }
//     }
// }

// //////////// WorkflowStep::Error <-> Step::Error conversions

// impl From<<Step0 as Step<Workflow2>>::Error>
//     for <<Workflow2 as Workflow>::Step as WorkflowStep>::Error
// {
//     fn from(error: <Step0 as Step<Workflow2>>::Error) -> Self {
//         SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step0(error))
//     }
// }

// impl From<<Step1 as Step<Workflow2>>::Error>
//     for <<Workflow2 as Workflow>::Step as WorkflowStep>::Error
// {
//     fn from(error: <Step1 as Step<Workflow2>>::Error) -> Self {
//         SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step1(error))
//     }
// }

// impl TryFrom<<<Workflow2 as Workflow>::Step as WorkflowStep>::Error>
//     for <Step1 as Step<Workflow2>>::Error
// {
//     type Error = ConvertingWorkflowStepToStepError;

//     fn try_from(
//         error: <<Workflow2 as Workflow>::Step as WorkflowStep>::Error,
//     ) -> Result<Self, Self::Error> {
//         match error {
//             SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step1(e)) => Ok(e),
//             _ => Err(ConvertingWorkflowStepToStepError),
//         }
//     }
// }

// impl TryFrom<<<Workflow2 as Workflow>::Step as WorkflowStep>::Error>
//     for <Step0 as Step<Workflow2>>::Error
// {
//     type Error = ConvertingWorkflowStepToStepError;

//     fn try_from(
//         error: <<Workflow2 as Workflow>::Step as WorkflowStep>::Error,
//     ) -> Result<Self, Self::Error> {
//         match error {
//             SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step0(e)) => Ok(e),
//             _ => Err(ConvertingWorkflowStepToStepError),
//         }
//     }
// }

// ////////////

// impl WorkflowStep for Workflow2Step {
//     type Workflow = Workflow2;
//     type Error = <Self as Step<Workflow2>>::Error;

//     // async fn run(
//     //     &self,
//     //     wf: Self::Workflow,
//     //     event: <Self::Workflow as Workflow>::Event,
//     //     // TODO: WorkflowStep should not be hardcoded here, but rather there should be a "Workflow" associated type,
//     //     // where we can get the WorkflowStep type from
//     // ) -> Result<
//     //     Option<WorkflowStepWithSettings<Self::Workflow>>,
//     //     SurgeflowWorkflowStepError<<Self as WorkflowStep>::Error>,
//     // > {
//     //     let res = match self {
//     //         Workflow2Step::Step0(step) => step
//     //             .run(wf, event.try_into()?)
//     //             .await
//     //             .map_err(|e| SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step0(e)))?,
//     //         Workflow2Step::Step1(step) => step
//     //             .run(wf, event.try_into()?)
//     //             .await
//     //             .map_err(|e| SurgeflowWorkflowStepError::StepError(Workflow2StepError::Step1(e)))?,
//     //     };
//     //     Ok(res)
//     // }

//     // fn is_event<T: Event + 'static>(&self) -> bool {
//     //     match self {
//     //         Workflow2Step::Step0(_) => <Step0 as Step<Workflow2>>::Event::is::<T>(),
//     //         Workflow2Step::Step1(_) => <Step1 as Step<Workflow2>>::Event::is::<T>(),
//     //     }
//     // }

//     // fn is_workflow_event(&self, event: &<Self::Workflow as Workflow>::Event) -> bool {
//     //     match self {
//     //         Workflow2Step::Step0(_) => event.is_event::<<Step0 as Step<Workflow2>>::Event>(),
//     //         Workflow2Step::Step1(_) => event.is_event::<<Step1 as Step<Workflow2>>::Event>(),
//     //     }
//     // }
// }

// ///// Steps

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct Step0;

// #[derive(thiserror::Error, Debug)]
// pub enum Step0Error {
//     // TODO
//     #[error("Step0 error")]
//     Unknown,
// }

// impl Step<Workflow2> for Step0 {
//     type Event = Event0;
//     type Error = Step0Error;

//     async fn run(
//         &self,
//         wf: Workflow2,
//         event: Self::Event,
//     ) -> Result<Option<WorkflowStepWithSettings<Workflow2>>, <Self as Step<Workflow2>>::Error> {
//         tracing::info!("Running Step0 in Workflow2");
//         Ok(Some(WorkflowStepWithSettings {
//             step: Workflow2Step::Step1(Step1),
//             settings: StepSettings { max_retries: 3 },
//         }))
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct Step1;

// #[derive(thiserror::Error, Debug)]
// pub enum Step1Error {
//     // TODO
//     #[error("Step1 error")]
//     Unknown,
// }

// impl Step<Workflow2> for Step1 {
//     type Event = Immediate;
//     type Error = Step1Error;

//     async fn run(
//         &self,
//         wf: Workflow2,
//         event: Self::Event,
//     ) -> Result<Option<WorkflowStepWithSettings<Workflow2>>, <Self as Step<Workflow2>>::Error> {
//         tracing::info!("Running Step1 in Workflow2");
//         Ok(None)
//     }
// }

// // events

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub enum Workflow2Event {
//     Event0(Event0),
//     #[serde(skip)]
//     Immediate(Immediate),
// }

// impl WorkflowEvent for Workflow2Event {
//     type Workflow = Workflow2;

//     // fn is_event<T: Event + 'static>(&self) -> bool {
//     //     match self {
//     //         Workflow2Event::Event0(_) => Event0::is::<T>(),
//     //         Workflow2Event::Immediate(_) => Immediate::is::<T>(),
//     //     }
//     // }
// }

// impl TryFrom<Workflow2Event> for Immediate {
//     type Error = ConvertingWorkflowEventToEventError;

//     fn try_from(event: Workflow2Event) -> Result<Self, Self::Error> {
//         match event {
//             Workflow2Event::Immediate(immediate) => Ok(immediate),
//             _ => Err(ConvertingWorkflowEventToEventError),
//         }
//     }
// }

// impl From<Immediate> for Workflow2Event {
//     fn from(immediate: Immediate) -> Self {
//         Workflow2Event::Immediate(immediate)
//     }
// }

// impl TryFrom<Workflow2Event> for Event0 {
//     type Error = ConvertingWorkflowEventToEventError;

//     fn try_from(event: Workflow2Event) -> Result<Self, Self::Error> {
//         if let Workflow2Event::Event0(event0) = event {
//             Ok(event0)
//         } else {
//             Err(ConvertingWorkflowEventToEventError)
//         }
//     }
// }

// impl TryFrom<MyProjectEvent> for Workflow2Event {
//     type Error = ConvertingProjectEventToWorkflowEventError;

//     fn try_from(event: MyProjectEvent) -> Result<Self, Self::Error> {
//         match event {
//             MyProjectEvent::Workflow2(workflow_event) => Ok(workflow_event),
//             MyProjectEvent::Immediate(immediate) => Ok(Workflow2Event::Immediate(immediate)),
//             _ => Err(ConvertingProjectEventToWorkflowEventError),
//         }
//     }
// }

// impl TryFromRef<MyProjectEvent> for Workflow2Event {
//     type Error = ConvertingProjectEventToWorkflowEventError;

//     fn try_from_ref(event: &MyProjectEvent) -> Result<&Self, Self::Error> {
//         match event {
//             MyProjectEvent::Workflow2(workflow_event) => Ok(workflow_event),
//             // TODO: why is this an error?
//             MyProjectEvent::Immediate(_) => Err(ConvertingProjectEventToWorkflowEventError),
//             _ => Err(ConvertingProjectEventToWorkflowEventError),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct Event0 {}

// impl From<Event0> for Workflow2Event {
//     fn from(event: Event0) -> Self {
//         Workflow2Event::Event0(event)
//     }
// }

// impl Event for Event0 {}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surgeflow::{
    __Event, __Step, __Workflow, Event, Immediate, Project, Step, StepWithSettings, Workflow,
};

#[derive(thiserror::Error, Debug)]
#[error("Temporary error")]
struct TempError;

impl From<Immediate> for TempError {
    fn from(_: Immediate) -> Self {
        TempError
    }
}

impl From<TempError> for Immediate {
    fn from(_: TempError) -> Self {
        Immediate
    }
}

#[derive(Debug, Clone)]
struct MyProject;

impl Project for MyProject {
    type Step = MyProjectStep;

    type Event = MyProjectEvent;

    type Workflow = MyProjectWorkflow;

    type Error = TempError;

    fn workflow_for_step(&self, step: &Self::Step) -> Self::Workflow {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Copy)]
enum MyProjectWorkflow {
    Workflow1,
}

impl From<MyProjectWorkflow> for Workflow1 {
    fn from(_: MyProjectWorkflow) -> Self {
        todo!()
    }
}

impl __Workflow<MyProject> for MyProjectWorkflow {
    type Event = MyProjectEvent;
    type Step = MyProjectStep;
    type Error = TempError;

    const NAME: &'static str = "my_project_workflow";

    fn entrypoint(&self) -> StepWithSettings<MyProject> {
        todo!()
    }

    fn project_workflow(&self) -> <MyProject as Project>::Workflow {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
enum MyProjectStep {
    Workflow1(<Workflow1 as Workflow<MyProject>>::Step),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
enum MyProjectEvent {
    Workflow1(<Workflow1 as Workflow<MyProject>>::Event),
}

impl From<Immediate> for MyProjectEvent {
    fn from(immediate: Immediate) -> Self {
        todo!()
    }
}

impl __Event<MyProject, MyProjectWorkflow> for MyProjectEvent {
    fn value_is<WInner: Workflow<MyProject>, T: Event<MyProject, WInner> + 'static>(&self) -> bool {
        match self {
            MyProjectEvent::Workflow1(event) => event.value_is::<WInner, T>(),
        }
    }
}

impl __Step<MyProject, MyProjectWorkflow> for MyProjectStep {
    type Event = MyProjectEvent;
    type Error = TempError;

    async fn run(
        &self,
        wf: MyProjectWorkflow,
        event: Self::Event,
    ) -> Result<Option<StepWithSettings<MyProject>>, Self::Error> {
        match self {
            MyProjectStep::Workflow1(step) => {
                step.run(wf.try_into().unwrap(), event.try_into().unwrap())
                    .await
            }
        }
    }
    
    fn value_has_event_value<T: __Event<MyProject, MyProjectWorkflow> + 'static>(&self, e: &T) -> bool {
        todo!()
        // match self {
        //     MyProjectStep::Workflow1(step) => step.value_has_event_value(e),
        // }
    }
}

impl TryFrom<MyProjectEvent> for Workflow1Event {
    type Error = TempError;

    fn try_from(value: MyProjectEvent) -> Result<Self, Self::Error> {
        match value {
            MyProjectEvent::Workflow1(event) => Ok(event),
        }
    }
}

impl From<Workflow1Event> for MyProjectEvent {
    fn from(event: Workflow1Event) -> Self {
        MyProjectEvent::Workflow1(event)
    }
}

#[derive(Debug, Clone)]
struct Workflow1;

impl Workflow<MyProject> for Workflow1 {
    type Event = Workflow1Event;

    type Step = Workflow1Step;

    type Error = TempError;

    const NAME: &'static str = "workflow_1";

    fn entrypoint() -> StepWithSettings<MyProject> {
        todo!()
    }

    fn project_workflow() -> <MyProject as Project>::Workflow {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
enum Workflow1Event {
    Step0(<Step0 as __Step<MyProject, Workflow1>>::Event),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
enum Workflow1Step {
    Step0(Step0),
}

impl __Step<MyProject, Workflow1> for Workflow1Step {
    type Event = Workflow1Event;
    type Error = TempError;

    async fn run(
        &self,
        _wf: Workflow1,
        _event: Self::Event,
    ) -> Result<Option<StepWithSettings<MyProject>>, Self::Error> {
        match self {
            Workflow1Step::Step0(step) => {
                step.run(Workflow1, Immediate).await.map_err(|_| TempError)
            }
        }
    }
    
    fn value_has_event_value<T: __Event<MyProject, Workflow1> + 'static>(&self, e: &T) -> bool {
        todo!()
    }
}

impl __Event<MyProject, Workflow1> for Workflow1Event {
    fn value_is<WInner: Workflow<MyProject>, T: Event<MyProject, WInner> + 'static>(&self) -> bool {
        match self {
            Workflow1Event::Step0(event) => <Immediate as __Event<MyProject, WInner>>::value_is::<WInner, T>(event),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Step0;

impl Step<MyProject, Workflow1> for Step0 {
    type Event = Immediate;
    type Error = TempError;
}

impl __Step<MyProject, Workflow1> for Step0 {
    type Event = Immediate;
    type Error = TempError;

    async fn run(
        &self,
        _wf: Workflow1,
        _event: Self::Event,
    ) -> Result<Option<StepWithSettings<MyProject>>, Self::Error> {
        Ok(None)
    }
    
    fn value_has_event_value<T: __Event<MyProject, Workflow1> + 'static>(&self, e: &T) -> bool {
        todo!()
    }
}
