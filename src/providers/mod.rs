pub mod docker;

pub struct Container {
    pub id: String,
}

pub trait Provider {
    fn get_containers() -> anyhow::Result<Vec<Container>>;
}
