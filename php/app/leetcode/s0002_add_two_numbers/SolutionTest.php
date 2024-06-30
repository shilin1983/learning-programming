<?php

use App\LeetCode\s0002_add_two_numbers\Solution;
use Yosymfony\Toml\Toml;

$solution = new Solution();

function readTestFile(): array
{
    $cases = [];
    $test = Toml::parseFile(__DIR__ . "/../../../../test.toml");

    for ($i = 0; $i < count($test["table2"]["expected"]); $i++) {
        $cases["示例" . ($i + 1)] = [
            "expected" => $test["table2"]["expected"][$i],
            "nums1" => $test["table2"]["nums1"][$i],
            "nums2" => $test["table2"]["nums2"][$i],
        ];
    }

    return $cases;
}

test("2.两数相加", function (array $expected, array $nums1, array $nums2) use ($solution): void {
    expect($solution->addTwoNumbers(intToList($nums1), intToList($nums2)))->toEqual(intToList
    ($expected));
})->with(readTestFile());
