f = open("input.txt", "r")
trash = f.readline()
buses = {i: int(bus) for i, bus in enumerate(f.readline().strip().split(',')) if bus != 'x'}

def gcdExtended(a, b):
    if a == 0:
        return b, 0, 1
    gcd, x1, y1 = gcdExtended(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return gcd, x, y


# use chinese remainder theorem: we have res = -i % bus
N = 1
for bus in buses.values():
    N *= bus

res = 0
for i, ni in buses.items():
    Ni = N // ni
    g, x, y = gcdExtended(ni, Ni)
    res += (-i) * y * Ni

print(f"The earliest timestamp is: {res % N}")