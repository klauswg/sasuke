use std::panic::{AssertUnwindSafe, catch_unwind};

use camino::Utf8PathBuf;
use sasuke::config::RuntimeConfig;
use sasuke::observability::init_tracing;
use sasuke::storage::{SasukePaths, StoragePathConfig};

#[test]
fn runtime_log_preserves_panic_payload_location_and_backtrace() {
    let temp = tempfile::tempdir().unwrap();
    let repo_root = Utf8PathBuf::from_path_buf(temp.path().join("repo")).unwrap();
    let paths = SasukePaths::new_with_path_config(
        repo_root,
        StoragePathConfig {
            app_key: "sasuke-panic-log-test",
            config_dir_name: ".sasuke-panic-log-test",
            home_env_var: "SASUKE_PANIC_LOG_TEST_HOME_UNSET",
        },
    );
    let guard = init_tracing(&paths, &RuntimeConfig::default(), false)
        .expect("test process initializes runtime tracing once");

    let panic = catch_unwind(AssertUnwindSafe(|| {
        panic!("persisted panic diagnostic probe");
    }));
    assert!(panic.is_err());
    drop(guard);

    let log = std::fs::read_to_string(paths.runtime_log_file()).unwrap();
    assert!(log.contains("event=\"runtime_panic\""));
    assert!(log.contains("panic_payload="));
    assert!(log.contains("persisted panic diagnostic probe"));
    assert!(log.contains("panic_location="));
    assert!(log.contains("tests\\runtime_panic_logging.rs:"));
    assert!(log.contains("panic_backtrace="));
}
