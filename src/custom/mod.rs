use crate::ta::{ta_ind, ta_ind_async};
use napi::bindgen_prelude::*;
use ta::Next;

pub mod ind;

pub mod adx;
pub mod atr;
pub mod channel_adx;
pub mod fx_high_band;
pub mod fx_low_band;
pub mod max_adx;
pub mod rach_sup_trend;

ta_ind!(
  RSI,
  "RSI",
  AsyncRSI,
  ind::RelativeStrengthIndex,
  u16,
  f64,
  f64
);
