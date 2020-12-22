import re

f = open("input", "r")
rules = []

# parse rules
cur = f.readline()
while 'or' in cur:
    rule = re.findall(r'.+: (\d+)-(\d+) or (\d+)-(\d+)', cur)[0]
    rules.append([int(rule[0]), int(rule[1])])
    rules.append([int(rule[2]), int(rule[3])])
    cur = f.readline()

# ignore my ticket
while 'nearby tickets' not in cur:
    cur = f.readline()

# read all nearby tickets
invalid = 0
lines = [line.strip() for line in f.readlines()]
for line in lines:
    ticket = line.split(',')
    for num in ticket:
        valid = False
        for r in rules:
            if r[0] <= int(num) <= r[1]:
                valid = True
        if not valid:
            invalid += int(num)

print(invalid)