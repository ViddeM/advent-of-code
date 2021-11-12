with open("input.txt") as file:
    input = file.read().split(",")

codes = []
for code in input:
    codes.append(int(code))

def intcode_program(instructions):
    index = 0
    while instructions[index] != 99 and index < len(instructions):
        num = instructions[index]
        a = instructions[instructions[index + 1]]
        b = instructions[instructions[index + 2]]
        if num == 1:
            instructions[instructions[index + 3]] = a + b
        elif num == 2:
            instructions[instructions[index + 3]] = a * b
        elif num == 99:
            break
        else:
            Exception("WHAT: " + str(index) + ", num: " + str(num))

        index += 4

    return instructions[0]

desired = 19690720

# Assignment 2
def main():
    for noun in range(0, 100):
        for verb in range(0, 100):
            codes[1] = noun
            codes[2] = verb
            result = intcode_program(codes.copy())
            if result == desired:
                print("Done: " + str(100 * noun + verb))
                return

            print("Tried " + str(noun) + ", " + str(verb) + " got " + str(result))

main()