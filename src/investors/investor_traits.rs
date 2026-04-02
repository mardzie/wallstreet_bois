use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum Risk {
    Conservative,
    Moderate,
    Aggressive,
}

#[derive(Component, Debug)]
pub enum TimeHorizon {
    /// Hold for only a day at maximum. Very rarely longer.
    DayTrader,
    /// Hold for days or weeks based on short term price movements. Very rarely longer than a month.
    SwingTrader,
    /// Buy and Hold forever. Sell after 40 years. Rarely within a decade.
    LongTerm,
}
