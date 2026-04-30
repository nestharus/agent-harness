use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use crate::contracts::fake_agents_fixture::{
    ChildAcceptanceState, FakeAgentsFixture, FakeAgentsFixtureError,
};
use crate::contracts::temp_harness::TempHarnessHandle;
use crate::test_harness::temp_harness::harness_app_state;

const REAL_AGENTS_PATH: &str = "/home/nes/.local/bin/agents";

#[derive(Debug, Clone)]
struct ScenarioSpec {
    manifest_relative_path: &'static str,
}

#[derive(Debug, Deserialize)]
struct ScenarioManifest {
    scenario_name: String,
    bin_path: Option<String>,
    argv_log_ref: String,
    stdin_ref: Option<String>,
    stdout_ref: Option<String>,
    stderr_ref: Option<String>,
    exit_status: i32,
    oulipoly_invocation: Option<String>,
    parent_invocation_id: Option<String>,
    session_id: Option<String>,
    child_acceptance_state: ChildAcceptanceState,
    recorded_argv: Vec<String>,
    stdin: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
    force_install_failure: Option<bool>,
    skip_argv_log_materialization: Option<bool>,
    skip_transcript_materialization: Option<bool>,
}

pub fn install_fake_agents_fixture(
    handle: TempHarnessHandle,
    scenario_name: &str,
) -> Result<FakeAgentsFixture, FakeAgentsFixtureError> {
    let spec = scenario_registry()
        .get(scenario_name)
        .cloned()
        .ok_or(FakeAgentsFixtureError::UnknownScenario)?;
    let manifest = load_scenario_manifest(&spec)?;

    let state = harness_app_state(&handle).ok_or(FakeAgentsFixtureError::FixtureInstallFailed)?;
    let fixture_root =
        Path::new(&state.storage_layout.fixture_root).join(&handle.fixture_manifest_id);
    fs::create_dir_all(&fixture_root).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;

    let bin_path = resolve_bin_path(&fixture_root, scenario_name, manifest.bin_path.as_deref())?;
    if manifest.force_install_failure.unwrap_or(false) {
        force_fixture_install_failure(&fixture_root)?;
    }

    install_fake_executable(&bin_path, &manifest)?;
    validate_installed_bin_path(&fixture_root, &bin_path)?;

    let argv_log_ref = resolve_ref_path(&fixture_root, &manifest.argv_log_ref)?;
    if !manifest.skip_argv_log_materialization.unwrap_or(false) {
        write_json_ref(&argv_log_ref, &manifest.recorded_argv)?;
    }
    if !argv_log_ref.exists() {
        return Err(FakeAgentsFixtureError::ArgvLogMissing);
    }

    let stdin_ref = materialize_optional_text_ref(
        &fixture_root,
        manifest.stdin_ref.as_deref(),
        manifest.stdin.as_deref().unwrap_or_default(),
        false,
    )?;
    let skip_transcripts = manifest.skip_transcript_materialization.unwrap_or(false);
    let stdout_ref = materialize_optional_text_ref(
        &fixture_root,
        manifest.stdout_ref.as_deref(),
        manifest.stdout.as_deref().unwrap_or_default(),
        skip_transcripts,
    )?;
    let stderr_ref = materialize_optional_text_ref(
        &fixture_root,
        manifest.stderr_ref.as_deref(),
        manifest.stderr.as_deref().unwrap_or_default(),
        skip_transcripts,
    )?;
    validate_transcript_ref(stdout_ref.as_deref())?;
    validate_transcript_ref(stderr_ref.as_deref())?;

    Ok(FakeAgentsFixture {
        bin_path: bin_path.to_string_lossy().into_owned(),
        scenario_name: manifest.scenario_name,
        argv_log_ref: argv_log_ref.to_string_lossy().into_owned(),
        stdin_ref,
        stdout_ref,
        stderr_ref,
        exit_status: manifest.exit_status,
        oulipoly_invocation: manifest.oulipoly_invocation,
        parent_invocation_id: manifest.parent_invocation_id,
        session_id: manifest.session_id,
        child_acceptance_state: manifest.child_acceptance_state,
    })
}

