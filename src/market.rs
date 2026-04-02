use bevy::prelude::*;

use crate::{
    all_stocks::ALL_STOCKS,
    companies::Company,
    market::stock::{SharePerformance, ShareValue, Stock},
    timer::{MarketTimeFinishedEvent, MarketUpdateTimeFinishedEvent},
};

pub mod chart;
pub mod stock;

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_market_time_finished)
            .add_observer(on_market_update_time_finished)
            .add_observer(on_update_performance)
            .add_systems(Startup, setup)
            .add_observer(
                |_: On<UpdatePerformanceEvent>,
                 time: Res<Time>,
                 query: Query<(&Company, &Stock, &ShareValue, &SharePerformance)>| {
                    println!("{:.2}", time.elapsed_secs());
                    for (company, stock, value, perf) in query {
                        println!(
                            "{} ({}): {:.2} $ {:.2} ({:.2} %) : Volatility: {:.2}",
                            company.name(),
                            stock.ticker(),
                            value.current() as f32 / 100.0,
                            perf.change_abs() as f32 / 100.0,
                            perf.change_percent() as f32 / 100.0,
                            perf.volatility() as f32 / 100.0,
                        );
                    }
                    println!();
                },
            )
            .configure_sets(FixedUpdate, MarketCalculationsSet);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketCalculationsSet;

#[derive(Event, Debug)]
pub struct UpdatePerformanceEvent;

fn setup(mut command: Commands) {
    let mut batch = Vec::with_capacity(ALL_STOCKS.len());
    for (name, ticker, about, sector, value) in ALL_STOCKS.iter() {
        batch.push((
            Company::from_ref(name, about, sector),
            Stock::new(ticker),
            ShareValue::from_current(*value),
            SharePerformance::new(0, 0),
        ));
    }
    command.spawn_batch(batch);
}

fn on_market_time_finished(
    _: On<MarketTimeFinishedEvent>,
    query: Query<(&mut ShareValue, &mut SharePerformance), With<Stock>>,
) {
    for (mut value, mut performance) in query {
        let new_current = value.current();
        let candle = value.close_candle(new_current);
        let _ = performance.push_candle(candle);
    }
}

fn on_market_update_time_finished(
    _: On<MarketUpdateTimeFinishedEvent>,
    mut commands: Commands,
    stocks: Query<&mut ShareValue, (With<Stock>, With<SharePerformance>)>,
) {
    for mut value in stocks {
        let new_current = value
            .current()
            .wrapping_add_signed(rand::random_range(-200..200));
        value.update_current_value(new_current);
    }

    commands.trigger(UpdatePerformanceEvent);
}

fn on_update_performance(
    _: On<UpdatePerformanceEvent>,
    stocks: Query<(&ShareValue, &mut SharePerformance), With<Stock>>,
) {
    for (value, mut performance) in stocks {
        performance.update_change(
            value.calculate_change_abs(),
            value.calculate_change_percentage(),
        );
    }
}
