use ui_test::{color_eyre::Result, run_tests, Config, Mode, OutputConflictHandling};

fn main() -> Result<()> {
    let output_conflict_handling = if std::env::args().any(|arg| arg == "--bless") {
        OutputConflictHandling::Bless
    } else {
        OutputConflictHandling::Error
    };

    let config = Config {
        root_dir: "examples".into(),
        dependencies_crate_manifest_path: Some("test_dependencies/Cargo.toml".into()),
        args: vec!["--cfg".into(), "feature=\"tracing-log\"".into()],
        out_dir: Some("target/ui_test".into()),
        mode: Mode::Run { exit_code: 0 },
        output_conflict_handling,
        ..Default::default()
    };

    run_tests(config)
}
