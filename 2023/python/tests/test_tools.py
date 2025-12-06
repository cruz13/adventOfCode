import pytest
from pyadvent23.tools import get_asset_file


def test_get_asset_file_input_missing_part():
    path = get_asset_file(1, "input", None)
    assert path.name == 'day01.input.txt'

def test_get_asset_file_input_wrong_part():
    path = get_asset_file(1, "input", 1)
    assert path.name == 'day01.input.txt'

def test_get_asset_file_example():
    with pytest.raises(ValueError) as exception:
        get_asset_file(1, "example", None)
    assert str(exception.value) == 'Argument part should not be None'

def test_get_asset_file_example_with_part():
    path = get_asset_file(1, "example", 2)
    assert path.name == 'day01.example.02.txt'
