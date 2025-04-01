/*
Project: reth https://github.com/paradigmxyz/reth
Version: 1.2.2
Path: crates/stages/api/src/stage.rs
*/

/// Stage execution input, see [`Stage::execute`].
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct ExecInput {
    /// The target block number the stage needs to execute towards.
    pub target: Option<BlockNumber>,
    /// The checkpoint of this stage the last time it was executed.
    pub checkpoint: Option<StageCheckpoint>,
}

impl ExecInput {
    //...
}

/// Stage unwind input, see [`Stage::unwind`].
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct UnwindInput {
    /// The current highest checkpoint of the stage.
    pub checkpoint: StageCheckpoint,
    /// The block to unwind to.
    pub unwind_to: BlockNumber,
    /// The bad block that caused the unwind, if any.
    pub bad_block: Option<BlockNumber>,
}

impl UnwindInput {
    // ...
}

/// The output of a stage execution.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExecOutput {
    /// How far the stage got.
    pub checkpoint: StageCheckpoint,
    /// Whether or not the stage is done.
    pub done: bool,
}

impl ExecOutput {
    //...
}

/// The output of a stage unwinding.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnwindOutput {
    /// The checkpoint at which the stage has unwound to.
    pub checkpoint: StageCheckpoint,
}

// using auto_impl for smart pointer Box so we just need to implement the trait once.
#[auto_impl::auto_impl(Box)]
// Stage is a generic trait with Provider as the type placeholder
pub trait Stage<Provider>: Send + Sync {
    /// Stages must have a unique [ID][StageId] and implement a way to "roll forwards"
    /// ([Stage::execute]) and a way to "roll back" ([Stage::unwind]).
    fn id(&self) -> StageId;

    // using std::task to handle async tasks
    /// Returns `Poll::Ready(Ok(()))` when the stage is ready to execute the given range.
    /// If the stage has any pending external state, then `Poll::Pending` is returned.
    fn poll_execute_ready(
        &mut self,
        _cx: &mut Context<'_>,
        _input: ExecInput,
    ) -> Poll<Result<(), StageError>> {
        Poll::Ready(Ok(()))
    }

    /// Execute the stage. Triggering the DB operations.
    fn execute(&mut self, provider: &Provider, input: ExecInput) -> Result<ExecOutput, StageError>;


    /// This is called after the stage has been executed and the data has been committed by the
    /// provider. 
    fn post_execute_commit(&mut self) -> Result<(), StageError> {
        Ok(())
    }

    /// Unwind the stage.
    fn unwind(
        &mut self,
        provider: &Provider,
        input: UnwindInput,
    ) -> Result<UnwindOutput, StageError>;

    /// This is called after the stage has been unwound and the data has been committed by the
    /// provider. 
    fn post_unwind_commit(&mut self) -> Result<(), StageError> {
        Ok(())
    }
}

/// [Stage] trait extension.
// Stage is a supertrait of StageExt (StageExt bounds Stage)
pub trait StageExt<Provider>: Stage<Provider> {
    fn execute_ready(
        &mut self,
        input: ExecInput,
    // The Future implementation will have an Output type Result<(), StageError>
    ) -> impl Future<Output = Result<(), StageError>> + Send {
        poll_fn(move |cx| self.poll_execute_ready(cx, input))
    }
}

// empty implementation
impl<Provider, S: Stage<Provider>> StageExt<Provider> for S {}