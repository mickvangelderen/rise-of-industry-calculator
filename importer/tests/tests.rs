use rise_of_industry_importer::{Document, KnownDocument, KnownMonoBehaviour, MonoBehaviour};
use serde::Deserialize;

#[test]
fn test() {
    let contents =
        rise_of_industry_importer::rewrite_yaml_tags(include_str!("CoalGatherer.prefab"));

    let documents = serde_yaml::Deserializer::from_str(&contents)
        .filter_map(|deserializer| {
            let document = rise_of_industry_importer::Document::deserialize(deserializer).unwrap();
            match document {
                Document::Known(KnownDocument::MonoBehaviour(MonoBehaviour::Known(
                    mono_behaviour,
                ))) => Some(mono_behaviour),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    let buildings = documents
        .iter()
        .filter_map(|mono_behaviour| match mono_behaviour {
            KnownMonoBehaviour::Building(building) => Some(building),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(buildings.len(), 1);
    assert_eq!(buildings[0].name, "COAL MINE");
    assert_eq!(buildings[0].base_cost, 225000);

    let gatherer_hubs = documents
        .iter()
        .filter_map(|mono_behaviour| match mono_behaviour {
            KnownMonoBehaviour::GathererHub(gatherer_hub) => Some(gatherer_hub),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(gatherer_hubs.len(), 1);
    assert_eq!(gatherer_hubs[0].available_recipes.len(), 1);
    assert_eq!(
        gatherer_hubs[0].available_recipes[0]
            .0
            .as_ref()
            .map(|x| x.guid.as_str()),
        Some("a32e46a2e1865bc4c906a19d47e6b100")
    );
}
