use std::env;
use std::fs;

const CONTAINER_TYPES: [&str; 2] = ["con", "floating_con"];

pub fn node_tree() -> Result<serde_json::Value, crate::error::Error> {
    let response = crate::sway_ipc::MessageType::GetTree.execute()?;
    Ok(serde_json::from_slice(&response)?)
}

fn containers(node: &serde_json::Value) -> Vec<&serde_json::Value> {
    let mut con_list = Vec::new();
    let children = node["nodes"].as_array().into_iter().flatten();
    for child in children {
        let child_type = child["type"].as_str().unwrap_or_default();
        let child_pid = child["pid"].as_u64();
        if CONTAINER_TYPES.contains(&child_type) && child_pid.is_some() {
            con_list.push(child);
        } else {
            con_list.extend(containers(child));
        }
    }

    return con_list;
}

fn app_containers<'a>(
    name: &str,
    node: &'a serde_json::Value,
) -> Result<Vec<&'a serde_json::Value>, crate::error::Error> {
    let con_list = containers(node)
        .into_iter()
        .filter(|c| {
            c["app_id"]
                .as_str()
                .or(c["window_properties"]["class"].as_str())
                .unwrap_or_default()
                == name
        })
        .collect::<Vec<_>>();
    Ok(con_list)
}

fn launch_application(name: &str) {
    for f in env::var("XDG_DATA_DIRS")
        .unwrap_or_default()
        .split(':')
        .map(|p| fs::read_dir(format!("{}/applications", p)))
        .filter_map(Result::ok)
        .flatten()
        .map(Result::unwrap)
        .map(|e| String::from(e.file_name().to_str().unwrap_or_default()))
        .filter(|n| n.ends_with("desktop"))
        .filter(|n| n.contains(name))
    {
        // println!("{:?}", f);
        std::process::Command::new("gtk-launch")
            .arg(f)
            .spawn()
            .unwrap();
    }
}

pub fn switch_or_launch_application(
    name: &str,
    node: &serde_json::Value,
) -> Result<(), crate::error::Error> {
    let app_containers = app_containers(name, node)?;
    if app_containers.is_empty() {
        launch_application(name);
    } else {
        let result = if let Some(focused_index) = app_containers
            .iter()
            .position(|c| c["focused"].as_bool().unwrap_or_default())
        {
            app_containers
                .iter()
                .nth(focused_index + 1)
                .or(app_containers.first())
                .unwrap_or(&&serde_json::Value::Null)
        } else {
            app_containers
                .iter()
                .find(|c| c["visible"].as_bool().unwrap_or_default())
                .or(app_containers.first())
                .unwrap_or(&&serde_json::Value::Null)
        };
        // TODO: unwrap
        let command = format!("[con_id={}] focus", result["id"].as_u64().unwrap());
        // println!("{}", command);
        crate::sway_ipc::MessageType::RunCommand(&command).execute()?;
    }
    Ok(())
}
