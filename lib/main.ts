import bindings from 'bindings';

const addon = bindings('skyra-ai') as Addon;

export const ticTacToe = addon.TicTacToe;
export const connectFour = addon.ConnectFour;

export interface Addon {
	TicTacToe(data: Uint8Array): number;
	ConnectFour(data: Uint8Array, maximumDepth?: number): number;
}
