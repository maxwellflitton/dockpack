//! Defines the actions around downloading and unpacking docker images to access the files.
use super::cache::process_image_name;
use anyhow::{anyhow, Context, Result};
use bollard::query_parameters::CreateImageOptionsBuilder;
use bollard::Docker;
use futures_util::stream::TryStreamExt;
use futures_util::StreamExt;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_tar::Archive;

async fn pull_image(image_name: &str, docker: &Docker) -> Result<()> {
    let options = Some(
        CreateImageOptionsBuilder::new()
            .from_image(image_name)
            .build(),
    );
    println!("image_name: {}", image_name);
    // confirmed: this works
    docker
        .create_image(options, None, None)
        .try_collect::<Vec<_>>()
        .await
        .with_context(|| "Error creating image")?;
    Ok(())
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
pub async fn save_docker_image(image_name: &str, tar_path: &str) -> Result<String> {
    // pull image
    // // TODO: consider moving to receive in the function
    let docker = Docker::connect_with_socket_defaults()
        .expect("Could not connect to docker socket. Is docker running?");
    pull_image(image_name, &docker).await?;

    // create file
    let tar_path = std::path::Path::new(tar_path);
    let tar_file = image_name;
    let tar_file = process_image_name(tar_file);
    let tar_name = &format!("{}.tar", tar_file);

    let binding = tar_path.join(format!("{}.tar", tar_file));
    let file_path = binding.to_str().unwrap_or(tar_name);

    // Create tar
    let mut tar = docker.export_image(image_name);
    let mut archive_file = File::create(file_path)
        .await
        .with_context(|| "Could not create file path for tar file")?;
    while let Some(chunk) = tar.next().await {
        let data = chunk.with_context(|| "Could not read bytes from zip stream")?;
        archive_file
            .write_all(&data)
            .await
            .with_context(|| "Error writing bytes to file")?;
        archive_file
            .sync_all()
            .await
            .with_context(|| "Error syncing all data to file")?;
    }
    println!("Synced to tar file");

    let file = File::open(file_path)
        .await
        .expect("Could not reopen archive file");
    let mut archive = Archive::new(file);

    archive
        .unpack(tar_path)
        .await
        .with_context(|| "Error unpacking tar file")?;
    //
    // // return statement
    Ok(match tar_path.to_str() {
        Some(v) => v.to_string(),
        None => return Err(anyhow!("Failed to convert path to string")),
    })
}

// directory is the build context

pub fn build_docker_image(directory: &str, image: &str) -> Result<(), String> {
    let platforms = "linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/s390x,linux/ppc64le";
    let status = Command::new("docker")
        .args(["build", "--platform", platforms, "-t", image])
        .arg(directory) // Add the directory as a separate argument
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to build Docker image".to_string())
    }
}
