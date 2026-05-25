use crate::utils::cache;
use anyhow::{Context, Result};
use bollard::models::{CreateImageInfo, PushImageInfo};
use bollard::query_parameters::{CreateImageOptionsBuilder, PushImageOptionsBuilder};
use bollard::Docker;
use futures_util::stream::TryStreamExt;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

async fn dir_to_tar(dir: &str, image_name: &str) -> Result<String> {
    let tar_name = format!("{}.tar", cache::process_image_name(image_name));
    let tar = File::create(&tar_name)
        .await
        .with_context(|| "Could not create archive file")?;
    let mut tar = tokio_tar::Builder::new(tar);
    tar.append_dir_all("", dir)
        .await
        .with_context(|| "Could not add path to target")?;
    tar.finish()
        .await
        .with_context(|| "An error occured in converting dir to tar")?;
    Ok(tar_name)
}

pub async fn execute_docker_build(directory: &str, image: &str) -> Result<()> {
    // Convert directory to a tar file
    let tar_path = dir_to_tar(directory, image).await?;

    let file = File::open(tar_path)
        .await
        .with_context(|| format!("Could not find archive at path {}.tar", image))?;
    let stream = ReaderStream::new(file);

    let docker = Docker::connect_with_socket_defaults()
        .with_context(|| "Could not connect to docker socket. Is docker running?")?;

    let options = CreateImageOptionsBuilder::default()
        .from_src("-") // from_src must be "-" when sending the archive in the request body
        .repo(image) // The name of the image in the docker daemon.
        .build();
    let _: Vec<CreateImageInfo> = docker
        .create_image(Some(options), Some(bollard::body_try_stream(stream)), None)
        .try_collect()
        .await
        .with_context(|| "Could not create image")?;

    let options = PushImageOptionsBuilder::new().tag("latest").build();
    let _: Vec<PushImageInfo> = docker
        .push_image(image, Some(options), None)
        .try_collect()
        .await
        .with_context(|| "Could not push image")?;

    Ok(())
}

// run from core_dockpack containing test_image directory with dockerfile
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_dir_to_tar() {
        let image_name = "test_image:latest";
        let directory = "./test_image";
        fs::create_dir_all(directory).expect("Failed to create test directory");
        let dockerfile_path = format!("{}/Dockerfile", directory);
        let dockerfile_content = "FROM scratch\nCOPY . .\n";
        fs::write(&dockerfile_path, dockerfile_content).expect("Failed to write Dockerfile");

        let result = dir_to_tar(directory, image_name).await;

        assert!(result.is_ok());

        fs::remove_dir_all(directory).expect("Failed to remove test directory");
    }

    #[tokio::test]
    async fn test_execute_push() {
        let image_name = "test_image:latest";
        let directory = "./test_image";

        fs::create_dir_all(directory).expect("Failed to create test directory");
        let dockerfile_path = format!("{}/Dockerfile", directory);
        let dockerfile_content = "FROM scratch\nCOPY . .\n";
        fs::write(&dockerfile_path, dockerfile_content).expect("Failed to write Dockerfile");

        let result = execute_docker_build(directory, image_name).await;
        assert!(result.is_ok());

        fs::remove_dir_all(directory).expect("Failed to remove test directory");
    }
}
