with open("input.txt") as file:
    text = file.read().split(",")


codes = []
for code in text:
    codes.append(int(code))


inp = [5]
out = []


def get_params(instructions, pc, num, count):
    for i in range(1, count + 1):
        mode = int(num[3 - i])
        param = instructions[pc + i]
        if mode == 0:
            yield instructions[param]
        elif mode == 1:
            yield param
        else:
            raise Exception("Unknown mode: " + str(mode))


def intcode_program(instructions):
    index = 0
    while index < len(instructions):
        num = str(instructions[index]).zfill(5)
        op = int(num[3:])
        print("Index: " + str(index) + " Num instructions: " + str(len(instructions)) + " OP:::" + str(op))
        if op == 1:
            a, b = get_params(instructions, index, num, 2)
            c = instructions[index + 3]
            instructions[c] = a + b
            index += 4
        elif op == 2:
            a, b = get_params(instructions, index, num, 2)
            c = instructions[index + 3]
            instructions[c] = a * b
            index += 4
        elif op == 3:
            # Take input and save to only parameter
            a = instructions[index + 1]
            instructions[a] = inp.pop()
            index += 2
        elif op == 4:
            # Take only parameter and save to output
            a = next(get_params(instructions, index, num, 1))
            out.append(a)
            index += 2
        elif op == 5:
            a, b = get_params(instructions, index, num, 2)
            #b = instructions[index + 2]
            if a != 0:
                index = b
            else:
                index += 3
        elif op == 6:
            a, b = get_params(instructions, index, num, 2)
#            b = instructions[index + 2]
            if a == 0:
                index = b
            else:
                index += 3
        elif op == 7:
            a, b = get_params(instructions, index, num, 2)
            c = instructions[index + 3]
            if a < b:
                instructions[c] = 1
            else:
                instructions[c] = 0
            index += 4
        elif op == 8:
            a, b = get_params(instructions, index, num, 2)
            c = instructions[index + 3]
            if a == b:
                instructions[c] = 1
            else:
                instructions[c] = 0
            index += 4
        elif op == 99:
            return
        else:
            raise Exception("WHAT: " + str(index) + ", num: " + str(num))

    return


def main():
    try:
        intcode_program(codes)
    except Exception as e:
        print(e)
        print("OUT: " + str(out))
    print(out)

main()
