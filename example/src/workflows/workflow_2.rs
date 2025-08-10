use derive_more::{From, TryInto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surgeflow::{
    __Event, __Step, __Workflow, Immediate, Project, StepWithSettings, TryAsRef, TryFromRef,
    Workflow, next_step,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct MyProject;

impl Project for MyProject {
    type Workflow = ProjectWorkflow;

    fn workflow_for_step(
        &self,
        step: &<Self::Workflow as __Workflow<Self>>::Step,
    ) -> Self::Workflow {
        todo!()
    }

    fn workflow<T: __Workflow<Self>>() -> Self::Workflow {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
enum ProjectWorkflow {
    Workflow1(MyWorkflow),
}

impl __Workflow<MyProject> for ProjectWorkflow {
    type Step = MyProjectWorkflowStep;

    fn entrypoint(&self) -> StepWithSettings<MyProject> {
        match self {
            ProjectWorkflow::Workflow1(my_workflow) => my_workflow.entrypoint(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            ProjectWorkflow::Workflow1(my_workflow) => my_workflow.name(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
enum MyProjectWorkflowStep {
    Workflow1(<MyWorkflow as __Workflow<MyProject>>::Step),
    // Immediate(Immediate),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
enum MyProjectWorkflowStepEvent {
    Workflow1(
        <<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event,
    ),
    Immediate(Immediate),
}
impl TryFromRef<MyProjectWorkflowStepEvent> for Immediate {
    type Error = TempError;

    fn try_from_ref(value: &MyProjectWorkflowStepEvent) -> Result<&Self, Self::Error> {
        match value {
            MyProjectWorkflowStepEvent::Workflow1(event) => Ok(event.try_as_ref().expect("TODO")),
            MyProjectWorkflowStepEvent::Immediate(immediate) => Ok(immediate),
        }
    }
}

impl TryFromRef<MyProjectWorkflowStepEvent>
    for <<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event
{
    type Error = TempError;

    fn try_from_ref(value: &MyProjectWorkflowStepEvent) -> Result<&Self, Self::Error> {
        match value {
            MyProjectWorkflowStepEvent::Workflow1(event) => Ok(event),
            MyProjectWorkflowStepEvent::Immediate(immediate) => Err(TempError),
        }
    }
}

impl __Event<MyProject, ProjectWorkflow> for MyProjectWorkflowStepEvent {
    fn value_is<WInner: __Workflow<MyProject>, T: __Event<MyProject, WInner> + 'static>(
        &self,
    ) -> bool {
        match self {
            MyProjectWorkflowStepEvent::Workflow1(event) => event.value_is::<WInner, T>(),
            MyProjectWorkflowStepEvent::Immediate(immediate) => {
                <Immediate as __Event<MyProject, WInner>>::value_is::<WInner, T>(immediate)
            }
        }
    }
}

// TODO: create a "FromRef" trait that auto implements this?
impl TryFromRef<MyProjectWorkflowStepEvent> for MyProjectWorkflowStepEvent {
    type Error = TempError;

    fn try_from_ref(value: &MyProjectWorkflowStepEvent) -> Result<&Self, Self::Error> {
        Ok(value)
    }
}

impl __Step<MyProject, ProjectWorkflow> for MyProjectWorkflowStep {
    type Event = MyProjectWorkflowStepEvent;

    type Error = TempError;

    async fn run(
        &self,
        wf: ProjectWorkflow,
        event: Self::Event,
    ) -> Result<
        Option<StepWithSettings<MyProject>>,
        <Self as __Step<MyProject, ProjectWorkflow>>::Error,
    > {
        match self {
            MyProjectWorkflowStep::Workflow1(workflow) => {
                workflow
                    .run(wf.try_into().unwrap(), event.try_into().unwrap())
                    .await
            }
        }
    }

    fn value_has_event_value(&self, e: &Self::Event) -> bool {
        match self {
            MyProjectWorkflowStep::Workflow1(workflow) => {
                let event = match e.try_as_ref() {
                    Ok(event) => event,
                    Err(_) => return false,
                };
                workflow.value_has_event_value(event)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct MyWorkflow;

impl Workflow<MyProject> for MyWorkflow {
    const NAME: &'static str = "MyWorkflow";
    type Step = MyWorkflowStep;

    fn entrypoint() -> StepWithSettings<MyProject> {
        let step = <Self as __Workflow<MyProject>>::Step::from(MyWorkflowStep::Step0(MyStep));
        next_step(step).max_retries(0).call()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
enum MyWorkflowStep {
    Step0(MyStep),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
enum MyWorkflowStepEvent {
    Immediate(Immediate),
}
impl TryFromRef<MyWorkflowStepEvent> for Immediate {
    type Error = TempError;

    fn try_from_ref(value: &MyWorkflowStepEvent) -> Result<&Self, Self::Error> {
        match value {
            MyWorkflowStepEvent::Immediate(immediate) => Ok(immediate),
        }
    }
}

impl TryFromRef<MyWorkflowStepEvent> for MyWorkflowStepEvent {
    type Error = TempError;

    fn try_from_ref(value: &MyWorkflowStepEvent) -> Result<&Self, Self::Error> {
        Ok(value)
    }
}

impl __Event<MyProject, MyWorkflow> for MyWorkflowStepEvent {
    fn value_is<WInner: __Workflow<MyProject>, T: __Event<MyProject, WInner> + 'static>(
        &self,
    ) -> bool {
        match self {
            MyWorkflowStepEvent::Immediate(immediate) => {
                <Immediate as __Event<MyProject, WInner>>::value_is::<WInner, T>(immediate)
            }
        }
    }
}

impl __Step<MyProject, MyWorkflow> for MyWorkflowStep {
    type Event = MyWorkflowStepEvent;

    type Error = TempError;

    async fn run(
        &self,
        wf: MyWorkflow,
        event: Self::Event,
    ) -> Result<Option<StepWithSettings<MyProject>>, <Self as __Step<MyProject, MyWorkflow>>::Error>
    {
        match self {
            MyWorkflowStep::Step0(step) => {
                let event = match event.try_into() {
                    Ok(event) => event,
                    Err(_) => return Err(TempError),
                };
                step.run(wf, event).await
            }
        }
    }

    fn value_has_event_value(&self, e: &Self::Event) -> bool {
        match self {
            MyWorkflowStep::Step0(step) => {
                let event = match e.try_as_ref() {
                    Ok(event) => event,
                    Err(_) => return false,
                };
                step.value_has_event_value(event)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct MyStep;

impl __Step<MyProject, MyWorkflow> for MyStep {
    type Event = Immediate;

    type Error = TempError;

    async fn run(
        &self,
        wf: MyWorkflow,
        event: Self::Event,
    ) -> Result<Option<StepWithSettings<MyProject>>, <Self as __Step<MyProject, MyWorkflow>>::Error>
    {
        tracing::info!("Running MyStep with event: {:?}", event);
        Ok(None)
    }

    fn value_has_event_value(&self, e: &Self::Event) -> bool {
        <Self::Event as __Event<MyProject, MyWorkflow>>::value_is::<MyWorkflow, Self::Event>(e)
    }
}
