"""Manage days"""
from pathlib import Path
import pathlib
from typing import Optional
from pyadvent23 import ASSET_DIRECTORY, DATA_TYPE


class Day:
    def __init__(self, day: int, data_type: Optional[str], part: Optional[int]) -> None:
        self.day = day
        self.data_type = self._set_data_type(data_type)
        self.part = part if part else 1
        self.data_path = self._get_asset_file(Path(ASSET_DIRECTORY), self.day, self.data_type, self.part)

    def _set_data_type(self, data_type: Optional[str]) -> str:
        if data_type not in DATA_TYPE:
            raise ValueError(f"{self.data_type} is not an allowed data type")
        return data_type if data_type else "input"

    def _get_asset_file(self, directory_path: pathlib.Path, day: int, data_type: str, part: Optional[int]) -> Path:
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
        file_name = ""
        if data_type == "input":
            file_name = f"day{str(day).zfill(2)}.{data_type}.txt"
        else:
            if not part:
                raise ValueError("Argument part should not be None")
            file_name = f"day{str(day).zfill(2)}.{data_type}.{str(part).zfill(2)}.txt"

        return directory_path / file_name

    def get_results_part1(self) -> int:
        return 0

    def get_results_part2(self) -> int:
        return 0

    def print_results(self) -> None:
        print(f"================== Day {self.day} ==================")
        print(self.get_results_part1())
        print(self.get_results_part2())
        print("====================================================")

    def print_result_part1(self) -> None:
        print("====== part 01 =====")
        print(self.get_results_part1())
        print("====================")

    def print_result_part2(self) -> None:
        print("====== part 02 =====")
        print(self.get_results_part2())
        print("====================")

