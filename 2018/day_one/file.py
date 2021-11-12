result = 0

lines = []
steps = []

with open("input.txt", "r") as file_obj:
    for line in file_obj:
        lines.append(line)

steps.append(0)
firstDuplicate = 0
quit = False

while not quit:
    for line in lines:
        number = int(line[1:])
        if "+" in line:
            result += number
        else:
            result -= number

        print(result)
        if result in steps:
            firstDuplicate = result
            quit = True
            break
        else:
            steps.append(result)

print("FIRST DUPLICATE :::: " + str(firstDuplicate))