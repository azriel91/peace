use peace::rt::JobRunner;

#[test]
fn returns_ok_when_job_is_successful() {
    assert_eq!(Ok(()), JobRunner::run());
}