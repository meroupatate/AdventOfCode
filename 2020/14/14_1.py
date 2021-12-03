import re


f = open("input", 'r')
lines = [line.strip() for line in f.readlines()]
mem = {}
cur_mask = 'X' * 36

def apply_mask(value, mask):
    result = list('{0:036b}'.format(value))
    for i, bit in enumerate(mask):
        if bit in ['0', '1']:
            result[i] = bit
    result = ''.join(result)
    return int(result, 2)



for line in lines:
    if 'mask' in line:
        cur_mask = re.findall(r'mask = ([01X]{36})', line)[0]
    else:
        indice, value = re.findall(r'mem\[(\d+)\] = (\d+)', line)[0]
        result = apply_mask(int(value), cur_mask)
        mem[indice] = result

print(f"The sum of all values in memory after initialization is: {sum(mem.values())}")