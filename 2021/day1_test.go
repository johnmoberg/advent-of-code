package main

import (
	"reflect"
	"testing"
)

func TestParseNumbers(t *testing.T) {
	input := `199
200
208
210
200`
	numbers := ParseNumbers(input)

	expected := []int{199, 200, 208, 210, 200}
	actual := numbers

	if !reflect.DeepEqual(actual, expected) {
		t.Errorf("Expected %v, got %v", expected, actual)
	}
}

func TestAscending(t *testing.T) {
	input := `199
200
208
210
200
207
240
269
260
263`
	numbers := ParseNumbers(input)

	t.Run("window size is 1", func(t *testing.T) {
		got := CountAscending(numbers, 1)
		want := 7

		if got != want {
			t.Errorf("Got %d, want %d", got, want)
		}
	})
	t.Run("window size is 3", func(t *testing.T) {
		got := CountAscending(numbers, 3)
		want := 5

		if got != want {
			t.Errorf("Got %d, want %d", got, want)
		}
	})
}
