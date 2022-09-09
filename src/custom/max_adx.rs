use crate::Candle;

use super::ind::MaximumAverageDirectionalIndex;
use napi::bindgen_prelude::*;
use ta::Next;

struct AsyncMaxADX<'a>(&'a mut MaximumAverageDirectionalIndex, Candle);

impl napi::Task for AsyncMaxADX<'_> {
  type Output = f64;
  type JsValue = f64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi(js_name = "MaxADX")]
#[allow(dead_code)]
struct MaxADX {
  indicator: MaximumAverageDirectionalIndex,
}

#[napi]
#[allow(dead_code)]
impl MaxADX {
  #[napi(constructor)]
  pub fn new(period: u16, adx_period: u16) -> Self {
    Self {
      indicator: MaximumAverageDirectionalIndex::new(period.into(), adx_period.into())
        .expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: Candle) -> AsyncTask<AsyncMaxADX> {
    AsyncTask::new(AsyncMaxADX(&mut self.indicator, value))
  }
}
