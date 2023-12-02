use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use ignore::DirEntry;
use log::{error, warn};
use rise_of_industry_calculator::serialization::{
    Building, BuildingModule, CountedProductId, GameData, Product, ProductCategory, Recipe,
};
use rise_of_industry_importer::*;
use serde::Deserialize;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // let asset_path: PathBuf = std::env::args_os()
    //     .nth(1)
    //     .expect("Pass the path to the assets")
    //     .into();

    let input_path = PathBuf::from("rise-of-industry-data");
    let output_path = PathBuf::from("data.json");

    let walk = ignore::Walk::new(input_path);

    let mut game_data = GameData::default();
    let mut product_categories = Vec::new();
    let mut product_category_modifier_infos = Vec::new();

    for entry in walk {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warn!("Encountered an error while walking the assets: {error:?}.");
                continue;
            }
        };

        match entry.path().extension() {
            Some(ext) if ext == OsStr::new("asset") || ext == OsStr::new("prefab") => {
                process_asset(
                    &mut game_data,
                    entry,
                    &mut product_categories,
                    &mut product_category_modifier_infos,
                );
            }
            _ => {
                continue;
            }
        }
    }

    game_data.product_categories = product_categories
        .into_iter()
        .map(|(guid, product_category)| {
            // TODO: Handle vanilla / 2130. Select PCMI based on the asset list or something..
            let entry = product_category_modifier_infos
                .iter()
                .find_map(|info| info.modifiers.iter().find(|&x| x.category.guid == guid))
                .unwrap();
            (
                guid,
                ProductCategory {
                    name: product_category.name,
                    price_modifier: entry.price_modifier,
                    growth_modifier: entry.growth_modifier,
                },
            )
        })
        .collect();

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&game_data).unwrap(),
    )
    .unwrap();
}

fn read_yaml(path: &Path) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(rewrite_yaml_tags(&contents).to_string())
}

fn read_meta_yaml(original_path: &Path) -> std::io::Result<MetaDocument> {
    let file_path = {
        let mut file_path = original_path.file_name().unwrap().to_owned();
        file_path.push(".meta");
        original_path.with_file_name(file_path)
    };
    let meta_contents = read_yaml(&file_path)?;
    Ok(serde_yaml::from_str(&meta_contents).unwrap())
}

fn process_asset(
    game_data: &mut GameData,
    entry: DirEntry,
    product_categories: &mut Vec<(String, ProductCategoryMonoBehaviour)>,
    product_category_modifier_info: &mut Vec<ProductCategoryModifierInfoMonoBehaviour>,
) {
    let contents = read_yaml(entry.path()).unwrap();

    enum Base {
        Recipe(RecipeMonoBehaviour),
        Product(ProductDefinitionMonoBehaviour),
        Building(BuildingMonoBehaviour),
    }

    enum Detail {
        GathererHub(GathererHubMonoBehaviour),
        Factory(FactoryMonoBehaviour),
        Harvester(HarvesterMonoBehaviour),
    }

    let mut base = None;
    let mut detail = None;

    for deserializer in serde_yaml::Deserializer::from_str(&contents) {
        match Document::deserialize(deserializer) {
            Ok(Document::Known(KnownDocument::MonoBehaviour(MonoBehaviour::Known(known)))) => {
                match known {
                    KnownMonoBehaviour::Building(value) => {
                        assert!(base.replace(Base::Building(value)).is_none())
                    }
                    KnownMonoBehaviour::Recipe(value) => {
                        assert!(base.replace(Base::Recipe(value)).is_none())
                    }
                    KnownMonoBehaviour::ProductDefinition(value) => {
                        assert!(base.replace(Base::Product(value)).is_none())
                    }
                    KnownMonoBehaviour::GathererHub(value) => {
                        assert!(detail.replace(Detail::GathererHub(value)).is_none())
                    }
                    KnownMonoBehaviour::Factory(value) => {
                        assert!(detail.replace(Detail::Factory(value)).is_none())
                    }
                    KnownMonoBehaviour::Harvester(value) => {
                        assert!(detail.replace(Detail::Harvester(value)).is_none())
                    }
                    KnownMonoBehaviour::ProductCategory(value) => {
                        let meta_document = read_meta_yaml(entry.path()).unwrap();
                        product_categories.push((meta_document.guid, value));
                    }
                    KnownMonoBehaviour::ProductCategoryModifierInfo(value) => {
                        product_category_modifier_info.push(value);
                    }
                }
            }
            Ok(_) => {
                continue;
            }
            Err(error) => {
                error!(
                    "Failed to parse document in {:?}: {error:?}",
                    entry.path().display()
                );
            }
        }
    }

    match (base, detail) {
        (Some(Base::Recipe(recipe)), None) => {
            let meta_document = read_meta_yaml(entry.path()).unwrap();
            process_recipe(game_data, meta_document, recipe);
        }
        (Some(Base::Product(product)), None) => {
            let meta_document = read_meta_yaml(entry.path()).unwrap();
            process_product(game_data, meta_document, product);
        }
        (Some(Base::Building(building)), Some(Detail::Factory(factory))) => {
            let meta_document = read_meta_yaml(entry.path()).unwrap();
            process_factory(game_data, meta_document, factory, building);
        }
        (Some(Base::Building(building)), Some(Detail::GathererHub(gatherer_hub))) => {
            let meta_document = read_meta_yaml(entry.path()).unwrap();
            process_gatherer_hub(game_data, meta_document, gatherer_hub, building);
        }
        (Some(Base::Building(building)), Some(Detail::Harvester(harvester))) => {
            let meta_document = read_meta_yaml(entry.path()).unwrap();
            process_harvester(game_data, meta_document, harvester, building);
        }
        (Some(Base::Building(building)), None) => {
            let meta_document = read_meta_yaml(entry.path()).unwrap();
            process_building_without_detail(game_data, meta_document, building);
            warn!("Building without detail in {:?}", entry.path())
        }
        (None, None) => {}
        (_base, _detail) => {
            error!(
                "Invalid combination of scripts in {:?}",
                entry.path().display()
            );
        }
    }
}

