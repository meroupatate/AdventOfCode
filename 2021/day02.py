from helpers.utils import * 

DAY = 2


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    x = 0
    y = 0
    for line in lines:
        parsed_line = line.split()
        direction, count = parsed_line[0], int(parsed_line[1])
        if direction == "forward":
            x += count
        elif direction == "down":
            y += count
        elif direction == "up":
            y -= count
    return x * y


print("== FIRST PART ==")
assert_example(first_part, 150)
get_result(first_part)


def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    x = 0
    y = 0
    aim = 0
    for line in lines:
        parsed_line = line.split()
        direction, count = parsed_line[0], int(parsed_line[1])
        if direction == "forward":
            x += count
            y += count * aim
        elif direction == "down":
            aim += count
        elif direction == "up":
            aim -= count
    return x * y


print("== SECOND PART ==")
assert_example(second_part, 900)
get_result(second_part)