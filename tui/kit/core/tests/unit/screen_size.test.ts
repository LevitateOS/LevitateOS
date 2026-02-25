import { describe, expect, it } from "bun:test";
import {
	DEFAULT_TERMINAL_SIZE,
	normalizeTerminalSize,
	readScreenSize,
	terminalMeetsMinimum,
} from "../../src/runtime/terminal/screen-size";

describe("screen size helpers", () => {
	it("normalizes invalid dimensions to fallback", () => {
		expect(
			normalizeTerminalSize(
				{
					columns: Number.NaN,
					rows: -1,
				},
				{
					columns: 120,
					rows: 40,
				},
			),
		).toEqual({
			columns: 120,
			rows: 40,
		});
	});

	it("reads provided target values with fallback defaults", () => {
		expect(
			readScreenSize({
				columns: 200,
				rows: 60,
			}),
		).toEqual({
			columns: 200,
			rows: 60,
		});

		expect(readScreenSize(undefined, DEFAULT_TERMINAL_SIZE)).toEqual(DEFAULT_TERMINAL_SIZE);
	});

	it("checks minimum bounds", () => {
		expect(
			terminalMeetsMinimum(
				{
					columns: 100,
					rows: 30,
				},
				{
					columns: 90,
					rows: 24,
				},
			),
		).toBe(true);

		expect(
			terminalMeetsMinimum(
				{
					columns: 80,
					rows: 20,
				},
				{
					columns: 90,
					rows: 24,
				},
			),
		).toBe(false);
	});
});
