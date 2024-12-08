//! This module contains the function to execute the push command to push a docker image to the docker registry.
use crate::utils::docker_commands;


/// Executes the push command to push a docker image to the docker registry.
/// 
/// # Arguments
/// * `image` - A string slice that holds the name of the docker image to push.
/// 
/// # Returns
/// * `Result<(), String>` - A result that indicates if the docker image was pushed successfully or an error message
pub fn execute_push_image(image: &str) -> Result<(), String> {
    docker_commands::push_docker_image(image)?;
    Ok(())
}
