package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const (
	ADD = iota
	MUL
)

type Expression struct {
	left *Expression
	leftNum int
	right *Expression
	rightNum int
	operator int
	prioRight bool
}

func main() {
	expressions := parseInput()

	/*solution := solvePartOne(world)
	fmt.Printf("Part One -- The solution is %d\n", solution)*/

/*	solution2 := solvePartTwo(world)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)*/

}

func parseInput() []Expression {
	f, err := os.Open("18/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	var expressions []Expression

	for scanner.Scan() {
		text := scanner.Text()
		text = strings.ReplaceAll(text, " ", "")
		for _, charInt := range text {
			char := string(charInt)
			switch char {
			case "(":
				break
			case ")":
				break
			case "*":
				break
			case "+":
				break
			default:
				num, err := strconv.Atoi(char)
				if err != nil {
					log.Panic(num)
				}
				break
			}
		}
	}
	return expressions
}