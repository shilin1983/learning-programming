<?php

use App\LeetCode\s0003_longest_substring_without_repeating_characters\Solution;
use Yosymfony\Toml\Toml;

$solution = new Solution();

function readTestFile(): array
{
    $cases = [];
    $test = Toml::parseFile(__DIR__ . "/../../../../test.toml");

    for ($i = 0; $i < count($test["table3"]["expected"]); $i++) {
        $cases["示例" . ($i + 1)] = [
            "expected" => $test["table3"]["expected"][$i],
            "s" => $test["table3"]["s"][$i],
        ];
    }

    return $cases;
}

test("3.无重复字符的最长子串", function (int $expected, string $s) use ($solution): void {
    expect($solution->lengthOfLongestSubstring($s))->toEqual($expected);
})->with(readTestFile());
