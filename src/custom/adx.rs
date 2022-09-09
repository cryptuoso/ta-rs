use crate::Candle;

use super::ind::AverageDirectionalIndex;
use napi::bindgen_prelude::*;
use ta::Next;

struct AsyncADX<'a>(&'a mut AverageDirectionalIndex, Candle);

impl napi::Task for AsyncADX<'_> {
  type Output = f64;
  type JsValue = f64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi(js_name = "ADX")]
#[allow(dead_code)]
struct ADX {
  indicator: AverageDirectionalIndex,
}

#[napi]
#[allow(dead_code)]
impl ADX {
  #[napi(constructor)]
  pub fn new(period: u16) -> Self {
    Self {
      indicator: AverageDirectionalIndex::new(period.into()).expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: Candle) -> AsyncTask<AsyncADX> {
    AsyncTask::new(AsyncADX(&mut self.indicator, value))
  }
}
