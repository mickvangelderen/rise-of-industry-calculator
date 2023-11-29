// use std::{
//     collections::BTreeMap,
//     ffi::{OsStr, OsString},
//     fs::FileType,
//     path::{Path, PathBuf},
// };

// use ignore::DirEntry;
// use log::{debug, info, warn};
// use rise_of_industry_calculator::serialization::{
//     CountedProductId, GameData, Module, Product, Recipe,
// };
// use rise_of_industry_importer::*;
// use serde::{Deserialize, Serialize};

// fn main() {
//     env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

//     // let asset_path: PathBuf = std::env::args_os()
//     //     .nth(1)
//     //     .expect("Pass the path to the assets")
//     //     .into();

//     let input_path = PathBuf::from("rise-of-industry-data");
//     let output_path = PathBuf::from("data.json");

//     let walk = ignore::Walk::new(input_path);

//     let mut game_data = GameData::default();

//     for entry in walk {
//         let entry = match entry {
//             Ok(entry) => entry,
//             Err(error) => {
//                 warn!("Encountered an error while walking the assets: {error:?}.");
//                 continue;
//             }
//         };

//         match entry.path().extension() {
//             Some(ext) if ext == OsStr::new("asset") => {
//                 process_asset(&mut game_data, entry);
//             }
//             Some(ext) if ext == OsStr::new("prefab") => {
//                 process_prefab(&mut game_data, entry);
//             }
//             _ => {
//                 continue;
//             }
//         }
//     }

//     std::fs::write(
//         output_path,
//         serde_json::to_string_pretty(&game_data).unwrap(),
//     )
//     .unwrap();
// }

// fn read_yaml(path: &Path) -> std::io::Result<String> {
//     let contents = std::fs::read_to_string(path)?;
//     Ok(rewrite_yaml_tags(&contents).to_string())
// }

// fn read_meta_yaml(original_path: &Path) -> std::io::Result<MetaDocument> {
//     let file_path = {
//         let mut file_path = original_path.file_name().unwrap().to_owned();
//         file_path.push(".meta");
//         original_path.with_file_name(file_path)
//     };
//     let meta_contents = read_yaml(&file_path)?;
//     Ok(serde_yaml::from_str(&meta_contents).unwrap())
// }

// fn process_asset(game_data: &mut GameData, entry: DirEntry) {
//     let contents = read_yaml(entry.path()).unwrap();
//     for deserializer in serde_yaml::Deserializer::from_str(&contents) {
//         let Ok(document) = MonoBehaviourDocument::deserialize(deserializer) else {
//             continue;
//         };

//         match document.script_guid() {
//             x if x == RecipeMonoBehaviour::GUID => {
//                 info!("Found recipe at {:?}", entry.path().display());
//                 let meta_document = read_meta_yaml(entry.path()).unwrap();
//                 let recipe = RecipeMonoBehaviour::deserialize(&document.mono_behavior).unwrap();
//                 process_recipe(game_data, meta_document, recipe);
//             }
//             x if x == ProductDefinitionMonoBehaviour::GUID => {
//                 info!("Found product definition at {:?}", entry.path().display());
//                 let meta_document = read_meta_yaml(entry.path()).unwrap();
//                 let product =
//                     ProductDefinitionMonoBehaviour::deserialize(&document.mono_behavior).unwrap();
//                 process_product(game_data, meta_document, product);
//             }
//             _ => {
//                 continue;
//             }
//         }
//     }
// }

// fn process_recipe(
//     game_data: &mut GameData,
//     meta_document: MetaDocument,
//     recipe: RecipeMonoBehaviour,
// ) {
//     assert!(game_data
//         .recipes
//         .insert(
//             meta_document.guid,
//             Recipe {
//                 name: recipe.name,
//                 products: std::iter::Iterator::chain(
//                     recipe
//                         .ingredients
//                         .entries
//                         .into_iter()
//                         .map(|ingredient| CountedProductId {
//                             product_id: ingredient.definition.guid,
//                             count: -(i64::try_from(ingredient.amount).unwrap()),
//                         }),
//                     recipe
//                         .result
//                         .entries
//                         .into_iter()
//                         .map(|ingredient| CountedProductId {
//                             product_id: ingredient.definition.guid,
//                             count: i64::try_from(ingredient.amount).unwrap(),
//                         }),
//                 )
//                 .collect(),
//             }
//         )
//         .is_none());
// }

// fn process_product(
//     game_data: &mut GameData,
//     meta_document: MetaDocument,
//     product: ProductDefinitionMonoBehaviour,
// ) {
//     assert!(game_data
//         .products
//         .insert(meta_document.guid, Product { name: product.name })
//         .is_none());
// }

// fn process_prefab(game_data: &mut GameData, entry: DirEntry) {
//     let contents = read_yaml(entry.path()).unwrap();
//     for deserializer in serde_yaml::Deserializer::from_str(&contents) {
//         let Ok(document) = MonoBehaviourDocument::deserialize(deserializer) else {
//             continue;
//         };

//         let Ok(meta_data) = MonoBehaviourMetaData::deserialize(&document.mono_behavior) else {
//             continue;
//         };

//         match meta_data.script.guid {
//             x if x == GATHERER_HUB_SCRIPT.guid => {
//                 info!("Found module at {:?}", entry.path().display());
//                 let meta_document = read_meta_yaml(entry.path()).unwrap();
//                 let module = GathererHubMonoBehaviour::deserialize(&document.mono_behavior).unwrap();
//                 process_module(game_data, meta_document, module);
//             }
//             _ => {
//                 continue;
//             }
//         }
//     }
// }

// fn process_module(
//     game_data: &mut GameData,
//     meta_document: MetaDocument,
//     module: GathererHubMonoBehaviour,
// ) {
//     assert!(game_data
//         .modules
//         .insert(
//             meta_document.guid,
//             Module {
//                 name: module.name,
//                 available_recipes: module
//                     .available_recipes
//                     .into_iter()
//                     .map(|x| x.guid)
//                     .collect(),
//             }
//         )
//         .is_none());
// }

fn main() {}
