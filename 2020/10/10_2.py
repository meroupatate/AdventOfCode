with open("input.txt", "r") as f:
    adapters = [int(line.strip()) for line in f.readlines()]
    adapters = sorted(adapters + [0, max(adapters) + 3])
    paths = [0 for i in range(len(adapters))]
    paths[0] = 1
    for i, ad in enumerate(adapters):
        for j in range(1, 4):
            print(ad - j)
            if ad - j in adapters:
                paths[i] += paths[adapters.index(ad-j)]
    print(paths[-1])
