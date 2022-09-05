use crate::Candle;

use napi::bindgen_prelude::*;
use ta::Next;

struct AsyncFXHighBand<'a>(&'a mut super::ind::fx_high_band::FXHighBand, Candle);

impl napi::Task for AsyncFXHighBand<'_> {
  type Output = f64;
  type JsValue = f64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "FXHighBand")]
#[allow(dead_code)]
struct FXHighBand {
  indicator: super::ind::fx_high_band::FXHighBand,
}

#[napi]
#[allow(dead_code)]
impl FXHighBand {
  #[napi(constructor)]
  pub fn new(period: u16, rsi_period: u16, modifier: u16) -> Self {
    Self {
      indicator: super::ind::fx_high_band::FXHighBand::new(
        period.into(),
        rsi_period.into(),
        modifier,
      )
      .expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: Candle) -> AsyncTask<AsyncFXHighBand> {
    AsyncTask::new(AsyncFXHighBand(&mut self.indicator, value))
  }
}
