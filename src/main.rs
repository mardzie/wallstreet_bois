use bevy::prelude::*;

use crate::market::MarketPlugin;

mod market;
mod all_stocks;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    
    app.add_plugins(MarketPlugin);

    app.run();
}
