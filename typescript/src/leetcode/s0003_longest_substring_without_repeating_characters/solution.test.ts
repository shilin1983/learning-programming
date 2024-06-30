import { readFileSync } from "fs";
import { parse } from "@iarna/toml";
import { describe, expect, test } from "bun:test";
import lengthOfLongestSubstring
    from "@/leetcode/s0003_longest_substring_without_repeating_characters/solution";

interface Test {
    table3: Owner;
}

interface Owner {
    expected: number[];
    s: string[];
}

interface Case {
    expected: number;
    s: string;
}

const cases: Case[] = [];

const readTestFile = (): void => {
    const test: Test = parse(readFileSync("../../../../test.toml", "utf-8")) as any;

    for (let i = 0; i < test.table3.expected.length; i++) {
        cases.push({
            expected: test.table3.expected[i],
            s: test.table3.s[i],
        });
    }
};

describe("3.无重复字符的最长子串", (): void => {
    readTestFile();

    test.each(cases)("示例%#", ({ expected, s }): void => {
        expect(lengthOfLongestSubstring(s)).toEqual(expected);
    });
});

