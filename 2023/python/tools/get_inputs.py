"""Download the inputs"""

import httpx
import sys
from pathlib import Path

DESTINATION_PATH = Path("src/pyadvent23/assets")
sessionToken = "53616c7465645f5f8f73a54f2d01c5b555bedb2e3905f4c4a6cbead0d837071ea06585b37bb420c713d1e398a5d0a68703b391e05b937b8abdc2ed7e60ffb50d"

def get_inputs(year: int, day: int) -> httpx.Response:
    """Get inputs for given year and day

    :param year: a year (4 digits)
    :type year: int
    :param day: a day from 1 to 25
    :type day: int
    :return: response of the http request
    :rtype: httpx.Response
    """
    # https://adventofcode.com/2023/day/1/input
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    headers = {"Cookie": "session=" + sessionToken}

    with httpx.Client() as client:
        resp = client.get(url, headers=headers)
    return resp


def save_inputs(response: httpx.Response, destination_path: Path):
    with open(destination_path / f"day{str(day).zfill(2)}.input.txt", "wb") as f:
        f.write(response.content)


def _get_assets_path(mypath: Path) -> Path:
    myparent = mypath.parent.parent
    return Path.joinpath(myparent, DESTINATION_PATH)


if __name__ == "__main__":
    print("Arguments", sys.argv)
    year = "2023"
    day = sys.argv[1]
    try:
        year = "2023"
    except ValueError:
        print(f"Error: given param for year ({year}) is not an integer")
    try:
        day = int(sys.argv[1])
    except ValueError:
        print(f"Error: given param for day ({day}) is not an integer")

    assets_path = _get_assets_path(Path(__file__))
    request = get_inputs(int(year), int(day))
    save_inputs(request, assets_path)
