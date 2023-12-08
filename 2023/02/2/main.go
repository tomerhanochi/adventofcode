package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

/*
--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/

//go:embed input.txt
var input string

var totalCountPerColor = map[string]uint{
	"red":   12,
	"green": 13,
	"blue":  14,
}

type Round map[string]uint

type Game struct {
	id     uint
	rounds []Round
}

func parseRound(s string) (Round, error) {
	round := make(Round)
	for _, turn := range strings.Split(s, ", ") {
		parts := strings.Split(turn, " ")
		if len(parts) != 2 {
			return Round{}, fmt.Errorf("invalid turn '%s' in round %s", turn, s)
		}
		count, err := strconv.Atoi(parts[0])
		if err != nil {
			return Round{}, fmt.Errorf("invalid count in turn '%s' in round %s", turn, s)
		}
		round[parts[1]] = uint(count)
	}
	return round, nil
}

func parseGame(s string) (Game, error) {
	parts := strings.Split(s, ": ")
	if len(parts) != 2 {
		return Game{}, fmt.Errorf("invalid game: %s", s)
	}
	id, err := strconv.Atoi(strings.TrimPrefix(parts[0], "Game "))
	if err != nil {
		return Game{}, fmt.Errorf("invalid game id: %s", s)
	}
	var rounds []Round
	for _, roundString := range strings.Split(parts[1], "; ") {
		if roundString == "" {
			continue
		}
		round, err := parseRound(roundString)
		if err != nil {
			return Game{}, fmt.Errorf("invalid round '%s' in game %d, got %e", roundString, id, err)
		}
		rounds = append(rounds, round)
	}

	return Game{id: uint(id), rounds: rounds}, nil
}

func minimumRequiredCubes(game Game) map[string]uint {
	/*
		Calculates the minimum number of cubes of each color required to complete a game.
		For example, for the game:
			3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
		The minimum required cubes would be
			* 4 red
			* 2 green
			* 6 blue.
	*/
	minimum := make(map[string]uint)
	for _, round := range game.rounds {
		for color, count := range round {
			currentCount, ok := minimum[color]
			if ok {
				minimum[color] = max(currentCount, count)
			} else {
				minimum[color] = count
			}
		}
	}
	return minimum
}

func main() {
	var sum uint
	for _, gameString := range strings.Split(input, "\n") {
		if gameString == "" {
			continue
		}
		g, err := parseGame(gameString)
		if err != nil {
			panic(err)
		}
		var mult uint = 1
		for _, count := range minimumRequiredCubes(g) {
			mult *= count
		}
		sum += mult
	}
	fmt.Println(sum)
}
