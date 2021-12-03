import re

f = open("input.txt", "r")
lines = [line.strip() for line in f.readlines()]
dirs = {"E": (1, 0),
        "W": (-1, 0),
        "N": (0, 1),
        "S": (0, -1)}
dirs2 = {0: (1, 0),
         90: (0, 1),
         180: (-1, 0),
         270: (0, -1)}
cur_dir = 0
cur_x = 0
cur_y = 0
for line in lines:
    data = re.findall("([A-Z])(\d+)", line)
    action = data[0][0]
    value = int(data[0][1])
    if action in dirs.keys():
        cur_x += dirs[action][0] * value
        cur_y += dirs[action][1] * value
    elif action == "R":
        cur_dir = (cur_dir - value) % 360
    elif action == "L":
        cur_dir = (cur_dir + value) % 360
    elif action == "F":
        cur_x += dirs2[cur_dir][0] * value
        cur_y += dirs2[cur_dir][1] * value
    print(cur_x, cur_y)
    print(abs(cur_x) + abs(cur_y))