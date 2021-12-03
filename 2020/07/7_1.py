import re

with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    bags = {}
    for line in lines:
        data = re.findall('([a-z]+ [a-z]+) bags?', line)
        bags[data[0]] = data[1:]
    queue = ["shiny gold"]
    results = []
    while len(queue) != 0:
        for bag in bags.keys():
            if bag not in results and queue[0] in bags[bag]:
                results.append(bag)
                queue.append(bag)
        queue.pop(0)
    print(f"{len(results)} colors of bags can contain our shiny gold bag")