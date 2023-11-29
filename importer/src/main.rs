use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf}, collections::BTreeMap, fs::FileType,
};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use rise_of_industry_importer::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // let asset_path: PathBuf = std::env::args_os()
    //     .nth(1)
    //     .expect("Pass the path to the assets")
    //     .into();

    let asset_path = PathBuf::from("import_data");

    let walk = ignore::Walk::new(asset_path);
    // let walk = ignore::Walk::new(asset_path.join("ExportedProject\\Assets\\Resources\\gamedata"));

    let mut extension_to_files: BTreeMap<OsString, Vec<(PathBuf, u64)>> = BTreeMap::new();
    
    for entry in walk {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warn!("Encountered an error while walking the assets: {error:?}.");
                continue;
            }
        };

        let Ok(meta) = entry.metadata().map_err(|error| {
            warn!(
                "Failed to get file {:?} metadata: {error:?}.",
                entry.path().display()
            );
        }) else {
            continue;
        };

        if (!meta.file_type().is_file()) {
            continue;
        }

        let Some(extension) = entry.path().extension() else {
            continue;
        };

        extension_to_files.entry(extension.to_owned()).or_default().push((entry.path().to_owned(), meta.len()));

        // let Some(ext) = entry.path().extension() {
        //     Some(ext) if ext == OsStr::new("asset") || ext == OsStr::new("prefab") => {
        //         // Great.
        //     }
        //     _ => {
        //         continue;
        //     }
        // }

        // let contents = std::fs::read_to_string(entry.path()).unwrap();
        // let regex = regex::RegexBuilder::new("^---.*$")
        //     .multi_line(true)
        //     .build()
        //     .unwrap();
        // let contents = regex.replace_all(&contents, "---");

        // for deserializer in serde_yaml::Deserializer::from_str(&contents) {
        //     let Ok(document) = MonoBehaviourDocument::deserialize(deserializer) else {
        //         continue;
        //     };

        //     let Ok(meta_data) = MonoBehaviourMetaData::deserialize(&document.mono_behavior) else {
        //         continue;
        //     };

        //     match meta_data.script.guid {
        //         x if x == RECIPE_SCRIPT.guid => {
        //             info!("Found recipe at {:?}", entry.path().display());
        //             let recipe = RecipeMonoBehaviour::deserialize(&document.mono_behavior).unwrap();
        //         }
        //         x if x == PRODUCT_GUID => {}
        //         _ => {
        //             continue;
        //         }
        //     }
        // }
    }

    for (extension, files) in &extension_to_files {
        let total_bytes = files.iter().map(|&(_, len)| len).sum::<u64>();
        println!("{} ({total_bytes} bytes):", extension.to_string_lossy());
        for (file, len) in files {
            println!("  {:?} ({len} bytes)", file.display());
        }
    }
}
