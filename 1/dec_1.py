with open("input_1.txt", "r") as f:
    lines = f.readlines()
    for line in lines:
        for other in lines:
            for third in lines:
                if other != line and third != line and third != other:
                    if int(line.rstrip()) + int(other.rstrip()) + int(third.rstrip()) == 2020:
                        print(f"{line.rstrip()} * {other.rstrip()} * {third.rstrip()} = {int(line.rstrip()) * int(other.rstrip()) * int(third.rstrip())}")