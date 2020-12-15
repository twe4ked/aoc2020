package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func LoadThen(fileName string, separator string, handler func(s string)) error {
	data, err := ioutil.ReadFile(fileName)

	if err != nil {
		return err
	}

	records := strings.Split(string(data), separator)

	for _, record := range records {
		if record == "" {
			continue
		}

		handler(strings.TrimSpace(record))
	}

	return nil
}

func main() {
	var input []string
	err := LoadThen("input", "\n", func(s string) {
		for _, rowStr := range strings.Split(s, "\n") {
			input = append(input, rowStr)
		}
	})

	if err != nil {
		panic(err)
	}

	part1 := part1(input) // 9628746976360

	fmt.Printf("Visible (part 1): %v\n", part1)
}

func create_bitmasks(mask string) (int, int) {
	zer_mask := 0
	one_mask := 0

	for _, c := range mask {
		char := string(c)
		if char == "X" {
			zer_mask = zer_mask<<1 | 1
			one_mask = one_mask<<1 | 0
		}
		if char == "0" {
			zer_mask = zer_mask<<1 | 0
			one_mask = one_mask<<1 | 0
		}
		if char == "1" {
			zer_mask = zer_mask<<1 | 1
			one_mask = one_mask<<1 | 1
		}
	}

	return zer_mask, one_mask
}

func apply_bitmasks(value, zer_mask, one_mask int) int {
	return (value & zer_mask) | one_mask
}

func part1(lines []string) int {
	var zer_mask, one_mask int
	register := map[int]int{}

	// "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
	// "mem[8] = 11",
	// "mem[7] = 101",
	// "mem[8] = 0",
	for _, line := range lines {
		// mask line
		if string(line[1]) == "a" {
			parts := strings.Split(line, " = ")

			zer_mask, one_mask = create_bitmasks(parts[1])
		}

		// mem line
		if string(line[1]) == "e" {
			line := strings.Split(line, "[")[1]
			parts := strings.Split(line, "]")

			index, _ := strconv.Atoi(parts[0])

			value_str := strings.Split(parts[1], " = ")[1]

			value, _ := strconv.Atoi(value_str)

			masked_value := apply_bitmasks(value, zer_mask, one_mask)

			register[index] = masked_value
		}
	}

	sum := 0

	for _, value := range register {
		sum += value
	}

	return sum
}
