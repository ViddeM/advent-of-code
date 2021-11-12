package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

type Entry struct {
	min int
	max int
	character string
	password string
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var entries []Entry

	for scanner.Scan() {
		text := scanner.Text()

		min := -1
		max := -1
		character := ""

		current_number := ""
		current_text := ""
		for _, char_int := range text {
			char := string(char_int)

			switch char {
			case "-":
				min = getInt(current_number)
				current_number = ""
				break
			case " ":
				if max < 0 {
					max = getInt(current_number)
				}
				break
			case ":":
				character = current_text
				current_text = ""
				break
			default:
				if unicode.IsDigit(char_int) {
					current_number += char
				} else {
					current_text += char
				}
				break
			}
		}

		entries = append(entries, Entry{
			min: min,
			max: max,
			character: character,
			password: current_text,
		})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	valid_part_one := 0
	valid_part_two := 0

	for _, entry := range entries {
		count := strings.Count(entry.password, entry.character)
		if entry.min <= count && count <= entry.max {
			valid_part_one++
		}

		first_char := string(entry.password[entry.min - 1])
		second_char := string(entry.password[entry.max - 1])
		if first_char != second_char && (first_char == entry.character || second_char == entry.character) {
			valid_part_two++
		}
	}

	fmt.Printf("Part 1 -- There are %d valid passwords\n", valid_part_one)
	fmt.Printf("Part 2 -- There are %d valid passwords\n", valid_part_two)
}

func getInt(text string) int {
	cast, err := strconv.Atoi(text)
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	return cast
}