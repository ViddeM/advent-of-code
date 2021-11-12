package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Layout struct {
	arr []int
	width int
	height int
}

type Coord struct {
	x int
	y int
}

const(
	NONEXISTANT = -1
	OCCUPIED = 0
	EMPTY = 1
	FLOOR = 2
)

func main() {
	lay := parseInput()

	solution := solvePartOne(lay)
	fmt.Printf("Part One -- The solution is %d\n", solution)


	solution2 := solvePartTwo(lay)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}

func parseInput() Layout {

	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var area []int
	y := 0
	width := 0
	for scanner.Scan() {
		text := scanner.Text()
		if width == 0 {
			width = len(text)
		}

		for _, charInt := range text {
			char := string(charInt)
			switch char {
			case "L":
				area = append(area, EMPTY)
				break
			case ".":
				area = append(area, FLOOR)
				break
			default:
				log.Fatal(fmt.Sprintf("What the fuck? %s", char))
			}
		}
		y++
	}

	return Layout{
		arr: area,
		width: width,
		height: y,
	}
}

func getAt(coord Coord, lay Layout) int {
	if coord.x < 0 || coord.x >= lay.width || coord.y < 0 || coord.y >= lay.height {
		return NONEXISTANT
	}
	return lay.arr[coord.y * lay.width + coord.x]
}

func occupiedNeighbours(pos Coord, lay Layout) int {
	count := 0
	for x := pos.x - 1; x <= pos.x + 1; x++ {
		for y := pos.y -1; y <= pos.y + 1; y++ {
			if !(x == pos.x && y == pos.y) {
				if getAt(Coord{x: x, y: y}, lay) == OCCUPIED {
					count += 1
				}
			}
		}
	}
	return count
}

func updateArea(lay Layout) (int, Layout) {
	var updatedArea []int
	updated := 0
	for index, val := range lay.arr {
		if val != FLOOR {
			x := index % lay.width
			y := (index - x) / lay.width
			occNeigh := occupiedNeighbours(Coord{x: x, y: y}, lay)

			switch val {
			case EMPTY:
				if occNeigh == 0 {
					updatedArea = append(updatedArea, OCCUPIED)
					updated += 1
				} else {
					updatedArea = append(updatedArea, EMPTY)
				}
				break
			case OCCUPIED:
				if occNeigh >= 4 {
					updatedArea = append(updatedArea, EMPTY)
					updated += 1
				} else {
					updatedArea = append(updatedArea, OCCUPIED)
				}
			}
		} else {
			updatedArea = append(updatedArea, FLOOR)
		}
	}

	return updated, Layout{
		arr: updatedArea,
		width: lay.width,
		height: lay.height,
	}
}

func countOccupied(lay Layout) int {
	count := 0
	for _, val := range lay.arr {
		if val == OCCUPIED {
			count += 1
		}
	}
	return count
}

func solvePartOne(lay Layout) int {
	updated, lay := updateArea(lay)
	iteration := 1
	for updated > 0 {
		updated, lay = updateArea(lay)
		//fmt.Printf("Iteration: %d\n", iteration)
		iteration++
	}

	return countOccupied(lay)
}

func occupiedDir(origin Coord, i, j int, lay Layout) bool {
	x := origin.x + i
	y := origin.y + j
	for {
		curr := getAt(Coord{x: x, y: y}, lay)
		switch curr {
		case OCCUPIED:
			return true
			break
		case FLOOR:
			break
		default:
			return false
		}

		x += i
		y += j
	}
}

func occupiedVisible(pos Coord, lay Layout) int {
	occupied := 0
	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			if !(i == 0 && j == 0) {
				if occupiedDir(pos, i, j, lay) {
					occupied += 1
				}
			}
		}
	}
	return occupied
}

func updateAreaTwo(lay Layout) (int, Layout) {
	var updatedArea []int
	updated := 0
	for index, val := range lay.arr {
		if val != FLOOR {
			x := index % lay.width
			y := (index - x) / lay.width
			occNeigh := occupiedVisible(Coord{x: x, y: y}, lay)

			switch val {
			case EMPTY:
				if occNeigh == 0 {
					updatedArea = append(updatedArea, OCCUPIED)
					updated += 1
				} else {
					updatedArea = append(updatedArea, EMPTY)
				}
				break
			case OCCUPIED:
				if occNeigh >= 5 {
					updatedArea = append(updatedArea, EMPTY)
					updated += 1
				} else {
					updatedArea = append(updatedArea, OCCUPIED)
				}
			}
		} else {
			updatedArea = append(updatedArea, FLOOR)
		}
	}

	return updated, Layout{
		arr: updatedArea,
		width: lay.width,
		height: lay.height,
	}
}

func solvePartTwo(lay Layout) int {
	updated, lay := updateAreaTwo(lay)
	iteration := 1
	for updated > 0 {
		updated, lay = updateAreaTwo(lay)
		//fmt.Printf("2: Iteration: %d\n", iteration)
		iteration++
	}

	return countOccupied(lay)
}
