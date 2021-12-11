from helpers.utils import * 

DAY = 9

def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    low_points_height = []
    for i in range(len(lines)):
        for j in range(len(lines[0])):
            possible_neighbors = [
                (i-1, j-1), (i-1, j), (i-1, j+1),
                (i, j-1), (i, j+1),
                (i+1, j-1), (i+1, j), (i+1, j+1)
            ]
            neighbors = [neighbor for neighbor in possible_neighbors if 0 <= neighbor[0] < len(lines) and 0 <= neighbor[1] < len(lines[0])]
            lower_neighbors = [neighbor for neighbor in neighbors if lines[neighbor[0]][neighbor[1]] <= lines[i][j]]
            is_low_point = len(lower_neighbors) == 0
            if is_low_point:
                low_points_height.append(int(lines[i][j]))
    return sum(low_points_height) + len(low_points_height)




print("== FIRST PART ==")
assert_example(first_part, 15)
get_result(first_part)


def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    largest_basin = []
    for i in range(len(lines)):
        for j in range(len(lines[0])):
            neighbors = get_neighbors(lines, i, j)
            lower_neighbors = [neighbor for neighbor in neighbors if lines[neighbor[0]][neighbor[1]] <= lines[i][j]]
            is_low_point = len(lower_neighbors) == 0
            if is_low_point:
                size = find_basin_size_from_low_point(lines, [(i, j)], [(i, j)])
                if len(largest_basin) < 3:
                    largest_basin.append(size)
                    largest_basin = sorted(largest_basin)
                elif size > largest_basin[0]:
                    largest_basin = sorted([size] + largest_basin[1:])
    return largest_basin[0] * largest_basin[1] * largest_basin[2]


def get_neighbors(lines, i, j):
    possible_neighbors = [
        (i-1, j),
        (i, j-1), (i, j+1),
        (i+1, j)
    ]
    return [neighbor for neighbor in possible_neighbors if 0 <= neighbor[0] < len(lines) and 0 <= neighbor[1] < len(lines[0])]

def find_basin_size_from_low_point(lines, visited, queue):
    ci, cj = queue.pop(0)
    neighbors = get_neighbors(lines, ci, cj)
    count = 1
    for ni, nj in neighbors:
        if belongs_to_same_basin(int(lines[ci][cj]), int(lines[ni][nj])) and (ni, nj) not in visited:
            queue.append((ni, nj))
            visited.append((ni, nj))
            count += find_basin_size_from_low_point(lines, visited, queue)
    return count
    
def belongs_to_same_basin(cur, neighbor):
    return neighbor >= cur and neighbor != 9


print("== SECOND PART ==")
assert_example(second_part, 1134)
get_result(second_part)