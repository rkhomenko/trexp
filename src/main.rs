use std::fs;
use std::io;
use std::path::Path;

fn visit_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir(&path).unwrap();
            } else {
                println!("{}", path.to_str().unwrap());
            }
        }
    }
    Ok(())
}

fn main() {
    visit_dir(Path::new("/home/rk/Documents")).unwrap();
}
