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
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'ta-rs.win32-x64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./ta-rs.win32-x64-msvc.node')
          } else {
            nativeBinding = require('ta-rs-win32-x64-msvc')
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
            nativeBinding = require('ta-rs-win32-ia32-msvc')
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
            nativeBinding = require('ta-rs-win32-arm64-msvc')
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
            nativeBinding = require('ta-rs-darwin-x64')
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
            nativeBinding = require('ta-rs-darwin-arm64')
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
        nativeBinding = require('ta-rs-freebsd-x64')
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
              nativeBinding = require('ta-rs-linux-x64-musl')
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
              nativeBinding = require('ta-rs-linux-x64-gnu')
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
              nativeBinding = require('ta-rs-linux-arm64-musl')
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
              nativeBinding = require('ta-rs-linux-arm64-gnu')
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
            nativeBinding = require('ta-rs-linux-arm-gnueabihf')
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
  SMA,
  WMA,
  EMA,
  WSMA,
  RMA,
  SMM,
  HMA,
  SWMA,
  LSMA,
  Conv,
  VWMA,
  TRIMA,
  Diff,
  Sum,
  Momentum,
  ROC,
  TSI,
  StDev,
  LinearVolatility,
  CCI,
  MeanAbsDev,
  MedianAbsDev,
  Vidya,
  Cross,
  CrossAbove,
  CrossUnder,
  ReversalSignal,
  HighestLowestDelta,
} = nativeBinding

module.exports.SMA = SMA
module.exports.WMA = WMA
module.exports.EMA = EMA
module.exports.WSMA = WSMA
module.exports.RMA = RMA
module.exports.SMM = SMM
module.exports.HMA = HMA
module.exports.SWMA = SWMA
module.exports.LSMA = LSMA
module.exports.Conv = Conv
module.exports.VWMA = VWMA
module.exports.TRIMA = TRIMA
module.exports.Diff = Diff
module.exports.Sum = Sum
module.exports.Momentum = Momentum
module.exports.ROC = ROC
module.exports.TSI = TSI
module.exports.StDev = StDev
module.exports.LinearVolatility = LinearVolatility
module.exports.CCI = CCI
module.exports.MeanAbsDev = MeanAbsDev
module.exports.MedianAbsDev = MedianAbsDev
module.exports.Vidya = Vidya
module.exports.Cross = Cross
module.exports.CrossAbove = CrossAbove
module.exports.CrossUnder = CrossUnder
module.exports.ReversalSignal = ReversalSignal
module.exports.HighestLowestDelta = HighestLowestDelta
