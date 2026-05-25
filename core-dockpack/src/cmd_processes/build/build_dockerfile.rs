//! Builds a Dockerfile from a directory

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;

// directory is the build context

pub fn create_dockerfile(directory: &str) -> Result<()> {
    let docker_file_content = "FROM scratch\nCOPY . .\n".to_string();

    let dockerfile_path = format!("{}/Dockerfile", directory);

    let mut dockerfile = File::create(&dockerfile_path)
        .with_context(|| format!("Error creatining file at path {}", dockerfile_path))?;

    dockerfile
        .write_all(docker_file_content.as_bytes())
        .with_context(|| {
            format!(
                "Could not write all from scratch content to dockerfile at path {}",
                dockerfile_path,
            )
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // test from core_dockpack

    #[test]
    fn test_build_dockerfile() {
        let directory = "./test_image";
        let dockerfile_path = format!("{}/Dockerfile", directory);

        fs::create_dir_all(directory).expect("Failed to create test directory");
        let result = create_dockerfile(directory);
        assert!(result.is_ok());

        let dockerfile_content =
            fs::read_to_string(&dockerfile_path).expect("Error reading dockerfile");
        assert_eq!(dockerfile_content, format!("FROM scratch\nCOPY . .\n"));

        fs::remove_dir_all(directory).expect("Failed to remove test directory");
    }
}
