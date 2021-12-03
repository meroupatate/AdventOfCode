puzzle = [9, 3, 1, 0, 8, 4]
last_seen = {puzzle[i]: i+1
             for i in range(len(puzzle)-1)}
nb = len(puzzle)
last_spoken = puzzle[-1]

while nb != 30000000:
    if last_spoken not in last_seen:
        spoken = 0
    else:
        spoken = nb - last_seen[last_spoken]
    last_seen[last_spoken] = nb
    last_spoken = spoken
    nb += 1

print(f"The {nb}th number spoken is: {last_spoken}")
