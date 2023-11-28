// Import assets exported from the game files with https://assetripper.github.io/AssetRipper/articles/Downloads.html.

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // let asset_path: PathBuf = std::env::args_os()
    //     .nth(1)
    //     .expect("Pass the path to the assets")
    //     .into();

    let asset_path = PathBuf::from("C:\\Users\\mick\\Downloads\\export\\Rise of Industry");

    let walk = ignore::Walk::new(asset_path);

    let mut gatherers = vec![];

    for entry in walk {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warn!("Encountered an error while walking the assets: {error:?}.");
                continue;
            }
        };

        let path = entry.path();
        if is_gatherer_path(path) {
            gatherers.push(parse_gatherer(path));
        }
    }
}

#[derive(Debug, Serialize)]
struct Module {
    name: String,
    initial_recipe: serde_yaml::Value,
    available_recipe: serde_yaml::Value,
}

fn is_gatherer_path(path: &Path) -> bool {
    is_prefab_path(path)
        && path.parent().map_or(false, |parent_path| {
            parent_path.ends_with(Path::new("Assets/Resources/gamedata/buildings/gatherers"))
        })
}

#[derive(Debug, Serialize)]
struct Gatherer {
    name: String,
    description: String,
    base_cost: i64,
}

fn parse_gatherer_document(document: &serde_yaml::Value) -> Option<Gatherer> {
    let mono_behavior = document.as_mapping()?.get("MonoBehaviour")?.as_mapping()?;
    Some(Gatherer {
        name: mono_behavior.get("buildingName")?.as_str()?.to_owned(),
        description: mono_behavior.get("description")?.as_str()?.to_owned(),
        base_cost: mono_behavior.get("baseCost")?.as_i64()?,
    })
}

fn parse_gatherer(path: &Path) -> Gatherer {
    let contents = std::fs::read_to_string(path).unwrap();
    let regex = regex::RegexBuilder::new("^---.*$")
        .multi_line(true)
        .build()
        .unwrap();
    let contents = regex.replace_all(&contents, "---");

    for deserializer in serde_yaml::Deserializer::from_str(&contents) {
        let document = serde_yaml::Value::deserialize(deserializer).unwrap();
        if let Some(gatherer) = parse_gatherer_document(&document) {
            return gatherer;
        }
    }

    panic!("Failed to parse gatherer from {}", path.display());
}

// fn is_farm_path(path: &Path) -> bool {
//     is_prefab_path(path)
//         && path.parent().map_or(false, |parent_path| {
//             parent_path.ends_with(Path::new("Assets/Resources/gamedata/buildings/farms"))
//         })
// }

// fn is_factory_path(path: &Path) -> bool {
//     is_prefab_path(path)
//         && path.parent().map_or(false, |parent_path| {
//             parent_path.ends_with(Path::new("Assets/Resources/gamedata/buildings/factories"))
//         })
// }

// fn is_logistical_path(path: &Path) -> bool {
//     is_prefab_path(path)
//         && path.parent().map_or(false, |parent_path| {
//             parent_path.ends_with(Path::new("Assets/Resources/gamedata/buildings/logistical"))
//         })
// }

fn is_prefab_path(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("prefab"))
}

fn is_field_path(path: &Path) -> bool {
    is_prefab_path(path)
        && path.parent().map_or(false, |parent_path| {
            parent_path.ends_with(Path::new("Assets/Resources/gamedata/modules/fields"))
        })
}
