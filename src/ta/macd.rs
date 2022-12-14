use napi::bindgen_prelude::*;
use ta::Next;

#[napi(object, js_name = "TaMACDOutput")]
pub struct TaMACDOutput {
  pub macd: f64,
  pub signal: f64,
  pub histogram: f64,
}

struct AsyncMACD<'a>(
  &'a mut ta::indicators::MovingAverageConvergenceDivergence,
  f64,
);

impl napi::Task for AsyncMACD<'_> {
  type Output = TaMACDOutput;
  type JsValue = TaMACDOutput;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let result = self.0.next(self.1);

    Ok(TaMACDOutput {
      macd: result.macd,
      signal: result.signal,
      histogram: result.histogram,
    })
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "TaMACD")]
#[allow(dead_code)]
struct TaMACD {
  indicator: ta::indicators::MovingAverageConvergenceDivergence,
}

#[napi]
#[allow(dead_code)]
impl TaMACD {
  #[napi(constructor)]
  pub fn new(fast_period: u16, slow_period: u16, signal_period: u16) -> Self {
    Self {
      indicator: ta::indicators::MovingAverageConvergenceDivergence::new(
        fast_period.into(),
        slow_period.into(),
        signal_period.into(),
      )
      .expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<TaMACDOutput>")]
  pub fn next(&mut self, value: f64) -> AsyncTask<AsyncMACD> {
    AsyncTask::new(AsyncMACD(&mut self.indicator, value))
  }
}
