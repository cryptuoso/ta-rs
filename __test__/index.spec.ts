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
  const adx = new ADX(2)
  const firstResult = await adx.next({
    time: 1577836800000,
    open: 7189.43,
    high: 7260.43,
    low: 7170.15,
    close: 7197.57,
    volume: 56801.329,
  })
  t.is(firstResult, 0)
  await adx.next({
    time: 1577886800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  const result = await adx.next({
    time: 1577896800000,
    open: 9189.43,
    high: 9260.43,
    low: 9170.15,
    close: 9197.57,
    volume: 56801.329,
  })
  t.is(result, 59.33684517024007)
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
