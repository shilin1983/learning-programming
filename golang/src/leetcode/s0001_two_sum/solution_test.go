package s0001_two_sum

import (
	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/assert"
	"log"
	"testing"
)

type Test struct {
	Table1 Owner `toml:"table1"`
}

type Owner struct {
	Expected [][]int `toml:"expected"`
	Nums     [][]int `toml:"nums"`
	Target   []int   `toml:"target"`
}

type Case struct {
	expected []int
	nums     []int
	target   int
}

var cases []Case

func readTestFile() {
	var test Test

	if _, err := toml.DecodeFile("../../../../test.toml", &test); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < len(test.Table1.Expected); i++ {
		cases = append(cases, Case{
			expected: test.Table1.Expected[i],
			nums:     test.Table1.Nums[i],
			target:   test.Table1.Target[i]},
		)
	}
}

func TestTwoSum(t *testing.T) {
	readTestFile()

	for _, c := range cases {
		t.Run("1.两数之和", func(t *testing.T) {
			assert.Equal(t, c.expected, twoSum(c.nums, c.target))
		})
	}
}
