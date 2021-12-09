from helpers.utils import * 

DAY = 8


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    count = 0
    for line in lines:
        output = line.split(" | ")[1].split(" ")
        for c in output:
            if len(c) in [2, 3, 4, 7]:
                count += 1
    return count

        


print("== FIRST PART ==")
assert_example(first_part, 26)
get_result(first_part)


def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    total = 0
    for line in lines:
        digits, output = line.split(" | ")
        mapping = dict()
        digits = ["".join(sorted(digit)) for digit in digits.split(' ')]
        output = ["".join(sorted(digit)) for digit in output.split(' ')]
        six_segments_digits = []
        five_segments_digits = []
        for digit in digits:
            if len(digit) == 2:
                mapping[1] = digit
            elif len(digit) == 3:
                mapping[7] = digit
            elif len(digit) == 4:
                mapping[4] = digit
            elif len(digit) == 7:
                mapping[8] = digit
            elif len(digit) == 6:
                six_segments_digits.append(digit)
            elif len(digit) == 5:
                five_segments_digits.append(digit)

        topright_or_bottomright = list(mapping[1])
        top = [c for c in mapping[7] if c not in mapping[1]][0]
        middle_or_topleft = [c for c in mapping[4] if c not in mapping[7]]
        for ssd in six_segments_digits:
            for segment in topright_or_bottomright:
                if segment not in ssd:
                    mapping[6] = ssd
                    topright = segment
                    bottomright = [segment for segment in topright_or_bottomright if segment != topright][0]
            for segment in middle_or_topleft:
                if segment not in ssd:
                    mapping[0] = ssd
                    middle = segment
                    topleft = [segment for segment in middle_or_topleft if segment != middle][0]
        mapping[9] = "".join([digit for digit in six_segments_digits if digit != mapping[6] and digit != mapping[0]])
        for fsd in five_segments_digits:
            if bottomright not in fsd and topleft not in fsd:
                mapping[2] = fsd
        five_or_three = [fsd for fsd in five_segments_digits if fsd != mapping[2]]
        mapping[5] = [segment for segment in five_or_three if topright not in segment][0]
        mapping[3] = [segment for segment in five_or_three if segment != mapping[5]][0]
        reversed_mapping = {v: k for k, v in mapping.items()}
        code = int("".join([str(reversed_mapping[digit]) for digit in output]))
        total += code
    return total


print("== SECOND PART ==")
assert_example(second_part, 61229)
get_result(second_part)
