use napi::bindgen_prelude::*;
use ta::Next;

use crate::Candle;

struct AsyncATR<'a>(&'a mut ta::indicators::AverageTrueRange, Candle);

impl napi::Task for AsyncATR<'_> {
  type Output = f64;
  type JsValue = f64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "ATR")]
#[allow(dead_code)]
struct ATR {
  indicator: ta::indicators::AverageTrueRange,
}

#[napi]
#[allow(dead_code)]
impl ATR {
  #[napi(constructor)]
  pub fn new(period: u16) -> Self {
    Self {
      indicator: ta::indicators::AverageTrueRange::new(period.into())
        .expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: Candle) -> AsyncTask<AsyncATR> {
    AsyncTask::new(AsyncATR(&mut self.indicator, value))
  }
}
