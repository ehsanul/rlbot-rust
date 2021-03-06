//! The ball is a neutron star. The cars are planets.

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::{Point3, Vector3};
use rlbot::{ffi::MatchSettings, state};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    rlbot.start_match(MatchSettings::allstar_vs_allstar("Earth", "Mars"))?;
    rlbot.wait_for_match_start()?;

    let mut packets = rlbot.packeteer();
    let mut i = 0;
    loop {
        let packet = packets.next()?;

        // Check that the game is not showing a replay.
        // Also don't set state on every frame, that can make it laggy.
        if packet.GameInfo.RoundActive && i % 8 == 0 {
            let desired_state = get_desired_state(&packet);
            rlbot.set_game_state_struct(desired_state)?;
        }
        i += 1;
    }
}

fn get_desired_state(packet: &rlbot::ffi::LiveDataPacket) -> state::DesiredGameState {
    let ball_loc = packet.GameBall.Physics.Location;
    let ball_loc = Point3::new(ball_loc.X, ball_loc.Y, ball_loc.Z);

    let mut desired_game_state = state::DesiredGameState::new();

    for (i, car) in packet.cars().enumerate() {
        let car_phys = car.Physics;
        let v = car_phys.Velocity;
        let a = gravitate_towards_ball(&ball_loc, &car);

        // Note: You can ordinarily just use `na::Vector3::new(x, y, z)` here. There's a
        // cargo build oddity which prevents that from working in code inside the
        // `examples/` directory.
        let new_velocity = rlbot::state::Vector3Partial::new()
            .x(v.X + a.x)
            .y(v.Y + a.y)
            .z(v.Z + a.z);

        let physics = state::DesiredPhysics::new().velocity(new_velocity);
        let car_state = state::DesiredCarState::new().physics(physics);
        desired_game_state = desired_game_state.car_state(i, car_state);
    }

    desired_game_state
}

/// Generate an acceleration to apply to the car towards the ball, as if the
/// ball exerted a large gravitational force
fn gravitate_towards_ball(ball_loc: &Point3<f32>, car: &rlbot::ffi::PlayerInfo) -> Vector3<f32> {
    let car_loc = car.Physics.Location;
    let car_loc = Point3::new(car_loc.X, car_loc.Y, car_loc.Z);
    let ball_delta = ball_loc - car_loc;
    let distance = ball_delta.norm();
    let k = 1_000_000.0;
    let a = k / (distance / 5.0).powf(2.0);
    a * ball_delta.normalize()
}
