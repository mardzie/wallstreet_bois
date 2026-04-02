use bevy::prelude::*;

use crate::{
    companies::CompaniesPlugin,
    investors::InvestorPlugin,
    market::MarketPlugin,
    timer::{MarketTimePlugin, UpdateMarketTimerInterval},
};

mod all_stocks;
mod companies;
mod investors;
mod market;
mod timer;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins((
        MarketTimePlugin,
        CompaniesPlugin,
        MarketPlugin,
        InvestorPlugin,
    ));

    app.world_mut().trigger(UpdateMarketTimerInterval(Some(
        std::time::Duration::from_millis(1000),
    )));

    app.run();
}
