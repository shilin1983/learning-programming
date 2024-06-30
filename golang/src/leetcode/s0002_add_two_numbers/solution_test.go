package s0002_add_two_numbers

import (
	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/assert"
	"golang/src/utils"
	"log"
	"testing"
)

type Test struct {
	Table2 Owner `toml:"table2"`
}

type Owner struct {
	Expected [][]int `toml:"expected"`
	Nums1    [][]int `toml:"nums1"`
	Nums2    [][]int `toml:"nums2"`
}

type Case struct {
	expected []int
	nums1    []int
	nums2    []int
}

var cases []Case

func readTestFile() {
	var test Test

	if _, err := toml.DecodeFile("../../../../test.toml", &test); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < len(test.Table2.Expected); i++ {
		cases = append(cases, Case{
			expected: test.Table2.Expected[i],
			nums1:    test.Table2.Nums1[i],
			nums2:    test.Table2.Nums2[i]},
		)
	}
}

func TestAddTwoNumbers(t *testing.T) {
	readTestFile()

	for _, c := range cases {
		t.Run("2.两数相加", func(t *testing.T) {
			assert.Equal(t, utils.IntToList(c.expected), addTwoNumbers(utils.IntToList(c.nums1), utils.IntToList(c.nums2)))
		})
	}
}
