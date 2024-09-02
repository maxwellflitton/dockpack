//! Defines the actions around downloading and unpacking docker images to access the files.
use std::process::Command;
use tar::Archive;
use std::fs::File;
use super::cache::process_image_name;


/// Pulls a docker image from the docker registry.
///
/// # Arguments
/// * `image_name` - A string slice that holds the name of the docker image to pull.
///
/// # Returns
/// None
pub fn pull_docker_image(image_name: &str) -> Result<(), String> {
    let status = Command::new("docker")
        .args(["pull", image_name])
        .status().map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to pull Docker image".to_string())
    }
}


/// Extracts the Tar file from the Docker image, and saves it to the specified path.
///
/// # Notes
/// The pulling of the Docker image is also handled in this function.
///
/// # Arguments
/// * `image_name` - The name of the Docker image to pull and unpack.
/// * `tar_path` - The path to save the unpacked Docker image.
///
/// # Returns
/// The path to where the compressed Docker image files are stored
pub fn save_docker_image(image_name: &str, tar_path: &str) -> Result<String, String> {
    pull_docker_image(image_name)?; // Ensure the image is pulled before saving it

    let tar_path = std::path::Path::new(tar_path);
    let tar_file = image_name;
    let tar_file = process_image_name(&tar_file.to_string());

    let binding = tar_path.join(format!("{}.tar", tar_file));
    let unpack_tar_path = match binding.to_str() {
        Some(v) => v,
        None => {
            return Err("Failed to convert path to string".to_string())
        }
    };
    let package_path = tar_path.join(tar_file);

    println!("Tar path: {:?}", tar_path);

    let _ = Command::new("docker")
        .args(["save", "-o", unpack_tar_path, image_name])
        .status().map_err(|e| e.to_string())?;

    let tar_file = File::open(unpack_tar_path).map_err(|e| e.to_string())?;
    let mut archive = Archive::new(tar_file);

    archive.unpack(&package_path).map_err(|e| e.to_string())?;

    // return statement
    Ok(match package_path.to_str() {
        Some(v) => v.to_string(),
        None => {
            return Err("Failed to convert path to string".to_string())
        }
    })
}
