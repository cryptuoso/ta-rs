use napi::bindgen_prelude::*;
use yata::prelude::*;

yata_method_async!(AsyncTSI, yata::methods::TSI, f64);
#[napi(js_name = "YataTSI")]
#[allow(dead_code)]
struct YataTSI {
  indicator: yata::methods::TSI,
}

#[napi]
#[allow(dead_code)]
impl YataTSI {
  #[napi(constructor)]
  pub fn new(short_period: u16, long_period: u16, initial: f64) -> Self {
    Self {
      indicator: yata::methods::TSI::new(short_period, long_period, &initial)
        .expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: f64) -> AsyncTask<AsyncTSI> {
    AsyncTask::new(AsyncTSI(&mut self.indicator, value))
  }

  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}
