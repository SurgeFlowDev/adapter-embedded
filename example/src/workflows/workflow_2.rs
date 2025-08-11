use aide::axum::ApiRouter;
use derive_more::{From, TryInto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surgeflow::{
    __Event, __Step, __Workflow, __WorkflowStatic, ArcAppState, Event, Immediate, Project,
    ProjectWorkflowControl, RawStep, Step, TryAsRef, TryFromRef, Workflow, WorkflowControl,
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
    fn entrypoint(&self) -> RawStep<MyProject, <MyProject as Project>::Workflow> {
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum MyProjectWorkflowStepEvent {
    Workflow1(
        <<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event,
    ),
    Immediate(Immediate),
}
impl From<<<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event>
    for MyProjectWorkflowStepEvent
{
    fn from(
        event: <<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event,
    ) -> Self {
        MyProjectWorkflowStepEvent::Workflow1(event)
    }
}

impl TryFrom<MyProjectWorkflowStepEvent>
    for <<MyWorkflow as __Workflow<MyProject>>::Step as __Step<MyProject, MyWorkflow>>::Event
{
    type Error = TempError;

    fn try_from(value: MyProjectWorkflowStepEvent) -> Result<Self, Self::Error> {
        match value {
            MyProjectWorkflowStepEvent::Workflow1(event) => Ok(event.into()),
            MyProjectWorkflowStepEvent::Immediate(immediate) => Ok(immediate.into()),
        }
    }
}

// identity conversion
impl TryFromRef<MyProjectWorkflowStepEvent> for MyProjectWorkflowStepEvent {
    type Error = TempError;

    fn try_from_ref(value: &MyProjectWorkflowStepEvent) -> Result<&Self, Self::Error> {
        Ok(value)
    }
}

impl __Event<MyProject, ProjectWorkflow> for MyProjectWorkflowStepEvent {}

impl __Step<MyProject, ProjectWorkflow> for MyProjectWorkflowStep {
    type Event = MyProjectWorkflowStepEvent;

    type Error = TempError;

    async fn run(
        &self,
        wf: ProjectWorkflow,
        event: Self::Event,
    ) -> Result<
        Option<RawStep<MyProject, <MyProject as Project>::Workflow>>,
        <Self as __Step<MyProject, ProjectWorkflow>>::Error,
    > {
        tracing::info!("Running MyProjectWorkflowStep with event: {:?}", event);
        match self {
            MyProjectWorkflowStep::Workflow1(workflow) => {
                workflow
                    .run(
                        // TODO
                        wf.try_into().map_err(|err| {
                            tracing::error!("Failed to convert workflow: {:?}", err);
                            TempError
                        })?,
                        // TODO
                        event.try_into().map_err(|err| {
                            tracing::error!("Failed to convert event: {:?}", err);
                            TempError
                        })?,
                    )
                    .await
            }
        }
    }

    fn event_is_event(&self, event: &Self::Event) -> bool {
        match (self, event) {
            (
                MyProjectWorkflowStep::Workflow1(step),
                MyProjectWorkflowStepEvent::Workflow1(event),
            ) => step.event_is_event(event),
            (
                // for multiple workflows
                // MyProjectWorkflowStep::Workflow1(step) |
                MyProjectWorkflowStep::Workflow1(step),
                MyProjectWorkflowStepEvent::Immediate(event),
            ) => step.event_is_event(&(*event).into()),
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

    fn entrypoint() -> RawStep<MyProject, <MyProject as Project>::Workflow> {
        let step = <Self as __Workflow<MyProject>>::Step::from(MyWorkflowStep::Step1(MyStep));
        next_step(step).max_retries(0).call()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum MyWorkflowStep {
    Step1(MyStep),
    Step2(MyAnotherStep),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, From, TryInto)]
pub enum MyWorkflowStepEvent {
    MyEvent(MyEvent),
    #[serde(skip)]
    Immediate(Immediate),
}

impl __Event<MyProject, MyWorkflow> for MyWorkflowStepEvent {}

// identity conversion
impl TryFromRef<MyWorkflowStepEvent> for MyWorkflowStepEvent {
    type Error = TempError;

    fn try_from_ref(value: &MyWorkflowStepEvent) -> Result<&Self, Self::Error> {
        Ok(value)
    }
}

impl __Step<MyProject, MyWorkflow> for MyWorkflowStep {
    type Event = MyWorkflowStepEvent;

    type Error = TempError;

    async fn run(
        &self,
        wf: MyWorkflow,
        event: Self::Event,
    ) -> Result<
        Option<RawStep<MyProject, <MyProject as Project>::Workflow>>,
        <Self as __Step<MyProject, MyWorkflow>>::Error,
    > {
        tracing::info!("Running MyWorkflowStep with event: {:?}", event);
        match self {
            MyWorkflowStep::Step1(step) => {
                let event = match event.try_into() {
                    Ok(event) => event,
                    Err(_) => return Err(TempError),
                };
                __Step::run(step, wf, event).await
            }
            MyWorkflowStep::Step2(step) => {
                let event = match event.try_into() {
                    Ok(event) => event,
                    Err(_) => return Err(TempError),
                };
                step.run(wf, event).await
            }
        }
    }

    fn event_is_event(&self, event: &Self::Event) -> bool {
        match self {
            MyWorkflowStep::Step1(step) => match event.try_as_ref() {
                Ok(event) => __Step::event_is_event(step, event),
                Err(_) => false,
            },
            MyWorkflowStep::Step2(step) => match event.try_as_ref() {
                Ok(event) => __Step::event_is_event(step, event),
                Err(_) => false,
            },
        }
    }
}

impl TryFromRef<MyWorkflowStepEvent> for MyEvent {
    type Error = TempError;

    fn try_from_ref(value: &MyWorkflowStepEvent) -> Result<&Self, Self::Error> {
        match value {
            MyWorkflowStepEvent::MyEvent(event) => Ok(event),
            _ => Err(TempError),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MyStep;

impl __Step<MyProject, MyWorkflow> for MyStep {
    type Event = MyEvent;

    type Error = TempError;

    async fn run(
        &self,
        wf: MyWorkflow,
        event: Self::Event,
    ) -> Result<
        Option<RawStep<MyProject, <MyProject as Project>::Workflow>>,
        <Self as __Step<MyProject, MyWorkflow>>::Error,
    > {
        let res = <Self as Step<MyProject, MyWorkflow>>::run(self, wf, event).await?;
        let Some(res) = res else {
            return Ok(None);
        };

        // TODO: there should be some custom Into<> trait
        // I can't use `Into::into` because of the identity implementation in the core library
        // it conflicts with the generic implementation
        Ok(Some(RawStep {
            step: res.step.into(),
            event: res.event.map(Into::into),
            settings: res.settings,
        }))
    }

    fn event_is_event(&self, ev: &Self::Event) -> bool {
        <Self as Step<MyProject, MyWorkflow>>::event_is_event(self, ev)
    }
}

impl Step<MyProject, MyWorkflow> for MyStep {
    type Event = MyEvent;

    type Error = TempError;

    async fn run(
        &self,
        wf: MyWorkflow,
        event: <Self as Step<MyProject, MyWorkflow>>::Event,
    ) -> Result<
        Option<RawStep<MyProject, MyWorkflow>>,
        <Self as __Step<MyProject, MyWorkflow>>::Error,
    > {
        tracing::info!("Running MyStep with event: {:?}", event);
        let step =
            <MyWorkflow as __Workflow<MyProject>>::Step::from(MyWorkflowStep::Step2(MyAnotherStep));
        Ok(Some(
            next_step(step)
                .max_retries(0)
                .event(Immediate)
                .call(),
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MyEvent {
    data: String,
}

impl Event<MyProject, MyWorkflow> for MyEvent {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MyAnotherStep;

impl TryFromRef<MyWorkflowStepEvent> for Immediate {
    type Error = TempError;

    fn try_from_ref(value: &MyWorkflowStepEvent) -> Result<&Self, Self::Error> {
        match value {
            MyWorkflowStepEvent::Immediate(immediate) => Ok(immediate),
            _ => Err(TempError),
        }
    }
}

impl __Step<MyProject, MyWorkflow> for MyAnotherStep {
    type Event = Immediate;

    type Error = TempError;

    async fn run(
        &self,
        wf: MyWorkflow,
        event: Self::Event,
    ) -> Result<
        Option<RawStep<MyProject, <MyProject as Project>::Workflow>>,
        <Self as __Step<MyProject, MyWorkflow>>::Error,
    > {
        tracing::info!("Running MyStep with event: {:?}", event);
        Ok(None)
    }

    fn event_is_event(&self, _: &Self::Event) -> bool {
        // this check, on the bare step is always true since we're only comparing the type
        // TODO: we could allow custom logic here, or use PartialEq, to allow the user to make value-based comparisons
        true
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
