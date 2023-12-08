package main

import (
	"reflect"
	"testing"
)

func TestParseRound(t *testing.T) {
	testCases := []struct {
		input string
		want  Round
	}{
		{input: "2 green", want: Round{"green": 2}},
		{input: "3 blue, 4 red", want: Round{"red": 4, "blue": 3}},
		{input: "1 red, 2 green, 6 blue", want: Round{"red": 1, "green": 2, "blue": 6}},
		{input: "6 red, 1 blue, 3 green", want: Round{"red": 6, "green": 3, "blue": 1}},
	}
	for _, tc := range testCases {
		got, err := parseRound(tc.input)
		if err != nil {
			t.Fatalf("want: %v, got: %e", tc.want, err)
		}
		if !reflect.DeepEqual(got, tc.want) {
			t.Fatalf("want: %v, got: %v", tc.want, got)
		}
	}
}

func TestParseGame(t *testing.T) {
	testCases := []struct {
		input string
		want  Game
	}{
		{
			input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			want:  Game{id: 1, rounds: []Round{{"red": 4, "blue": 3}, {"red": 1, "green": 2, "blue": 6}, {"green": 2}}},
		},
		{
			input: "Game 23: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			want:  Game{id: 23, rounds: []Round{{"green": 2, "blue": 1}, {"red": 1, "green": 3, "blue": 4}, {"green": 1, "blue": 1}}},
		},
		{
			input: "Game 456: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			want:  Game{id: 456, rounds: []Round{{"red": 20, "green": 8, "blue": 6}, {"red": 4, "green": 13, "blue": 5}, {"red": 1, "green": 5}}},
		},
		{
			input: "Game 7890: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			want:  Game{id: 7890, rounds: []Round{{"red": 3, "green": 1, "blue": 6}, {"red": 6, "green": 3}, {"red": 14, "green": 3, "blue": 15}}},
		},
		{
			input: "Game 12345: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
			want:  Game{id: 12345, rounds: []Round{{"red": 6, "green": 3, "blue": 1}, {"red": 1, "green": 2, "blue": 2}}},
		},
	}
	for _, tc := range testCases {
		got, err := parseGame(tc.input)
		if err != nil {
			t.Fatalf("want: %v, got: %e", tc.want, err)
		}
		if !reflect.DeepEqual(got, tc.want) {
			t.Fatalf("want: %v, got: %v", tc.want, got)
		}
	}
}

func TestMinimumRequiredCubes(t *testing.T) {
	testCases := []struct {
		input Game
		want  map[string]uint
	}{
		{
			input: Game{id: 1, rounds: []Round{{"red": 4, "blue": 3}, {"red": 1, "green": 2, "blue": 6}, {"green": 2}}},
			want:  map[string]uint{"red": 4, "green": 2, "blue": 6},
		},
		{
			input: Game{id: 23, rounds: []Round{{"green": 2, "blue": 1}, {"red": 1, "green": 3, "blue": 4}, {"green": 1, "blue": 1}}},
			want:  map[string]uint{"red": 1, "green": 3, "blue": 4},
		},
		{
			input: Game{id: 456, rounds: []Round{{"red": 20, "green": 8, "blue": 6}, {"red": 4, "green": 13, "blue": 5}, {"red": 1, "green": 5}}},
			want:  map[string]uint{"red": 20, "green": 13, "blue": 6},
		},
		{
			input: Game{id: 7890, rounds: []Round{{"red": 3, "green": 1, "blue": 6}, {"red": 6, "green": 3}, {"red": 14, "green": 3, "blue": 15}}},
			want:  map[string]uint{"red": 14, "green": 3, "blue": 15},
		},
		{
			input: Game{id: 12345, rounds: []Round{{"red": 6, "green": 3, "blue": 1}, {"red": 1, "green": 2, "blue": 2}}},
			want:  map[string]uint{"red": 6, "green": 3, "blue": 2},
		},
	}
	for _, tc := range testCases {
		got := minimumRequiredCubes(tc.input)
		if !reflect.DeepEqual(got, tc.want) {
			t.Fatalf("want: %v, got: %v", tc.want, got)
		}
	}
}
