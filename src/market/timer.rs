use std::time::Duration;

use bevy::prelude::*;

pub const MARKET_TIMER_DEFAULT_DURATION: Duration = Duration::from_secs(60);
pub const MARKET_UPDATE_DEFAULT_INTERVALL: u32 = 60;

pub struct MarketTimePlugin;

impl Plugin for MarketTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketTimerIntervall>()
            .init_resource::<MarketTimer>()
            .init_resource::<MarketUpdateTimer>()
            .add_observer(on_update_market_timer_interval)
            .add_systems(
                FixedFirst,
                (
                    (countdown_market_timer, countdown_market_update_timer),
                    (market_timer_finished, market_update_timer_finished),
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct MarketTimerIntervall(Duration);

/// Set the [`MarketTimer`] to the interval or if None to the default interval.
#[derive(Event, Debug)]
pub struct UpdateMarketTimerInterval(pub Option<Duration>);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct MarketTimer(Timer);

#[derive(Event, Debug)]
pub struct MarketTimeFinishedEvent;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct MarketUpdateTimer(Timer);

#[derive(Event, Debug)]
pub struct MarketUpdateTimeFinishedEvent;

impl Default for MarketTimerIntervall {
    fn default() -> Self {
        Self(MARKET_TIMER_DEFAULT_DURATION)
    }
}

impl Default for MarketTimer {
    fn default() -> Self {
        Self(Timer::new(
            MARKET_TIMER_DEFAULT_DURATION,
            TimerMode::Repeating,
        ))
    }
}

impl Default for MarketUpdateTimer {
    fn default() -> Self {
        Self(Timer::new(
            MARKET_TIMER_DEFAULT_DURATION / MARKET_UPDATE_DEFAULT_INTERVALL,
            TimerMode::Repeating,
        ))
    }
}

fn on_update_market_timer_interval(
    new_interval: On<UpdateMarketTimerInterval>,
    mut market_timer: ResMut<MarketTimer>,
    mut market_update_timer: ResMut<MarketUpdateTimer>,
) {
    let new_duration = if let Some(new_duration) = new_interval.0 {
        new_duration
    } else {
        MARKET_TIMER_DEFAULT_DURATION
    };

    market_timer.set_duration(new_duration);
    market_update_timer.set_duration(new_duration / MARKET_UPDATE_DEFAULT_INTERVALL);
}

fn countdown_market_timer(time: Res<Time>, mut market_timer: ResMut<MarketTimer>) {
    market_timer.tick(time.delta());
}

fn market_timer_finished(mut commands: Commands, market_timer: Res<MarketTimer>) {
    if market_timer.is_finished() {
        commands.trigger(MarketTimeFinishedEvent);
    }
}

fn countdown_market_update_timer(
    time: Res<Time>,
    mut market_update_timer: ResMut<MarketUpdateTimer>,
) {
    market_update_timer.tick(time.delta());
}

fn market_update_timer_finished(
    mut commands: Commands,
    market_update_timer: Res<MarketUpdateTimer>,
) {
    if market_update_timer.is_finished() {
        commands.trigger(MarketUpdateTimeFinishedEvent);
    }
}
