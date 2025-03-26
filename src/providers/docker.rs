use std::{process::Command, str::FromStr};

use serde::Deserialize;

use super::{Container, Provider};

#[derive(Debug, Deserialize)]
pub struct DockerContainer {
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Labels")]
    pub labels: String,
    #[serde(rename = "LocalVolumes")]
    pub local_volumes: String,
    #[serde(rename = "Mounts")]
    pub mounts: String,
    #[serde(rename = "Names")]
    pub names: String,
    #[serde(rename = "Networks")]
    pub networks: String,
    #[serde(rename = "Ports")]
    pub ports: String,
    #[serde(rename = "RunningFor")]
    pub running_for: String,
    #[serde(rename = "Size")]
    pub size: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
}

pub struct Docker {}

impl Provider for Docker {
    fn get_containers() -> anyhow::Result<Vec<Container>> {
        let output = Command::new("docker")
            .arg("ps")
            .arg("-a")
            .arg("--format")
            .arg("json")
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "failed to execute docker command, status code: {}",
                output.status
            ));
        }

        let stdout = std::str::from_utf8(&output.stdout)?;
        let docker_containers: Vec<DockerContainer> = stdout
            .lines()
            .map(serde_json::from_str::<DockerContainer>)
            .collect::<Result<Vec<DockerContainer>, serde_json::Error>>()?;

        Ok(docker_containers
            .iter()
            .map(|docker_container| Container {
                id: docker_container.id.clone(),
                state: super::State::from_str(&docker_container.state).unwrap(),
                name: docker_container.names.clone(),
                image: docker_container.image.clone(),
            })
            .collect())
    }
}
