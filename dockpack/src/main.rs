use clap::{Arg, Command};
mod inner_api;


fn main() {
    // Create the Clap command line app
    let matches = Command::new("Docker Unpacker")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Unpacks Docker images into a specified directory")
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .value_name("IMAGE")
                .help("The name of the Docker image to unpack")  // Updated from .about() to .help()
                .required(true)
                // .takes_value(true),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("The directory to unpack the Docker image into")  // Updated from .about() to .help()
                .required(true)
                // .takes_value(true),
        )
        .get_matches();

    // Get the values of the arguments
    let image = matches.get_one::<String>("image").expect("Image argument is required");
    let directory = matches.get_one::<String>("directory").expect("Directory argument is required");

    // Call the unpacking function and handle the result
    match inner_api::unpack_files_from_image(image, directory) {
        Ok(path) => println!("Successfully unpacked to: {}", path),
        Err(e) => eprintln!("Error unpacking image: {}", e),
    }
}
