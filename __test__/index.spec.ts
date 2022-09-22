/* eslint-disable no-console */
import test from 'ava'

import { TaSMA, ADX, TaATR, ChanADX, RachSupTrend } from '../index'

test('TaSMA', async (t) => {
  const sma = new TaSMA(10)
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

test('ChanADX', async (t) => {
  const adx = new ChanADX(2, 2, 2)
  const firstResult = await adx.next({
    time: 1577836800000,
    open: 7189.43,
    high: 7260.43,
    low: 7170.15,
    close: 7197.57,
    volume: 56801.329,
  })

  t.is(firstResult.value, 0)
  t.is(firstResult.high, 0)
  t.is(firstResult.low, 0)
  await adx.next({
    time: 1577886800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  await adx.next({
    time: 1577896800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  await adx.next({
    time: 1577986800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  await adx.next({
    time: 1579886800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  const result = await adx.next({
    time: 1597896800000,
    open: 9189.43,
    high: 9260.43,
    low: 9170.15,
    close: 9197.57,
    volume: 56801.329,
  })

  t.is(result.value, 1)
  t.is(result.high, 9260.43)
  t.is(result.low, 9170.15)
})

test('TaATR', async (t) => {
  const atr = new TaATR(30)
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

test('RST', async (t) => {
  const rst = new RachSupTrend(2, 2)

  const firstResult = await rst.next({
    time: 1577836800000,
    open: 7189.43,
    high: 7260.43,
    low: 7170.15,
    close: 7197.57,
    volume: 56801.329,
  })

  t.is(firstResult.buy, 0)
  t.is(firstResult.sell, 0)
  t.is(firstResult.buyEntry, 0)
  t.is(firstResult.sellEntry, 0)
  await rst.next({
    time: 1577886800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  await rst.next({
    time: 1577896800000,
    open: 3189.43,
    high: 3260.43,
    low: 3170.15,
    close: 3197.57,
    volume: 56801.329,
  })
  await rst.next({
    time: 1577896800000,
    open: 2189.43,
    high: 2260.43,
    low: 2170.15,
    close: 2197.57,
    volume: 56801.329,
  })
  await rst.next({
    time: 1577896800000,
    open: 1189.43,
    high: 1260.43,
    low: 1170.15,
    close: 1197.57,
    volume: 56801.329,
  })
  await rst.next({
    time: 1577896800000,
    open: 2189.43,
    high: 2260.43,
    low: 2170.15,
    close: 2197.57,
    volume: 56801.329,
  })
  await rst.next({
    time: 1577986800000,
    open: 3189.43,
    high: 3260.43,
    low: 3170.15,
    close: 3197.57,
    volume: 56801.329,
  })
  await rst.next({
    time: 1579886800000,
    open: 4189.43,
    high: 4260.43,
    low: 4170.15,
    close: 4197.57,
    volume: 56801.329,
  })
  const result = await rst.next({
    time: 1597896800000,
    open: 2189.43,
    high: 2260.43,
    low: 2170.15,
    close: 2197.57,
    volume: 56801.329,
  })

  t.is(result.buy, 0)
  t.is(result.sell, 0)
  t.is(result.buyEntry, 0)
  t.is(result.sellEntry, 0)
})

//TODO: add more tests
