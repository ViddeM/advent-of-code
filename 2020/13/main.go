package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
	"math/big"

)

func main() {
	timestamp, busses := parseInput()

	solution := solvePartOne(timestamp, busses)
	fmt.Printf("Part One -- The solution is %d\n", solution)


	solution2 := solvePartTwo(busses)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}

func parseInput() (int, []int){
	f, err := os.Open("13/input.txt")
	//f, err := os.Open("12/test.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	timestamp := -1
	var busses []int

	for scanner.Scan() {
		text := scanner.Text()
		if timestamp < 0 {
			timestamp, err = strconv.Atoi(text)
			if err != nil {
				log.Panic(err)
			}
		} else {
			ids := strings.Split(text, ",")
			for _, idStr := range ids {
				if idStr == "x" {
					busses = append(busses, -1)
				} else {
					id, err := strconv.Atoi(idStr)
					if err != nil {
						log.Panic(err)
					}
					busses = append(busses, id)
				}
			}
		}
	}

	return timestamp, busses
}

func findDepartTime(bus, timestamp int) int {
	i := 1
	val := bus * i
	for val < timestamp {
		i++
		val = bus * i
	}
	return val
}

func solvePartOne(timestamp int, busses []int) int {
	var bestId, bestDiff int
	bestDiff = math.MaxInt32
	for _, bus := range busses {
		if bus >= 0 {
			depart := findDepartTime(bus, timestamp)
			diff := depart - timestamp
			if diff <= bestDiff {
				bestId = bus
				bestDiff = diff
			}
		}
	}

	return bestId * bestDiff
}

func getNi(busses []int, avoidIndex int) *big.Int {
	prod := big.NewInt(1)
	for i, bus := range busses {
		if i != avoidIndex && bus >= 0 {
			prod.Mul(prod, big.NewInt(int64(bus)))
		}
	}
	return prod
}

func solvePartTwo(busses []int) int64 {
	N := big.NewInt(1)
	sum := big.NewInt(0)
	for index, bus := range busses {
		if bus >= 0 {
			bigBus := big.NewInt(int64(bus))

			N.Mul(N, bigBus)
			Ni := getNi(busses, index)
			bi := big.NewInt(0)
			bi = bi.Sub(bigBus, big.NewInt(int64(index)))
			xi := big.NewInt(0)
			xi = xi.ModInverse(Ni, bigBus)

			xPart := big.NewInt(1).Mul(bi, Ni)
			xPart.Mul(xPart, xi)
			sum.Add(sum, xPart)
		}
	}
	return sum.Mod(sum, N).Int64()
}
