package main

/*
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all the calibration values?
*/
import (
	_ "embed"
	"errors"
	"fmt"
	"strings"
	"unicode"
)

const asciiDigitOffset = '0'

//go:embed input.txt
var input string
var digitStrings = map[string]int32{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func extractDigits(s string) []int32 {
	length := len(s)
	var digits []int32
	for pos, char := range s {
		if unicode.IsDigit(char) {
			digits = append(digits, char-asciiDigitOffset)
		} else {
			for digitString, digit := range digitStrings {
				endPos := pos + len(digitString)
				if endPos <= length && s[pos:endPos] == digitString {
					digits = append(digits, digit)
					break
				}
			}
		}
	}
	return digits
}

func calculateCalibrationValue(digits []int32) (int32, error) {
	length := len(digits)
	if length == 0 {
		return 0, errors.New("expected at least one digit, got []")
	}
	return digits[0]*10 + digits[length-1], nil
}

func main() {
	var sum int32
	for _, s := range strings.Split(input, "\n") {
		if s == "" {
			continue
		}
		digits := extractDigits(s)
		cv, err := calculateCalibrationValue(digits)
		if err != nil {
			fmt.Printf("Couldn't calculate calibration value for '%s'", s)
			continue
		}
		sum += cv
	}
	fmt.Println(sum)
}
