//! Log rotation utilities

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Log rotator for managing rotated log files
pub struct LogRotator {
    base_path: PathBuf,
    max_files: usize,
    max_size: u64,
    compress: bool,
}

impl LogRotator {
    /// Create a new log rotator
    pub fn new<P: AsRef<Path>>(
        base_path: P,
        max_files: usize,
        max_size: u64,
        compress: bool,
    ) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            max_files,
            max_size,
            compress,
        }
    }

    /// Check if rotation is needed
    pub fn should_rotate(&self) -> Result<bool> {
        if !self.base_path.exists() {
            return Ok(false);
        }

        let metadata = fs::metadata(&self.base_path)?;
        Ok(metadata.len() >= self.max_size)
    }

    /// Perform log rotation
    pub fn rotate(&self) -> Result<()> {
        if !self.base_path.exists() {
            return Ok(());
        }

        // Get the directory and file stem
        let dir = self.base_path.parent().unwrap_or(Path::new("."));
        let file_name = self.base_path.file_name().unwrap_or_default();

        // Generate timestamp for rotated file
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let rotated_name = format!("{}.{}", file_name.to_string_lossy(), timestamp);
        let rotated_path = dir.join(&rotated_name);

        // Move current log to rotated name
        fs::rename(&self.base_path, &rotated_path)?;

        // Compress if enabled
        if self.compress {
            self.compress_file(&rotated_path)?;
        }

        // Clean up old rotated files
        self.cleanup_old_files(dir, file_name)?;

        Ok(())
    }

    /// Compress a rotated log file
    fn compress_file(&self, path: &Path) -> Result<()> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::{Read, Write};

        let mut input = fs::File::open(path)?;
        let mut output = fs::File::create(path.with_extension("gz"))?;
        let mut encoder = GzEncoder::new(&mut output, Compression::default());
        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;
        encoder.write_all(&buffer)?;
        encoder.finish()?;
        fs::remove_file(path)?;
        Ok(())
    }

    /// Remove old rotated files beyond max_files
    fn cleanup_old_files(&self, dir: &Path, file_name: &std::ffi::OsStr) -> Result<()> {
        let mut files: Vec<_> = fs::read_dir(dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name();
                name.to_string_lossy()
                    .starts_with(&file_name.to_string_lossy().to_string())
                    && name != file_name
            })
            .collect();

        // Sort by modification time (newest first)
        files.sort_by_key(|entry| {
            entry
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        });
        files.reverse();

        // Remove excess files
        for entry in files.into_iter().skip(self.max_files) {
            fs::remove_file(entry.path())?;
        }

        Ok(())
    }

    /// Create a new log file after rotation
    pub fn create_new(&self) -> Result<fs::File> {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.base_path)
            .map_err(Into::into)
    }
}

/// Builder for LogRotator
pub struct LogRotatorBuilder {
    base_path: Option<PathBuf>,
    max_files: usize,
    max_size: u64,
    compress: bool,
}

impl LogRotatorBuilder {
    pub fn new() -> Self {
        Self {
            base_path: None,
            max_files: 10,
            max_size: 100 * 1024 * 1024, // 100 MB
            compress: true,
        }
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.base_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn max_files(mut self, max_files: usize) -> Self {
        self.max_files = max_files;
        self
    }

    pub fn max_size(mut self, max_size: u64) -> Self {
        self.max_size = max_size;
        self
    }

    pub fn compress(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    pub fn build(self) -> Result<LogRotator> {
        let base_path = self
            .base_path
            .ok_or_else(|| anyhow::anyhow!("base_path is required"))?;
        Ok(LogRotator::new(
            base_path,
            self.max_files,
            self.max_size,
            self.compress,
        ))
    }
}

impl Default for LogRotatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
