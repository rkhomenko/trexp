use std::fs;
use std::io;
use std::path::Path;

fn visit_dir(dir: &Path) -> io::Result<u64> {
    let mut dir_size: u64 = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                dir_size += visit_dir(&path)?;
            } else {
                let file_size = entry.metadata()?.len();
                dir_size += file_size;
                println!("{}, size {}", path.display(), file_size);
            }
        }

        println!("{}, size {}", dir.display(), dir_size);
    }

    Ok(dir_size)
}

fn main() {
    visit_dir(Path::new("/home/rk/Documents")).unwrap();
}
