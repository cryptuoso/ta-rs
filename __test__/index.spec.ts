import test from 'ava'

import { SMA } from '../index'

test('SMA', async (t) => {
  const sma = new SMA(10, 0)
  const result = await sma.next(1)
  t.is(result, 0.1)
})
//TODO: add more tests
