use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsStr,
    path::{Path, PathBuf},
};

use ignore::DirEntry;
use log::{error, warn};
use rise_of_industry_calculator::serialization::{
    Building, BuildingModule, RecipeEntry, GameData, Product, ProductCategory, Recipe,
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

    let walk = ignore::WalkBuilder::new(input_path)
        .filter_entry(|entry| {
            !(entry
                .file_type()
                .as_ref()
                .map_or(false, std::fs::FileType::is_dir)
                && entry.file_name() == "2130")
        })
        .build();

    let mut assets = Assets::default();

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
                process_asset(&mut assets, entry);
            }
            _ => {
                continue;
            }
        }
    }

    let game_data = GameData::from(assets);

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

enum BuildingDetail {
    GathererHub(GathererHubMonoBehaviour),
    Factory(FactoryMonoBehaviour),
    Harvester(HarvesterMonoBehaviour),
}

#[derive(Default)]
struct Assets {
    common_manifest: Option<GameDataManifestMonoBehaviour>,
    roi_2130_manifest: Option<GameDataManifestMonoBehaviour>,
    roi_standard_manifest: Option<GameDataManifestMonoBehaviour>,
    recipes: HashMap<String, RecipeMonoBehaviour>,
    products: HashMap<String, ProductDefinitionMonoBehaviour>,
    buildings: HashMap<String, (BuildingMonoBehaviour, Option<BuildingDetail>)>,
    product_categories: HashMap<String, ProductCategoryMonoBehaviour>,
    product_category_modifier_info: HashMap<String, ProductCategoryModifierInfoMonoBehaviour>,
    tech_tree_recipe_unlocks: HashMap<String, TechTreeRecipeUnlockMonoBehaviour>,
    product_definition_data_category_providers:
        HashMap<String, ProductDefinitionDataCategoryProviderMonoBehaviour>,
}

