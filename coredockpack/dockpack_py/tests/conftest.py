import pytest
import os


@pytest.fixture
def cache_dir(tmp_path):
    """A fixture for a cache directory pathlib.Path."""
    return tmp_path


@pytest.fixture
def test_image_dir(tmp_path):
    """A fixture for a test_image directory which mimics pathlib.Path."""
    return tmp_path


@pytest.fixture
def dockerfile_path(test_image_dir):
    return os.path.join(str(test_image_dir), "test_image:latest")
