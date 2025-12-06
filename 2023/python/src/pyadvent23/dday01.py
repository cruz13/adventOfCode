"""day01"""

import sys
from pathlib import Path
from typing import Optional

from pyadvent23 import day01
from pyadvent23.tools import get_asset_file
from pyadvent23.Day import Day

DAY = 1
numbers = {
    "zero": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


class Day01(Day):
    def __init__(self, data_type: Optional[str], part: Optional[int]) -> None:
        super().__init__(1, data_type, part)

    def get_first_digit(self, line: str) -> tuple[int, int]:
        for ind, char in enumerate(line):
            # print(f'ind: {ind} - char: {char}')
            try:
                int(char)
            except ValueError:
                continue
            return ind, int(char)
        raise ValueError

    def get_first_digit_writen_in_letters(self, line: str, numbers: dict[str, int]) -> tuple[int, int]:
        # print(f"- Line: {line}")
        ind, number = (1000000, 0)
        for numbers_in_word in numbers.keys():
            # print(f"    - Looking for {numbers_in_word}")
            try:
                if line.index(numbers_in_word) < ind:
                    ind = line.index(numbers_in_word)
                    number = numbers[numbers_in_word]
            except ValueError:
                continue
        return ind, number

    def get_results_part1(self) -> int:
        with open(self.data_path) as input:
            result = 0
            for line in input:
                line = line.strip()
                index_first, first = self.get_first_digit(line)
                index_last, last = self.get_first_digit(line[::-1])
                line_number = first * 10 + last
                result += line_number
        return result

    def get_results_part2(self) -> int:
        index_first = 0
        value = 0
        with open(self.data_path) as input:
            result = 0
            for line in input:
                line = line.strip()
                try:
                    ind_first_number_in_digit, val_first_number_in_digit = self.get_first_digit(line)
                except ValueError:
                    ind_first_number_in_digit = 99999
                    val_first_number_in_digit = 0
                ind_first_number_in_letter, val_first_number_in_letter = self.get_first_digit_writen_in_letters(line, numbers)
                val_first = val_first_number_in_digit if ind_first_number_in_digit < ind_first_number_in_letter else val_first_number_in_letter

                try:
                    ind_last_number_in_digit, val_last_number_in_digit = self.get_first_digit(line[::-1])
                except ValueError:
                    ind_last_number_in_digit = 99999
                    val_last_number_in_digit = 0
                numbers_in_reverse = {key[::-1]: value for key, value in numbers.items()}
                ind_last_number_in_letter, val_last_number_in_letter  = self.get_first_digit_writen_in_letters("".join(reversed(line)), numbers_in_reverse)
                val_last = val_last_number_in_digit if ind_last_number_in_digit < ind_last_number_in_letter else val_last_number_in_letter

                line_number = val_first * 10 + val_last
                # print(f"Line number : {line_number}")
                result += line_number
        return result



if __name__ == "__main__":
    data_type = sys.argv[1]
    part = None
    if len(sys.argv) == 3:
        part = int(sys.argv[2])

    day = Day01(data_type, part)
    day.print_results()
