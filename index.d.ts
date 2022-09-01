/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Candle {
  time: number
  open: number
  high: number
  low: number
  close: number
  volume: number
}
export interface MACDOutput {
  macd: number
  signal: number
  histogram: number
}
export class ADX {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class RSI {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class MACD {
  constructor(fastPeriod: number, slowPeriod: number, signalPeriod: number)
  next(value: number): Promise<MACDOutput>
}
export class SMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class WMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class EMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class WSMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class RMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class SMM {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class HMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class SWMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class LSMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class Conv {
  constructor(weights: Array<number>, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class VWMA {
  constructor(period: number, initial: Array<number>)
  next(values: Array<number>): Promise<number>
  getLastValue(): number
}
export class TRIMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class Diff {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class Sum {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class Momentum {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class ROC {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class TSI {
  constructor(shortPeriod: number, longPeriod: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class StDev {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class LinearVolatility {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class CCI {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class MeanAbsDev {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class MedianAbsDev {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class Vidya {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class Cross {
  constructor(initial: Array<number>)
  next(values: Array<number>): Promise<number>
}
export class CrossAbove {
  constructor(initial: Array<number>)
  next(values: Array<number>): Promise<number>
}
export class CrossUnder {
  constructor(initial: Array<number>)
  next(values: Array<number>): Promise<number>
}
export class ReversalSignal {
  constructor(left: number, right: number, initial: number)
  next(value: number): Promise<number>
}
export class HighestLowestDelta {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
