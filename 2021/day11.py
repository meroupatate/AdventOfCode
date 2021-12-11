from helpers.utils import * 

DAY = 11


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    lines = [list(map(int, list(line))) for line in lines]
    total_flashes = 0
    for _ in range(100):
        lines, flashes = step(lines)
        total_flashes += flashes
    return total_flashes


def step(lines):
    lines = [
        list(map(lambda x: x+1, line)) for line in lines
    ]
    flashed = []
    need_loop = True
    while need_loop:
        for i in range(len(lines)):
            for j in range(len(lines[0])):
                octopus = lines[i][j]
                if octopus > 9 and (i, j) not in flashed:
                    neighbors = get_neighbors(lines, i, j)
                    for ni, nj in neighbors:
                        if (ni, nj) not in flashed:
                            lines[ni][nj] += 1
                    lines[i][j] = 0
                    flashed.append((i, j))
        need_loop = any([octopus > 9  for p in range(len(lines)) for octopus in lines[p]])

    return lines, len(flashed)

def get_neighbors(lines, i, j):
    possible_neighbors = [
        (i-1, j-1), (i-1, j), (i-1, j+1),
        (i, j-1), (i, j+1),
        (i+1, j-1), (i+1, j), (i+1, j+1)
    ]
    return [neighbor for neighbor in possible_neighbors if 0 <= neighbor[0] < len(lines) and 0 <= neighbor[1] < len(lines[0])]

print("== FIRST PART ==")
assert_example(first_part, 1656)
get_result(first_part)

def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    lines = [list(map(int, list(line))) for line in lines]
    steps = 0
    all_flashes = False
    while not all_flashes:
        lines, flashes = step(lines)
        all_flashes = all([octopus == 0 for i in range(len(lines)) for octopus in lines[i]])
        steps += 1
    return steps 


print("== SECOND PART ==")
assert_example(second_part, 195)
get_result(second_part)