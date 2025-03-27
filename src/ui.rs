use crate::providers::Container;

/// Formats a list of containers as a list of lines.
pub fn draw_container_list(containers: Vec<Container>) -> Vec<String> {
    containers.iter().fold(Vec::new(), |mut acc, container| {
        acc.extend_from_slice(&[
            format!(
                "{} {} ({})",
                container.state.get_symbol(),
                container.name,
                container.id
            ),
            format!("├─ image: {}", container.image),
            format!("└─ created: {}", jiffy::HumanTime::from(container.created)),
            "".to_string(),
        ]);

        acc
    })
}
