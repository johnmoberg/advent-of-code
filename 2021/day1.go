package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func ParseNumbers(input string) []int {
	var numbers []int

	for _, number := range strings.Split(input, "\n") {
		iNumber, err := strconv.Atoi(number)
		if err != nil {
			panic(err)
		}

		numbers = append(numbers, iNumber)
	}
	return numbers
}

func CountAscending(numbers []int, windowSize int) int {
	var count int
	for i := windowSize; i < len(numbers); i++ {
		if numbers[i] > numbers[i-windowSize] {
			count++
		}
	}
	return count
}

func main() {
	input, err := ioutil.ReadFile("input/day1.txt")
	if err != nil {
		panic(err)
	}

	numbers := ParseNumbers(string(input))
	fmt.Printf("Day 1, part 1: %d\n", CountAscending(numbers, 1))
	fmt.Printf("Day 1, part 1: %d\n", CountAscending(numbers, 3))
}
