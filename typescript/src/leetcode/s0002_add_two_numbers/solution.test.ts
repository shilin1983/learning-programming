import { readFileSync } from "fs";
import { parse } from "@iarna/toml";
import { describe, expect, test } from "bun:test";
import { intToList } from "@/utils/helpers";
import addTwoNumbers from "@/leetcode/s0002_add_two_numbers/solution";

interface Test {
    table2: Owner;
}

interface Owner {
    expected: number[][];
    nums1: number[][];
    nums2: number[][];
}

interface Case {
    expected: number[];
    nums1: number[];
    nums2: number[];
}

const cases: Case[] = [];

const readTestFile = (): void => {
    const test: Test = parse(readFileSync("../../../../test.toml", "utf-8")) as any;

    for (let i = 0; i < test.table2.expected.length; i++) {
        cases.push({
            expected: test.table2.expected[i],
            nums1: test.table2.nums1[i],
            nums2: test.table2.nums2[i],
        });
    }
};

describe("2.两数相加", (): void => {
    readTestFile();

    test.each(cases)("示例%#", ({ expected, nums1, nums2 }): void => {
        expect(addTwoNumbers(intToList(nums1), intToList(nums2))).toEqual(intToList(expected));
    });
});

