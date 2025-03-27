use std::{process::Command, str::FromStr};

use jiff::Timestamp;
use serde_json::Value;

use crate::providers::State;

use super::{Container, Provider};

pub struct Docker {}

impl Provider for Docker {
    fn get_containers() -> anyhow::Result<Vec<Container>> {
        let mut containers = Vec::new();

        let output = Command::new("docker")
            .arg("ps")
            .arg("-a")
            .arg("--format")
            .arg("json")
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "failed to execute docker ps command, status code: {}",
                output.status
            ));
        }

        let stdout = std::str::from_utf8(&output.stdout)?;

        for line in stdout.lines() {
            let container = serde_json::from_str::<Value>(line)?;
            let id = container["ID"].as_str().unwrap();

            let output = Command::new("docker").arg("inspect").arg(id).output()?;
            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "failed to execute docker inspect command, status code: {}",
                    output.status
                ));
            }

            let stdout = std::str::from_utf8(&output.stdout)?;
            let inspect = serde_json::from_str::<Value>(stdout)?;

            containers.push(Container {
                created: Timestamp::from_str(inspect[0]["Created"].as_str().unwrap())?,
                id: id.to_string(),
                image: inspect[0]["Name"].as_str().unwrap().to_string(),
                name: inspect[0]["Config"]["Image"].as_str().unwrap().to_string(),
                state: State::Dead,
            });
        }

        Ok(containers)
    }
}
