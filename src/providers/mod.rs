use std::str::FromStr;

pub mod docker;

#[derive(Debug)]
pub struct Container {
    pub id: String,
    pub state: State,
    pub name: String,
    pub image: String,
    pub created: jiff::Timestamp,
}

#[derive(Debug)]
pub enum State {
    Created,
    Running,
    Paused,
    Restarting,
    Exited,
    Dead,
}

impl State {
    pub fn get_symbol(&self) -> char {
        match self {
            State::Created => '◇',
            State::Running => '●',
            State::Paused => '⏸',
            State::Restarting => '↻',
            State::Exited => '◯',
            State::Dead => '☠',
        }
    }
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "created" => Ok(State::Created),
            "running" => Ok(State::Running),
            "paused" => Ok(State::Paused),
            "restarting" => Ok(State::Restarting),
            "exited" => Ok(State::Exited),
            "dead" => Ok(State::Dead),
            _ => Err(()),
        }
    }
}

pub trait Provider {
    fn get_containers() -> anyhow::Result<Vec<Container>>;
}
