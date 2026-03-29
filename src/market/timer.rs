use bevy::prelude::*;

pub const MARKET_TIMER_DEFAULT_DURATION: std::time::Duration =
    std::time::Duration::from_millis(1000);

pub struct MarketTimePlugin;

impl Plugin for MarketTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedFirst, (countdown, market_time_finished).chain());
    }
}

#[derive(Resource)]
pub struct MarketTime(Timer);

#[derive(Event, Debug)]
pub struct MarketTimeFinishedEvent;

impl Default for MarketTime {
    fn default() -> Self {
        Self(Timer::new(
            MARKET_TIMER_DEFAULT_DURATION,
            TimerMode::Repeating,
        ))
    }
}

fn countdown(time: Res<Time>, mut market_time: ResMut<MarketTime>) {
    market_time.0.tick(time.delta());
}

fn market_time_finished(mut commands: Commands, market_time: Res<MarketTime>) {
    if market_time.0.is_finished() {
        commands.trigger(MarketTimeFinishedEvent);
    }
}
