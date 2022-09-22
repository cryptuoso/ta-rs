const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.android-arm64.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.android-arm-eabi.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.win32-x64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.win32-ia32-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.win32-arm64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.darwin-x64.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.darwin-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.darwin-arm64.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'ta-rs.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./ta-rs.freebsd-x64.node')
      } else {
        nativeBinding = require('@cryptuoso/ta-rs-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(join(__dirname, 'ta-rs.linux-x64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./ta-rs.linux-x64-musl.node')
            } else {
              nativeBinding = require('@cryptuoso/ta-rs-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'ta-rs.linux-x64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./ta-rs.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@cryptuoso/ta-rs-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(join(__dirname, 'ta-rs.linux-arm64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./ta-rs.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@cryptuoso/ta-rs-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'ta-rs.linux-arm64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./ta-rs.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@cryptuoso/ta-rs-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.linux-arm-gnueabihf.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('@cryptuoso/ta-rs-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const {
  ADX,
  ATR,
  ChanADX,
  FXHighBand,
  FXLowBand,
  MaxADX,
  RachSupTrend,
  RSI,
  TaSMA,
  TaEMA,
  TaATR,
  TaRSI,
  TaMACD,
  TaMaximum,
  TaMinimum,
  TaStandardDeviation,
  TaMeanAbsoluteDeviation,
  YataSMA,
  YataWMA,
  YataEMA,
  YataWSMA,
  YataRMA,
  YataSMM,
  YataHMA,
  YataSWMA,
  YataLSMA,
  YataConv,
  YataVWMA,
  YataTRIMA,
  YataDiff,
  YataSum,
  YataMomentum,
  YataROC,
  YataTSI,
  YataStDev,
  YataLinearVolatility,
  YataCCI,
  YataMeanAbsDev,
  YataMedianAbsDev,
  YataVidya,
  YataCross,
  CrossAbove,
  CrossUnder,
  YataReversalSignal,
  YataHighestLowestDelta,
} = nativeBinding

module.exports.ADX = ADX
module.exports.ATR = ATR
module.exports.ChanADX = ChanADX
module.exports.FXHighBand = FXHighBand
module.exports.FXLowBand = FXLowBand
module.exports.MaxADX = MaxADX
module.exports.RachSupTrend = RachSupTrend
module.exports.RSI = RSI
module.exports.TaSMA = TaSMA
module.exports.TaEMA = TaEMA
module.exports.TaATR = TaATR
module.exports.TaRSI = TaRSI
module.exports.TaMACD = TaMACD
module.exports.TaMaximum = TaMaximum
module.exports.TaMinimum = TaMinimum
module.exports.TaStandardDeviation = TaStandardDeviation
module.exports.TaMeanAbsoluteDeviation = TaMeanAbsoluteDeviation
module.exports.YataSMA = YataSMA
module.exports.YataWMA = YataWMA
module.exports.YataEMA = YataEMA
module.exports.YataWSMA = YataWSMA
module.exports.YataRMA = YataRMA
module.exports.YataSMM = YataSMM
module.exports.YataHMA = YataHMA
module.exports.YataSWMA = YataSWMA
module.exports.YataLSMA = YataLSMA
module.exports.YataConv = YataConv
module.exports.YataVWMA = YataVWMA
module.exports.YataTRIMA = YataTRIMA
module.exports.YataDiff = YataDiff
module.exports.YataSum = YataSum
module.exports.YataMomentum = YataMomentum
module.exports.YataROC = YataROC
module.exports.YataTSI = YataTSI
module.exports.YataStDev = YataStDev
module.exports.YataLinearVolatility = YataLinearVolatility
module.exports.YataCCI = YataCCI
module.exports.YataMeanAbsDev = YataMeanAbsDev
module.exports.YataMedianAbsDev = YataMedianAbsDev
module.exports.YataVidya = YataVidya
module.exports.YataCross = YataCross
module.exports.CrossAbove = CrossAbove
module.exports.CrossUnder = CrossUnder
module.exports.YataReversalSignal = YataReversalSignal
module.exports.YataHighestLowestDelta = YataHighestLowestDelta
