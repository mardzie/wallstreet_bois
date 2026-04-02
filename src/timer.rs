use std::time::Duration;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

pub const MARKET_TIMER_DEFAULT_DURATION: Duration = Duration::from_secs(60);
pub const MARKET_UPDATE_DEFAULT_INTERVALL: u32 = 60;
pub const MIN_MARKET_UPDATE_INTERVALL: Duration = Duration::from_millis(1000);

pub struct MarketTimePlugin;

impl Plugin for MarketTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketTimerIntervall>()
            .init_resource::<MarketTimer>()
            .init_resource::<MarketUpdateTimer>()
            .add_schedule(Schedule::new(MarketTimeFinishedSchedule))
            .add_schedule(Schedule::new(MarketTimeUpdateSchedule))
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

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketTimeFinishedSchedule;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct MarketUpdateTimer(Timer);

#[derive(Event, Debug)]
pub struct MarketUpdateTimeFinishedEvent;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketTimeUpdateSchedule;

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
    market_update_timer.set_duration(
        (new_duration / MARKET_UPDATE_DEFAULT_INTERVALL).max(MIN_MARKET_UPDATE_INTERVALL),
    );
}

fn countdown_market_timer(time: Res<Time>, mut market_timer: ResMut<MarketTimer>) {
    market_timer.tick(time.delta());
}

fn market_timer_finished(world: &mut World) {
    if world
        .get_resource::<MarketTimer>()
        .expect("`MarketTimer` must exist at this point!")
        .is_finished()
    {
        world.trigger(MarketTimeFinishedEvent);
        world.run_schedule(MarketTimeFinishedSchedule);
    }
}

fn countdown_market_update_timer(
    time: Res<Time>,
    mut market_update_timer: ResMut<MarketUpdateTimer>,
) {
    market_update_timer.tick(time.delta());
}

fn market_update_timer_finished(world: &mut World) {
    if world
        .get_resource::<MarketUpdateTimer>()
        .expect("`MarketUpdateTimer` must exist at this point!")
        .is_finished()
    {
        world.trigger(MarketUpdateTimeFinishedEvent);
        world.run_schedule(MarketTimeUpdateSchedule);
    }
}
