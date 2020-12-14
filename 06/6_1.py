with open("input.txt", "r") as f:
    lines = f.readlines()
    total = 0
    group = []
    for line in lines:
        if line == "\n":
            for char in group[0]:
                if all([char in person for person in group]):
                    total += 1
            group = []
        else:
            group.append(line.strip())
    for char in group[0]:
        if all([char in person for person in group]):
            total += 1
    print(f"Total: {total}")