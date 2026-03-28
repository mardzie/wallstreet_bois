use bevy::prelude::*;

use crate::{
    all_stocks::ALL_STOCKS,
    market::{
        sector::Sector,
        stock::{Performance, Stock, Value},
    },
};

pub mod chart;
pub mod sector;
pub mod stock;

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut command: Commands) {
    let mut batch = Vec::with_capacity(ALL_STOCKS.len());
    for (name, ticker, about, sector, value) in ALL_STOCKS.iter() {
        batch.push((
            Stock::new(name, ticker, about, *sector),
            Value::from_current(*value),
            Performance::new(0, 0),
        ));
    }
    command.spawn_batch(batch);
}
