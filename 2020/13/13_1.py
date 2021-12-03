f = open("input.txt", "r")
earliest = int(f.readline().strip())
buses = list(filter(('x').__ne__, f.readline().strip().split(',')))
depart = earliest
found = False
while not found:
    for bus in buses:
        if not depart % int(bus):
            found = True
            print(f"The earliest bus you could taks is bus ID {bus} with {depart - earliest} minutes waiting: {bus} * {depart - earliest} = {int(bus) * (depart - earliest)}")
            break
    depart += 1