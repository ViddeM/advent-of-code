def find_tree_count(map, slope_x = 3, slope_y = 1):
    curr_x = 0
    curr_y = 0
    tree_count = 0
    while curr_y < len(map):
        # print(f"X: {curr_x}, Y: {curr_y}, tree_count {tree_count}")

        tile = map[curr_y][curr_x]
        if tile == "#":
            tree_count += 1
        elif tile != ".":
            print(f"WHAT THE FUCK? {tile}")

        curr_x = (curr_x + slope_x) % len(map[curr_y])
        curr_y += slope_y

    return tree_count


map = {}

with open("input.txt", "r") as file:
    lines = file.readlines()

    for y in range(len(lines)):
        map[y] = {}
        line = lines[y].replace("\n", "")
        for x in range(len(line)):
            map[y][x] = line[x]


print(f"Part one -- Found '{find_tree_count(map)}' trees in the path")


slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
tree_counts = [find_tree_count(map, x, y) for (x, y) in slopes]
product = 1
for tree_count in tree_counts:
    product *= tree_count

print(f"Part two -- The product is {product}")