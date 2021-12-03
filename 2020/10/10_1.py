import mmh3
import bitarray


def path_finding(adapters, cur, visited, done):
    if cur == max(adapters) and not done[mmh3.hash(str(visited + [cur]))]:
        done[mmh3.hash(str(visited + [cur]))] = True
        return True
    res1 = False
    res2 = False
    res3 = False
    if cur+1 in adapters:
        res1 = path_finding(adapters, cur+1, visited + [cur], done)
    if cur+2 in adapters:
        res2 = path_finding(adapters, cur+2, visited + [cur], done)
    if cur+3 in adapters:
        res3 = path_finding(adapters, cur+3, visited + [cur], done)
    return res1 or res2 or res3 or False



with open("input2.txt", "r") as f:
    adapters = [int(line.strip()) for line in f.readlines()]
    adapters = sorted(adapters + [0, max(adapters) + 3])
    visited = []
    done = bitarray.bitarray(10000000000)
    done.setall(False)
    path_finding(adapters, min(adapters), visited, done)
    print(done.count())