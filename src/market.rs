use bevy::prelude::*;

use crate::{
    all_stocks::ALL_STOCKS,
    market::{
        stock::{Performance, Stock, Value},
        timer::{MarketTimeFinishedEvent, MarketTimePlugin},
    },
};

pub mod chart;
pub mod sector;
pub mod stock;
pub mod timer;

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MarketTimePlugin)
            .add_observer(on_market_time_finished)
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (simulate_stocks, update_performance)
                    .chain()
                    .in_set(MarketSet),
            )
            .add_systems(
                FixedPostUpdate,
                |query: Query<(&Stock, &Value, &Performance)>| {
                    for (stock, value, perf) in query {
                        println!(
                            "{} ({}): {:.2} $ {:.2} ({:.2} %)",
                            stock.name(),
                            stock.ticker(),
                            value.current() as f32 / 100.0,
                            perf.change_abs() as f32 / 100.0,
                            perf.change_percent() as f32 / 100.0
                        );
                    }
                    println!();
                },
            )
            .configure_sets(FixedUpdate, MarketSet);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketSet;

fn setup(mut command: Commands) {
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

fn on_market_time_finished(
    _: On<MarketTimeFinishedEvent>,
    query: Query<(&mut Value, &mut Performance), With<Stock>>,
) {
    for (mut value, mut performance) in query {
        let new_current = value.current();
        let candle = value.close_candle(new_current);
        let _ = performance.push_candle(candle);
    }
}

fn simulate_stocks(stocks: Query<&mut Value, (With<Stock>, With<Performance>)>) {
    for mut value in stocks {
        let new_current = value
            .current()
            .strict_add_signed(rand::random_range(-20..20));
        value.update_current_value(new_current);
    }
}

fn update_performance(stocks: Query<(&Value, &mut Performance), With<Stock>>) {
    for (value, mut performance) in stocks {
        performance.update_change(
            value.calculate_change_abs(),
            value.calculate_change_percentage(),
        );
    }
}
