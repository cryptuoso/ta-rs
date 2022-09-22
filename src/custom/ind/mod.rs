pub mod max_adx;
pub use self::max_adx::MaximumAverageDirectionalIndex;

pub mod channel_adx;
pub use self::channel_adx::ChannelAverageDirectionalIndex;

pub mod fx_high_band;
pub use self::fx_high_band::FXHighBand;

pub mod fx_low_band;
pub use self::fx_low_band::FXLowBand;

pub fn add_percent(value: f64, percent: f64) -> f64 {
  value + (value / 100.0) * percent
}

pub mod average_directional_index;
pub use self::average_directional_index::AverageDirectionalIndex;

pub mod average_true_range;
pub use self::average_true_range::AverageTrueRange;

pub mod relative_strength_index;
pub use self::relative_strength_index::RelativeStrengthIndex;


pub mod rajandran_super_trend;
pub use self::rajandran_super_trend::RajandranSuperTrend;