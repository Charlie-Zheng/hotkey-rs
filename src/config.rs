use std::{str::FromStr, sync::Arc};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::key::Key;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    pub active_keys: Vec<Key>,
    pub actions: Arc<Vec<Action>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Up" | "U" => Ok(Direction::Up),
            "Down" | "D" => Ok(Direction::Down),
            _ => Err(format!("expected 'Up' or 'Down', got {s}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Key(KeyAction),
    Delay(usize),
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Action::Key(action) => {
                let key_str = &action.key.to_string();
                let direction_str = match action.direction {
                    Direction::Up => "Up",
                    Direction::Down => "Down",
                };
                serializer.serialize_str(&format!("{} {}", key_str, direction_str))
            }
            Action::Delay(ms) => serializer.serialize_str(&format!("Delay {}", ms)),
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.starts_with("Delay ") {
            let ms_str = s.strip_prefix("Delay ").unwrap();
            let ms = ms_str.parse::<f64>().map_err(|_| {
                serde::de::Error::custom(format!("Failed to parse delay: '{}'", ms_str))
            })?;
            Ok(Action::Delay((ms * 1e6) as usize))
        } else {
            let key_action = KeyAction::try_from(s).map_err(serde::de::Error::custom)?;
            Ok(Action::Key(key_action))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KeyAction {
    pub key: Key,
    pub direction: Direction,
}

impl Serialize for KeyAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let key_json = self.key.to_string();
        let direction_str = match self.direction {
            Direction::Up => "Up",
            Direction::Down => "Down",
        };
        serializer.serialize_str(&format!("{} {}", key_json, direction_str))
    }
}

impl<'de> Deserialize<'de> for KeyAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        KeyAction::try_from(s).map_err(serde::de::Error::custom)
    }
}

impl From<KeyAction> for String {
    fn from(action: KeyAction) -> Self {
        let key_str = action.key.to_string();
        let direction_str = match action.direction {
            Direction::Up => "Up",
            Direction::Down => "Down",
        };
        format!("{} {}", key_str, direction_str)
    }
}

impl TryFrom<String> for KeyAction {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (key_str, direction_str) = value
            .rsplit_once(' ')
            .ok_or_else(|| format!("Expected format 'KEY DIRECTION', got '{}'", value))?;

        let direction = direction_str.parse()?;

        // Try to parse as a named key via serde
        let key = key_str
            .parse()
            .map_err(|e| format!("Could not parse '{}' as a key: {}", key_str, e))?;

        Ok(KeyAction { key, direction })
    }
}

impl KeyAction {
    pub fn perform(&self) -> Result<(), rdev::SimulateError> {
        let rdev_key: rdev::Key = self.key.into();
        let event_type = match self.direction {
            Direction::Down => rdev::EventType::KeyPress(rdev_key),
            Direction::Up => rdev::EventType::KeyRelease(rdev_key),
        };
        println!("Performing action: {}", String::from(*self));
        rdev::simulate(&event_type)
    }
}

#[cfg(test)]
mod config_test {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config_str = include_str!("../test.yaml");
        let _: Config = serde_yaml::from_str(config_str).unwrap();
    }
}
