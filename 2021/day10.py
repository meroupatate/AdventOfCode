from helpers.utils import * 

DAY = 10

points = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
}

closes = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">"
}

def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    score = 0
    for line in lines:
        expected_close_char = []
        for char in line:
            if char in closes.keys():
                expected_close_char.append(closes[char])
            elif char in closes.values():
                if char != expected_close_char.pop(-1):
                    score += points[char]
                    break
    return score


print("== FIRST PART ==")
assert_example(first_part, 26397)
get_result(first_part)


second_part_points = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4
    }

def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    scores = []
    for line in lines:
        score = 0
        is_corrupted = False
        expected_close_char = []
        for char in line:
            if char in closes.keys():
                expected_close_char.append(closes[char])
            elif char in closes.values():
                if char != expected_close_char.pop(-1):
                    is_corrupted = True
        if not is_corrupted:
            for char in expected_close_char[::-1]:
                score = score * 5 + second_part_points[char]
            scores.append(score)
    return sorted(scores)[(len(scores) + 1) // 2 - 1]


print("== SECOND PART ==")
assert_example(second_part, 288957)
get_result(second_part)