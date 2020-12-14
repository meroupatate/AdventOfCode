with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    x = 0
    y = 0
    trees = 0
    for line in lines:
        trees += lines[y][x % len(lines[y])] == "#"
        x += 3
        y += 1
    print(trees)