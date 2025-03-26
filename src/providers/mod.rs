use std::str::FromStr;

pub mod docker;

pub struct Container {
    pub id: String,
    pub state: State,
    pub name: String,
    pub image: String,
}

pub enum State {
    Exited,
    Running,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exited" => Ok(State::Exited),
            "running" => Ok(State::Running),
            _ => Err(()),
        }
    }
}

pub trait Provider {
    fn get_containers() -> anyhow::Result<Vec<Container>>;
}
