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
export interface ChanADXOutput {
  value: number
  high: number
  low: number
}
export interface TaMACDOutput {
  macd: number
  signal: number
  histogram: number
}
export class ADX {
  constructor(period: number)
  next(value: Candle): Promise<number>
}
export class ChanADX {
  constructor(period: number, adxPeriod: number, ratio: number)
  next(value: Candle): Promise<ChanADXOutput>
}
export class FXHighBand {
  constructor(period: number, rsiPeriod: number, modifier: number)
  next(value: Candle): Promise<number>
}
export class FXLowBand {
  constructor(period: number, rsiPeriod: number, modifier: number)
  next(value: Candle): Promise<number>
}
export class MaxADX {
  constructor(period: number, adxPeriod: number)
  next(value: Candle): Promise<number>
}
export class TaSMA {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class TaEMA {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class TaATR {
  constructor(period: number)
  next(value: Candle): Promise<number>
}
export class TaRSI {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class TaMACD {
  constructor(fastPeriod: number, slowPeriod: number, signalPeriod: number)
  next(value: number): Promise<TaMACDOutput>
}
export class TaMaximum {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class TaMinimum {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class TaStandardDeviation {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class TaMeanAbsoluteDeviation {
  constructor(period: number)
  next(value: number): Promise<number>
}
export class YataSMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataWMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataEMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataWSMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataRMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataSMM {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataHMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataSWMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataLSMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataConv {
  constructor(weights: Array<number>, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class YataVWMA {
  constructor(period: number, initial: Array<number>)
  next(values: Array<number>): Promise<number>
  getLastValue(): number
}
export class YataTRIMA {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataDiff {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataSum {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class YataMomentum {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataROC {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataTSI {
  constructor(shortPeriod: number, longPeriod: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class YataStDev {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataLinearVolatility {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataCCI {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataMeanAbsDev {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataMedianAbsDev {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
}
export class YataVidya {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
export class YataCross {
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
export class YataReversalSignal {
  constructor(left: number, right: number, initial: number)
  next(value: number): Promise<number>
}
export class YataHighestLowestDelta {
  constructor(period: number, initial: number)
  next(value: number): Promise<number>
  getLastValue(): number
}
