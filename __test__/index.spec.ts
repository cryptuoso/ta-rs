/* eslint-disable no-console */
import test from 'ava'

import { SMA, ADX, ATR } from '../index'

test('SMA', async (t) => {
  const sma = new SMA(10)
  await sma.next(10)
  const result = await sma.next(20)
  t.is(result, 15)
})

test('ADX', async (t) => {
  const adx = new ADX(30)
  const firstResult = await adx.next(6497)
  t.is(firstResult, NaN)
  await adx.next(6498)
  const result = await adx.next(6482.84)
  t.is(result, 0.001294319747925534)
})

test('ATR', async (t) => {
  const atr = new ATR(30)
  const result = await atr.next({
    time: 1577836800000,
    open: 7189.43,
    high: 7260.43,
    low: 7170.15,
    close: 7197.57,
    volume: 56801.329,
  })
  t.is(result, 90.28000000000065)
})

//TODO: add more tests
