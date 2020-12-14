import re


def valid_height(height):
    if "cm" in height:
        h = re.findall("([0-9]+)cm", height)[0]
        return 150 <= int(h) <= 193
    elif "in" in height:
        h = re.findall("([0-9]+)in", height)[0]
        return 59 <= int(h) <= 76
    return False


def valid_hair_color(color):
    return len(color) == 7 and len(re.findall("#[0-9a-f]{6}", color))


def valid_eye_color(color):
    return color in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


def valid_pid(pid):
    return len(pid) == 9 and len(re.findall("[0-9]{9}", pid))


def is_valid(passport):
    keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    if all([key in passport for key in keys]):
        conditions = [1920 <= int(passport["byr"]) <= 2002,
                      2010 <= int(passport["iyr"]) <= 2020,
                      2020 <= int(passport["eyr"]) <= 2030,
                      valid_height(passport["hgt"]),
                      valid_hair_color(passport["hcl"]),
                      valid_eye_color(passport["ecl"]),
                      valid_pid(passport["pid"])
                      ]
        return all(conditions)
    return False


with open("input.txt", "r") as f:
    lines = f.readlines()
    valids = 0
    current = {}
    for line in lines:
        if line == "\n":
            valids += is_valid(current)
            current = {}
        else:
            data = re.findall(r'([a-z]+):([#a-z0-9]+)', line.strip())
            for d in data:
                current[d[0]] = d[1]
    print(valids)