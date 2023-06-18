use peace::rt_model::output::CliOutputTarget;

#[test]
fn clone() {
    let _cli_output_target = Clone::clone(&CliOutputTarget::Stderr);
}

#[test]
fn debug() {
    assert_eq!("Stderr", format!("{:?}", CliOutputTarget::Stderr));
}

#[test]
fn partial_eq() {
    assert_eq!(CliOutputTarget::Stderr, CliOutputTarget::Stderr);
    assert_ne!(CliOutputTarget::Stdout, CliOutputTarget::Stderr);

    #[cfg(feature = "output_in_memory")]
    {
        use peace::rt_model::indicatif::TermLike;

        let mut term_0 = CliOutputTarget::in_memory(100, 20);
        let term_1 = CliOutputTarget::in_memory(100, 20);
        let term_2 = CliOutputTarget::in_memory(101, 20);

        assert_eq!(term_0, term_1);

        // Currently we define equality in terms of the contents,
        // and don't consider terminal width and height.
        assert_eq!(term_0, term_2);

        if let CliOutputTarget::InMemory(term_0) = &mut term_0 {
            term_0
                .write_line("abc")
                .expect("Expected to write line to in-memory term.");
        }
        assert_ne!(term_0, term_1);
    }
}
