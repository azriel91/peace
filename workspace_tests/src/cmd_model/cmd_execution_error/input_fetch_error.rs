use miette::{Diagnostic, SourceOffset, SourceSpan};
use peace::cmd_model::InputFetchError;

#[test]
fn coverage_diagnostic() {
    let cmd_execution_src = String::from(
        r#"CmdExecution:
  ExecutionOutcome: (States<Previous>, States<Ensured>, States<Goal>)
CmdBlocks:
  - StatesCurrentReadCmdBlock:
      Input: States<Current>
      Outcome: States<Goal>
  - StatesGoalReadCmdBlock:
      Input: States<Current>
      Outcome: States<Goal>
  - StatesDiscoverCmdBlock:
      Input: ()
      Outcome: (States<Current>, States<Goal>)
  - ApplyStateSyncCheckCmdBlock:
      Input: (States<CurrentStored>, States<Current>, States<GoalStored>, States<Goal>)
      Outcome: (States<CurrentStored>, States<Current>, States<GoalStored>, States<Goal>)
  - ApplyExecCmdBlock:
      Input: (States<Current>, States<Goal>)
      Outcome: (States<Previous>, States<Ensured>, States<Goal>)
"#,
    );
    let full_span = SourceSpan::new(SourceOffset::from_location(&cmd_execution_src, 4, 5), 25);
    let error = Box::new(InputFetchError {
        cmd_block_descs: Vec::new(),
        cmd_block_index: 0,
        input_name_short: String::from("StatesCurrentReadCmdBlock"),
        input_name_full: String::from("peace_rt::cmd_blocks::StatesCurrentReadCmdBlock"),
        cmd_execution_src,
        input_span: None,
        full_span,
    });

    std::borrow::Borrow::<dyn miette::Diagnostic + '_>::borrow(&error);

    let _ = error.code();
    let _ = error.severity();
    let _ = error.help();
    let _ = error.url();
    let _ = error.source_code();
    let _ = error.labels();
    let _ = error.related();
    let _ = error.diagnostic_source();
}
