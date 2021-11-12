package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Group struct {
	forms []string
}

func main() {
	f, err := os.Open("6/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var groups []Group

	var forms []string
	for scanner.Scan() {
		text := scanner.Text()
		if text == "" {
			groups = append(groups, Group{
				forms: forms,
			})
			forms = []string{}
		} else {
			forms = append(forms, text)
		}
	}
	if len(forms) > 0 {
		groups = append(groups, Group{
			forms: forms,
		})
	}

	sum := 0
	for _, group := range groups {
		common := ""
		count := 0
		for _, form := range group.forms {
			for _, charInt := range form {
				char := string(charInt)
				if strings.Contains(common, char) == false {
					count += 1
					common = common + char
				}
			}
		}
		sum += count
	}

	fmt.Printf("Part One -- The sum is %d\n", sum)

	sum_2 := 0
	for _, group := range groups {
		charMap := make(map[int32]int32)
		for _, form := range group.forms {
			for _, charInt := range form {
				if val, ok := charMap[charInt]; ok {
					charMap[charInt] = val + 1
				} else {
					charMap[charInt] = 1
				}
			}
		}


		for _, val := range charMap {
			if val == int32(len(group.forms)) {
				sum_2 += 1
			}
		}
	}

	fmt.Printf("Part Two -- The new sum is %d\n", sum_2)
}