package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func GetWrongPosition(input string) (horizontal int, depth int) {
	lines := strings.Split(input, "\n")

	for _, line := range lines {
		commands := strings.Split(line, " ")

		direction := commands[0]
		distance, _ := strconv.Atoi(commands[1])

		switch direction {
		case "up":
			depth -= distance
		case "down":
			depth += distance
		case "forward":
			horizontal += distance
		}
	}

	return horizontal, depth
}

func GetPosition(input string) (horizontal int, depth int) {
	lines := strings.Split(input, "\n")

	aim := 0

	for _, line := range lines {
		commands := strings.Split(line, " ")

		direction := commands[0]
		distance, _ := strconv.Atoi(commands[1])

		switch direction {
		case "up":
			aim -= distance
		case "down":
			aim += distance
		case "forward":
			horizontal += distance
			depth += aim * distance
		}
	}

	return horizontal, depth
}

func main() {
	input, err := ioutil.ReadFile("../input/day2.txt")
	if err != nil {
		panic(err)
	}

	horizontal1, depth1 := GetWrongPosition(string(input))
	fmt.Printf("Day 2, part 1: %d\n", horizontal1*depth1)

	horizontal2, depth2 := GetPosition(string(input))
	fmt.Printf("Day 2, part 2: %d\n", horizontal2*depth2)
}
