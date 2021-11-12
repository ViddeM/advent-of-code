package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)


type Rule struct {
	number int
	options [][]int
	matchTo string
	hasSubRules bool
}


func main() {
	rules, messages := parseInput()

	solution := solvePartOne(rules, messages)
	fmt.Printf("Part One -- The solution is %d\n", solution)

	solution2 := solvePartTwo(rules, messages)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}


func stringsToInts(text string) []int {
	texts := strings.Split(text, " ")
	var ints []int
	for _, t := range texts {
		i, err := strconv.Atoi(t)
		if err != nil {
			log.Panic(err)
		}
		ints = append(ints, i)
	}
	return ints
}


func parseInput() (map[int]Rule, []string) {
	f, err := os.Open("19/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	rules := make(map[int]Rule)
	var messages []string

	readingRules := true
	for scanner.Scan() {
		text := scanner.Text()
		if text == "" {
			readingRules = false
			continue
		}

		if readingRules {
			parts := strings.Split(text, ": ")
			number, err := strconv.Atoi(parts[0])
			if err != nil {
				log.Panic(err)
			}

			if strings.Contains(parts[1], "\"") {
				matchTo := strings.ReplaceAll(parts[1], "\"", "")
				rules[number] = Rule{
					number:      number,
					matchTo:     matchTo,
					hasSubRules: false,
				}
			} else {
				orParts := strings.Split(parts[1], " | ")
				var options [][]int
				for _, orPart := range orParts {
					options = append(options, stringsToInts(orPart))
				}

				rules[number] = Rule{
					number:      number,
					matchTo:     "",
					options: options,
					hasSubRules: true,
				}
			}
		} else {
			messages = append(messages, text)
		}
	}
	return rules, messages
}

func merge(beginning, toAdd []string) []string {
	if len(beginning) == 0 {
		return toAdd
	}

	var new []string
	for _, beg := range beginning {
		for _, add := range toAdd {
			new = append(new, beg + add)
		}
	}
	return new
}


func getAlternativesForRules(subRules []int, valid map[int][]string) ([]string, bool) {
	var alternatives [] string
	for _, subRule := range subRules {
		if val, exists := valid[subRule]; exists {
			alternatives = merge(alternatives, val)
		} else {
			return []string{}, false
		}
	}
	return alternatives, true
}


func findForOptions(options [][]int, valid map[int][]string) ([]string, bool) {
	var newValid []string
	for _, subRules := range options {
		alternatives, ok := getAlternativesForRules(subRules, valid)
		if ok == false {
			return []string{}, false
		}
		newValid = append(newValid, alternatives...)
	}
	return newValid, true
}


func findAllValid(rules map[int]Rule) map[int][]string {
	valid := make(map[int][]string)
	for len(valid) < len(rules) {
		for num, rule := range rules {
			if rule.hasSubRules == false {
				valid[num] = []string{rule.matchTo}
			} else {
				validForNum, ok := findForOptions(rule.options, valid)
				if ok {
					valid[num] = validForNum
				}
			}
		}
	}
	return valid
}


func solvePartOne(rules map[int]Rule, messages []string) int {
	validMessages := 0
	valid := findAllValid(rules)
	validForZero, exists := valid[0]

	if exists == false {
		log.Panic("Map does not contain 0!")
	}

	for _, message := range messages {
		for _, checkFor := range validForZero {
			if message == checkFor {
				validMessages += 1
				break
			}
		}
	}

	return validMessages
}


func findAllValidTwo(rules map[int]Rule) map[int][]string {
	rules[8] = Rule{
		number: 8,
		options: [][]int{{42}},
		matchTo: "",
		hasSubRules: true,
	}

	rules[11] = Rule{
		number: 11,
		options: [][]int{{42, 31}},
		matchTo: "",
		hasSubRules: true,
	}

	valid := findAllValid(rules)

	// Random number of plausible ''loops''
	fortyTwo, _ := valid[42]
	thirtyOne, _ := valid[31]
	currEightOpt := fortyTwo
	currElevenOptB := fortyTwo

	var newEightOpts []string
	var newElevenOpts []string
	for i := 0; i < 5; i++ {
		currEightOpt = merge(currEightOpt, fortyTwo)
		currElevenOptB = merge(fortyTwo, currElevenOptB)
		newEightOpts = append(newEightOpts, currEightOpt...)
		newElevenOpts = append(newElevenOpts, merge(currElevenOptB, thirtyOne)...)
		valid[8] = append(valid[8], newEightOpts...)
		valid[11] = append(valid[11], newElevenOpts...)
	}

	return valid
}


func solvePartTwo(rules map[int]Rule, messages []string) int {
	valid := findAllValidTwo(rules)

	validMessages := 0
	validForZero, exists := valid[0]

	if exists == false {
		log.Panic("Map does not contain 0!")
	}

	for _, message := range messages {
		for _, checkFor := range validForZero {
			if message == checkFor {
				validMessages += 1
				break
			}
		}
	}

	return validMessages
}