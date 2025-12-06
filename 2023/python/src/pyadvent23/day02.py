"""day02"""

import sys
import re
from pathlib import Path
from pyadvent23.tools import get_asset_file

DAY = 2

COLORS = {"red": 12, "green": 13, "blue": 14}

def get_game_number(game_str: str) -> int:
    match = re.fullmatch(r"Game ([0-9]*)", game_str)
    if match:
        return int(match.group(1))
    else:
        raise ValueError("Game number not found")

def get_cubes_number_and_color(cube_str: str) -> dict[str, int]:
    match = re.fullmatch(r"([0-9]*) (red|green|blue)", cube_str.strip())
    if match and match.group(1) and match.group(2):
        return dict([(str(match.group(2)), int(match.group(1)))])
    else:
        raise ValueError

def build_dict_of_cubes(subsets: str) -> dict[str, int]:
    # print(f"  - {subsets}")
    cube_subset = {}
    for random_cubes in subsets.split(","):
        # print(f"      - [{random_cubes}]")
        number_per_color = get_cubes_number_and_color(random_cubes)
        cube_subset.update(number_per_color)
    return cube_subset

def is_game_is_possible(game: dict[str, int], ref: dict[str, int]) -> bool:
    for cube_color, cube_number in game.items():
        if cube_number > ref[cube_color]:
            return False
    return True


def get_part_01(data: Path):
    result = 0
    with open(data) as input:
        for line in input:
            line = line.strip()
            game_str, cube_str = line.split(":")
            # print(f"Line: {game_str} - {cube_str}")
            game_number = get_game_number(game_str)
            for subsets in cube_str.split(";"):
                cubes = build_dict_of_cubes(subsets)
                # print(f"Throw: {cubes}")
                # print(f"{is_game_is_possible(cubes, COLORS)}")
                if not is_game_is_possible(cubes, COLORS):
                    break
            else:
                result += game_number
                # print(f"====== {result}")
    return result



def get_part_02(data: Path):
    """We have to search for each games the max number for each colors"""
    result = 0
    with open(data) as input:
        for line in input:
            line = line.strip()
            game_str, cube_str = line.split(":")
            game_number = get_game_number(game_str)
            minimal_set_of_cubes = {"red": 0, "green": 0, "blue": 0}
            power_of_cubes = 1
            for subsets in cube_str.split(";"):
                # cubes = build_dict_of_cubes(subsets)
                for cube_color, cube_number in build_dict_of_cubes(subsets).items():
                    minimal_set_of_cubes[cube_color] = cube_number if cube_number > minimal_set_of_cubes[cube_color] else minimal_set_of_cubes[cube_color]
            for min_color in minimal_set_of_cubes.values():
                power_of_cubes *= min_color
            result += power_of_cubes
        return result



if __name__ == "__main__":
    data_type = sys.argv[1]
    part = None
    if len(sys.argv) == 3:
        part = int(sys.argv[2])

    if part == 1:
        data_path = get_asset_file(DAY, data_type, part)
        print(get_part_01(data_path))
    elif part == 2:
        data_path = get_asset_file(DAY, data_type, part)
        print(get_part_02(data_path))
    elif part is None:
        data_path = get_asset_file(DAY, data_type, 1)
        print(get_part_01(data_path))
        data_path = get_asset_file(DAY, data_type, 2)
        print(get_part_02(data_path))
    else:
        raise ValueError(f"Argument part should be either 1 or 2 (and not {part})")
