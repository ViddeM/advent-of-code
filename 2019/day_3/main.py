import math

with open("input.txt") as file:
    input = file.readlines()
    cable_1_inp = input[0].split(",")
    cable_2_inp = input[1].split(",")


def get_directives(cable):
    directives = []
    for inp in cable:
        dir = inp[:1]
        moves = int(inp[1:])
        directives.append((dir, moves))
    return directives


cable_1 = get_directives(cable_1_inp)
cable_2 = get_directives(cable_2_inp)


def get_positions(x, y, dir, num):
    new_x = x
    new_y = y
    if dir == "R":
        new_x += num
        return [(a, y) for a in range(x + 1, x + num + 1)], new_x, new_y
    if dir == "L":
        new_x -= num
        return [(a, y) for a in range(x - 1, x - num - 1, -1)], new_x, new_y
    if dir == "U":
        new_y += num
        return [(x, a) for a in range(y + 1, y + num + 1)], new_x, new_y
    if dir == "D":
        new_y -= num
        return [(x, a) for a in range(y - 1, y - num - 1, -1)], new_x, new_y

    Exception("Invalid directive: " + str(dir))


x = 0
y = 0
steps = 0
intersections = []
points = {}
for dir, num in cable_1:
    positions, x, y = get_positions(x, y, dir, num)
    for a, b in positions:
        steps += 1
        key = (a, b)
        if points.get(key) is None:
            points[key] = steps

x = 0
y = 0
steps = 0
points_2 = {}
for dir, num in cable_2:
    positions, x, y = get_positions(x, y, dir, num)
    for a, b in positions:
        steps += 1
        key = (a, b)
        if points.get(key) is not None and points_2.get(key) is None:
            if not (a == 0 and b == 0):
                points_2[key] = steps
                print("INTERSECTION AT " + str(a) + "_" + str(b))
                print("After: " + str(points.get(key)) + " and " + str(steps))
                intersections.append((a, b, points.get(key), steps))


def find_closest_intersection():
    closest = None
    closest_dist = math.inf
    min_steps = math.inf
    min_steps_a = math.inf
    min_steps_b = math.inf
    for x, y, steps_a, steps_b in intersections:
        dist = abs(x) + abs(y)
        if dist <= closest_dist:
            closest = (x, y)
            closest_dist = dist

        combined_steps = steps_a + steps_b
        if combined_steps <= min_steps:
            min_steps_a = steps_a
            min_steps_b = steps_b
            min_steps = combined_steps

    return closest[0], closest[1], closest_dist, min_steps, min_steps_a, min_steps_b


a, b, dist, step, min_a, min_b = find_closest_intersection()
print("CLOSEST::" + str(a) + "_" + str(b) + " at distance::" + str(dist))
print("MIN STEPS::" + str(step) + " " + str(min_a) + "_" + str(min_b))