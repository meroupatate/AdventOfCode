import re

with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    run = set()
    accumulator = 0
    ptr = 0
    while ptr not in run:
        run.add(ptr)
        line = lines[ptr]
        print(line)
        inst, sign, arg = re.findall('([a-z]{3}) ([+-])(\d+)', line)[0]
        assert inst in ["acc", "nop", "jmp"]
        assert sign == '+' or sign == '-'
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
    print(f"The value in the accumulator is: {accumulator}")

