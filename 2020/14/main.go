package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const (
	MEM = iota
	MASK
)

type Command struct {
	command int
	val int64
	secondVal int64
	orig string
}

func main() {
	commands := parseInput()

	solution := solvePartOne(commands)
	fmt.Printf("Part One -- The solution is %d\n", solution)

	solution2 := solvePartTwo(commands)
	fmt.Printf("Part Two -- The solution is %d\n", solution2)
}

func getMasks(text string) (int64, int64) {
	andMask := strings.Replace(text, "X", "1", -1)
	orMask := strings.Replace(text, "X", "0", -1)
	iAndMask, err := strconv.ParseInt(andMask, 2, 64)
	if err != nil {
		log.Panic(err)
	}
	iOrMask, err := strconv.ParseInt(orMask, 2, 64)
	if err != nil {
		log.Panic(err)
	}
	return iAndMask, iOrMask
}

func parseInput() []Command {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var commands []Command

	for scanner.Scan() {
		text := scanner.Text()
		parts := strings.Split(text, " = ")
		if strings.Contains(parts[0], "mask") {
			andMask, orMask := getMasks(parts[1])
			commands = append(commands, Command{
				command: MASK,
				val: andMask,
				secondVal: orMask,
				orig: parts[1],
			})
		} else if strings.Contains(parts[0], "mem") {
			re := regexp.MustCompile(`\d+`)
			matched := re.FindString(parts[0])

			a, err  := strconv.ParseInt(matched, 10, 64)
			if err != nil {
				log.Panic(err)
			}
			b, err := strconv.ParseInt(parts[1], 10, 64)
			if err != nil {
				log.Panic(err)
			}

			commands = append(commands, Command{
				command: MEM,
				val: a,
				secondVal: b,
			})
		} else {
			log.Fatal(fmt.Sprintf("Invalid command %s\n", text))
		}
	}

	return commands
}

func solvePartOne(commands []Command) int64 {
	memory := make(map[int64]int64)
	var andMask, orMask int64
	for _, command := range commands {
		switch command.command {
		case MEM:
			maskedVal := command.secondVal & andMask
			maskedVal = maskedVal | orMask
			memory[command.val] = maskedVal
			break
		case MASK:
			andMask = command.val
			orMask = command.secondVal
			break
		default:
			log.Panic(fmt.Sprintf("Invalid command %s\n", command.command))
		}
	}

	var sum int64 = 0
	for _, val := range memory {
		sum += val
	}
	return sum
}

func duplicateAt(currIndex int, number string) []string {
	var a []string
	numA := []rune(number)
	numA[currIndex] = '0'
	numB := []rune(number)
	numB[currIndex] = '1'
	a = append(a, string(numA))
	a = append(a, string(numB))
	return a
}

func getMemoryAddressesRec(mask string, number string, currIndex int) []string {
	if currIndex == len(number) - 1 {
		char := string(mask[currIndex])
		if char == "X" {
			return duplicateAt(currIndex, number)
		} else if char == "1" {
			num := []rune(number)
			num[currIndex] = '1'
			number = string(num)
		}

		var a []string
		a = append(a, number)
		return a
	} else {
		a := getMemoryAddressesRec(mask, number, currIndex + 1)
		var c []string
		char := string(mask[currIndex])
		for _, num := range a {
			if char == "X" {
				b := duplicateAt(currIndex, num)
				c = append(c, b...)
			} else if char == "1" {
				newNum := []rune(num)
				newNum[currIndex] = '1'
				c = append(c, string(newNum))
			} else {
				c = append(c, num)
			}
		}
		return c
	}
}

func getMemoryAddresses(mask string, number int64) []int64 {
	addresses := getMemoryAddressesRec(mask, fmt.Sprintf("%036b", number), 0)
	var iAddresses []int64
	for _, address := range addresses {
		val, err := strconv.ParseInt(address, 2, 64)
		if err != nil {
			log.Panic(err)
		}
		iAddresses = append(iAddresses, val)
	}
	return iAddresses
}

func solvePartTwo(commands []Command) int64 {
	memory := make(map[int64]int64)
	var mask string
	for _, command := range commands {
		switch command.command {
		case MEM:
			addresses := getMemoryAddresses(mask, command.val)
			for _, address := range addresses {
				memory[address] = command.secondVal
			}
			break
		case MASK:
			mask = command.orig
			break
		default:
			log.Panic(fmt.Sprintf("Invalid command %s\n", command.command))
		}
	}

	var sum int64 = 0
	for _, val := range memory {
		sum += val
	}
	return sum
}