with open("input.txt", "r") as f:
    numbers = [int(line.strip()) for line in f.readlines()]
    valid = True
    i = 25
    while i < len(numbers) and valid:
        valid = False
        for num1 in numbers[i-25:i]:
            for num2 in numbers[i-25:i]:
                if num1 != num2 and num1 + num2 == numbers[i]:
                    valid = True
        i += 1
    print(f"The first invalid number is {numbers[i-1]}")

    invalid = numbers[i-1]
    for j in range(len(numbers)):
        total = 0
        k = 0
        while total < invalid:
            total += numbers[j+k]
            k += 1
        if total == invalid and k > 1:
            print(sum(numbers[j:j+k]))
            print(max(numbers[j:j+k]) + min(numbers[j:j+k]))




