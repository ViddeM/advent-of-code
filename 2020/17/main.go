package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const (
	ACTIVE = iota
	INACTIVE
)

type Cube struct {
	x int
	y int
	z int
	w int
}

type World []Cube

func main() {
	world := parseInput()

	solution := solvePartOne(world)
	fmt.Printf("Part One -- The solution is %d\n", solution)

	solution2 := solvePartTwo(world)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)

}


func parseInput() World {
	f, err := os.Open("17/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	var world World

	y := 0
	for scanner.Scan() {
		text := scanner.Text()
		for x, charInt := range text {
			char := string(charInt)
			switch char {
			case ".":
				break
			case "#":
				world = append(world, Cube{
					x: x,
					y: y,
					z: 0,
					w: 0,
				})
				break
			default:
				log.Fatal(fmt.Sprintf("Invalid char %s", char))
			}
		}
		y += 1
	}
	return world
}

func cubeCompare(a, b Cube) bool {
	return a.x == b.x && a.y == b.y && a.z == b.z && a.w == b.w
}

func getNeighbours3D(cube Cube) World {
	var neighbours World
	for x := cube.x - 1; x <= cube.x + 1; x ++ {
		for y := cube.y - 1; y <= cube.y + 1; y++ {
			for z := cube.z - 1; z <= cube.z + 1; z++ {
				if !(x == cube.x && y == cube.y && z == cube.z) {
					neighbours = append(neighbours, Cube{
						x: x,
						y: y,
						z: z,
					})
				}
			}
		}
	}
	return neighbours
}

func inWorld(cube Cube, world World) bool {
	for _, c := range world {
		if cubeCompare(cube, c) {
			return true
		}
	}
	return false
}

func activeNeighbours(neighbours, world World) (int, World) {
	active := 0
	var inactiveNeighbours World
	for _, neighbour := range neighbours {
		if inWorld(neighbour, world) {
			active += 1
		} else {
			inactiveNeighbours = append(inactiveNeighbours, neighbour)
		}
	}
	return active, inactiveNeighbours
}

func simulate3D(world World) World {
	var newWorld World
	var toCheck World

	for _, cube := range world {
		neighbours := getNeighbours3D(cube)
		active, inactiveNeighbours := activeNeighbours(neighbours, world)

		if active == 2 || active == 3 {
			newWorld = append(newWorld, cube)
		}

		for _, inactive := range inactiveNeighbours {
			if inWorld(inactive, toCheck) == false {
				toCheck = append(toCheck, inactive)
			}
		}
	}

	for _, cube := range toCheck {
		active, _ := activeNeighbours(getNeighbours3D(cube), world)
		if active == 3 {
			newWorld = append(newWorld, cube)
		}
	}

	return newWorld
}

func solvePartOne(world World) int {
	/*fmt.Printf("Before any cyles:\n\n")
	printWorld3D(world)
	fmt.Printf("\n")*/

	for i := 0; i < 6; i++ {
		world = simulate3D(world)

		/*s := ""
		if i > 0 {
			s = "s"
		}
		fmt.Printf("After %d cycle%s:\n\n", i + 1, s)
		printWorld3D(world)
		fmt.Printf("\n")*/
	}
	return len(world)
}

func printWorld3D(world World) {
	// Z, Y, X
	floors := make(map[int]map[int]map[int]Cube)
	minX := 1000
	maxX := -1000
	minY := 1000
	maxY := -1000
	minZ := 1000
	maxZ := -1000

	for _, cube := range world {
		if _, ok := floors[cube.z]; ok == false {
			floors[cube.z] = make(map[int]map[int]Cube)
		}

		plane, _ := floors[cube.z]

		row, ok := plane[cube.y]
		if ok == false {
			plane[cube.y] = make(map[int]Cube)
			row, _ = plane[cube.y]
		}

		row[cube.x] = cube

		if cube.x < minX {
			minX = cube.x
		}
		if cube.x > maxX {
			maxX = cube.x
		}
		if cube.y < minY {
			minY = cube.y
		}
		if cube.y > maxY {
			maxY = cube.y
		}
		if cube.z < minZ {
			minZ = cube.z
		}
		if cube.z > maxZ {
			maxZ = cube.z
		}
	}

	for z := minZ; z <= maxZ; z++{
		fmt.Printf("z=%d\n", z)
		for y := minY; y <= maxY; y++ {
			for x := minX; x <= maxX; x++ {
				plane, ok := floors[z]
				if ok {
					row, ok2 := plane[y]
					if ok2 {
						_, ok2 = row[x]
					}

					ok = ok2
				}

				if ok {
					fmt.Printf("#")
				} else {
					fmt.Printf(".")
				}
			}
			fmt.Printf("\n")
		}
		fmt.Printf("\n")
	}
}

func getNeighbours4D(cube Cube) World {
	var neighbours World
	for x := cube.x - 1; x <= cube.x + 1; x ++ {
		for y := cube.y - 1; y <= cube.y + 1; y++ {
			for z := cube.z - 1; z <= cube.z + 1; z++ {
				for w := cube.w -1; w <= cube.w + 1; w ++ {
					if !(x == cube.x && y == cube.y && z == cube.z && w == cube.w) {
						neighbours = append(neighbours, Cube{
							x: x,
							y: y,
							z: z,
							w: w,
						})
					}
				}
			}
		}
	}
	return neighbours
}

func simulate4D(world World) World {
	var newWorld World
	var toCheck World

	for _, cube := range world {
		neighbours := getNeighbours4D(cube)
		active, inactiveNeighbours := activeNeighbours(neighbours, world)

		if active == 2 || active == 3 {
			newWorld = append(newWorld, cube)
		}

		for _, inactive := range inactiveNeighbours {
			if inWorld(inactive, toCheck) == false {
				toCheck = append(toCheck, inactive)
			}
		}
	}

	for _, cube := range toCheck {
		active, _ := activeNeighbours(getNeighbours4D(cube), world)
		if active == 3 {
			newWorld = append(newWorld, cube)
		}
	}

	return newWorld
}

func solvePartTwo(world World) int {
/*	fmt.Printf("Before any cyles:\n\n")
	printWorld4D(world)
	fmt.Printf("\n")*/

	for i := 0; i < 6; i++ {
		world = simulate4D(world)
/*
		s := ""
		if i > 0 {
			s = "s"
		}
		fmt.Printf("After %d cycle%s:\n\n", i + 1, s)
		printWorld4D(world)
		fmt.Printf("\n")*/
	}
	return len(world)
}

func printWorld4D(world World) {
	// W, Z, Y, X
	worlds := make(map[int]map[int]map[int]map[int]Cube)
	minX := 1000
	maxX := -1000
	minY := 1000
	maxY := -1000
	minZ := 1000
	maxZ := -1000
	minW := 1000
	maxW := -1000

	for _, cube := range world {
		if _, ok := worlds[cube.w]; ok == false {
			worlds[cube.w] = make(map[int]map[int]map[int]Cube)
		}

		floors, _ := worlds[cube.w]

		if _, ok := floors[cube.z]; ok == false {
			floors[cube.z] = make(map[int]map[int]Cube)
		}

		plane, _ := floors[cube.z]

		row, ok := plane[cube.y]
		if ok == false {
			plane[cube.y] = make(map[int]Cube)
			row, _ = plane[cube.y]
		}

		row[cube.x] = cube

		if cube.x < minX {
			minX = cube.x
		}
		if cube.x > maxX {
			maxX = cube.x
		}
		if cube.y < minY {
			minY = cube.y
		}
		if cube.y > maxY {
			maxY = cube.y
		}
		if cube.z < minZ {
			minZ = cube.z
		}
		if cube.z > maxZ {
			maxZ = cube.z
		}
		if cube.w < minW {
			minW = cube.w
		}
		if cube.w > maxW {
			maxW = cube.w
		}
	}

	for w := minW; w <= maxW; w++ {
		for z := minZ; z <= maxZ; z++ {
			fmt.Printf("z=%d, w=%d\n", z, w)
			for y := minY; y <= maxY; y++ {
				for x := minX; x <= maxX; x++ {
					floors, ok := worlds[w]

					if ok {
						plane, ok2 := floors[z]
						if ok2 {
							row, ok3 := plane[y]
							if ok3 {
								_, ok3 = row[x]
							}

							ok = ok3
						} else {
							ok = ok2
						}
					}

					if ok {
						fmt.Printf("#")
					} else {
						fmt.Printf(".")
					}
				}
				fmt.Printf("\n")
			}
			fmt.Printf("\n")
		}
	}
}
