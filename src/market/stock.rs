use std::collections::VecDeque;

use bevy::prelude::*;

use crate::market::{chart::Candle, sector::Sector};

/// The history to preserve for each stock.
pub const HISTORY_LENGHT: usize = 1024 * 16;

#[derive(Component, Debug)]
pub struct Stock {
    name: String,
    ticker: String,
    /// A description of the stocks company.
    about: String,
    sector: Sector,
}

#[derive(Component, Debug)]
pub struct Value {
    /// The current Value of the Stock in ct.
    current: u32,
    open: u32,
    max: u32,
    min: u32,
}

#[derive(Component, Debug, Default)]
pub struct Performance {
    change_abs: i32,
    change_percent: i32,
    history: VecDeque<Candle>,
}

impl Stock {
    pub fn new(name: &str, ticker: &str, about: &str, sector: Sector) -> Self {
        Self {
            name: name.to_string(),
            ticker: ticker.to_uppercase(),
            about: about.to_string(),
            sector,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn about(&self) -> &str {
        &self.about
    }

    pub fn sector(&self) -> &Sector {
        &self.sector
    }
}

impl Value {
    /// New Value from cent values.
    pub fn new(current: u32, open: u32, max: u32, min: u32) -> Self {
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
    pub fn from_current(current: u32) -> Self {
        Self {
            current,
            open: current,
            max: current,
            min: current,
        }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn update_current_value(&mut self, new_current: u32) {
        self.current = new_current;
        self.min = self.min.min(new_current);
        self.max = self.max.max(new_current);
    }

    pub fn calculate_change_abs(&self) -> i32 {
        self.current as i32 - self.open as i32
    }

    /// Calculate the percentage change.
    ///
    /// The change gets returned as an `i32` with two decimal places.
    ///
    /// `100 % = 10000`
    /// `150.25 % = 15025`
    /// `150.2581 % = 15025`
    pub fn calculate_change_percentage(&self) -> i32 {
        /// Percent in i32 with two decimal places.
        ///
        /// `100 % = 10000`
        /// `150.25 % = 15025`
        const MULTIPLICATOR: i32 = 10_000;
        (self.current as i32 - self.open as i32) * MULTIPLICATOR / self.open as i32
    }

    pub fn open(&self) -> u32 {
        self.open
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn close_candle(&mut self, new_current: u32) -> Candle {
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

impl Performance {
    pub fn new(change_abs: i32, change_percent: i32) -> Self {
        Self {
            change_abs,
            change_percent,
            history: VecDeque::with_capacity(HISTORY_LENGHT),
        }
    }

    /// Update the change.
    ///
    /// Takes the format that [`Value`] puts out.
    pub fn update_change(&mut self, abs: i32, percent: i32) {
        self.change_abs = abs;
        self.change_percent = percent;
    }

    pub fn history(&self) -> &VecDeque<Candle> {
        &self.history
    }

    /// Pushes a new candle on to the history and removes the oldest when the histories capacity is full.
    pub fn push_candle(&mut self, candle: Candle) -> Option<Candle> {
        let pop_candle = self.history.len() == self.history.capacity();
        let old_candle = self.history.pop_back_if(|_| pop_candle);

        self.history.push_front(candle);

        old_candle
    }

    pub fn change_abs(&self) -> i32 {
        self.change_abs
    }

    pub fn change_percent(&self) -> i32 {
        self.change_percent
    }
}
