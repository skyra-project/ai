import { connectFour } from '../lib/main';

describe('ConnectFour', () => {
	test('GIVEN no args THEN throws TypeError', () => {
		// @ts-expect-error
		expect(() => connectFour()).toThrow('data must be a typed array');
	});

	test('GIVEN null THEN throws TypeError', () => {
		// @ts-expect-error
		expect(() => connectFour(null)).toThrow('data must be a typed array');
	});

	test('GIVEN array THEN throws TypeError', () => {
		// @ts-expect-error
		// prettier-ignore
		expect(() => connectFour([
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0
		])).toThrow('data must be a typed array');
	});

	test('GIVEN Uint8Array with too little elements THEN throws TypeError', () => {
		// prettier-ignore
		expect(() => connectFour(new Uint8Array([
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0
		]))).toThrow('data must have exactly 42 numbers');
	});

	test('GIVEN Uint8Array with too many elements THEN throws TypeError', () => {
		// prettier-ignore
		expect(() => connectFour(new Uint8Array([
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0
		]))).toThrow('data must have exactly 42 numbers');
	});

	test('GIVEN empty board THEN returns 4', () => {
		// prettier-ignore
		expect(connectFour(new Uint8Array([
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0
		]))).toEqual(3);
	});

	test('GIVEN possible horizontal row (0..2) THEN returns 3', () => {
		// prettier-ignore
		expect(connectFour(new Uint8Array([
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			1, 1, 0, 0, 0, 0, 0,
			2, 2, 2, 0, 0, 0, 0
		]))).toEqual(3);
	});

	test('GIVEN possible horizontal row (1..3) THEN returns 4', () => {
		// prettier-ignore
		expect(connectFour(new Uint8Array([
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0,
			0, 0, 1, 1, 0, 0, 0,
			1, 2, 2, 2, 0, 0, 0
		]))).toEqual(4);
	});

	test.skip('GIVEN possible vertical row (0) THEN returns 4', () => {
		expect(connectFour(new Uint8Array([0, 1, 2, 0, 0, 2, 0, 1, 0]))).toEqual(4);
	});

	test.skip('GIVEN possible vertical row (1) THEN returns 7', () => {
		expect(connectFour(new Uint8Array([0, 2, 0, 0, 2, 0, 1, 0, 1]))).toEqual(7);
	});

	test.skip('GIVEN possible vertical row (2) THEN returns 3', () => {
		expect(connectFour(new Uint8Array([1, 0, 2, 0, 0, 2, 1, 0, 0]))).toEqual(3);
	});

	test.skip('GIVEN ascending diagonal (0) THEN returns 2', () => {
		expect(connectFour(new Uint8Array([1, 0, 0, 0, 2, 0, 2, 1, 0]))).toEqual(2);
	});

	test.skip('GIVEN ascending diagonal (1) THEN returns 4', () => {
		expect(connectFour(new Uint8Array([1, 0, 2, 0, 0, 0, 2, 0, 1]))).toEqual(4);
	});

	test.skip('GIVEN ascending diagonal (2) THEN returns 6', () => {
		expect(connectFour(new Uint8Array([1, 0, 2, 1, 2, 0, 0, 0, 0]))).toEqual(6);
	});

	test.skip('GIVEN descending diagonal (0) THEN returns 0', () => {
		expect(connectFour(new Uint8Array([0, 0, 0, 1, 2, 0, 1, 0, 2]))).toEqual(0);
	});

	test.skip('GIVEN descending diagonal (1) THEN returns 4', () => {
		expect(connectFour(new Uint8Array([2, 0, 1, 0, 0, 0, 1, 0, 2]))).toEqual(4);
	});

	test.skip('GIVEN descending diagonal (2) THEN returns 8', () => {
		expect(connectFour(new Uint8Array([2, 0, 0, 0, 2, 0, 1, 1, 0]))).toEqual(8);
	});

	test.skip('GIVEN filled board (loss) THEN returns -1', () => {
		expect(connectFour(new Uint8Array([1, 1, 1, 1, 1, 1, 1, 1, 1]))).toEqual(-1);
	});

	test.skip('GIVEN filled board (draw) THEN returns -1', () => {
		expect(connectFour(new Uint8Array([1, 2, 1, 1, 2, 1, 2, 1, 2]))).toEqual(-1);
	});

	test.skip('GIVEN filled board (win) THEN returns -1', () => {
		expect(connectFour(new Uint8Array([2, 2, 2, 2, 2, 2, 2, 2, 2]))).toEqual(-1);
	});
});
