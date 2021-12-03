import copy


def get_visible_seats(lines, i, j):
    max_i = len(lines)
    max_j = len(lines[0])
    visible_seats = set()

    # up
    for k in range(i-1, -1, -1):
        if lines[k][j] != ".":
            visible_seats.add((k, j))
            break
    # down
    for k in range(i+1, max_i):
        if lines[k][j] != ".":
            visible_seats.add((k, j))
            break
    # left
    for l in range(j-1, -1, -1):
        if lines[i][l] != ".":
            visible_seats.add((i, l))
            break
    # right
    for l in range(j+1, max_j):
        if lines[i][l] != ".":
            visible_seats.add((i, l))
            break
    # top left
    d = 1
    while i-d >= 0 and j-d >= 0:
        if lines[i-d][j-d] != ".":
            visible_seats.add((i-d, j-d))
            break
        d += 1
    # top right
    d = 1
    while i-d >= 0 and j+d < max_j:
        if lines[i-d][j+d] != ".":
            visible_seats.add((i-d, j+d))
            break
        d += 1
    # bottom left
    d = 1
    while i+d < max_i and j-d >= 0:
        if lines[i+d][j-d] != ".":
            visible_seats.add((i+d, j-d))
            break
        d += 1
    # botoom right
    d = 1
    while i+d < max_i and j+d < max_j:
        if lines[i+d][j+d] != ".":
            visible_seats.add((i+d, j+d))
            break
        d += 1
    return visible_seats


def all_empty(lines, seats):
    return all([lines[i][j] != "#" for i, j in seats])


def five_occupied(lines, seats):
    occupied = 0
    for i, j in seats:
        if lines[i][j] == "#":
            occupied += 1
    return occupied >= 5


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
            visibles = get_visible_seats(lines, i, j)
            if lines[i][j] == "L" and all_empty(lines, visibles):
                new_lines[i][j] = "#"
                updated = True
            elif lines[i][j] == "#" and five_occupied(lines, visibles):
                new_lines[i][j] = "L"
                updated = True
    lines = copy.deepcopy(new_lines)
#    for line in lines:
#        print("".join(line))
#    input()
print(f"There are {sum([line.count('#') for line in lines])} occupied seats")
