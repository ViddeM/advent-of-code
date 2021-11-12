with open("input.txt") as file:
    input = file.read().split("-")

low = int(input[0])
high = int(input[1])


def test_pass(num):
    repeats = {}
    prev = -1
    for a in [int(digit) for digit in str(num)]:
        if a not in repeats.keys():
            repeats[a] = 0
        repeats[a] += 1
        if a < prev:
            return False, False
        prev = a

    # Returns first part 1 and then part 2
    a = True in [True for b in repeats.values() if b >= 2]
    b = 2 in repeats.values()
    return a, b


# Part 1
num_ok = 0
# Part 2
num_o_2 = 0
for num in range(low, high + 1):
    part_1, part_2 = test_pass(num)
    if part_1:
        num_ok += 1
    if part_2:
        num_o_2 += 1

print("1::Found: " + str(num_ok) + " passwords")
print("2::Found: " + str(num_o_2) + " passwords")

