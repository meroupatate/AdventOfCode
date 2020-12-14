import re

with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    bags = {}
    for line in lines:
        print(line)
        data = re.findall('((?:\d+ )?[a-z]+ [a-z]+) bags?', line)
        bags[data[0]] = data[1:]
    queue = [("1", "shiny gold")]
    total = 0
    while len(queue) != 0:
        cur_nb, cur_type = queue[0]
        contained = bags[cur_type]
        for c in contained:
            if c == "no other":
                continue
            nb, type = re.findall("(\d+) ([a-z ]+)", c)[0]
            total += int(cur_nb) * int(nb)
            queue.append((int(cur_nb) * int(nb), type))
        queue.pop(0)
    print(f"A single shiny gold bag must contain {total} other bags")
