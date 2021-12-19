from enum import Enum

class InputFile(Enum):
    EXAMPLE = "example_input.txt"
    EXAMPLE2 = "example_input_2.txt"
    EXAMPLE3 = "example_input_3.txt"
    REAL = "input.txt"


def extract_lines(day, input_file):
    return open(f"inputs/{day:02d}/{input_file.value}", "r").read().splitlines()


def extract_ints(day, input_file):
    return list(map(int, extract_lines(day, input_file)))


def assert_example(f, expected):
    res = f(InputFile.EXAMPLE)
    print(f"Example: {res}")
    assert res == expected


def assert_example2(f, expected):
    res = f(InputFile.EXAMPLE2)
    print(f"Example: {res}")
    assert res == expected


def assert_example3(f, expected):
    res = f(InputFile.EXAMPLE3)
    print(f"Example: {res}")
    assert res == expected


def get_result(f):
    print(f"Result: {f(InputFile.REAL)}\n")