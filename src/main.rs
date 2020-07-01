use std::env;
use std::fs;
use std::fs::Metadata;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

struct FileInfo {
    path_buf: PathBuf,
    meta: Metadata,
}

impl FileInfo {
    fn new(path: &Path, meta: Metadata) -> FileInfo {
        FileInfo {
            path_buf: path.to_path_buf(),
            meta,
        }
    }

    fn get_path(&self) -> &Path {
        &self.path_buf.as_path()
    }

    fn get_size(&self) -> u64 {
        self.meta.len()
    }
}

struct FileSearcher {
    files: Vec<FileInfo>,
}

impl FileSearcher {
    fn new() -> FileSearcher {
        FileSearcher {
            files: Vec::<FileInfo>::new(),
        }
    }

    fn visit_directory(files: &mut Vec<FileInfo>, path: &Path) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                FileSearcher::visit_directory(files, &path)?;
            } else {
                files.push(FileInfo::new(&path, fs::metadata(&path)?))
            }
        }
        Ok(())
    }

    fn search(&mut self, path: &Path) -> io::Result<()> {
        FileSearcher::visit_directory(&mut self.files, &path)?;
        Ok(())
    }

    fn sort_by_size(&mut self) {
        self.files
            .sort_by(|a, b| a.meta.len().cmp(&b.meta.len()).reverse())
    }

    fn get(&self) -> &Vec<FileInfo> {
        &self.files
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from("Incorrect number of arguments!"));
    }

    let mut searcher = FileSearcher::new();

    let dir_path = Path::new(args[1].as_str());
    match searcher.search(dir_path) {
        Ok(()) => (),
        Err(e) => return Err(e.to_string()),
    }
    searcher.sort_by_size();

    for file in searcher.get() {
        println!("{}, size: {}", file.get_path().display(), file.get_size())
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        process::exit(1);
    }
}
