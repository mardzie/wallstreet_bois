use std::collections::VecDeque;

use bevy::prelude::*;

use crate::market::chart::Candle;

/// The history to preserve for each stock.
pub const HISTORY_LENGHT: usize = 1024 * 16;

pub const DEFAULT_CANDLE_HISTORY_SIZE: usize = 24;

#[derive(Component, Debug)]
pub struct Stock {
    ticker: String,
}

#[derive(Component, Debug)]
pub struct ShareValue {
    /// The current Value of the Stock in ct.
    current: u64,
    open: u64,
    max: u64,
    min: u64,
}

#[derive(Component, Debug, Default)]
pub struct SharePerformance {
    change_abs: i64,
    change_percent: i64,
    history: VecDeque<Candle>,
    volatility: i64,
}

impl Stock {
    pub fn new(ticker: &str) -> Self {
        Self {
            ticker: ticker.to_uppercase(),
        }
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }
}

impl ShareValue {
    /// New Value from cent values.
    pub fn new(current: u64, open: u64, max: u64, min: u64) -> Self {
        assert!(max >= min, "Max has to be greater or equals to min!");
        assert!(
            max >= current && current >= min,
            "The current price has to be between the max and min price!"
        );
        assert!(
            max >= open && open >= min,
            "The open price needs to be between the max and min price!"
        );

        Self {
            current,
            open,
            max,
            min,
        }
    }

    /// New Value from cent.
    pub fn from_current(current: u64) -> Self {
        Self {
            current,
            open: current,
            max: current,
            min: current,
        }
    }

    pub fn current(&self) -> u64 {
        self.current
    }

    pub fn update_current_value(&mut self, new_current: u64) {
        self.current = new_current;
        self.min = self.min.min(new_current);
        self.max = self.max.max(new_current);
    }

    pub fn calculate_change_abs(&self) -> i64 {
        self.current as i64 - self.open as i64
    }

    /// Calculate the percentage change.
    ///
    /// The change gets returned as an `i32` with two decimal places.
    ///
    /// `100 % = 10000`
    /// `150.25 % = 15025`
    /// `150.2581 % = 15025`
    pub fn calculate_change_percentage(&self) -> i64 {
        /// Percent in i64 with two decimal places.
        ///
        /// `100 % = 10000`
        /// `150.25 % = 15025`
        const MULTIPLICATOR: i64 = 10_000;
        (self.current as i64 - self.open as i64) * MULTIPLICATOR / self.open as i64
    }

    pub fn open(&self) -> u64 {
        self.open
    }

    pub fn max(&self) -> u64 {
        self.max
    }

    pub fn min(&self) -> u64 {
        self.min
    }

    pub fn close_candle(&mut self, new_current: u64) -> Candle {
        let candle = Candle::new(self.open, self.current, self.max, self.min);

        self.open = new_current;
        self.min = new_current;
        self.max = new_current;
        self.current = new_current;

        // Just check if I didn't fuck up something in the new init.
        debug_assert!(self.max >= self.min, "Max must be bigger or equals to min!");
        debug_assert!(
            self.max >= self.open && self.open >= self.min,
            "Open price must be between max and min price!"
        );
        debug_assert!(
            self.max >= self.current && self.current >= self.min,
            "Current price must be between max and min price!"
        );

        candle
    }
}

impl SharePerformance {
    pub fn new(change_abs: i64, change_percent: i64) -> Self {
        Self {
            change_abs,
            change_percent,
            history: VecDeque::with_capacity(HISTORY_LENGHT),
            volatility: change_abs,
        }
    }

    /// Update the change.
    ///
    /// Takes the format that [`Value`] puts out.
    pub fn update_change(&mut self, abs: i64, percent: i64) {
        self.change_abs = abs;
        self.change_percent = percent;
        self.volatility = self.calculate_volatility(DEFAULT_CANDLE_HISTORY_SIZE);
    }

    pub fn history(&self) -> &VecDeque<Candle> {
        &self.history
    }

    /// Pushes a new candle on to the history and removes the oldest when the histories capacity is full.
    pub fn push_candle(&mut self, candle: Candle) -> Option<Candle> {
        let pop_candle = self.history.len() == self.history.capacity();
        let old_candle = self.history.pop_back_if(|_| pop_candle);

        self.history.push_front(candle);

        self.volatility = self.calculate_volatility(DEFAULT_CANDLE_HISTORY_SIZE);

        old_candle
    }

    pub fn change_abs(&self) -> i64 {
        self.change_abs
    }

    pub fn change_percent(&self) -> i64 {
        self.change_percent
    }

    pub fn volatility(&self) -> i64 {
        self.volatility
    }

    pub fn calculate_volatility(&self, candles: usize) -> i64 {
        if self.history.is_empty() {
            return 0;
        };

        let iter = self
            .history
            .iter()
            .take(candles)
            .map(|candle| candle.close());
        let count = iter.clone().count() as i64;
        let mean = iter
            .clone()
            .fold(0i64, |acc, close| acc + close as i64)
            .wrapping_div(count);
        let variance = iter
            .map(|close| close as i64 - mean)
            .map(|deviation| deviation.pow(2) / 100)
            .sum::<i64>()
            .wrapping_div(count);
        (variance * 100).isqrt() // Scale the variance up to not leave a decimal point behind.
    }
}

#[cfg(test)]
mod stock_test {
    use crate::market::{chart::Candle, stock::SharePerformance};

    #[test]
    fn volatility_calculation_test() {
        let mut perf = SharePerformance::new(0, 0);
        let v = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
        for v in v.iter() {
            perf.push_candle(Candle::new(*v, *v, *v, *v));
        }

        assert_eq!(5500, v.iter().sum::<u64>());
        let mean = v.iter().fold(0, |acc, x| acc + x) / v.len() as u64;
        assert_eq!(550, mean);
        let deviation = v.iter().map(|x| *x as i64 - mean as i64);
        assert_eq!(
            vec![-450, -350, -250, -150, -50, 50, 150, 250, 350, 450],
            deviation.clone().collect::<Vec<i64>>()
        );
        let deviation_squared = deviation.map(|x| x.pow(2) as u64 / 100);
        assert_eq!(
            vec![2025, 1225, 625, 225, 25, 25, 225, 625, 1225, 2025],
            deviation_squared.clone().collect::<Vec<u64>>()
        );
        let deviation_squared_sum = deviation_squared.sum::<u64>();
        assert_eq!(8250, deviation_squared_sum);
        let variance = deviation_squared_sum / v.len() as u64;
        assert_eq!(825, variance);
        let standart_deviation = (variance * 100).isqrt();
        assert_eq!(287, standart_deviation);

        assert_eq!(287, perf.calculate_volatility(v.len()));
    }
}
