use napi::bindgen_prelude::*;
use yata::prelude::*;

yata_method_async!(AsyncConv, yata::methods::Conv, f64);
#[napi]
#[allow(dead_code)]
struct Conv {
  indicator: yata::methods::Conv,
}

#[napi]
#[allow(dead_code)]
impl Conv {
  #[napi(constructor)]
  pub fn new(weights: Vec<f64>, initial: f64) -> Self {
    Self {
      indicator: yata::methods::Conv::new(weights, &initial).expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: f64) -> AsyncTask<AsyncConv> {
    AsyncTask::new(AsyncConv(&mut self.indicator, value))
  }

  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}
