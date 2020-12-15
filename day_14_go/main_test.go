package main

import (
	"testing"
)

// value:  000000000000000000000000000000001011  (decimal 11)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001001001  (decimal 73)

func Test_create_bitmasks(t *testing.T) {
	mask := "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"

	zer_mask, one_mask := create_bitmasks(mask)

	expected_zer_mask := 0b111111111111111111111111111111111101
	expected_one_mask := 0b000000000000000000000000000001000000

	if expected_zer_mask != zer_mask && expected_one_mask != one_mask {
		t.Errorf("create_bitmasks() = %v, %v : want %v, %v", zer_mask, one_mask, expected_zer_mask, expected_one_mask)
	}
}

func Test_apply_bitmasks(t *testing.T) {
	value := 0b000000000000000000000000000000001011
	mask := "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
	expected_result := 0b000000000000000000000000000001001001

	zer_mask, one_mask := create_bitmasks(mask)

	result := apply_bitmasks(value, zer_mask, one_mask)

	if expected_result != result {
		t.Errorf("apply_bitmask() = %v, want %v", result, expected_result)
	}
}

func Test_part1(t *testing.T) {
	input := []string{
		"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
		"mem[8] = 11",
		"mem[7] = 101",
		"mem[8] = 0",
	}

	result := part1(input)
	expected_result := 165

	if expected_result != result {
		t.Errorf("part1() = %v, want %v", result, expected_result)
	}
}
