use napi::bindgen_prelude::*;
use yata::prelude::*;

struct AsyncReversalSignal<'a>(&'a mut yata::methods::ReversalSignal, f64);

impl napi::Task for AsyncReversalSignal<'_> {
  type Output = i8;
  type JsValue = i8;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let result = self.0.next(&self.1);
    Ok(result.into())
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi(js_name = "YataReversalSignal")]
#[allow(dead_code)]
struct YataReversalSignal {
  indicator: yata::methods::ReversalSignal,
}

#[napi]
#[allow(dead_code)]
impl YataReversalSignal {
  #[napi(constructor)]
  pub fn new(left: u16, right: u16, initial: f64) -> Self {
    Self {
      indicator: yata::methods::ReversalSignal::new(left, right, &initial)
        .expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: f64) -> AsyncTask<AsyncReversalSignal> {
    AsyncTask::new(AsyncReversalSignal(&mut self.indicator, value))
  }
}
