import re

with open("input.txt", "r") as f:
    valid = 0
    lines = [line.strip() for line in f.readlines()]
    for line in lines:
        mini, maxi, char, string = re.findall(r"(\d+)-(\d+) ([a-z]): ([a-z]+)", line)[0]
        valid += int(mini) <= string.count(char) <= int(maxi)
    print(valid)