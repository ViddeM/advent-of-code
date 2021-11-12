class Claim:
    def __init__(self, id, x, y, width, height):
        self.id = id
        self.x = x
        self.y = y
        self.width = width
        self.height = height

    def toString(self):
        return ("Claim with: " + str(self.id) + ", " + str(self.x) + ", " + str(self.y) + ", " +
              str(self.width) + ", " + str(self.height))


def createClaim(text):
    specialSigns = ["#"," ","@",",",":","x"]
    previous = ""
    id = ""
    x = ""
    y = ""
    width = ""
    height = ""

    for char in text:
        if char in specialSigns:
            previous += char
        else:
            if len(previous) == 1:
                id += char
            elif len(previous) == 4:
                x += char
            elif len(previous) == 5:
                y += char
            elif len(previous) == 7:
                width += char
            elif len(previous) == 8:
                height += char

    return Claim(int(id), int(x), int(y), int(width), int(height))


claims = []
maxX = 0
maxY = 0

def findSize():
    global maxX, maxY
    for claim in claims:
        currWidth = claim.x + claim.width
        currHeight = claim.y + claim.height
        if currWidth > maxX:
            maxX = currWidth
        if currHeight > maxY:
            maxY = currHeight

with open("input.txt", "r") as file:
    for line in file:
        claims.append(createClaim(line))

findSize()
maxX += 5
maxY += 5
matrix = [[0 for x in range(maxX)] for y in range(maxY)]

for claim in claims:
    for x in range(claim.x, claim.x + claim.width):
        for y in range(claim.y, claim.y + claim.height):
            matrix[x][y] = matrix[x][y] + 1

count = 0

for x in range(maxX - 1):
    for y in range(maxY - 1):
        if matrix[x][y] >= 2:
            count += 1

print("Number of sqr-inches with two or more claims: " + str(count))

# ------ Task Two

def check_claim(claim):
    for x in range(claim.x, claim.x + claim.width):
        for y in range(claim.y, claim.y + claim.height):
            if (matrix[x][y] > 1):
                return False
    return True

for claim in claims:
    if check_claim(claim):
        print("Claim: " + str(claim.id) + " hasn't overlapped!")