import re

with open("input.txt", "r") as f:
    valid = 0
    lines = [line.strip() for line in f.readlines()]
    for line in lines:
        i, j, char, string = re.findall(r"(\d+)-(\d+) ([a-z]): ([a-z]+)", line)[0]
        i, j = int(i), int(j)
        valid += (string[i-1] == char) ^ (string[j-1] == char)
    print(valid)