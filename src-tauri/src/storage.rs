use std::path::{Component, Path, PathBuf};

use crate::contracts::harness_settings::HarnessSettings;
use crate::contracts::local_storage_layout::{LocalStorageLayout, StorageLayoutError};

const EVIDENCE_ROOT_NAME: &str = "evidence";
const FIXTURE_ROOT_NAME: &str = "fixtures";
const LOG_ROOT_NAME: &str = "logs";
const TEMP_ROOT_NAME: &str = "tmp";

pub fn derive_local_storage_layout(
    settings: &HarnessSettings,
) -> Result<LocalStorageLayout, StorageLayoutError> {
    let layout = LocalStorageLayout {
        storage_root: settings.storage_root.clone(),
        database_path: settings.database_path.clone(),
        evidence_root: join_child_path(&settings.storage_root, EVIDENCE_ROOT_NAME),
        fixture_root: join_child_path(&settings.storage_root, FIXTURE_ROOT_NAME),
        log_root: join_child_path(&settings.storage_root, LOG_ROOT_NAME),
        temp_root: join_child_path(&settings.storage_root, TEMP_ROOT_NAME),
    };

    validate_local_storage_layout(&layout)?;

    Ok(layout)
}

pub fn validate_local_storage_layout(
    layout: &LocalStorageLayout,
) -> Result<(), StorageLayoutError> {
    let storage_root = normalize_workspace_path(&layout.storage_root)
        .map_err(|_| StorageLayoutError::StorageRootEscapesWorkspace)?;

    ensure_under_storage_root(
        &storage_root,
        &layout.database_path,
        StorageLayoutError::DatabasePathOutsideStorageRoot,
    )?;
    ensure_under_storage_root(
        &storage_root,
        &layout.evidence_root,
        StorageLayoutError::EvidenceRootOutsideStorageRoot,
    )?;
    ensure_under_storage_root(
        &storage_root,
        &layout.fixture_root,
        StorageLayoutError::FixtureRootOutsideStorageRoot,
    )?;
    ensure_under_storage_root(
        &storage_root,
        &layout.log_root,
        StorageLayoutError::LogRootOutsideStorageRoot,
    )?;
    ensure_under_storage_root(
        &storage_root,
        &layout.temp_root,
        StorageLayoutError::TempRootOutsideStorageRoot,
    )?;

    Ok(())
}

fn ensure_under_storage_root(
    storage_root: &Path,
    path: &str,
    error: StorageLayoutError,
) -> Result<(), StorageLayoutError> {
    let path = normalize_workspace_path(path).map_err(|_| error)?;

    if path.starts_with(storage_root) {
        Ok(())
    } else {
        Err(error)
    }
}

fn join_child_path(storage_root: &str, child: &str) -> String {
    Path::new(storage_root)
        .join(child)
        .to_string_lossy()
        .into_owned()
}

fn normalize_workspace_path(path: &str) -> Result<PathBuf, ()> {
    let mut normalized = PathBuf::new();

    for component in Path::new(path).components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(());
                }
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        Ok(PathBuf::from("."))
    } else {
        Ok(normalized)
    }
}
