package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	f, err := os.Open("10/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	var numbers []int

	for scanner.Scan() {
		text := scanner.Text()
		num, err := strconv.Atoi(text)
		if err != nil {
			log.Fatal(err)
		}

		numbers = append(numbers, num)
	}
	sort.Ints(numbers)
	highest := numbers[len(numbers) - 1]
	numbers = append(numbers, highest + 3)

	solution := solvePartOne(numbers)
	fmt.Printf("Part One -- Solution: %d\n", solution)
	solution2 := solvePartTwo(numbers)
	fmt.Printf("Part Two -- Solution: %d\n", solution2)
}

func solvePartOne(numbers []int) int {
	oneDiffs := 0
	threeDiffs := 0
	currJoltage := 0

	for _, number := range numbers {
		diff := number - currJoltage
		if diff == 1 {
			oneDiffs += 1
		} else if diff == 3 {
			threeDiffs += 1
		} else {
			log.Fatal(fmt.Sprintf("Diff is not 1 or 3 | %d", diff))
		}
		currJoltage = number
	}


	return oneDiffs * threeDiffs
}

func solvePartTwo(numbers []int) int {
	possibilitiesMap := make(map[int]int)

	for i := len(numbers) -1; i >= 0; i-- {
		curr := numbers[i]
		currMax := curr + 3
		poss := 0
		for j := i + 1; j < len(numbers); j++ {
			jVal := numbers[j]
			if jVal <= currMax {
				if jPoss, ok := possibilitiesMap[jVal]; ok {
					poss += jPoss
				} else {
					log.Fatal(fmt.Sprintf("not in possibilitiesMap %d (%d)", j, jVal))
				}
			} else {
				break
			}
		}

		if poss == 0 {
			poss = 1
		}

		possibilitiesMap[curr] = poss
	}

	sum := 0
	for i := 0; i <= 3; i++ {
		if jPoss, ok := possibilitiesMap[i]; ok {
			sum += jPoss
		}
	}

	return sum
}