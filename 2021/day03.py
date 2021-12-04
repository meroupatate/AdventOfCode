from helpers.utils import * 
from collections import Counter

DAY = 3


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    columns = {i: {"0": 0, "1": 0} for i in range(len(lines[0]))}
    for line in lines:
        for i, char in enumerate(line):
            columns[i][char] += 1
    gamma = ""
    epsilon = ""
    for c in columns:
        if columns[c]["0"] > columns[c]["1"]:
            gamma += "0"
            epsilon += "1"
        else:
            gamma += "1"
            epsilon += "0"
    return int(gamma, 2) * int(epsilon, 2)

            
print("== FIRST PART ==")
assert_example(first_part, 198)
get_result(first_part)


def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    copy = lines.copy()
    column = 0
    while len(copy) != 1:
        most_common_bit = get_most_common_bit([line[column] for line in copy])
        copy = [c for c in copy if c[column] == most_common_bit]
        column += 1
    oxygen = int(copy[0], 2)

    copy = lines.copy()
    column = 0
    while len(copy) != 1:
        least_common_bit = get_least_common_bit([line[column] for line in copy])
        copy = [c for c in copy if c[column] == least_common_bit]
        column += 1
    co2 = int(copy[0], 2)
    return oxygen * co2
    

def get_most_common_bit(column):
    bits_count = [0, 0]
    for c in column:
        bits_count[int(c)] += 1
    if bits_count[0] > bits_count[1]:
        return "0"
    return "1"


def get_least_common_bit(column):
    bits_count = [0, 0]
    for c in column:
        bits_count[int(c)] += 1
    if bits_count[0] > bits_count[1]:
        return "1"
    return "0"


print("== SECOND PART ==")
assert_example(second_part, 230)
get_result(second_part)