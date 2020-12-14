with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    result = 1
    for slope in slopes:
        x = 0
        y = 0
        trees = 0
        while y < len(lines):
            trees += lines[y][x % len(lines[y])] == "#"
            x += slope[0]
            y += slope[1]
        result *= trees
    print(result)