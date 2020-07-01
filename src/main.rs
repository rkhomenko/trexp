use std::fs;
use std::io;
use std::path::Path;

fn visit_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir(&path)?;
            } else {
                println!("{}: size {}", path.display(), entry.metadata()?.len());
            }
        }
    }
    Ok(())
}

fn main() {
    visit_dir(Path::new("/home/rk/Documents")).unwrap();
}
