package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Bag struct {
	name string
	contents map[string]int
	heldBy map[string]int
}

func main() {
	f, err := os.Open("7/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	bagMap := make(map[string]Bag)
	heldBy := make(map[string]map[string]int)

	for scanner.Scan() {
		text := scanner.Text()
		text = strings.Replace(text, ".", "", 1)
		keyVals := strings.Split(text, " bags contain ")
		if len(keyVals) != 2 {
			log.Fatal("Err: invalid splicing, %s\n %d\n %v\n", text, len(keyVals), keyVals)
		}
		key := keyVals[0]
		fullVal := keyVals[1]

		contents := make(map[string]int)

		if fullVal != "no other bags" {
			vals := strings.Split(fullVal, ", ")

			if _, found := bagMap[key]; found {
				fmt.Printf("KEY ALREADY IN MAP %s", key)
				os.Exit(1)
			}

			for _, val := range vals {
				count, containsKey := contentToIntString(val)
				contents[containsKey] = count

				if entry, found := heldBy[containsKey]; found {
					entry[key] = count
					heldBy[containsKey] = entry
				} else {
					tmpMap := make(map[string]int)
					tmpMap[key] = count
					heldBy[containsKey] = tmpMap
				}
			}
		}

		bagMap[key] = Bag{
			name: key,
			contents: contents,
			heldBy: nil,
		}
	}

	for _, bag := range bagMap {
		if _, found := heldBy[bag.name]; found {
			bag.heldBy = heldBy[bag.name]
			bagMap[bag.name] = bag
		}
	}

	key := "shiny gold"
	result := solvePartOne(key, bagMap)
	fmt.Printf("Part One -- The solution is %d\n", result)

	// Minus one because we shouldn't count the shiny gold bag :sweat_smile:
	result2 := solvePartTwo(key, bagMap) - 1
	fmt.Printf("Part Two -- The solution is %d\n", result2)
}

func contentToIntString(text string) (int, string) {
	text = strings.ReplaceAll(text, " bags", "")
	text = strings.ReplaceAll(text, " bag", "")
	split := strings.Split(text, " ")
	if len(split) != 3 {
		fmt.Printf("ERROR, invalid split: %#v\nText: %s", split, text)
	}
	num, err := strconv.Atoi(split[0])
	if err != nil {
		panic(err)
	}
	return num, split[1] + " " + split[2]
}

func contains(list []string, elem string) bool {
	for _, e := range list {
		if e == elem {
			return true
		}
	}
	return false
}

func solvePartOne(key string, bagMap map[string]Bag) int {
	var canBeHeldBy []string
	var toCheck []string
	entry, _ := bagMap[key]
	for e, _ := range entry.heldBy {
		toCheck = append(toCheck, e)
		canBeHeldBy = append(canBeHeldBy, e)
	}

	for len(toCheck) > 0 {
		val := toCheck[len(toCheck) - 1]
		toCheck = toCheck[:len(toCheck) - 1]
		entry = bagMap[val]
		for e, _ := range entry.heldBy {
			if  contains(canBeHeldBy, e) == false {
				canBeHeldBy = append(canBeHeldBy, e)

				if contains(toCheck, e) == false{
					toCheck = append(toCheck, e)
				}
			}
		}
	}

	return len(canBeHeldBy)
}

func solvePartTwo(key string, bagMap map[string]Bag) int {
	total := 1
	for bag, count := range bagMap[key].contents {
		recursiveResult := solvePartTwo(bag, bagMap)
		total += count * recursiveResult
	}
	return total
}