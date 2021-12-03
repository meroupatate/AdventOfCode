import math
import re

f = open("input.txt", "r")
lines = [line.strip() for line in f.readlines()]
dirs = {"N": (0, 1),
        "W": (-1, 0),
        "S": (0, -1),
        "E": (1, 0)}
wp_x = 10
wp_y = 1
ship_x = 0
ship_y = 0
for line in lines:
    data = re.findall("([A-Z])(\d+)", line)
    action = data[0][0]
    value = int(data[0][1])
    if action in dirs.keys():
        wp_x += value * dirs[action][0]
        wp_y += value * dirs[action][1]
    elif action == "R":
        c = int(math.cos(math.radians(-value)))
        s = int(math.sin(math.radians(-value)))
        wp_x, wp_y = c * wp_x - s * wp_y, s * wp_x + c * wp_y
    elif action == "L":
        c = int(math.cos(math.radians(value)))
        s = int(math.sin(math.radians(value)))
        wp_x, wp_y = c * wp_x - s * wp_y, s * wp_x + c * wp_y
    elif action == "F":
        ship_x += value * wp_x
        ship_y += value * wp_y
print(abs(ship_x) + abs(ship_y))