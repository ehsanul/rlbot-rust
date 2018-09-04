#![cfg(windows)]

extern crate rlbot;
extern crate winapi;
extern crate winproc;

use std::error::Error;

mod common;

#[test]
fn integration_packeteer_error_when_no_packets() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        // We never started a match, so no gameplay packets should ever come. We should
        // get *maybe* one initial empty packet, but after that, only errors.
        let mut packeteer = rlbot.packeteer();
        assert!(packeteer.next().is_err() || packeteer.next().is_err());
        Ok(())
    })
}
