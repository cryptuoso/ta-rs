use napi::bindgen_prelude::*;
use ta::Next;

#[napi(object)]
pub struct MovingAverageConvergenceDivergenceOutput {
  pub macd: f64,
  pub signal: f64,
  pub histogram: f64,
}

struct AsyncMACD<'a>(
  &'a mut ta::indicators::MovingAverageConvergenceDivergence,
  f64,
);

impl napi::Task for AsyncMACD<'_> {
  type Output = MovingAverageConvergenceDivergenceOutput;
  type JsValue = MovingAverageConvergenceDivergenceOutput;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let result = self.0.next(self.1);

    Ok(MovingAverageConvergenceDivergenceOutput {
      macd: result.macd,
      signal: result.signal,
      histogram: result.histogram,
    })
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "MACD")]
#[allow(dead_code)]
struct MACD {
  indicator: ta::indicators::MovingAverageConvergenceDivergence,
}

#[napi]
#[allow(dead_code)]
impl MACD {
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

  #[napi(ts_return_type = "Promise<MACDOutput>")]
  pub fn next(&mut self, value: f64) -> AsyncTask<AsyncMACD> {
    AsyncTask::new(AsyncMACD(&mut self.indicator, value))
  }
}
