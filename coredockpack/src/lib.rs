// Build
use core_dockpack::cmd_processes::build::build_dockerfile::create_dockerfile;
use core_dockpack::cmd_processes::pull::unpack_files::unpack_files_from_image;
// Push
use core_dockpack::cmd_processes::push::execute_push::execute_docker_build;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use tokio::runtime::Runtime;

/// Unpacks the files from a Docker image into a directory.
///
/// # Arguments
/// * `image` - The name of the Docker image to unpack.
/// * `directory` - The directory to unpack the Docker image into.
///
/// # Returns
/// A C string with the path to the directory where the Docker image files are stored.
/// On error, returns a null pointer.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn unpack_files_from_image_c(
    image: *const c_char,
    directory: *const c_char,
) -> *const c_char {
    // Convert C strings to Rust strings
    let image = unsafe { CStr::from_ptr(image).to_string_lossy().into_owned() };
    let directory = unsafe { CStr::from_ptr(directory).to_string_lossy().into_owned() };

    let rt = Runtime::new().unwrap();
    let result = rt.block_on(unpack_files_from_image(&image, &directory));
    match result {
        Ok(path) => {
            let c_string = CString::new(path).unwrap();
            c_string.into_raw() // Return the C string
        }
        Err(err) => {
            eprintln!("Error unpacking image: {}", err);
            std::ptr::null() // Return null on error
        }
    }
}

/// Builds a docker image from a directory
///
/// # Arguments
/// * `image` - The name of the Docker image to create.
/// * `directory` - The directory to use for building the image.
///
/// # Returns
/// A C string with the path to the directory where the Docker image files are stored.
#[no_mangle]
pub extern "C" fn build_image_from_files_c(
    image: *const c_char,
    directory: *const c_char,
) -> *const c_char {
    let image = unsafe { CStr::from_ptr(image).to_string_lossy().into_owned() };
    let directory = unsafe { CStr::from_ptr(directory).to_string_lossy().into_owned() };
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(execute_docker_build(&image, &directory));
    match result {
        Ok(_) => {
            let c_string = CString::new(directory).unwrap();
            c_string.into_raw() // Return the C string
        }
        Err(err) => {
            eprintln!("Error unpacking image: {}", err);
            std::ptr::null() // Return null on error
        }
    }
}

/// Packs a directory into a docker file
///
/// # Arguments
/// * `directory` - The directory into with docker.
///
/// # Returns
/// A C string with the path to the directory where the Docker image files are stored.
#[no_mangle]
pub extern "C" fn create_dockerfile_c(directory: *const c_char) -> *const c_char {
    let directory = unsafe { CStr::from_ptr(directory).to_string_lossy().into_owned() };
    let result = create_dockerfile(&directory);
    match result {
        Ok(_) => {
            let c_string = CString::new(directory).unwrap();
            c_string.into_raw() // Return the C string
        }
        Err(err) => {
            eprintln!("Error unpacking image: {}", err);
            std::ptr::null() // Return null on error
        }
    }
}
