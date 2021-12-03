import {Connection, PublicKey} from '@solana/web3.js';
import {deserializeUnchecked, Schema, serialize} from 'borsh';

/**
 * The state of a greeting account managed by the hello world program
 */
export class GreetingAccount {
  counter = 0;
  constructor(fields: {counter: number} | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}

/**
 * Borsh schema definition for greeting accounts
 */
export const GreetingSchema = new Map([
  [GreetingAccount, {kind: 'struct', fields: [['counter', 'u32']]}],
]);

/**
 * The expected size of each greeting account.
 */
export const GREETING_SIZE = serialize(
  GreetingSchema,
  new GreetingAccount(),
).length;

/**
 * Borsh enum start within instruction range 0..n
 */
class Increment {
  instruction = 0;
  constructor(fields: {instruction: number} | undefined = undefined) {
    if (fields) {
      this.instruction = fields.instruction;
    }
  }
}

/**
 * Borsh schema definition for greeting accounts
 */
const IncrementSchema = new Map([
  [Increment, {kind: 'struct', fields: [['instruction', 'u8']]}],
]);

export const HelloWorldInstruction = {Increment, IncrementSchema};
