package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

const BOARD_SIZE = 5

func ParseBingo(input string) ([]int, [][][]int) {
	var numbers []int
	var bingo [][][]int

	lines := strings.Split(input, "\n")

	for _, s := range strings.Split(lines[0], ",") {
		num, _ := strconv.Atoi(s)
		numbers = append(numbers, num)
	}

	i := 2
	for i < len(lines) {
		var singleBingo [][]int
		for j := 0; j < BOARD_SIZE; j++ {
			var row []int
			for _, s := range strings.Split(lines[i+j], " ") {
				num, _ := strconv.Atoi(s)
				row = append(row, num)
			}

			singleBingo = append(singleBingo, row)
		}
		bingo = append(bingo, singleBingo)
		i += BOARD_SIZE + 1
	}

	return numbers, bingo
}

func computeWinIndex(m map[int]int, board [][]int) int {
	minRowOrCol := -1

	for i := 0; i < BOARD_SIZE; i++ {
		maxIndex := -1

		for j := 0; j < BOARD_SIZE; j++ {
			if m[board[i][j]] > maxIndex {
				maxIndex = m[board[i][j]]
			}
		}

		if minRowOrCol == -1 || maxIndex < minRowOrCol {
			minRowOrCol = maxIndex
		}
	}

	for i := 0; i < BOARD_SIZE; i++ {
		maxIndex := -1

		for j := 0; j < BOARD_SIZE; j++ {
			if m[board[j][i]] > maxIndex {
				maxIndex = m[board[j][i]]
			}
		}

		if minRowOrCol == -1 || maxIndex < minRowOrCol {
			minRowOrCol = maxIndex
		}
	}

	return minRowOrCol
}

func ComputeWinningScore(numbers []int, boards [][][]int) int {
	numMap := make(map[int]int)
	for i, num := range numbers {
		numMap[num] = i
	}

	minWinIndex := 2147483647
	winningBoardIndex := -1

	for boardIndex, board := range boards {
		winIndex := computeWinIndex(numMap, board)
		if winIndex < minWinIndex {
			minWinIndex = winIndex
			winningBoardIndex = boardIndex
		}
	}

	score := 0
	for i := 0; i < BOARD_SIZE; i++ {
		for j := 0; j < BOARD_SIZE; j++ {
			if numMap[boards[winningBoardIndex][i][j]] > minWinIndex {
				score += boards[winningBoardIndex][i][j]
			}
		}
	}

	return score * numbers[minWinIndex]
}

func ComputeLastWinningScore(numbers []int, boards [][][]int) int {
	numMap := make(map[int]int)
	for i, num := range numbers {
		numMap[num] = i
	}

	maxWinIndex := -1
	winningBoardIndex := -1

	for boardIndex, board := range boards {
		winIndex := computeWinIndex(numMap, board)
		if winIndex > maxWinIndex {
			maxWinIndex = winIndex
			winningBoardIndex = boardIndex
		}
	}

	score := 0
	for i := 0; i < BOARD_SIZE; i++ {
		for j := 0; j < BOARD_SIZE; j++ {
			if numMap[boards[winningBoardIndex][i][j]] > maxWinIndex {
				score += boards[winningBoardIndex][i][j]
			}
		}
	}

	return score * numbers[maxWinIndex]
}

func main() {
	input, err := ioutil.ReadFile("../input/day4.txt")
	if err != nil {
		panic(err)
	}

	numbers, boards := ParseBingo(string(input))
	score1 := ComputeWinningScore(numbers, boards)
	score2 := ComputeLastWinningScore(numbers, boards)

	fmt.Printf("Day 4, part 1: %d\n", score1)
	fmt.Printf("Day 4, part 2: %d\n", score2)
}
