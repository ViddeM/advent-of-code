orbits = {}

with open("input.txt") as file:
    input_ = file.readlines()


#with open("test.txt") as file:
#    input_ = file.readlines()

for inp in input_:
    parts = inp.split(")")
    if parts[1] not in orbits.keys():
        orbits[parts[1].replace("\n", "")] = parts[0]


orbits_count = {}
total = 0
for orbit in orbits.keys():
    num = 0
    curr = orbit
    while curr != "COM":
        if curr in orbits_count.keys():
            num += orbits_count[curr]
            break
        num += 1
        curr = orbits[curr]

    orbits_count[orbit] = num

    total += num

print(str(total) + " orbits")

# Part b
# Find the distances from sants to the way to COM
dist_from_santa = {}
curr = orbits["SAN"]
dist = 0
while curr != "COM":
    dist_from_santa[curr] = dist
    dist += 1
    curr = orbits[curr]

# Now find the first intersection from us.
curr = orbits["YOU"]
dist = 0
while curr != "COM":
    if curr in dist_from_santa:
        # We've found an intersection!
        dist += dist_from_santa[curr]
        break

    dist += 1
    curr = orbits[curr]

print("Dist from santa: " + str(dist))