fn scenario_registry() -> HashMap<&'static str, ScenarioSpec> {
    HashMap::from([
        (
            "success",
            ScenarioSpec {
                manifest_relative_path: "manifests/success.json",
            },
        ),
        (
            "nonzero-exit",
            ScenarioSpec {
                manifest_relative_path: "manifests/nonzero-exit.json",
            },
        ),
        (
            "cancellation",
            ScenarioSpec {
                manifest_relative_path: "manifests/cancellation.json",
            },
        ),
        (
            "timeout",
            ScenarioSpec {
                manifest_relative_path: "manifests/timeout.json",
            },
        ),
        (
            "unknown-state-fixture",
            ScenarioSpec {
                manifest_relative_path: "manifests/unknown-state-fixture.json",
            },
        ),
        (
            "rejected-fixture",
            ScenarioSpec {
                manifest_relative_path: "manifests/rejected-fixture.json",
            },
        ),
        (
            "install-failure",
            ScenarioSpec {
                manifest_relative_path: "errors/error-fixture-install-failed.json",
            },
        ),
        (
            "missing-argv-log",
            ScenarioSpec {
                manifest_relative_path: "errors/error-argv-log-missing.json",
            },
        ),
        (
            "missing-transcript-ref",
            ScenarioSpec {
                manifest_relative_path: "errors/error-transcript-ref-missing.json",
            },
        ),
        (
            "real-agents-path",
            ScenarioSpec {
                manifest_relative_path: "errors/error-real-agents-path-rejected.json",
            },
        ),
        (
            "invalid-child-acceptance-state",
            ScenarioSpec {
                manifest_relative_path: "errors/error-invalid-child-acceptance-state.json",
            },
        ),
    ])
}

fn load_scenario_manifest(spec: &ScenarioSpec) -> Result<ScenarioManifest, FakeAgentsFixtureError> {
    let path = fixture_root().join(spec.manifest_relative_path);
    let content =
        fs::read_to_string(path).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    let value: Value =
        serde_json::from_str(&content).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    validate_child_acceptance_state_value(&value)?;
    serde_json::from_value(value).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)
}

fn validate_child_acceptance_state_value(value: &Value) -> Result<(), FakeAgentsFixtureError> {
    match value
        .get("child_acceptance_state")
        .and_then(Value::as_str)
        .ok_or(FakeAgentsFixtureError::InvalidChildAcceptanceState)?
    {
        "unknown" | "accepted" | "rejected" | "timed_out" | "ambiguous" => Ok(()),
        _ => Err(FakeAgentsFixtureError::InvalidChildAcceptanceState),
    }
}

fn resolve_bin_path(
    fixture_root: &Path,
    scenario_name: &str,
    requested_bin_path: Option<&str>,
) -> Result<PathBuf, FakeAgentsFixtureError> {
    let relative_or_absolute =
        requested_bin_path.map_or_else(|| format!("fake-agents-{scenario_name}"), str::to_string);
    let requested = Path::new(&relative_or_absolute);
    if is_real_agents_path(requested) {
        return Err(FakeAgentsFixtureError::RealAgentsPathRejected);
    }
    if requested.is_absolute() || !is_safe_relative_path(requested) {
        return Err(FakeAgentsFixtureError::FixtureInstallFailed);
    }

    Ok(fixture_root.join(requested))
}

fn resolve_ref_path(
    fixture_root: &Path,
    ref_path: &str,
) -> Result<PathBuf, FakeAgentsFixtureError> {
    let relative = Path::new(ref_path);
    if relative.is_absolute() || !is_safe_relative_path(relative) {
        return Err(FakeAgentsFixtureError::FixtureInstallFailed);
    }

    Ok(fixture_root.join(relative))
}

