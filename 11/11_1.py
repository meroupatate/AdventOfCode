import copy

def get_adjacent(i, j, max_i, max_j):
    candidates = [(i-1, j-1), (i-1, j), (i-1, j+1),
                 (i, j-1), (i, j+1),
                 (i+1, j-1), (i+1, j), (i+1, j+1)]
    adjacents = []
    for can in candidates:
        if 0 <= can[0] < max_i and 0 <= can[1] < max_j:
            adjacents.append(can)
    return adjacents


def all_empty(lines, seats):
    return all([lines[i][j] in ["L", "."] for i, j in seats])


def four_occupied(lines, seats):
    occupied = 0
    for i, j in seats:
        if lines[i][j] == "#":
            occupied += 1
    return occupied >= 4


f = open("input.txt", "r")
lines = [list(line.strip()) for line in f.readlines()]
new_lines = copy.deepcopy(lines)
max_i = len(lines)
max_j = len(lines[0])

updated = True
while updated:
    updated = False
    for i in range(max_i):
        for j in range(max_j):
            adjacents = get_adjacent(i, j, max_i, max_j)
            if lines[i][j] == "L" and all_empty(lines, adjacents):
                new_lines[i][j] = "#"
                updated = True
            elif lines[i][j] == "#" and four_occupied(lines, adjacents):
                new_lines[i][j] = "L"
                updated = True
    lines = copy.deepcopy(new_lines)
print(f"There are {sum([line.count('#') for line in lines])} occupied seats")