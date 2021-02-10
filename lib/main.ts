import bindings from 'bindings';

const addon = bindings('skyra-ai') as Addon;

export const ticTacToe = addon.ticTacToe;
export const connectFour = addon.connectFour;

export interface Addon {
	ticTacToe(data: Uint8Array): number;
	connectFour(data: Uint8Array, maximumDepth?: number): number;
}
