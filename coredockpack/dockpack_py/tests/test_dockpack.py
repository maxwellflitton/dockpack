from dockpack_py import (
    unpack_files_from_image,
    build_files_from_image,
    create_dockerfile,
)
import os
import shutil


def test_execute_unpack():
    image_name = "maxwellflitton/nan-one"
    directory = "./cache/two"
    if os.path.exists(directory):
        shutil.rmtree(directory)

    result = unpack_files_from_image(image_name, directory)
    assert os.path.exists(directory)
    assert result == directory
    shutil.rmtree(directory)


# This currently fails/panics, because pushing to remote needs remote auth
# It's the same failure/behaviour as the rust crate test
def test_push(test_image_dir, dockerfile_path):
    image_name = "test_image:latest"
    with open(dockerfile_path, "w") as f:
        f.write("FROM scratch\nCOPY . .\n")
    res = build_files_from_image(image_name, "./cache")
    assert res == dir


def test_build(test_image_dir):
    create_dockerfile(str(test_image_dir))
    dockerfile_path = os.path.join(test_image_dir, "Dockerfile")
    with open(dockerfile_path, "r") as f:
        file_content = f.read()
    exp = "FROM scratch\nCOPY . .\n"
    assert file_content == exp
    print(file_content)
