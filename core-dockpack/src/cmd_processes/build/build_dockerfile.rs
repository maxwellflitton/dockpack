//! Builds a Dockerfile from a directory
use std::fs:: File;
use std::io:: Write;


/// Creates a Dockerfile in the given directory
/// 
/// # Arguments
/// * `directory` - A string slice that holds the path to the directory where the Dockerfile will be created
/// 
/// # Returns
/// * `Result<(), String>` - A result that indicates if the Dockerfile was created successfully or an error message
pub fn create_dockerfile(directory: &str) -> Result<(), String> {
    let docker_file_content = format! {
        "FROM scratch\nCOPY . .\n"
    }; 
    // let dockerfile_path = format!("{}/Dockerfile", directory);
    let mut dockerfile = File::create(&directory).map_err(|e| e.to_string())?;
    dockerfile.write_all(docker_file_content.as_bytes()).map_err(|e| e.to_string())?;
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

        let dockerfile_content = fs::read_to_string(&dockerfile_path).expect("Error reading dockerfile");
        assert_eq!(dockerfile_content, format!("FROM scratch\nCOPY . .\n"));

        fs::remove_dir_all(directory).expect("Failed to remove test directory");
    }

}