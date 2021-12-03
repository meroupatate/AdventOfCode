import re


f = open("input", 'r')
lines = [line.strip() for line in f.readlines()]
mem = {}
cur_mask = 'X' * 36


def gen_addr(indice, mask):
    base = list('{0:036b}'.format(int(indice)))
    addresses = [""]
    for i, bit in enumerate(mask):
        if bit == '0':
            new_addresses = []
            for addr in addresses:
                new_addresses.append(addr + base[i])
            addresses = new_addresses
        elif bit == '1':
            new_addresses = []
            for addr in addresses:
                new_addresses.append(addr + '1')
            addresses = new_addresses
        else:
            new_addresses = []
            for addr in addresses:
                new_addresses.append(addr + '0')
                new_addresses.append(addr + '1')
            addresses = new_addresses
    for addr in addresses:
        assert len(addr) == 36
    return [int(addr, 2) for addr in addresses]



for line in lines:
    if 'mask' in line:
        cur_mask = re.findall(r'mask = ([01X]{36})', line)[0]
    else:
        indice, value = re.findall(r'mem\[(\d+)\] = (\d+)', line)[0]
        addresses = gen_addr(indice, cur_mask)
        for addr in addresses:
            mem[addr] = int(value)


print(f"The sum of all values in memory after initialization is: {sum(mem.values())}")