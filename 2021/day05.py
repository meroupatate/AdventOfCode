from helpers.utils import * 
import re

DAY = 5


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    covered = {}
    for line in lines:
        x1, y1, x2, y2 = map(int, re.findall(r"^(\d+),(\d+) -> (\d+),(\d+)$", line)[0])
        if x1 == x2:
            y_range = sorted([y1, y2])
            for y in range(y_range[0], y_range[1]+1):
                if (x1, y) not in covered.keys():
                    covered[(x1, y)] = 0
                covered[(x1, y)] += 1
        elif y1 == y2:
            x_range = sorted([x1, x2])
            for x in range(x_range[0], x_range[1]+1):
                if (x, y1) not in covered.keys():
                    covered[(x, y1)] = 0
                covered[(x, y1)] += 1
    return len([c for c in covered.keys() if covered[c] > 1])
            



print("== FIRST PART ==")
assert_example(first_part, 5)
get_result(first_part)

def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    covered = {}
    for line in lines:
        x1, y1, x2, y2 = map(int, re.findall(r"^(\d+),(\d+) -> (\d+),(\d+)$", line)[0])
        if x1 == x2:
            y_range = sorted([y1, y2])
            for y in range(y_range[0], y_range[1]+1):
                if (x1, y) not in covered.keys():
                    covered[(x1, y)] = 0
                covered[(x1, y)] += 1
        elif y1 == y2:
            x_range = sorted([x1, x2])
            for x in range(x_range[0], x_range[1]+1):
                if (x, y1) not in covered.keys():
                    covered[(x, y1)] = 0
                covered[(x, y1)] += 1
        elif abs(x1 - x2) == abs (y1 - y2):
            x_step = (x2 - x1) / abs(x1 - x2)
            y_step = (y2 - y1) / abs(y1 - y2)
            for i in range(abs(x1 - x2)+1):
                point = (x1 + i * x_step, y1 + i * y_step)
                if point not in covered.keys():
                    covered[point] = 0
                covered[point] += 1
    return len([c for c in covered.keys() if covered[c] > 1])


print("== SECOND PART ==")
assert_example(second_part, 12)
get_result(second_part)