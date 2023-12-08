package main

import (
	"reflect"
	"testing"
)

func TestExtractDigits(t *testing.T) {
	testCases := []struct {
		input string
		want  []int32
	}{
		{input: "two1nine", want: []int32{2, 1, 9}},
		{input: "eightwothree", want: []int32{8, 2, 3}},
		{input: "abcone2threexyz", want: []int32{1, 2, 3}},
		{input: "xtwone3four", want: []int32{2, 1, 3, 4}},
		{input: "4nineeightseven2", want: []int32{4, 9, 8, 7, 2}},
		{input: "zoneight234", want: []int32{1, 8, 2, 3, 4}},
		{input: "7pqrstsixteen", want: []int32{7, 6}},
	}
	for _, tc := range testCases {
		got := extractDigits(tc.input)
		if !reflect.DeepEqual(got, tc.want) {
			t.Fatalf("want: %v, got: %v", tc.want, got)
		}
	}
}

func TestCalculateCalibrationValue(t *testing.T) {
	testCases := []struct {
		input []int32
		want  int32
	}{
		{input: []int32{1, 2}, want: 12},
		{input: []int32{1, 2, 3, 4, 5}, want: 15},
		{input: []int32{7}, want: 77},
	}
	for _, tc := range testCases {
		got, err := calculateCalibrationValue(tc.input)
		if err != nil {
			t.Fatalf("want: %v, got: %e", tc.want, err)
		}
		if got != tc.want {
			t.Fatalf("want: %v, got: %v", tc.want, got)
		}
	}
}
