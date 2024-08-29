use std::ffi::{CString, CStr};
use std::os::raw::c_char;

mod inner_api;


/// Unpacks the files from a Docker image into a directory.
///
/// # Arguments
/// * `image` - The name of the Docker image to unpack.
/// * `directory` - The directory to unpack the Docker image into.
///
/// # Returns
/// A C string with the path to the directory where the Docker image files are stored.
/// On error, returns a null pointer.
#[no_mangle]
pub extern "C" fn unpack_files_from_image_c(
    image: *const c_char, 
    directory: *const c_char
) -> *const c_char {
    // Convert C strings to Rust strings
    let image = unsafe { CStr::from_ptr(image).to_string_lossy().into_owned() };
    let directory = unsafe { CStr::from_ptr(directory).to_string_lossy().into_owned() };

    match inner_api::unpack_files_from_image(&image, &directory) {
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
