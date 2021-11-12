package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

const(
	N = "N"
	S = "S"
	E = "E"
	W = "W"
	R = "R"
	L = "L"
	F = "F"
)

type Move struct {
	dir string
	steps int
}

func main() {
	moves := parseInput()

	solution := solvePartOne(moves)
	fmt.Printf("Part One -- The solution is %d\n", solution)


	solution2 := solvePartTwo(moves)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}

func parseInput() []Move {
	f, err := os.Open("input.txt")
	//f, err := os.Open("12/test.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var moves []Move
	for scanner.Scan() {
		text := scanner.Text()
		dir := string(text[0])
		steps, err := strconv.Atoi(text[1:])
		if err != nil {
			log.Panic(err)
		}

		moves = append(moves, Move{
			dir: dir,
			steps: steps,
		})
	}
	return moves
}

func absInt(a int) int {
	return int(math.Abs(float64(a)))
}

func forward(dir, steps int) (int, int) {
	switch dir {
	case 0:
		return 0, steps
	case 1:
		return steps, 0
	case 2:
		return 0, -steps
	case 3:
		return -steps, 0
	default:
		log.Fatal(fmt.Sprintf("Invalid dir num %d", dir))
	}
	return 0, 0
}

func solvePartOne(moves []Move) int {
	x := 0
	y := 0
	dir := 1 // 0 = N, 1 = E...
	for _, move := range moves {
		switch move.dir {
		case N:
			y += move.steps
			break
		case S:
			y -= move.steps
			break
		case E:
			x += move.steps
			break
		case W:
			x -= move.steps
			break
		case R:
			dir = (dir + (move.steps / 90)) % 4
			break
		case L:
			dir = (dir - (move.steps / 90)) % 4
			if dir < 0 {
				dir += 4
			}
			break
		case F:
			xInc, yInc := forward(dir, move.steps)
			x += xInc
			y += yInc
			break
		default:
			log.Panic(fmt.Sprintf("Invalid direction dir: '%s'", move.dir))
		}
	}

	x = absInt(x)
	y = absInt(y)

	return x + y
}

func rotateRight(amount, x , y int) (int, int) {
	for i := 0; i < amount; i++ {
		tmp := y
		y = -x
		x = tmp
	}
	return x, y
}

func solvePartTwo(moves []Move) int {
	wayX := 10
	wayY := 1
	shipX := 0
	shipY := 0
	dir := 1 // 0 = N, 1 = E...
	for _, move := range moves {
		switch move.dir {
		case N:
			wayY += move.steps
			break
		case S:
			wayY -= move.steps
			break
		case E:
			wayX += move.steps
			break
		case W:
			wayX -= move.steps
			break
		case R:
			amount := move.steps / 90
			dir = (dir + amount) % 4
			wayX, wayY = rotateRight(amount, wayX, wayY)
			break
		case L:
			amount := move.steps / 90
			wayX, wayY = rotateRight(4 - amount, wayX, wayY)
			dir = (dir - amount) % 4
			if dir < 0 {
				dir += 4
			}
			break
		case F:
			shipX += move.steps * wayX
			shipY += move.steps * wayY
			break
		default:
			log.Panic(fmt.Sprintf("Invalid direction dir: '%s'", move.dir))
		}
	}

	shipX = absInt(shipX)
	shipY = absInt(shipY)

	return shipX + shipY
}