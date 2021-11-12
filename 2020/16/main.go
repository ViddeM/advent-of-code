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
	name string
	rangeOneMin int
	rangeOneMax int
	rangeTwoMin int
	rangeTwoMax int
}

type Ticket []int

func main() {
	rules, myTicket, otherTickets := parseInput()

	solution := solvePartOne(rules, otherTickets)
	fmt.Printf("Part One -- The solution is %d\n", solution)

	solution2 := solvePartTwo(rules, myTicket, otherTickets)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)

}

func validForRule(rule Rule, number int) bool {
	if rule.rangeOneMin <= number && number <= rule.rangeOneMax {
		return true
	}
	if rule.rangeTwoMin <= number && number <= rule.rangeTwoMax {
		return true
	}
	return false
}

func numIsValid(rules []Rule, number int) bool {
	for _, rule := range rules {
		if validForRule(rule, number) {
			return true
		}
	}
	return false
}

func solvePartOne(rules []Rule, tickets []Ticket) int {
	sum := 0
	for _, ticket := range tickets {
		for _, number := range ticket {
			if numIsValid(rules, number) == false {
				sum += number
			}
		}
	}
	return sum
}

func filterInvalidTickets(rules []Rule, tickets []Ticket) []Ticket {
	var filtered []Ticket

	for _, ticket := range tickets {
		valid := true
		for _, number := range ticket {
			if numIsValid(rules, number) == false {
				valid = false
			}
		}
		if valid {
			filtered = append(filtered, ticket)
		}
	}

	return filtered
}

func transposeTickets(tickets []Ticket) []Ticket {
	// Takes the tickets and makes new tickets with the values from each ticket that had the same index.
	var newTickets []Ticket
	for i := 0; i < len(tickets[0]); i++ {
		var ticket Ticket
		for _, tick := range tickets {
			ticket = append(ticket, tick[i])
		}
		newTickets = append(newTickets, ticket)
	}
	return newTickets
}

func removeOnExist(list []string, key string) []string {
	var newList []string
	for _, str := range list {
		if str != key {
			newList = append(newList, str)
		}
	}
	return newList
}

func solvePartTwo(rules []Rule, myTicket Ticket, tickets []Ticket) int64 {
	tickets = filterInvalidTickets(rules, tickets)
	transposedTickets := transposeTickets(tickets)

	nameIndicies := make(map[int][]string)

	for ind, ticket := range transposedTickets {
		for _, rule := range rules {
			correct := true
			for _, num := range ticket {
				if validForRule(rule, num) == false {
					correct = false
					break
				}
			}

			if correct {
				nameIndicies[ind] = append(nameIndicies[ind], rule.name)
			}
		}
	}

	done := false
	for done == false {
		done = true
		for i, list := range nameIndicies {
			if len(list) > 1 {
				done = false
			} else if len(list) == 1 {
				name := list[0]
				// Remove this value from all other nameIndicies
				for j, names := range nameIndicies {
					if i != j {
						nameIndicies[j] = removeOnExist(names, name)
					}
				}
			}
		}
	}

	var product int64 = 1
	for ind, names := range nameIndicies {
		if strings.Contains(names[0], "departure") {
			product *= int64(myTicket[ind])
		}
	}

	return product
}

func parseRange(text string) (int, int) {
	numbers := strings.Split(text, "-")
	numOne, err := strconv.Atoi(numbers[0])
	if err != nil {
		log.Panic(err)
	}

	numTwo, err := strconv.Atoi(numbers[1])
	if err != nil {
		log.Panic(err)
	}

	return numOne, numTwo
}

func parseTicket(text string) Ticket {
	nums := strings.Split(text, ",")
	var ticket Ticket
	for _, numText := range nums {
		num, err := strconv.Atoi(numText)
		if err != nil {
			log.Panic(err)
		}
		ticket = append(ticket, num)
	}
	return ticket
}

func parseInput() ([]Rule, Ticket, []Ticket) {
	f, err := os.Open("16/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	var rules []Rule
	var myTicket Ticket
	var otherTickets []Ticket

	mode := 0

	for scanner.Scan() {
		text := scanner.Text()

		if text != "" {
			if strings.Contains(text, "your ticket:") {
				mode = 1
			} else if strings.Contains(text, "nearby tickets") {
				mode = 2
			} else {
				switch mode {
				case 0:
					splitOne := strings.Split(text, ": ")
					name := splitOne[0]
					ranges := strings.Split(splitOne[1], " or ")
					rangeOneMin, rangeOneMax := parseRange(ranges[0])
					rangeTwoMin, rangeTwoMax := parseRange(ranges[1])
					rules = append(rules, Rule{
						name:        name,
						rangeOneMin: rangeOneMin,
						rangeOneMax: rangeOneMax,
						rangeTwoMin: rangeTwoMin,
						rangeTwoMax: rangeTwoMax,
					})
					break
				case 1:
					myTicket = parseTicket(text)
					break
				case 2:
					ticket := parseTicket(text)
					otherTickets = append(otherTickets, ticket)
					break
				default:
					log.Panic(fmt.Sprintf("Invalid mode %d", mode))
					break
				}
			}
		}
	}

	return rules, myTicket, otherTickets
}