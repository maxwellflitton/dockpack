# Dockpack
basic library for distributing files via docker

This tool is for unpacking the files from a Docker image into a directory, essentially
enabling you to use Docker to distribute files. Because Docker is integrated into
every major cloud provider and CI/CD tool, and has caching and version control built-in,
you can use Dockpack to package and distribute private code libraries of any language.
You can also combine multiple projects and tools in different language into a single Docker 
image for distribution. I've personally used this tool to distribute private Rust code
libraries, trained ML models, and private python packages. I've also used it to bundle
multiple tools and scripts together to setup a build package for servers.

## Installation
I plan on supporting `brew` and `apt get` in the future but for now you can
install the tool with cargo by using the following command:
```bash
cargo install dockpack
```

## Unpacking files from a Docker image
To unpack the files from a Docker image into a directory, you can the following `pull` command:
```bash
dockpack pull -i <image> -d <directory>
```
For a toy example, you can unpack the `maxwellflitton/nan-one` image into a directory called `cache`
with the command below:
```bash
dockpack pull -i maxwellflitton/nan-one -d cache
```
This will give you the following file structure:
```plaintext
├── cache
│   ├── Cargo.toml
│   ├── src
│   │   └── lib.rs
│   └── tar
│       ├── <Various tar files from the Docker image>
```

## Packing files into a Docker image

You can pack all files and subdirectories in the current working directory (except for anything in the `.dockerignore` file) into a Docker image with the following `build` command:
```bash
dockpack build -i <image>
```

We can then push the image to a Docker registry with the following `push` command:
```bash
dockpack push -i <image>
```

Though to be honest, a standard docker push will work just fine as you will see the packed files in you local images.

## Future features
- [X] Add a `push` command to pack push
- [X] Add a `build` command to pack files into a Docker image
- [ ] Add a `ls` command to list all the unpacked images
- [ ] Add a `rm` command to remove unpacked images
- [ ] Add data store for tracking unpacked images and their locations
- [ ] Add an update command to update downloaded images in their existing directories
- [ ] Add buckets for bundling multiple images together for distribution
- [ ] Dynamic C library so other languages can directly interact with core functionalities to build on top of it.


## Structure of Repo
The repo is structured as follows:

- `core-dockpack`: The core Rust library that other Rust programs can use to directly use the core functionalities of Dockpack.
- `coredockpack`: Wraps the `core-dockpack` library in a C interface for a dynamic C library so we can directly interact with the core functionalities of Dockpack from other languages.
- `dockpack`: The CLI tool that uses the `core-dockpack` library to provide the user with the functionalities of Dockpack in the terminal.
