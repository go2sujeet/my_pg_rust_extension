// steps
// 1. find all the files in the ./packages directory
// 2. find the root under the ./packages directory
// 3. find all the files in the root directory
// 4. the path of the files mimics the following structure ./packages/<target_path>/<file>
// 5. move all the files to their respective <target_path> directory


use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn deploy() -> io::Result<()> {
    let package_path = "./packages";
    println!("Step 1: Finding all files in the ./packages directory...");
    let root = find_root(package_path)?;
    println!("Step 2: Finding the root under the ./packages directory...");
    println!("Root directory: {:?}", root);

    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let file_path = entry.path();
        let target_path = file_path.strip_prefix(package_path)?;
        println!("Moving {:?} to {:?}", file_path, target_path);
        move_file(&file_path, &target_path)?;
    }

    Ok(())
}

fn find_root(package_path: &str) -> io::Result<PathBuf> {
    for entry in fs::read_dir(package_path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_dir() {
            println!("Root directory found: {:?}", file_path);
            return Ok(file_path);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No root directory found",
    ))
}

fn move_file(source_path: &Path, target_path: &Path) -> io::Result<()> {
    let file_name = source_path.file_name().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name")
    })?;
    let new_path = target_path.join(file_name);

    fs::create_dir_all(&target_path)?;
    println!("Moving {:?} to {:?}", source_path, new_path);
    fs::rename(source_path, &new_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_deploy() {
        // Create some files in the ./packages directory
        // fs::create_dir_all("./packages/dir1/dir2").unwrap();
        // File::create("./packages/dir1/file1.txt").unwrap();
        // File::create("./packages/dir1/file2.txt").unwrap();
        // File::create("./packages/dir1/dir2/file3.txt").unwrap();

        // Execute deploy function
        deploy().unwrap();

        // // Check if files are moved properly
        // assert!(!Path::new("./packages/dir1/file1.txt").exists());
        // assert!(!Path::new("./packages/dir1/file2.txt").exists());
        // assert!(!Path::new("./packages/dir1/dir2/file3.txt").exists());
        // assert!(Path::new("./packages/dir1/file1.txt").exists());
        // assert!(Path::new("./packages/dir1/file2.txt").exists());
        // assert!(Path::new("./packages/dir2/file3.txt").exists());

        // // Cleanup
        // fs::remove_dir_all("./packages").unwrap();
    }

    #[test]
    fn test_find_root() {
        fs::create_dir_all("./packages/dir1").unwrap();
        fs::create_dir_all("./packages/dir2").unwrap();
        let root = find_root("./packages").unwrap();
        assert_eq!(root, Path::new("./packages/dir1"));
        fs::remove_dir_all("./packages").unwrap();
    }
}
