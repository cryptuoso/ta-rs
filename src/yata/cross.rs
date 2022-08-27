use napi::bindgen_prelude::*;
use yata::prelude::*;

struct AsyncCross<'a>(&'a mut yata::methods::Cross, (f64, f64));

impl napi::Task for AsyncCross<'_> {
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
#[napi]
#[allow(dead_code)]
struct Cross {
  indicator: yata::methods::Cross,
}

#[napi]
#[allow(dead_code)]
impl Cross {
  #[napi(constructor)]
  pub fn new(initial: Vec<f64>) -> Self {
    let initial = match initial.len() {
      2 => (initial[0], initial[1]),
      _ => panic!("Invalid initial length"),
    };
    Self {
      indicator: yata::methods::Cross::new((), &initial).expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, values: Vec<f64>) -> AsyncTask<AsyncCross> {
    let values = match values.len() {
      2 => (values[0], values[1]),
      _ => panic!("Invalid values length"),
    };
    AsyncTask::new(AsyncCross(&mut self.indicator, values))
  }
}

struct AsyncCrossAbove<'a>(&'a mut yata::methods::CrossAbove, (f64, f64));

impl napi::Task for AsyncCrossAbove<'_> {
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
#[napi]
#[allow(dead_code)]
struct CrossAbove {
  indicator: yata::methods::CrossAbove,
}

#[napi]
#[allow(dead_code)]
impl CrossAbove {
  #[napi(constructor)]
  pub fn new(initial: Vec<f64>) -> Self {
    let initial = match initial.len() {
      2 => (initial[0], initial[1]),
      _ => panic!("Invalid initial length"),
    };
    Self {
      indicator: yata::methods::CrossAbove::new((), &initial).expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, values: Vec<f64>) -> AsyncTask<AsyncCrossAbove> {
    let values = match values.len() {
      2 => (values[0], values[1]),
      _ => panic!("Invalid values length"),
    };
    AsyncTask::new(AsyncCrossAbove(&mut self.indicator, values))
  }
}

struct AsyncCrossUnder<'a>(&'a mut yata::methods::CrossUnder, (f64, f64));

impl napi::Task for AsyncCrossUnder<'_> {
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
#[napi]
#[allow(dead_code)]
struct CrossUnder {
  indicator: yata::methods::CrossUnder,
}

#[napi]
#[allow(dead_code)]
impl CrossUnder {
  #[napi(constructor)]
  pub fn new(initial: Vec<f64>) -> Self {
    let initial = match initial.len() {
      2 => (initial[0], initial[1]),
      _ => panic!("Invalid initial length"),
    };
    Self {
      indicator: yata::methods::CrossUnder::new((), &initial).expect("Failed to create method"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, values: Vec<f64>) -> AsyncTask<AsyncCrossUnder> {
    let values = match values.len() {
      2 => (values[0], values[1]),
      _ => panic!("Invalid values length"),
    };
    AsyncTask::new(AsyncCrossUnder(&mut self.indicator, values))
  }
}
