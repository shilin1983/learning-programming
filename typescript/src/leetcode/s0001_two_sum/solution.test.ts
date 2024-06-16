import { readFileSync } from "fs";
import { parse } from "@iarna/toml";
import { describe, expect, test } from "bun:test";
import twoSum from "@/leetcode/s0001_two_sum/solution";

interface Test {
    table1: Owner;
}

interface Owner {
    expected: number[][];
    nums: number[][];
    target: number[];
}

interface Case {
    expected: number[];
    nums: number[];
    target: number;
}

const cases: Case[] = [];

const readTestFile = (): void => {
    const test: Test = parse(readFileSync("../../../../test.toml", "utf-8")) as any;

    for (let i = 0; i < test.table1.expected.length; i++) {
        cases.push({
            expected: test.table1.expected[i],
            nums: test.table1.nums[i],
            target: test.table1.target[i],
        });
    }
};

describe("1.两数之和", (): void => {
    readTestFile();

    test.each(cases)("示例%#", ({ expected, nums, target }): void => {
        expect(twoSum(nums, target)).toEqual(expected);
    });
});

