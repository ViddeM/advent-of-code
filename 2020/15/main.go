package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	numbers := parseInput()

	solution := solvePartOne(numbers)
	fmt.Printf("Part One -- The solution is %d\n", solution)

	solution2 := solvePartTwo(numbers)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}

func parseInput() []int {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	var numbers []int
	for scanner.Scan() {
		text := scanner.Text()
		nums := strings.Split(text, ",")
		for _, char := range nums {
			num, err := strconv.Atoi(char)
			if err != nil {
				log.Panic(err)
			}
			numbers = append(numbers, num)
		}
	}

	return numbers
}

func solvePartOne(numbers []int) int {
	spoken := make(map[int]int)
	turn := 0
	prev := 0
	for i, num := range numbers {
		if i > 0 {
			spoken[prev] = i
		}
		turn += 1
		prev = num
	}

	for turn = turn + 1; turn <= 2020; turn++ {
		if val, exist := spoken[prev]; exist {
			spoken[prev] = turn - 1
			prev = turn - 1 - val
		} else {
			spoken[prev] = turn - 1
			prev = 0
		}
	}
	return prev
}


func solvePartTwo(numbers []int) int {
	spoken := make(map[int]int)
	turn := 0
	prev := 0
	for i, num := range numbers {
		if i > 0 {
			spoken[prev] = i
		}
		turn += 1
		prev = num
	}

	for turn = turn + 1; turn <= 30000000; turn++ {
		if val, exist := spoken[prev]; exist {
			spoken[prev] = turn - 1
			prev = turn - 1 - val
		} else {
			spoken[prev] = turn - 1
			prev = 0
		}
	}
	return prev
}
