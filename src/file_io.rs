use anyhow::{bail, Context, Result};
use chrono::Local;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{create_dir_all, read_dir, remove_file, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

pub struct FileManipulator {
    input: PathBuf,
    output: PathBuf,
    max_files: usize,
}

impl FileManipulator {
    pub fn new(input: PathBuf, output: PathBuf, max_files: usize) -> Self {
        Self {
            input,
            output,
            max_files,
        }
    }

    pub fn rw_zip(&self) -> Result<()> {
        if !self.input.is_dir() {
            bail!("Input path is not a directory");
        }
        if !self.output.is_dir() {
            bail!("Output path is not a directory");
        }
        let walk_dir = WalkDir::new(&self.input);
        let iterator = walk_dir.into_iter();
        self.zip_dir(&mut iterator.filter_map(std::result::Result::ok))
            .with_context(|| "Failed to zip directory")?;
        Ok(())
    }

    fn zip_dir(&self, it: &mut dyn Iterator<Item = DirEntry>) -> Result<()> {
        let mut output_path = self.output.clone();
        output_path.push(format!(
            "Backup-{}.zip",
            Local::now().format("%Y-%m-%d-%H-%M-%S")
        ));
        let writer = File::create(output_path)?;
        let mut zip = zip::ZipWriter::new(writer);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        let mut buffer = Vec::new();
        for entry in it {
            let path = entry.path();
            let name = path.strip_prefix(Path::new(&self.input))?;
            if path.is_file() {
                if let Some(dir) = name.to_str() {
                    zip.start_file(dir, options)?;
                    let mut f = File::open(path)?;
                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&*buffer)?;
                    buffer.clear();
                }
            } else if !name.as_os_str().is_empty() {
                if let Some(dir) = name.to_str() {
                    zip.add_directory(dir, options)?;
                }
            }
        }
        zip.finish()?;
        Ok(())
    }

    pub fn truncate_target_dir(&self) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^Backup-.*zip$").unwrap();
        }
        create_dir_all(&self.output)?;
        let mut result = Vec::new();
        for path in read_dir(&self.output)? {
            let path = path?;
            if path.file_type()?.is_file() {
                if let Some(file_name) = path.file_name().to_str() {
                    if RE.is_match(file_name)
                        && path.metadata().is_ok()
                        && path.metadata().unwrap().modified().is_ok()
                    {
                        result.push(path);
                    }
                }
            }
        }
        result.sort_by_key(|path| path.metadata().unwrap().modified().unwrap());
        if result.len() >= self.max_files {
            let surplus = result.len() - self.max_files + 1;
            let files_to_remove = &result[0..surplus];
            for file in files_to_remove {
                remove_file(file.path())?;
            }
        }
        Ok(())
    }
}
