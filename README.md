<a href="https://cryptuoso.com">
 <img align="left" width="150" height="150" src="https://cryptuoso.com/favicon_color.svg">  
</a> 
<br>

<h2 align="center"><a href="https://cryptuoso.com">Cryptuoso - Cryptocurrency Trading Automation</a></h2>

<br>
<br>
<br>

# ⚠️ Under development! ⚠️

## Node.js bindings for varios Rust based Technical Analysis methods and indicators

## Install

```bash
npm install ta-rs
```

## Use

```js
import { SMA } from 'ta-rs'

const sma = new SMA(10, 0) // period = 0, initial value = 0

const result = await sma.next(12) // current value = 12

console.log(result) // current result
```

## Supports:

- [TA](https://github.com/virtualritz/ta-rs) - Technical Analysis for Rust
  - ADX
  - ATR
  - EMA
  - MACD
  - SMA
  - RSI
  - WMA
  - Minimum
  - Maximum
  - StandardDeviation
  - MeanAbsoluteDeviation
- Other
  - MaxADX (Maximum + ADX from TA)
- [YATA](https://github.com/amv-dev/yata) - Yet Another Technical Analysis library
  - WSMA
  - RMA
  - SMM
  - HMA
  - SWMA
  - LSMA
  - Conv
  - VWMA
  - TRIMA
  - Differential
  - Sum
  - ROC
  - TSI
  - LinearVolatility
  - CCI
  - MedianAbsDev
  - Vidya
  - Cross
  - ReversalSignal
  - HighestLowestDelta

---

### Operating systems support matrix

|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | x      | x      | x      |
| Android armv7    | x      | x      | x      |
| FreeBSD x64      | x      | x      | x      |
