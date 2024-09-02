//! The API for unpacking Docker images into a directory.
use crate::utils::{
    docker_commands,
    cache,
    unpacking
};
use std::path::PathBuf;


/// Unpacks the files from a Docker image into a directory.
/// 
/// # Arguments
/// * `image` - The name of the Docker image to unpack.
/// * `directory` - The directory to unpack the Docker image into.
/// 
/// # Returns
/// The path to the directory where the Docker image files are stored.
pub fn unpack_files_from_image(image: &str, directory: &str) -> Result<String, String> {
    let main_path = PathBuf::from(directory);
    cache::wipe_and_create_cache(&main_path);

    let tar_dir = main_path.join("tar");
    let tar_dir = tar_dir.to_str().unwrap();
    let main_tar_path = docker_commands::save_docker_image(
        image,
        tar_dir,
    )?;
    let final_path = unpacking::extract_layers(
        main_tar_path.as_str(),
        // unwrap is safe here because we are using a hardcoded path
        main_path.to_str().unwrap(),
    )?;
    Ok(final_path)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_unpack_files_from_image() {
        let image = "maxwellflitton/nan-one";
        let directory = "./cache/two";
        let result = unpack_files_from_image(image, directory);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(Path::new(&path).exists());
        // fs::remove_dir_all(directory).unwrap();
    }
}
