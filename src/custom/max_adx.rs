use super::ind::MaximumAverageDirectionalIndex;
use napi::bindgen_prelude::*;
use ta::Next;

crate::ta::ta_ind_async!(AsyncMaxADX, MaximumAverageDirectionalIndex, f64, f64);

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
  pub fn next(&mut self, value: f64) -> AsyncTask<AsyncMaxADX> {
    AsyncTask::new(AsyncMaxADX(&mut self.indicator, value))
  }
}
