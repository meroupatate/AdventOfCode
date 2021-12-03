import re


f = open("input", "r")


# define a Rule object
class Rule:
    def __init__(self, name, min1, max1, min2, max2):
        self.name = name
        self.min1 = int(min1)
        self.max1 = int(max1)
        self.min2 = int(min2)
        self.max2 = int(max2)

    def is_valid(self, value):
        return self.min1 <= value <= self.max1 or self.min2 <= value <= self.max2

    def __repr__(self):
        return f'Rule <{self.name}>: {self.min1}-{self.max1} or {self.min2}-{self.max2}'


rules = []
# parse rules
cur = f.readline()
while 'or' in cur:
    rule_t = re.findall(r'(.+): (\d+)-(\d+) or (\d+)-(\d+)', cur)[0]
    rules.append(Rule(*rule_t))
    cur = f.readline()

# get my ticket
f.readline()
my_ticket = list(map(int, f.readline().strip().split(',')))

# get all nearby tickets
f.readline()
f.readline()
nearby_tickets = [list(map(int, line.strip().split(','))) for line in f.readlines()]

# remove invalid tickets
valid_nearby_tickets = []
for ticket in nearby_tickets:
    valid_ticket = True
    for value in ticket:
        valid = False
        for rule in rules:
            if rule.is_valid(value):
                valid = True
        if not valid:
            valid_ticket = False
    if valid_ticket:
        valid_nearby_tickets.append(ticket)

# for each ticket value, get matching rules
matching_rules = {i: [] for i in range(len(rules))}
for i in range(len(valid_nearby_tickets[0])):
    for rule in rules:
        matching = all([rule.is_valid(ticket[i]) for ticket in valid_nearby_tickets])
        if matching:
            matching_rules[i].append(rule)

# associate tickets with indexes
done = []
final_association = {}
while len(final_association.keys()) != len(matching_rules.keys()):
    for i, rules in matching_rules.items():
        if len(rules) == 1:
            final_association[rules[0].name] = i
            done.append(rules[0])
        else:
            for rule in rules:
                if rule in done:
                    rules.remove(rule)


# get final result by multiplying the six fields that start with "departure"
result = 1
count = 0
for rule_name, i in final_association.items():
    if re.match(r'^departure', rule_name):
        count += 1
        result *= my_ticket[i]
print(f"The product of the {count} fields that start with the word departure is: {result}")