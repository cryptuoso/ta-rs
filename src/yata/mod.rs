use crate::Candle;
use napi::bindgen_prelude::*;
use yata::prelude::*;

impl OHLCV for Candle {
  #[inline]
  fn open(&self) -> f64 {
    self.open
  }

  #[inline]
  fn high(&self) -> f64 {
    self.high
  }

  #[inline]
  fn low(&self) -> f64 {
    self.low
  }

  #[inline]
  fn close(&self) -> f64 {
    self.close
  }

  #[inline]
  fn volume(&self) -> f64 {
    self.volume
  }
}

macro_rules! yata_method_async {
  ($src_name:ident, $indicator_type: ty, $output_type: ty) => {
    struct $src_name<'a>(&'a mut $indicator_type, $output_type);

    impl napi::Task for $src_name<'_> {
      type Output = $output_type;
      type JsValue = $output_type;

      fn compute(&mut self) -> napi::Result<Self::Output> {
        Ok(self.0.next(&self.1))
      }

      fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(output)
      }
    }
  };
}

macro_rules! yata_method {
  ($src_name:ident,
    $src_js_name:literal,
    $src_task_name:ident, $indicator_type: ty, $param_type: ty, $output_type: ty) => {
    yata_method_async!($src_task_name, $indicator_type, $output_type);
    #[napi(js_name = $src_js_name)]
    #[allow(dead_code)]
    struct $src_name {
      indicator: $indicator_type,
    }

    #[napi]
    #[allow(dead_code)]
    impl $src_name {
      #[napi(constructor)]
      pub fn new(period: $param_type, initial: $output_type) -> Self {
        Self {
          indicator: <$indicator_type>::new(period, &initial).expect("Failed to create method"),
        }
      }

      #[napi(ts_return_type = "Promise<number>")]
      pub fn next(&mut self, value: $output_type) -> AsyncTask<$src_task_name> {
        AsyncTask::new($src_task_name(&mut self.indicator, value))
      }
    }
  };
}

//SMA
yata_method!(SMA, "SMA", AsyncSMA, yata::methods::SMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl SMA {
  #[napi]
  pub fn get(&self, index: u16) -> Option<f64> {
    self.indicator.get_window().get(index).copied()
  }

  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//WMA
yata_method!(WMA, "WMA", AsyncWMA, yata::methods::WMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl WMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//EMA
yata_method!(EMA, "EMA", AsyncEMA, yata::methods::EMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl EMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//WSMA
yata_method!(WSMA, "WSMA", AsyncWSMA, yata::methods::WSMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl WSMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//RMA
yata_method!(RMA, "RMA", AsyncRMA, yata::methods::RMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl RMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//SMM
yata_method!(SMM, "SMM", AsyncSMM, yata::methods::SMM, u16, f64);
#[napi]
#[allow(dead_code)]
impl SMM {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//HMA
yata_method!(HMA, "HMA", AsyncHMA, yata::methods::HMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl HMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//SWMA
yata_method!(SWMA, "SWMA", AsyncSWMA, yata::methods::SWMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl SWMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//LSMA
yata_method!(LSMA, "LSMA", AsyncLSMA, yata::methods::LSMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl LSMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }

  #[napi]
  pub fn tan(&self) -> f64 {
    self.indicator.tan()
  }

  #[napi]
  pub fn b(&self) -> f64 {
    self.indicator.b()
  }
}

//Conv
pub mod conv;

//VWMA
pub mod vwma;

//TRIMA
yata_method!(TRIMA, "TRIMA", AsyncTRIMA, yata::methods::TRIMA, u16, f64);
#[napi]
#[allow(dead_code)]
impl TRIMA {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//Differential
yata_method!(
  Diff,
  "Diff",
  AsyncDiff,
  yata::methods::Differential,
  u16,
  f64
);

//Sum
yata_method!(Sum, "Sum", AsyncSum, yata::methods::Sum, u16, f64);
#[napi]
#[allow(dead_code)]
impl Sum {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//Momentum
yata_method!(
  Momentum,
  "Momentum",
  AsyncMomentum,
  yata::methods::Momentum,
  u16,
  f64
);

//ROC
yata_method!(ROC, "ROC", AsyncROC, yata::methods::ROC, u16, f64);

//TSI
pub mod tsi;

//StDev
yata_method!(StDev, "StDev", AsyncStDev, yata::methods::StDev, u16, f64);

//LinearVolatility
yata_method!(
  LinearVolatility,
  "LinearVolatility",
  AsyncLinearVolatility,
  yata::methods::LinearVolatility,
  u16,
  f64
);

//CCI
yata_method!(CCI, "CCI", AsyncCCI, yata::methods::CCI, u16, f64);

//MeanAbsDev
yata_method!(
  MeanAbsDev,
  "MeanAbsDev",
  AsyncMeanAbsDev,
  yata::methods::MeanAbsDev,
  u16,
  f64
);

//MedianAbsDev
yata_method!(
  MedianAbsDev,
  "MedianAbsDev",
  AsyncMedianAbsDev,
  yata::methods::MedianAbsDev,
  u16,
  f64
);

//Vidya
yata_method!(Vidya, "Vidya", AsyncVidya, yata::methods::Vidya, u16, f64);
#[napi]
#[allow(dead_code)]
impl Vidya {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//Cross
pub mod cross;

//ReversalSignal
pub mod reversal;

//HighestLowestDelta
yata_method!(
  HighestLowestDelta,
  "HighestLowestDelta",
  AsyncHighestLowestDelta,
  yata::methods::HighestLowestDelta,
  u16,
  f64
);
#[napi]
#[allow(dead_code)]
impl HighestLowestDelta {
  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}

//TODO: other methods and indicators
