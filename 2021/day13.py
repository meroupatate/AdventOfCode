from helpers.utils import *
import re

DAY = 13

class Paper:
    def __init__(self, dots):
        self.max_x = max([dot[0] for dot in dots]) + 1
        self.max_y = max([dot[1] for dot in dots]) + 1
        self.lines = [
            [ "." for _ in range(self.max_x) ] for _ in range(self.max_y)
        ]
        for dx, dy in dots:
            self.lines[dy][dx] = "#"

    def __repr__(self):
        res = ""
        for line in self.lines:
            res += "".join(line) + "\n"
        return res

    def fold(self, axis, index):
        if axis == "x":
            self.max_x = index
        elif axis == "y":
            self.max_y = index
        folded_paper = [
            [ "." for _ in range(self.max_x) ] for _ in range(self.max_y)
        ]
        for i in range(self.max_x):
            for j in range(self.max_y):
                if axis == "x":
                    if self.lines[j][i] == "#" or self.lines[j][-i-1] == "#":
                        folded_paper[j][i] = "#"
                elif axis == "y":
                    if self.lines[j][i] == "#" or self.lines[-j-1][i] == "#":
                        folded_paper[j][i] = "#"
        self.lines = folded_paper.copy()
    
    def count_dots(self):
        dots = 0
        for j in range(self.max_y):
            dots += self.lines[j].count("#")
        return dots


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    sep = lines.index("")
    dots = list(map(lambda line: list(map(int, line.split(','))), lines[:sep]))
    instructions = lines[sep+1:]
    paper = Paper(dots)
    axis, index = parse_instruction(instructions[0])
    paper.fold(axis, index)
    return paper.count_dots()
    


def parse_instruction(instruction):
    axis, index = re.findall(r'fold along ([xy])=(\d+)', instruction)[0]
    return axis, int(index)



print("== FIRST PART ==")
assert_example(first_part, 17)
get_result(first_part)


def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    sep = lines.index("")
    dots = list(map(lambda line: list(map(int, line.split(','))), lines[:sep]))
    instructions = lines[sep+1:]
    paper = Paper(dots)
    for instruction in instructions:
        axis, index = parse_instruction(instruction)
        paper.fold(axis, index)
    print(paper)


print("== SECOND PART ==")
get_result(second_part)