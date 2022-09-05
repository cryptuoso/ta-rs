use crate::Candle;

use super::ind::{channel_adx::ChanADXOutput, ChannelAverageDirectionalIndex};
use napi::bindgen_prelude::*;
use ta::Next;

struct AsyncChanADX<'a>(&'a mut ChannelAverageDirectionalIndex, Candle);

impl napi::Task for AsyncChanADX<'_> {
  type Output = ChanADXOutput;
  type JsValue = ChanADXOutput;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0.next(&self.1))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi(js_name = "ChanADX")]
#[allow(dead_code)]
struct ChanADX {
  indicator: ChannelAverageDirectionalIndex,
}

#[napi]
#[allow(dead_code)]
impl ChanADX {
  #[napi(constructor)]
  pub fn new(period: u16, adx_period: u16, ratio: u16) -> Self {
    Self {
      indicator: ChannelAverageDirectionalIndex::new(period.into(), adx_period.into(), ratio)
        .expect("Failed to create indicator"),
    }
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn next(&mut self, value: Candle) -> AsyncTask<AsyncChanADX> {
    AsyncTask::new(AsyncChanADX(&mut self.indicator, value))
  }
}
