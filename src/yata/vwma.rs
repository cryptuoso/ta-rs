use napi::bindgen_prelude::*;
use yata::prelude::*;

struct AsyncVWMA<'a>(&'a mut yata::methods::VWMA, (f64, f64));

impl napi::Task for AsyncVWMA<'_> {
  type Output = f64;
  type JsValue = f64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi(js_name = "YataVWMA")]
#[allow(dead_code)]
struct YataVWMA {
  indicator: yata::methods::VWMA,
}

#[napi]
#[allow(dead_code)]
impl YataVWMA {
  #[napi(constructor)]
  pub fn new(period: u16, initial: Vec<f64>) -> Self {
    let initial = match initial.len() {
      2 => (initial[0], initial[1]),
      _ => panic!("Invalid initial length"),
    };

    Self {
      indicator: yata::methods::VWMA::new(period, &initial).expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, values: Vec<f64>) -> AsyncTask<AsyncVWMA> {
    let values = match values.len() {
      2 => (values[0], values[1]),
      _ => panic!("Invalid values length"),
    };
    AsyncTask::new(AsyncVWMA(&mut self.indicator, values))
  }

  #[napi]
  pub fn get_last_value(&self) -> f64 {
    self.indicator.peek()
  }
}
