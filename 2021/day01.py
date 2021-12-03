from helpers.utils import * 

DAY = 1

def first_part(input_file):
    lines = extract_ints(DAY, input_file)
    result = 0
    for i in range(1, len(lines)):
        if lines[i] > lines[i-1]:
            result += 1
    return result

print("== FIRST PART ==")
assert_example(first_part, 7)
get_result(first_part)


def second_part(input_file):
    lines = extract_ints(DAY, input_file)
    result = 0
    i = 1
    previous_sum = sum(lines[:3])
    while i+3 < len(lines)+1:
        current_sum = sum(lines[i:i+3])
        if current_sum > previous_sum:
            result += 1
        previous_sum = current_sum
        i += 1
    return result

print("== SECOND PART ==")
assert_example(second_part, 5)
get_result(second_part)