fn process_recipe(
    game_data: &mut GameData,
    meta_document: MetaDocument,
    recipe: RecipeMonoBehaviour,
) {
    assert!(game_data
        .recipes
        .insert(
            meta_document.guid,
            Recipe {
                name: recipe.name,
                products: std::iter::Iterator::chain(
                    recipe
                        .ingredients
                        .entries
                        .into_iter()
                        .map(|ingredient| CountedProductId {
                            product_id: ingredient.definition.guid,
                            amount: -(i64::try_from(ingredient.amount).unwrap()),
                        }),
                    recipe
                        .result
                        .entries
                        .into_iter()
                        .map(|ingredient| CountedProductId {
                            product_id: ingredient.definition.guid,
                            amount: i64::try_from(ingredient.amount).unwrap(),
                        }),
                )
                .collect(),
                days: recipe.days,
                required_modules: recipe
                    .required_modules
                    .into_iter()
                    .map(|x| x.guid)
                    .collect::<Vec<_>>()
            }
        )
        .is_none());
}

fn process_product(
    game_data: &mut GameData,
    meta_document: MetaDocument,
    product: ProductDefinitionMonoBehaviour,
) {
    assert!(game_data
        .products
        .insert(
            meta_document.guid,
            Product {
                name: product.name,
                category_id: product.category.guid
            }
        )
        .is_none());
}

fn process_gatherer_hub(
    game_data: &mut GameData,
    meta_document: MetaDocument,
    gatherer_hub: GathererHubMonoBehaviour,
    building: BuildingMonoBehaviour,
) {
    assert!(game_data
        .buildings
        .insert(
            meta_document.guid,
            Building {
                name: building.name,
                base_cost: building.base_cost,
                available_recipes: gatherer_hub
                    .available_recipes
                    .into_iter()
                    .map(|x| x.guid)
                    .collect(),
            }
        )
        .is_none());
}

fn process_harvester(
    game_data: &mut GameData,
    meta_document: MetaDocument,
    harvester: HarvesterMonoBehaviour,
    building: BuildingMonoBehaviour,
) {
    // TODO: Do we care?
    _ = harvester.consumption;

    assert!(game_data
        .modules
        .insert(
            meta_document.guid,
            BuildingModule {
                name: building.name,
                base_cost: building.base_cost,
            }
        )
        .is_none());
}

fn process_factory(
    game_data: &mut GameData,
    meta_document: MetaDocument,
    factory: FactoryMonoBehaviour,
    building: BuildingMonoBehaviour,
) {
    assert!(game_data
        .buildings
        .insert(
            meta_document.guid,
            Building {
                name: building.name,
                base_cost: building.base_cost,
                available_recipes: factory
                    .available_recipes
                    .into_iter()
                    .map(|x| x.guid)
                    .collect::<Vec<_>>(),
            }
        )
        .is_none());
}

fn process_building_without_detail(
    game_data: &mut GameData,
    meta_document: MetaDocument,
    building: BuildingMonoBehaviour,
) {
    assert!(game_data
        .buildings
        .insert(
            meta_document.guid,
            Building {
                name: building.name,
                base_cost: building.base_cost,
                available_recipes: vec![],
            }
        )
        .is_none());
}