fn is_safe_relative_path(path: &Path) -> bool {
    path.components()
        .all(|component| matches!(component, Component::Normal(_)))
}

fn install_fake_executable(
    bin_path: &Path,
    manifest: &ScenarioManifest,
) -> Result<(), FakeAgentsFixtureError> {
    let parent = bin_path
        .parent()
        .ok_or(FakeAgentsFixtureError::FixtureInstallFailed)?;
    fs::create_dir_all(parent).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    fs::write(bin_path, fake_executable_body(manifest))
        .map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(bin_path)
            .map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(bin_path, permissions)
            .map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    }

    Ok(())
}

fn fake_executable_body(manifest: &ScenarioManifest) -> String {
    format!(
        "#!/bin/sh\n# WU-0A-15 fake agents fixture: {}\nexit {}\n",
        manifest.scenario_name, manifest.exit_status
    )
}

fn force_fixture_install_failure(fixture_root: &Path) -> Result<(), FakeAgentsFixtureError> {
    let blocked_path = fixture_root.join("install-failure-blocker");
    fs::create_dir_all(&blocked_path).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    fs::write(&blocked_path, b"this write targets a directory")
        .map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)
}

fn validate_installed_bin_path(
    fixture_root: &Path,
    bin_path: &Path,
) -> Result<(), FakeAgentsFixtureError> {
    let canonical_root =
        fs::canonicalize(fixture_root).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    let canonical_bin =
        fs::canonicalize(bin_path).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;

    if let Ok(real_agents) = fs::canonicalize(REAL_AGENTS_PATH) {
        if canonical_bin == real_agents {
            return Err(FakeAgentsFixtureError::RealAgentsPathRejected);
        }
    }
    if !canonical_bin.starts_with(canonical_root) {
        return Err(FakeAgentsFixtureError::FixtureInstallFailed);
    }

    Ok(())
}

fn is_real_agents_path(path: &Path) -> bool {
    if path == Path::new(REAL_AGENTS_PATH) {
        return true;
    }

    match (fs::canonicalize(path), fs::canonicalize(REAL_AGENTS_PATH)) {
        (Ok(candidate), Ok(real_agents)) => candidate == real_agents,
        _ => false,
    }
}

fn write_json_ref(path: &Path, argv: &[String]) -> Result<(), FakeAgentsFixtureError> {
    let parent = path
        .parent()
        .ok_or(FakeAgentsFixtureError::FixtureInstallFailed)?;
    fs::create_dir_all(parent).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    let content = serde_json::to_string_pretty(argv)
        .map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    fs::write(path, format!("{content}\n"))
        .map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)
}

fn materialize_optional_text_ref(
    fixture_root: &Path,
    ref_path: Option<&str>,
    content: &str,
    skip_materialization: bool,
) -> Result<Option<String>, FakeAgentsFixtureError> {
    let Some(ref_path) = ref_path else {
        return Ok(None);
    };
    let resolved = resolve_ref_path(fixture_root, ref_path)?;
    if !skip_materialization {
        let parent = resolved
            .parent()
            .ok_or(FakeAgentsFixtureError::FixtureInstallFailed)?;
        fs::create_dir_all(parent).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
        fs::write(&resolved, content).map_err(|_| FakeAgentsFixtureError::FixtureInstallFailed)?;
    }

    Ok(Some(resolved.to_string_lossy().into_owned()))
}

fn validate_transcript_ref(ref_path: Option<&str>) -> Result<(), FakeAgentsFixtureError> {
    if let Some(ref_path) = ref_path {
        if !Path::new(ref_path).exists() {
            return Err(FakeAgentsFixtureError::TranscriptRefMissing);
        }
    }

    Ok(())
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a repository parent")
        .join("product-strategy/contracts/fixtures/wu-0a-15")
}
