use bevy::prelude::*;

use crate::market::{MarketPlugin, timer::UpdateMarketTimerInterval};

mod all_stocks;
mod market;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(MarketPlugin);
    app.world_mut().trigger(UpdateMarketTimerInterval(Some(
        std::time::Duration::from_millis(1000),
    )));

    app.run();
}
