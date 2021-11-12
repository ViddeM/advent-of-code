package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Seat struct {
	row int
	col int
	id int
}

func getSeat(row int, col int) Seat {
	return Seat{
		row: row,
		col: col,
		id: row * 8 + col,
	}
}

func getMidPoint(low, high int) int {
	return low + int((high - low) / 2.0)
}

func textToSeat(text string) Seat {
	minRow := 0
	maxRow := 127
	minCol := 0
	maxCol := 7
	for _, char_int := range text {
		char := string(char_int)
		switch char {
		case "F":
			maxRow = getMidPoint(minRow, maxRow)
		case "B":
			minRow = getMidPoint(minRow, maxRow) + 1
		case "L":
			maxCol = getMidPoint(minCol, maxCol)
		case "R":
			minCol = getMidPoint(minCol, maxCol) + 1
		}
	}

	if minRow != maxRow || minCol != maxCol {
		log.Fatal(fmt.Sprintf("Not exhaustive search!\ncol: %d - %d\nrow: %d - %d", minCol, maxCol, minRow, maxRow))
	}

	return getSeat(minRow, minCol)
}

type KeyVal struct {
	key int
	val bool
}

func main() {
	f, err := os.Open("5/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var seats[] Seat

	for scanner.Scan() {
		text := scanner.Text()
		seats = append(seats, textToSeat(text))
	}

	highest := 0
	for _, seat := range seats {
		if seat.id >= highest {
			highest = seat.id
		}
	}

	fmt.Printf("Part One -- The highest seat id is %d\n", highest)

	var existingIds []KeyVal
	for i := 0; i <= highest; i++ {
		existingIds = append(existingIds, KeyVal{
			key: i,
			val: false,
		})
	}

	for _, seat := range seats {
		existingIds[seat.id].val = true
	}

	nonExistant := 0
	for _, keyVal := range existingIds {
		if keyVal.val == false {
			if keyVal.key == nonExistant || keyVal.key == nonExistant+1 {
				nonExistant = keyVal.key
			} else {
				fmt.Printf("Part Two -- My seat id is %d\n", keyVal.key)
			}
		}
	}
}

