use bevy::prelude::*;

pub mod investor_traits;

pub struct InvestorPlugin;

impl Plugin for InvestorPlugin {
    fn build(&self, app: &mut App) {}
}

type PRICE = u64;

#[derive(Component, Debug)]
pub struct Investor {
    cash: u64,
    assets: Vec<(Entity, u32, PRICE)>,
}
