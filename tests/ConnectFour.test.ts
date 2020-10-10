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
		expect(() => connectFour([0, 0, 0, 0, 0, 0, 0, 0, 0])).toThrow('data must be a typed array');
	});

	test('GIVEN Uint8Array with too little elements THEN throws TypeError', () => {
		expect(() => connectFour(new Uint8Array(40))).toThrow('data must have exactly 42 numbers');
	});

	test('GIVEN Uint8Array with too many elements THEN throws TypeError', () => {
		expect(() => connectFour(new Uint8Array(46))).toThrow('data must have exactly 42 numbers');
	});

	// TODO(kyranet): Add more tests.
});
