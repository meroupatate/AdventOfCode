from enum import Enum

class InputFile(Enum):
    EXAMPLE = "example_input.txt"
    REAL = "input.txt"


def extract_lines(day, input_file):
    return open(f"inputs/{day:02d}/{input_file.value}", "r").read().split()


def extract_ints(day, input_file):
    return list(map(int, extract_lines(day, input_file)))


def assert_example(f, expected):
    res = f(InputFile.EXAMPLE)
    print(f"Example: {res}")
    assert res == expected


def get_result(f):
    print(f"Result: {f(InputFile.REAL)}\n")