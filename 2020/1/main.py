target = 2020


def find_pair(numbers):
    for i in range(len(numbers)):
        for j in range(len(numbers)):
            if i != j:
                a = numbers[i]
                b = numbers[j]
                if a + b == target:
                    return a, b

    return None, None


def find_tripplet(numbers):
    count = len(numbers)
    for i in range(count):
        for j in range(count):
            for l in range(count):
                if i != j and i != l and j != l:
                    a = numbers[i]
                    b = numbers[j]
                    c = numbers[l]
                    if a + b + c == target:
                        return a, b, c


def solve():
    numbers = []
    with open("input.txt", "r") as file:
        lines = file.readlines()
        for line in lines:
            numbers.append(int(line))

    a, b = find_pair(numbers)
    if a is None or b is None:
        print("Error, no pair found")
        exit(1)

    print(f"Part 1 solution is {a * b} from the numbers a = {a} and b = {b}")

    a, b, c = find_tripplet(numbers)
    if a is None or b is None or c is None:
        print("Error, no pair found")
        exit(1)

    print(f"Part 2 solution is {a * b * c} from the numbers a = {a} and b = {b} and c = {c}")


if __name__ == '__main__':
    solve()

