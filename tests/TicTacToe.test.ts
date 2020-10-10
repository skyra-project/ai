import { ticTacToe } from '../lib/main';

describe('TicTacToe', () => {
	test('GIVEN no args THEN throws TypeError', () => {
		// @ts-expect-error
		expect(() => ticTacToe()).toThrow('data must be a typed array');
	});

	test('GIVEN null THEN throws TypeError', () => {
		// @ts-expect-error
		expect(() => ticTacToe(null)).toThrow('data must be a typed array');
	});

	test('GIVEN array THEN throws TypeError', () => {
		// @ts-expect-error
		expect(() => ticTacToe([0, 0, 0, 0, 0, 0, 0, 0, 0])).toThrow('data must be a typed array');
	});

	test('GIVEN Uint8Array with too little elements THEN throws TypeError', () => {
		expect(() => ticTacToe(new Uint8Array([0, 0, 0, 0, 0, 0]))).toThrow('data must have exactly 9 numbers');
	});

	test('GIVEN Uint8Array with too many elements THEN throws TypeError', () => {
		expect(() => ticTacToe(new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))).toThrow('data must have exactly 9 numbers');
	});

	test('GIVEN empty board THEN returns 4', () => {
		expect(ticTacToe(new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0, 0]))).toEqual(4);
	});

	test('GIVEN possible horizontal row (0..2) THEN returns 2', () => {
		expect(ticTacToe(new Uint8Array([2, 2, 0, 1, 1, 0, 0, 0, 0]))).toEqual(2);
	});

	test('GIVEN possible horizontal row (3..5) THEN returns 5', () => {
		expect(ticTacToe(new Uint8Array([0, 0, 0, 2, 2, 0, 1, 1, 0]))).toEqual(5);
	});

	test('GIVEN possible horizontal row (6..8) THEN returns 4', () => {
		expect(ticTacToe(new Uint8Array([0, 0, 0, 1, 0, 1, 2, 2, 0]))).toEqual(4);
	});

	test('GIVEN possible vertical row (0) THEN returns 4', () => {
		expect(ticTacToe(new Uint8Array([0, 1, 2, 0, 0, 2, 0, 1, 0]))).toEqual(4);
	});

	test('GIVEN possible vertical row (1) THEN returns 7', () => {
		expect(ticTacToe(new Uint8Array([0, 2, 0, 0, 2, 0, 1, 0, 1]))).toEqual(7);
	});

	test('GIVEN possible vertical row (2) THEN returns 3', () => {
		expect(ticTacToe(new Uint8Array([1, 0, 2, 0, 0, 2, 1, 0, 0]))).toEqual(3);
	});

	test('GIVEN ascending diagonal (0) THEN returns 2', () => {
		expect(ticTacToe(new Uint8Array([1, 0, 0, 0, 2, 0, 2, 1, 0]))).toEqual(2);
	});

	test('GIVEN ascending diagonal (1) THEN returns 4', () => {
		expect(ticTacToe(new Uint8Array([1, 0, 2, 0, 0, 0, 2, 0, 1]))).toEqual(4);
	});

	test('GIVEN ascending diagonal (2) THEN returns 6', () => {
		expect(ticTacToe(new Uint8Array([1, 0, 2, 1, 2, 0, 0, 0, 0]))).toEqual(6);
	});

	test('GIVEN descending diagonal (0) THEN returns 0', () => {
		expect(ticTacToe(new Uint8Array([0, 0, 0, 1, 2, 0, 1, 0, 2]))).toEqual(0);
	});

	test('GIVEN descending diagonal (1) THEN returns 4', () => {
		expect(ticTacToe(new Uint8Array([2, 0, 1, 0, 0, 0, 1, 0, 2]))).toEqual(4);
	});

	test('GIVEN descending diagonal (2) THEN returns 8', () => {
		expect(ticTacToe(new Uint8Array([2, 0, 0, 0, 2, 0, 1, 1, 0]))).toEqual(8);
	});

	test('GIVEN filled board (loss) THEN returns -1', () => {
		expect(ticTacToe(new Uint8Array([1, 1, 1, 1, 1, 1, 1, 1, 1]))).toEqual(-1);
	});

	test('GIVEN filled board (draw) THEN returns -1', () => {
		expect(ticTacToe(new Uint8Array([1, 2, 1, 1, 2, 1, 2, 1, 2]))).toEqual(-1);
	});

	test('GIVEN filled board (win) THEN returns -1', () => {
		expect(ticTacToe(new Uint8Array([2, 2, 2, 2, 2, 2, 2, 2, 2]))).toEqual(-1);
	});
});
