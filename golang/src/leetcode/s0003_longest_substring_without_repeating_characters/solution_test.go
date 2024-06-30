package s0003_longest_substring_without_repeating_characters

import (
	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/assert"
	"log"
	"testing"
)

type Test struct {
	Table3 Owner `toml:"table3"`
}

type Owner struct {
	Expected []int    `toml:"expected"`
	S        []string `toml:"s"`
}

type Case struct {
	expected int
	s        string
}

var cases []Case

func readTestFile() {
	var test Test

	if _, err := toml.DecodeFile("../../../../test.toml", &test); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < len(test.Table3.Expected); i++ {
		cases = append(cases, Case{
			expected: test.Table3.Expected[i],
			s:        test.Table3.S[i]},
		)
	}
}

func TestLengthOfLongestSubstring(t *testing.T) {
	readTestFile()

	for _, c := range cases {
		t.Run("3.无重复字符的最长最长", func(t *testing.T) {
			assert.Equal(t, c.expected, lengthOfLongestSubstring(c.s))
		})
	}
}
