use aide::axum::ApiRouter;
use derive_more::{From, TryInto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surgeflow::{
    __Event, __Step, __Workflow, __WorkflowStatic, ArcAppState, Immediate, Project,
    ProjectWorkflowControl, StepWithSettings, TryAsRef, TryFromRef, Workflow, WorkflowControl,
    next_step,
    senders::{EventSender, NewInstanceSender},
};

#[derive(thiserror::Error, Debug)]
#[error("Temporary error")]
pub struct TempError;

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
pub struct MyProject {
    pub workflow: MyWorkflow,
}

impl Project for MyProject {
    type Workflow = ProjectWorkflow;

    fn workflow_for_step(
        &self,
        step: &<Self::Workflow as __Workflow<Self>>::Step,
    ) -> Self::Workflow {
        match step {
            MyProjectWorkflowStep::Workflow1(_) => {
                ProjectWorkflow::Workflow1(self.workflow.clone())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum ProjectWorkflow {
    Workflow1(MyWorkflow),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum ProjectWorkflowStatic {
    Workflow1(MyWorkflowStatic),
}

impl __WorkflowStatic<MyProject, <MyProject as Project>::Workflow> for ProjectWorkflowStatic {
    fn entrypoint(&self) -> StepWithSettings<MyProject> {
        match self {
            ProjectWorkflowStatic::Workflow1(w) => {
                <MyWorkflowStatic as __WorkflowStatic<MyProject, MyWorkflow>>::entrypoint(w)
            }
        }
    }

    fn name(&self) -> &'static str {
        match self {
            ProjectWorkflowStatic::Workflow1(w) => {
                <MyWorkflowStatic as __WorkflowStatic<MyProject, MyWorkflow>>::name(w)
            }
        }
    }
}

impl __Workflow<MyProject> for ProjectWorkflow {
    type Step = MyProjectWorkflowStep;
    type WorkflowStatic = ProjectWorkflowStatic;
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum MyProjectWorkflowStep {
    Workflow1(<MyWorkflow as __Workflow<MyProject>>::Step),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum MyProjectWorkflowStepEvent {
    Workflow1(
        <<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event,
    ),
    Immediate(Immediate),
}
impl TryFromRef<MyProjectWorkflowStepEvent> for Immediate {
    type Error = TempError;

    fn try_from_ref(value: &MyProjectWorkflowStepEvent) -> Result<&Self, Self::Error> {
        match value {
            MyProjectWorkflowStepEvent::Workflow1(event) => Ok(event.try_as_ref()?),
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
                    .run(
                        // TODO
                        wf.try_into().map_err(|err| TempError)?,
                        // TODO
                        event.try_into().map_err(|err| TempError)?,
                    )
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
pub struct MyWorkflow;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MyWorkflowStatic;

impl Workflow<MyProject> for MyWorkflow {
    const NAME: &'static str = "MyWorkflow";
    type Step = MyWorkflowStep;
    type WorkflowStatic = MyWorkflowStatic;
    const WORKFLOW_STATIC: <Self as __Workflow<MyProject>>::WorkflowStatic = MyWorkflowStatic;

    fn entrypoint() -> StepWithSettings<MyProject> {
        let step = <Self as __Workflow<MyProject>>::Step::from(MyWorkflowStep::Step0(MyStep));
        next_step(step).max_retries(0).call()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum MyWorkflowStep {
    Step0(MyStep),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum MyWorkflowStepEvent {
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
pub struct MyStep;

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

//////////////////////////////

impl ProjectWorkflowControl<MyProject> for ProjectWorkflow {
    async fn control_router<
        NewEventSenderT: EventSender<MyProject>,
        NewInstanceSenderT: NewInstanceSender<MyProject>,
    >() -> anyhow::Result<ApiRouter<ArcAppState<MyProject, NewEventSenderT, NewInstanceSenderT>>>
    {
        // let workflow_1_router =
        //     Workflow1::control_router::<NewEventSenderT, NewInstanceSenderT>().await?;
        let workflow_2_router =
            MyWorkflow::control_router::<NewEventSenderT, NewInstanceSenderT>().await?;

        Ok(ApiRouter::new()
            // .merge(workflow_1_router)
            .merge(workflow_2_router))
    }
}
