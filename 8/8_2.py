import re


def parse_line(line):
    inst, sign, arg = re.findall('([a-z]{3}) ([+-])(\d+)', line)[0]
    assert inst in ["acc", "nop", "jmp"]
    assert sign == '+' or sign == '-'
    return inst, sign, arg


with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    for i, line in enumerate(lines):
        inst, sign, arg = parse_line(line)

        if inst in ["nop", "jmp"]:
            modified = f'{["nop", "jmp"][inst == "nop"]} {sign}{arg}'
            candidate = lines[:i] + [modified] + lines[i+1:]
            done = set()
            accumulator = 0
            ptr = 0
            while ptr < len(lines) and ptr not in done:
                done.add(ptr)
                cur = candidate[ptr]
                inst, sign, arg = parse_line(cur)
                if inst == "acc":
                    if sign == '+':
                        accumulator += int(arg)
                    else:
                        accumulator -= int(arg)
                    ptr += 1
                elif inst == "jmp":
                    if sign == '+':
                        ptr += int(arg)
                    else:
                        ptr -= int(arg)
                else:
                    ptr += 1
            if ptr >= len(lines):
                print(f"The value in the accumulator is: {accumulator}")
                break