fn process_asset(assets: &mut Assets, entry: DirEntry) {
    let contents = read_yaml(entry.path()).unwrap();

    enum Base {
        Building(BuildingMonoBehaviour),
        Recipe(RecipeMonoBehaviour),
        Product(ProductDefinitionMonoBehaviour),
        ProductCategory(ProductCategoryMonoBehaviour),
        ProductCategoryModifierInfo(ProductCategoryModifierInfoMonoBehaviour),
        TechTreeRecipeUnlock(TechTreeRecipeUnlockMonoBehaviour),
        ProductDefinitionDataCategoryProvider(ProductDefinitionDataCategoryProviderMonoBehaviour),
        GameDataManifest(GameDataManifestMonoBehaviour),
    }

    let mut base = None;
    let mut building_detail = None;

    for deserializer in serde_yaml::Deserializer::from_str(&contents) {
        match Document::deserialize(deserializer) {
            Ok(Document::Known(KnownDocument::MonoBehaviour(MonoBehaviour::Known(known)))) => {
                match known {
                    KnownMonoBehaviour::Building(value) => {
                        assert!(base.replace(Base::Building(value)).is_none())
                    }
                    KnownMonoBehaviour::GathererHub(value) => {
                        assert!(building_detail
                            .replace(BuildingDetail::GathererHub(value))
                            .is_none())
                    }
                    KnownMonoBehaviour::Factory(value) => {
                        assert!(building_detail
                            .replace(BuildingDetail::Factory(value))
                            .is_none())
                    }
                    KnownMonoBehaviour::Harvester(value) => {
                        assert!(building_detail
                            .replace(BuildingDetail::Harvester(value))
                            .is_none())
                    }
                    KnownMonoBehaviour::Recipe(value) => {
                        assert!(base.replace(Base::Recipe(value)).is_none())
                    }
                    KnownMonoBehaviour::ProductDefinition(value) => {
                        assert!(base.replace(Base::Product(value)).is_none())
                    }
                    KnownMonoBehaviour::ProductCategory(value) => {
                        assert!(base.replace(Base::ProductCategory(value)).is_none())
                    }
                    KnownMonoBehaviour::ProductCategoryModifierInfo(value) => {
                        assert!(base
                            .replace(Base::ProductCategoryModifierInfo(value))
                            .is_none())
                    }
                    KnownMonoBehaviour::TechTreeRecipeUnlock(value) => {
                        assert!(base.replace(Base::TechTreeRecipeUnlock(value)).is_none())
                    }
                    KnownMonoBehaviour::ProductDefinitionDataCategoryProvider(value) => {
                        assert!(base
                            .replace(Base::ProductDefinitionDataCategoryProvider(value))
                            .is_none())
                    }
                    KnownMonoBehaviour::GameDataManifest(value) => {
                        assert!(base.replace(Base::GameDataManifest(value)).is_none());
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

    if let Some(base) = base {
        let meta_document = read_meta_yaml(entry.path()).unwrap();
        match (base, building_detail) {
            (Base::Building(base), detail) => {
                assert!(assets
                    .buildings
                    .insert(meta_document.guid, (base, detail))
                    .is_none());
            }
            (_, Some(_)) => {
                error!("encountered building detail without building");
            }
            (Base::Recipe(value), None) => {
                assert!(assets.recipes.insert(meta_document.guid, value).is_none());
            }
            (Base::Product(value), None) => {
                assert!(assets.products.insert(meta_document.guid, value).is_none());
            }
            (Base::ProductCategory(value), None) => {
                assert!(assets
                    .product_categories
                    .insert(meta_document.guid, value)
                    .is_none());
            }
            (Base::ProductCategoryModifierInfo(value), None) => {
                assert!(assets
                    .product_category_modifier_info
                    .insert(meta_document.guid, value)
                    .is_none());
            }
            (Base::TechTreeRecipeUnlock(value), None) => {
                assert!(assets
                    .tech_tree_recipe_unlocks
                    .insert(meta_document.guid, value)
                    .is_none());
            }
            (Base::ProductDefinitionDataCategoryProvider(value), None) => {
                assert!(assets
                    .product_definition_data_category_providers
                    .insert(meta_document.guid, value)
                    .is_none());
            }
            (Base::GameDataManifest(value), None) => match meta_document.guid.as_str() {
                "ba662a6bd9ec3b449b2375daa750a6dd" => {
                    assert!(assets.common_manifest.replace(value).is_none())
                }
                "9eb8f422ddeb0ac46812be5544560f25" => {
                    assert!(assets.roi_2130_manifest.replace(value).is_none())
                }
                "34ed83f2d30990442ad2274840669bc9" => {
                    assert!(assets.roi_standard_manifest.replace(value).is_none())
                }
                unknown => panic!("Unknown game data manifest {unknown:?}"),
            },
        }
    } else {
        assert!(building_detail.is_none());
    }
}

impl From<Assets> for GameData {
    fn from(assets: Assets) -> Self {
        enum WhichManifest {
            Common,
            RoI2130,
            RoIStandard,
        }

        impl WhichManifest {
            pub fn in_standard(&self) -> bool {
                matches!(*self, Self::Common | Self::RoIStandard)
            }
        }

        let which_manifest: HashMap<String, WhichManifest> = {
            std::iter::empty()
                .chain(
                    assets
                        .common_manifest
                        .unwrap()
                        .assets
                        .into_iter()
                        .filter_map(|r| r.0.map(|x| x.guid))
                        .map(|guid| (guid, WhichManifest::Common)),
                )
                .chain(
                    assets
                        .roi_2130_manifest
                        .unwrap()
                        .assets
                        .into_iter()
                        .filter_map(|r| r.0.map(|x| x.guid))
                        .map(|guid| (guid, WhichManifest::RoI2130)),
                )
                .chain(
                    assets
                        .roi_standard_manifest
                        .unwrap()
                        .assets
                        .into_iter()
                        .filter_map(|r| r.0.map(|x| x.guid))
                        .map(|guid| (guid, WhichManifest::RoIStandard)),
                )
                .collect()
        };

        let asset_in_standard = |guid: &str| {
            which_manifest
                .get(guid)
                .map_or(false, WhichManifest::in_standard)
        };

        // See TechTreeRecipeUnlock::OnGameDataLoaded in C#.
        let recipe_tiers: HashMap<String, i64> = {
            assets
                .tech_tree_recipe_unlocks
                .iter()
                .filter(|&(guid, _)| asset_in_standard(guid))
                .fold(Default::default(), |mut map, (_, unlock)| {
                    for recipe in &unlock.recipes {
                        map.entry(recipe.0.as_ref().unwrap().guid.clone())
                            .and_modify(|tier| {
                                *tier = i64::min(*tier, unlock.tier);
                            })
                            .or_insert(unlock.tier);
                    }
                    map
                })
        };

        let product_tiers: HashMap<String, i64> = {
            recipe_tiers
                .iter()
                .filter(|&(guid, _)| asset_in_standard(guid))
                .fold(
                    Default::default(),
                    |mut map, (recipe_guid, &recipe_tier)| {
                        let recipe = &assets.recipes[recipe_guid];
                        for entry in &recipe.result.entries {
                            let product_guid = &entry.definition.0.as_ref().unwrap().guid;
                            map.entry(product_guid.clone())
                                .and_modify(|tier| {
                                    *tier = i64::min(*tier, recipe_tier);
                                })
                                .or_insert(recipe_tier);
                        }
                        map
                    },
                )
        };

        let (modules, buildings) = {
            assets
                .buildings
                .into_iter()
                .filter(|(guid, _)| asset_in_standard(guid))
                .fold(
                    (
                        BTreeMap::<String, BuildingModule>::default(),
                        BTreeMap::<String, Building>::default(),
                    ),
                    |(mut modules, mut buildings), (guid, (base, detail))| {
                        match detail {
                            Some(BuildingDetail::GathererHub(gatherer_hub)) => {
                                buildings.insert(
                                    guid,
                                    Building {
                                        name: base.name,
                                        base_cost: base.base_cost,
                                        available_recipes: gatherer_hub
                                            .available_recipes
                                            .into_iter()
                                            .map(|x| x.0.unwrap().guid)
                                            .collect(),
                                    },
                                );
                            }
                            Some(BuildingDetail::Factory(factory)) => {
                                buildings.insert(
                                    guid,
                                    Building {
                                        name: base.name,
                                        base_cost: base.base_cost,
                                        available_recipes: factory
                                            .available_recipes
                                            .into_iter()
                                            .map(|x| x.0.unwrap().guid)
                                            .collect::<Vec<_>>(),
                                    },
                                );
                            }
                            Some(BuildingDetail::Harvester(_harvester)) => {
                                modules.insert(
                                    guid,
                                    BuildingModule {
                                        name: base.name,
                                        base_cost: base.base_cost,
                                    },
                                );
                            }
                            None => {
                                buildings.insert(
                                    guid,
                                    Building {
                                        name: base.name,
                                        base_cost: base.base_cost,
                                        available_recipes: vec![],
                                    },
                                );
                            }
                        }

                        (modules, buildings)
                    },
                )
        };

        let product_category_modifier_info = {
            let mut iter = assets
                .product_category_modifier_info
                .into_iter()
                .filter(|(guid, _)| asset_in_standard(guid));
            let first = iter
                .next()
                .expect("no product category modifier info found")
                .1;
            if iter.next().is_some() {
                panic!("multiple product category modifiers found");
            }
            first
        };

        Self {
            products: assets
                .products
                .into_iter()
                .filter(|(guid, _)| asset_in_standard(guid))
                .map(|(guid, product)| {
                    let tier = product_tiers.get(&guid).copied().unwrap();
                    let category_reference = match (
                        product.category.0.as_ref(),
                        product.category_provider.0.as_ref(),
                    ) {
                        (None, None) => panic!("no product category or provider for product"),
                        (None, Some(p)) => assets.product_definition_data_category_providers
                            [&p.guid]
                            .tier_to_category[usize::try_from(tier).unwrap()]
                        .0
                        .as_ref()
                        .unwrap(),
                        (Some(c), None) => c,
                        (Some(_), Some(_)) => panic!("both product category and provider were set"),
                    };
                    (
                        guid,
                        Product {
                            name: product.name,
                            category_id: Some(category_reference.guid.clone()),
                        },
                    )
                })
                .collect(),
            recipes: assets
                .recipes
                .into_iter()
                .filter(|(guid, _)| asset_in_standard(guid))
                .map(|(guid, recipe)| {
                    let tier = recipe_tiers[&guid];
                    (
                        guid,
                        Recipe {
                            name: recipe.name,
                            entries: std::iter::Iterator::chain(
                                recipe.ingredients.entries.into_iter().map(|ingredient| {
                                    RecipeEntry {
                                        product_id: ingredient.definition.0.unwrap().guid,
                                        amount: -(ingredient.amount),
                                    }
                                }),
                                recipe.result.entries.into_iter().map(|ingredient| {
                                    RecipeEntry {
                                        product_id: ingredient.definition.0.unwrap().guid,
                                        amount: ingredient.amount,
                                    }
                                }),
                            )
                            .collect(),
                            days: recipe.days,
                            tier,
                            required_modules: recipe
                                .required_modules
                                .into_iter()
                                .map(|x| x.0.unwrap().guid)
                                .collect::<Vec<_>>(),
                        },
                    )
                })
                .collect(),
            modules,
            buildings,
            product_categories: assets
                .product_categories
                .into_iter()
                .filter(|(guid, _)| asset_in_standard(guid))
                .map(|(guid, product_category)| {
                    let entry = product_category_modifier_info
                        .modifiers
                        .iter()
                        .find(|&x| x.category.0.as_ref().map_or(false, |x| x.guid == guid));
                    (
                        guid,
                        ProductCategory {
                            name: product_category.name,
                            price_modifier: entry.map_or(
                                product_category_modifier_info.default_price_modifier,
                                |x| x.price_modifier,
                            ),
                            growth_modifier: entry.map_or(
                                product_category_modifier_info.default_growth_modifier,
                                |x| x.growth_modifier,
                            ),
                        },
                    )
                })
                .collect(),
        }
    }
}
