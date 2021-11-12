package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Instruction struct {
	instruction string
	number int
	hasBeenRun bool
}

func main() {
	f, err := os.Open("8/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	var instructions []Instruction

	for scanner.Scan() {
		text := scanner.Text()
		instructions = append(instructions, parseInstruction(text))
	}

	acc := solvePartOne(instructions)
	fmt.Printf("Part One -- Acc ended up at %d\n", acc)

	instructions = reset(instructions)
	acc = solvePartTwo(instructions)
	fmt.Printf("Part Two -- Acc ended up at %d\n", acc)
}

func reset(instructions []Instruction) []Instruction {
	for i, _ := range instructions {
		instructions[i].hasBeenRun = false
	}
	return instructions
}

func parseInstruction(text string) Instruction {
	insVal := strings.Split(text, " ")
	if len(insVal) != 2 {
		log.Panic(fmt.Sprintf("Invalid input %+v (from: %s)", insVal, text))
	}
	instruction := insVal[0]
	number, err := strconv.Atoi(insVal[1])
	if err != nil {
		log.Panic(err)
	}

	return Instruction{
		instruction: instruction,
		number: number,
		hasBeenRun: false,
	}
}

func solvePartOne(instructions []Instruction) int {
	acc := 0
	pc := 0
	oldPc := 0
	currIns := instructions[0]
	for currIns.hasBeenRun == false {
		oldPc = pc
		switch currIns.instruction {
		case "acc":
			acc += currIns.number
			pc += 1
			break
		case "nop":
			pc +=1
			break
		case "jmp":
			pc += currIns.number
			break
		default:
			log.Panic(fmt.Sprintf("Invalid instruction! %s", currIns.instruction))
		}
		instructions[oldPc].hasBeenRun = true
		currIns = instructions[pc]
	}
	return acc
}

func solvePartTwo(instructions []Instruction) int {
	var nopsOrJmps []int
	for index, instruction := range instructions {
		if instruction.instruction == "jmp" || instruction.instruction == "nop" {
			nopsOrJmps = append(nopsOrJmps, index)
		}
	}

	for _, nOJ := range nopsOrJmps {
		tmpInstructions := replaceInstruction(instructions, nOJ)
		acc, ter := checkTermination(tmpInstructions)
		if ter {
			return acc
		}
	}
	log.Panic("Never executed currectly :(")
	return -1
}

func replaceInstruction(instructions []Instruction, index int) []Instruction {
	newInstructions := make([]Instruction, len(instructions))
	copy(newInstructions, instructions)

	ins := newInstructions[index]
	inst := ""
	if ins.instruction == "nop" {
		inst = "jmp"
	} else {
		inst = "nop"
	}
	newInstructions[index] = Instruction{
		instruction: inst,
		number: ins.number,
		hasBeenRun: false,
	}

	return newInstructions
}

func checkTermination(instructions []Instruction) (int, bool) {
	acc := 0
	pc := 0
	oldPc := 0
	currIns := instructions[0]
	for currIns.hasBeenRun == false {
		oldPc = pc
		switch currIns.instruction {
		case "acc":
			acc += currIns.number
			pc += 1
			break
		case "nop":
			pc +=1
			break
		case "jmp":
			pc += currIns.number
			break
		default:
			log.Panic(fmt.Sprintf("Invalid instruction! %s", currIns.instruction))
		}

		instructions[oldPc].hasBeenRun = true
		if pc == len(instructions) {
			return acc, true
		}
		currIns = instructions[pc]
	}
	return acc, false
}