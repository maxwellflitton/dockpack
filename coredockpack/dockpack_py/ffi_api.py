from .dockpack_py import ffi, lib


# I tried this with a decorator to be more DRY and it was way less readable
def unpack_files_from_image(image_name: str, directory: str) -> str:
    image: bytes = image_name.encode("utf-8")
    dir: bytes = directory.encode("utf-8")
    return ffi.string(lib.unpack_files_from_image_c(image, dir)).decode("utf-8")


def build_files_from_image(image_name: str, directory: str) -> str:
    image: bytes = image_name.encode("utf-8")
    dir: bytes = directory.encode("utf-8")
    return ffi.string(lib.build_image_from_files_c(image, dir)).decode("utf-8")


def create_dockerfile(directory: str) -> str:
    dir: bytes = directory.encode("utf-8")
    return ffi.string(lib.create_dockerfile_c(dir)).decode("utf-8")
