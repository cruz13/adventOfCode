"""day01"""

import sys
from pathlib import Path

from pyadvent23.tools import get_asset_file

result1 = ""
result2 = ""
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


def get_first_digit(line: str) -> tuple[int, int]:
    for ind, char in enumerate(line):
        # print(f'ind: {ind} - char: {char}')
        try:
            int(char)
        except ValueError:
            continue
        return ind, int(char)
    raise ValueError


def get_first_digit_writen_in_letters(
    line: str, numbers: dict[str, int]
) -> tuple[int, int]:
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


def get_result_01(data: Path):
    print(f"Data path is : {data}")
    with open(data) as input:
        result = 0
        for line in input:
            line = line.strip()
            index_first, first = get_first_digit(line)
            index_last, last = get_first_digit(line[::-1])
            line_number = first * 10 + last
            result += line_number
    return result


def get_result_02(data: Path):
    print(f"Data path is : {data}")
    index_first = 0
    value = 0
    with open(data) as input:
        result = 0
        for line in input:
            line = line.strip()
            try:
                ind_first_number_in_digit, val_first_number_in_digit = get_first_digit(line)
            except ValueError:
                ind_first_number_in_digit = 99999
                val_first_number_in_digit = 0
            ind_first_number_in_letter, val_first_number_in_letter = get_first_digit_writen_in_letters(line, numbers)
            val_first = val_first_number_in_digit if ind_first_number_in_digit < ind_first_number_in_letter else val_first_number_in_letter

            try:
                ind_last_number_in_digit, val_last_number_in_digit = get_first_digit(line[::-1])
            except ValueError:
                ind_last_number_in_digit = 99999
                val_last_number_in_digit = 0
            numbers_in_reverse = {key[::-1]: value for key, value in numbers.items()}
            ind_last_number_in_letter, val_last_number_in_letter  = get_first_digit_writen_in_letters("".join(reversed(line)), numbers_in_reverse)
            val_last = val_last_number_in_digit if ind_last_number_in_digit < ind_last_number_in_letter else val_last_number_in_letter
 
            line_number = val_first * 10 + val_last
            # print(f"Line number : {line_number}")
            result += line_number
    return result


# def result(day: int, data_type: str, part: int, variance: int):
#     file = get_asset_file(day, data_type, part, variance)
#     # result1 = get_result_01(data_type)
#     result1 = "Not done"
#     result2 = "Not done"
#     return result1, result2


if __name__ == "__main__":
    data_type = sys.argv[1]
    part = None
    if len(sys.argv) == 3:
        part = int(sys.argv[2])

    if part == 1:
        data_path = get_asset_file(1, data_type, part)
        print(get_result_01(data_path))
    elif part == 2:
        data_path = get_asset_file(1, data_type, part)
        print(get_result_02(data_path))
    elif part is None:
        data_path = get_asset_file(1, data_type, 1)
        print(get_result_01(data_path))
        data_path = get_asset_file(1, data_type, 2)
        print(get_result_02(data_path))
    else:
        raise ValueError(f"Argument part should be either 1 or 2 (and not {part})")

