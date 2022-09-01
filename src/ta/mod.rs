use crate::Candle;
use ::ta::{Close, High, Low, Open, Volume};
use napi::bindgen_prelude::*;
use ta::Next;

impl Open for Candle {
  fn open(&self) -> f64 {
    self.open
  }
}

impl High for Candle {
  fn high(&self) -> f64 {
    self.high
  }
}

impl Low for Candle {
  fn low(&self) -> f64 {
    self.low
  }
}

impl Close for Candle {
  fn close(&self) -> f64 {
    self.close
  }
}

impl Volume for Candle {
  fn volume(&self) -> f64 {
    self.volume
  }
}

macro_rules! ta_ind_async {
  ($src_name:ident, $indicator_type: ty, $input_type: ty, $output_type: ty) => {
    struct $src_name<'a>(&'a mut $indicator_type, $input_type);

    impl napi::Task for $src_name<'_> {
      type Output = $output_type;
      type JsValue = $output_type;

      fn compute(&mut self) -> napi::Result<Self::Output> {
        Ok(self.0.next(self.1))
      }

      fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(output)
      }
    }
  };
}

pub(crate) use ta_ind_async;

macro_rules! ta_ind {
  ($src_name:ident,
    $src_js_name:literal,
    $src_task_name:ident, $indicator_type: ty, $param_type: ty, $input_type: ty, $output_type: ty) => {
    ta_ind_async!($src_task_name, $indicator_type, $input_type, $output_type);
    #[napi(js_name = $src_js_name)]
    #[allow(dead_code)]
    struct $src_name {
      indicator: $indicator_type,
    }

    #[napi]
    #[allow(dead_code)]
    impl $src_name {
      #[napi(constructor)]
      pub fn new(period: $param_type) -> Self {
        Self {
          indicator: <$indicator_type>::new(period.into()).expect("Failed to create method"),
        }
      }

      #[napi(ts_return_type = "Promise<number>")]
      pub fn next(&mut self, value: $input_type) -> AsyncTask<$src_task_name> {
        AsyncTask::new($src_task_name(&mut self.indicator, value))
      }
    }
  };
}

pub(crate) use ta_ind;
//SMA
ta_ind!(
  Sma,
  "Sma",
  SmaTask,
  ta::indicators::SimpleMovingAverage,
  u16,
  f64,
  f64
);

//EMA
ta_ind!(
  EMA,
  "EMA",
  AsyncEMA,
  ta::indicators::ExponentialMovingAverage,
  u16,
  f64,
  f64
);

//WMA
ta_ind!(
  WMA,
  "WMA",
  AsyncWMA,
  ta::indicators::WeightedMovingAverage,
  u16,
  f64,
  f64
);

//ATR
ta_ind!(
  ATR,
  "ATR",
  AsyncATR,
  ta::indicators::AverageTrueRange,
  u16,
  f64,
  f64
);

//ADX
ta_ind!(
  ADX,
  "ADX",
  AsyncADX,
  ta::indicators::AverageDirectionalIndex,
  u16,
  f64,
  f64
);

//RSI
ta_ind!(
  RSI,
  "RSI",
  AsyncRSI,
  ta::indicators::RelativeStrengthIndex,
  u16,
  f64,
  f64
);

//MACD
pub mod macd;

//Maximum
ta_ind!(
  Maximum,
  "Maximum",
  AsyncMaximum,
  ta::indicators::Maximum,
  u16,
  f64,
  f64
);

//Minimum
ta_ind!(
  Minimum,
  "Minimum",
  AsyncMinimum,
  ta::indicators::Minimum,
  u16,
  f64,
  f64
);

//StandardDeviation
ta_ind!(
  StandardDeviation,
  "StandardDeviation",
  AsyncStandardDeviation,
  ta::indicators::StandardDeviation,
  u16,
  f64,
  f64
);

//MeanAbsoluteDeviation
ta_ind!(
  MeanAbsoluteDeviation,
  "MeanAbsoluteDeviation",
  AsyncMeanAbsoluteDeviation,
  ta::indicators::MeanAbsoluteDeviation,
  u16,
  f64,
  f64
);
