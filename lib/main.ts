import bindings from 'bindings';

const addon = bindings('skyra-ai') as Addon;

export const ticTacToe = addon.TicTacToe;

export interface Addon {
	TicTacToe(data: Uint8Array): number;
}
