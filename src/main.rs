use std::{collections::HashMap, sync::Arc, thread::JoinHandle};

use clap::Parser;
use env_logger::Env;
use log::{debug, info};
use thread_priority::{ThreadBuilderExt, ThreadPriority};

use crate::config::{Config, ProcessedAction};

mod args;
mod config;
mod key;
use rdev::{Event, listen};
fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();
    let args = args::Args::try_parse()?;
    let config: Config = serde_yaml::from_reader(std::fs::File::open(args.config)?)?;
    let config = config.process();
    info!("Config: {:?}", config);

    let mut state = HashMap::new();
    let mut physical_state = HashMap::new();
    for k in config.active_keys.iter().copied() {
        state.insert(k, false);
    }

    for k in config.active_keys.iter().copied() {
        if state.contains_key(&key::Key::Control) {
            physical_state.insert(key::Key::ControlLeft, false);
            physical_state.insert(key::Key::ControlRight, false);
            continue;
        }

        if state.contains_key(&key::Key::Shift) {
            physical_state.insert(key::Key::ShiftLeft, false);
            physical_state.insert(key::Key::ShiftRight, false);
            continue;
        }
        physical_state.insert(k, false);
    }

    let mut thread: Option<JoinHandle<()>> = None;
    let callback = move |event: Event| {
        match event.event_type {
            rdev::EventType::KeyPress(rdev_key) => {
                let key = rdev_key.into();
                if physical_state.contains_key(&key) {
                    physical_state.insert(key, true);
                }

                if state.contains_key(&key) {
                    state.insert(key, true);
                }

                match rdev_key {
                    rdev::Key::ControlLeft | rdev::Key::ControlRight
                        if state.contains_key(&key::Key::Control) =>
                    {
                        state.insert(key::Key::Control, true);
                    }
                    rdev::Key::ShiftLeft | rdev::Key::ShiftRight
                        if state.contains_key(&key::Key::Shift) =>
                    {
                        state.insert(key::Key::Shift, true);
                    }
                    _ => {}
                }

                if state.iter().all(|(_, &pressed)| pressed) {
                    if thread.is_none() || thread.as_ref().unwrap().is_finished() {
                        debug!("Trigger macro!");
                        let actions = config.actions.clone();
                        thread = Some(run_macro_on_thread(actions));
                    }
                }
            }
            rdev::EventType::KeyRelease(rdev_key) => {
                let key = rdev_key.into();
                if physical_state.contains_key(&key) {
                    physical_state.insert(key, false);
                }

                if state.contains_key(&key) {
                    state.insert(key, false);
                }

                if matches!(rdev_key, rdev::Key::ControlLeft | rdev::Key::ControlRight)
                    && state.contains_key(&key::Key::Control)
                    && !physical_state
                        .get(&key::Key::ControlLeft)
                        .is_some_and(|x| *x)
                    && !physical_state
                        .get(&key::Key::ControlRight)
                        .is_some_and(|x| *x)
                {
                    state.insert(key::Key::Control, false);
                }

                if matches!(rdev_key, rdev::Key::ShiftLeft | rdev::Key::ShiftRight)
                    && state.contains_key(&key::Key::Shift)
                    && !physical_state.get(&key::Key::ShiftLeft).is_some_and(|x| *x)
                    && !physical_state
                        .get(&key::Key::ShiftRight)
                        .is_some_and(|x| *x)
                {
                    state.insert(key::Key::Shift, false);
                }
            }
            _ => {} // rdev::EventType::ButtonPress(button) => println!("Button Pressed: {:?}", button),
                    // rdev::EventType::ButtonRelease(button) => println!("Button Released: {:?}", button),
                    // rdev::EventType::MouseMove { x, y } => println!("Mouse Moved to: ({}, {})", x, y),
                    // rdev::EventType::Wheel { delta_x, delta_y } => println!("Wheel Moved: ({}, {})", delta_x, delta_y),
        }
    };

    listen(callback).unwrap();

    Ok(())
}

fn run_macro_on_thread(actions: Arc<Vec<ProcessedAction>>) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name("send_inputs".to_owned())
        .spawn_with_priority(ThreadPriority::Max, move |_| {
            let mut deadline = std::time::Instant::now();
            for action in actions.iter() {
                match action {
                    ProcessedAction::Key(key_action) => key_action.perform().unwrap(),
                    ProcessedAction::Delay(ns) => {
                        deadline += std::time::Duration::from_nanos(*ns);
                        std::thread::sleep(
                            deadline.saturating_duration_since(std::time::Instant::now()),
                        );
                    }
                }
            }
        })
        .unwrap()
}
