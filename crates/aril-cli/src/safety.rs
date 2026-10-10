use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug)]
pub(crate) struct SafetyEngine;

impl SafetyEngine {
    pub(crate) fn is_binary(path: &Path) -> Result<bool> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 1024];
        let bytes_read = file.read(&mut buffer)?;

        Ok(buffer[..bytes_read].contains(&0))
    }

    pub(crate) fn atomic_write(path: &Path, content: &str, backup: bool) -> Result<()> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let temp_path = parent.join(format!(".aril_tmp_{}", uuid_fast()));

        {
            let mut tmp_file = File::create(&temp_path)
                .with_context(|| format!("Failed creating temp file {}", temp_path.display()))?;
            tmp_file.write_all(content.as_bytes())?;
            tmp_file.sync_all()?;
        }

        if backup && path.exists() {
            let backup_path = path.with_extension(format!(
                "{}.bak",
                path.extension().unwrap_or_default().to_string_lossy()
            ));
            fs::copy(path, backup_path)?;
        }

        fs::rename(&temp_path, path)
            .with_context(|| format!("Failed atomically replacing {}", path.display()))?;

        Ok(())
    }
}

fn uuid_fast() -> u128 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
