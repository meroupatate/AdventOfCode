from helpers.utils import * 

DAY = 7


def first_part(input_file):
    crabs = list(map(int, extract_lines(DAY, input_file)[0].split(',')))
    min_fuel = len(crabs) ** 2
    for position in range(len(crabs)):
        fuel = compute_fuel_p1(crabs, position)
        if fuel < min_fuel:
            min_fuel = fuel
    return min_fuel
        

def compute_fuel_p1(crabs, position):
    fuel = 0
    for c in crabs:
        fuel += abs(c - position)
    return fuel

print("== FIRST PART ==")
assert_example(first_part, 37)
get_result(first_part)


def second_part(input_file):
    crabs = list(map(int, extract_lines(DAY, input_file)[0].split(',')))
    min_fuel = float('inf')
    for position in range(len(crabs)):
        fuel = compute_fuel_p2(crabs, position)
        if fuel < min_fuel:
            min_fuel = fuel
    return min_fuel


def compute_fuel_p2(crabs, position):
    fuel = 0
    for c in crabs:
        fuel += sum([i+1 for i in range(abs(c-position))])
    return fuel

print("== SECOND PART ==")
assert_example(second_part, 168)
get_result(second_part)
