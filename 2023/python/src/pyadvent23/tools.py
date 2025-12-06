from pathlib import Path
from typing import Optional


def get_asset_file(day: int, data_type: str, part: Optional[int]) -> Path:
    """Return Path to the asset file

    The asset file might be either:
    - an input type (data for the response)
    - an example (the example given each times)
    There's usually two parts. The examples might be different for part 1 and part 2
    For the actual response (type input), there's usually only one file.

    :param day: day
    :type day: int
    :param data_type: type of data (either input or example)
    :type data_type: str
    :param part: part of the day
    :type part: int

    :return: path of the asset file
    :rtype: pathlib.Path
    """
    ASSET_DIRECTORY = Path("src/pyadvent23/assets")
    file_name = ""
    if data_type == "input":
        file_name = f"day{str(day).zfill(2)}.{data_type}.txt"
    else:
        if not part:
            raise ValueError("Argument part should not be None")
        file_name = f"day{str(day).zfill(2)}.{data_type}.{str(part).zfill(2)}.txt"

    return ASSET_DIRECTORY / file_name
