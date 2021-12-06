from helpers.utils import * 

DAY = 6


def first_part(input_file):
    timer = 6
    days = 80
    fishes = list(map(int, extract_lines(DAY, input_file)[0].split(",")))
    for _ in range(days):
        for f in range(len(fishes)):
            if fishes[f] == 0:
                fishes.append(timer + 2)
                fishes[f] = timer + 1
            fishes[f] -= 1
    return len(fishes)

print("== FIRST PART ==")
assert_example(first_part, 5934)
get_result(first_part)


def second_part(input_file):
    max_timer = 7
    days = 256
    fishes = list(map(int, extract_lines(DAY, input_file)[0].split(",")))
    fish_number = len(fishes)
    newborns = {d: fishes.count(d-1) for d in range(days+1)}
    for d in range(days+1):
        if d+max_timer+2 < days+1:
            newborns[d+max_timer+2] += newborns[d]
        if d+max_timer < days+1:
            newborns[d+max_timer] += newborns[d]
        fish_number += newborns[d]
    return fish_number
    


print("== SECOND PART ==")
assert_example(second_part, 26984457539)
get_result(second_part)