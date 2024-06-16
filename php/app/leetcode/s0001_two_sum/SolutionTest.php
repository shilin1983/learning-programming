<?php

use App\LeetCode\s0001_two_sum\Solution;
use Yosymfony\Toml\Toml;

$solution = new Solution();

function readTestFile(): array
{
    $cases = [];
    $test = Toml::parseFile(__DIR__ . "/../../../../test.toml");

    for ($i = 0; $i < count($test["table1"]["expected"]); $i++) {
        $cases["示例" . ($i + 1)] = [
            "expected" => $test["table1"]["expected"][$i],
            "nums" => $test["table1"]["nums"][$i],
            "target" => $test["table1"]["target"][$i],
        ];
    }

    return $cases;
}

test("1.两数之和", function (array $expected, array $nums, int $target) use ($solution): void {
    expect($solution->twoSum($nums, $target))->toEqual($expected);
})->with(readTestFile());
