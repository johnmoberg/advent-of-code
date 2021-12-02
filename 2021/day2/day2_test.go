package main

import (
	"testing"

	"github.com/matryer/is"
)

const testInput = `forward 5
down 5
forward 8
up 3
down 8
forward 2`

func TestGetWrongPosition(t *testing.T) {
	is := is.New(t)

	wantHorizontal := 15
	wantDepth := 10

	gotHorizontal, gotDepth := GetWrongPosition(testInput)

	is.Equal(gotHorizontal, wantHorizontal)
	is.Equal(gotDepth, wantDepth)
}

func TestGetPosition(t *testing.T) {
	is := is.New(t)

	wantHorizontal := 15
	wantDepth := 60

	gotHorizontal, gotDepth := GetPosition(testInput)

	is.Equal(gotHorizontal, wantHorizontal)
	is.Equal(gotDepth, wantDepth)
}
