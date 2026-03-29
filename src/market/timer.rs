use bevy::prelude::*;

pub const MARKET_TIMER_DEFAULT_DURATION: std::time::Duration =
    std::time::Duration::from_millis(1000);
pub const MARKET_UPDATE_DEFAULT_DURATION: std::time::Duration =
    std::time::Duration::from_millis(100);

pub struct MarketTimePlugin;

impl Plugin for MarketTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketTime>()
            .init_resource::<MarketUpdateTime>()
            .add_systems(
                FixedFirst,
                (
                    (countdown_market_time, countdown_market_update_time),
                    (market_time_finished, market_update_time_finished),
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Debug)]
pub struct MarketTime(Timer);

#[derive(Event, Debug)]
pub struct MarketTimeFinishedEvent;

#[derive(Resource, Debug)]
pub struct MarketUpdateTime(Timer);

#[derive(Event, Debug)]
pub struct MarketUpdateTimeFinishedEvent;

impl Default for MarketTime {
    fn default() -> Self {
        Self(Timer::new(
            MARKET_TIMER_DEFAULT_DURATION,
            TimerMode::Repeating,
        ))
    }
}

impl Default for MarketUpdateTime {
    fn default() -> Self {
        Self(Timer::new(
            MARKET_UPDATE_DEFAULT_DURATION,
            TimerMode::Repeating,
        ))
    }
}

fn countdown_market_time(time: Res<Time>, mut market_time: ResMut<MarketTime>) {
    market_time.0.tick(time.delta());
}

fn market_time_finished(mut commands: Commands, market_time: Res<MarketTime>) {
    if market_time.0.is_finished() {
        commands.trigger(MarketTimeFinishedEvent);
    }
}

fn countdown_market_update_time(time: Res<Time>, mut market_update_time: ResMut<MarketUpdateTime>) {
    market_update_time.0.tick(time.delta());
}

fn market_update_time_finished(mut commands: Commands, market_update_time: Res<MarketUpdateTime>) {
    if market_update_time.0.is_finished() {
        commands.trigger(MarketUpdateTimeFinishedEvent);
    }
}
