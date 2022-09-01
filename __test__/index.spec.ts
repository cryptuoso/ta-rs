import test from 'ava'

import { SMA } from '../index'

test('SMA', async (t) => {
  const sma = new SMA(10)
  await sma.next(10)
  const result = await sma.next(20)
  t.is(result, 15)
})
//TODO: add more tests
