lines = []

with open("input.txt", 'r') as file:
    for line in file:
        lines.append(line)

numDoubles = 0
numTripples = 0

for line in lines:
    foundDouble = False
    foundTripple = False
    for c in line:
        occurances = line.count(c)
        if occurances == 2 and not foundDouble:
            numDoubles = numDoubles + 1
            foundDouble = True
        if occurances == 3 and not foundTripple:
            numTripples = numTripples + 1
            foundTripple = True

print("NumDoubles: " + str(numDoubles))
print("NumTriples: " + str(numTripples))
print("Checksum: " + str(numDoubles * numTripples))

# Part two

def hasCorrectDiff(wordA, wordB):
    if (wordA == wordB):
        return False

    difference = 0
    for i in range(0, len(wordA)):
        if wordA[i] != wordB[i]:
            difference = difference + 1
            if difference > 1:
                return False
    return True

def findWord(lines):
    word = ""
    for line in lines:
        for other in lines:
            if hasCorrectDiff(line, other) == True:
                for c in line:
                    if c in other:
                        word += c
                return word, line, other


word, line, other = findWord(lines)

print("The common letters are:::: " + word + " came from " + line + " and " + other)