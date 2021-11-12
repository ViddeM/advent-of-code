package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

func main() {
	//f, err := os.Open("9/input.txt")
	f, err := os.Open("9/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	var numbers []int

	for scanner.Scan() {
		text := scanner.Text()
		number, err := strconv.Atoi(text)
		if err != nil {
			log.Fatal(err)
		}
		numbers = append(numbers, number)
	}



	solution := solvePartOne(numbers, 25)
	fmt.Printf("Part One -- The solution is %d\n", solution)

	solution2 := solvePartTwo(numbers, solution)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}

func pairExists(sumIndex int, numbers []int, preamble int) bool {
	sum := numbers[sumIndex]
	for i := sumIndex - preamble; i < sumIndex; i++ {
		a := numbers[i]
		for j := i + 1; j < sumIndex; j++ {
			b := numbers[j]
			if a + b == sum {
				return true
			}
		}
	}

	return false
}

func solvePartOne(numbers []int, preamble int) int {
	for i := preamble; i < len(numbers); i++ {
		if !pairExists(i, numbers, preamble) {
			return numbers[i]
		}
	}
	return -1
}

func findNumber(numbers []int, lowIndex, highIndex int) int {
	lowestNum := math.MaxInt32
	highestNum := -1
	for i := lowIndex; i <= highIndex; i++ {
		curr := numbers[i]
		if curr <= lowestNum {
			lowestNum = curr
		}

		if curr >= highestNum {
			highestNum = curr
		}
	}
	return highestNum + lowestNum
}

func solvePartTwo(numbers []int, invalid int) int {
	for i := 0; i < len(numbers); i++ {
		sum := 0
		for j := i; j < len(numbers); j++ {
			sum += numbers[j]
			if sum == invalid {
				return findNumber(numbers, i, j)
			}
		}
	}
	return -1
}
