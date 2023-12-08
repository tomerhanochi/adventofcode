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
		{input: "1abc2", want: []int32{1, 2}},
		{input: "pqr3stu8vwx", want: []int32{3, 8}},
		{input: "a1b2c3d4e5f", want: []int32{1, 2, 3, 4, 5}},
		{input: "treb7uchet", want: []int32{7}},
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
