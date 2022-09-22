use crate::Candle;

use super::ind::{rajandran_super_trend::RachSupTrendOutput, RajandranSuperTrend};
use napi::bindgen_prelude::*;
use ta::Next;

struct AsyncRachSupTrend<'a>(&'a mut RajandranSuperTrend, Candle);

impl napi::Task for AsyncRachSupTrend<'_> {
  type Output = RachSupTrendOutput;
  type JsValue = RachSupTrendOutput;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi(js_name = "RachSupTrend")]
#[allow(dead_code)]
struct RachSupTrend {
  indicator: RajandranSuperTrend,
}

#[napi]
#[allow(dead_code)]
impl RachSupTrend {
  #[napi(constructor)]
  pub fn new(period: u16, factor: u16) -> Self {
    Self {
      indicator: RajandranSuperTrend::new(period.into(), factor.into())
        .expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<RachSupTrendOutput>")]
  pub fn next(&mut self, value: Candle) -> AsyncTask<AsyncRachSupTrend> {
    AsyncTask::new(AsyncRachSupTrend(&mut self.indicator, value))
  }
}
