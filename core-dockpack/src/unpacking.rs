//! Defines the actions around unpacking compressed Docker files from the manifest.
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use serde_json::Value;
use tar::Archive;
use flate2::read::GzDecoder;


/// Checks if the given file is gzipped by reading its magic number.
///
/// # Arguments
/// * `file` - A mutable reference to the file to check.
///
/// # Returns
/// * `Result<bool, io::Error>` - `Ok(true)` if the file is gzipped, `Ok(false)` otherwise, or an `Err` on failure.
fn check_if_gzipped(file: &mut File) -> std::io::Result<bool> {
    let mut magic_number = [0; 2];

    // Seek to the start of the file and read the first two bytes
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut magic_number)?;
    // Seek back to the start for subsequent operations on the file
    file.seek(SeekFrom::Start(0))?;
    // Check if the magic number matches gzip's 1F 8B
    Ok(magic_number == [0x1f, 0x8b])
}


/// Reads a JSON file.
///
/// # Notes
/// This function is mainly used for reading the manifest.json file in the unpacked Docker image
/// directory so we can extract the layers.
///
/// # Arguments
/// * `path` - Path to the JSON file being read.
fn read_json_file<P: AsRef<Path>>(path: P) -> std::io::Result<Value> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}


/// Decompresses the layers from the Docker image and extracts them to a directory.
///
/// # Arguments
/// * `main_path` - The path to the compressed extracted layers from the Docker image.
/// * `unpack_path` - The path to where the layers will be extracted.
///
/// # Returns
/// The path to where the layers are extracted.
pub fn extract_layers(main_path: &str, unpack_path: &str) -> Result<String, String> {

    let manifest_path = std::path::Path::new(main_path).join("manifest.json");
    let blobs_dir = std::path::Path::new(main_path);
    let unpack_path = std::path::Path::new(unpack_path);

    if !unpack_path.exists() {
        std::fs::create_dir_all(&unpack_path).map_err(|e| e.to_string())?;
    }

    let manifest = read_json_file(&manifest_path).map_err(|e| e.to_string())?;

    if let Some(layers) = manifest[0]["Layers"].as_array() {
        println!("Found {} layers in manifest", layers.len());
        for layer in layers {
            println!("Extracting layer: {}", layer);
            let base_path = blobs_dir;

            let layer_path = base_path.join(
                
                match layer.as_str() {
                    Some(layer) => layer,
                    None => {
                        return Err(
                            "Failed to get the layer path when extracting a layer from the Docker image".to_string()
                        );
                }
            });

            // Extract the layer's tarball to a directory
            let mut tar_file = File::open(&layer_path).map_err(|e| e.to_string())?;
            let if_gzipped = check_if_gzipped(&mut tar_file).map_err(|e| e.to_string())?;
            match if_gzipped {
                true => {
                    println!("Layer is gzipped");
                    let decompressed = GzDecoder::new(tar_file);
                    let mut archive = Archive::new(decompressed);
                    archive.unpack(unpack_path).map_err(|e| e.to_string())?;
                },
                false => {
                    println!("Layer is not gzipped");
                    let mut archive = Archive::new(tar_file);
                    archive.unpack(unpack_path).map_err(|e| e.to_string())?;
                }
            }
        }
    }
    else {
        println!("No layers found in manifest");
    }

    Ok(match unpack_path.to_str(){
        Some(v) => v.to_string(),
        None => {
            return Err("Failed to convert path to string when extracting layers from the Docker image".to_string());
        }
    })
}
