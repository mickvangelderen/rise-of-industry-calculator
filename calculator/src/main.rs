#![allow(unused)]

use std::{collections::HashMap, rc::Rc};

use rise_of_industry_calculator::{BuildingId, GameData, ModuleId, Product, ProductId, RecipeId};

// Assets\Scripts\Assembly-CSharp\ProjectAutomata\Upkeep.cs
// This variable is called `buildingCostPercentage` and comes from the prefab files.
// While it varies for some logistic buildings, it's the same for everything else so I'll just hard code it.
/// The ratio of upkeep to base cost.
const BASE_COST_TO_UPKEEP: f64 = 0.025;

// Assets\Scripts\Assembly-CSharp\ProjectAutomata\BuildingEfficiency.cs
#[derive(Default)]
pub enum BuildingEfficiency {
    P025,
    P050,
    P075,
    #[default]
    P100,
    P125,
    P150,
    P200,
}

impl BuildingEfficiency {
    pub fn production(&self) -> f64 {
        match self {
            BuildingEfficiency::P025 => 0.25,
            BuildingEfficiency::P050 => 0.50,
            BuildingEfficiency::P075 => 0.75,
            BuildingEfficiency::P100 => 1.00,
            BuildingEfficiency::P125 => 1.25,
            BuildingEfficiency::P150 => 1.50,
            BuildingEfficiency::P200 => 2.00,
        }
    }

    // See Assets\Scripts\Assembly-CSharp\ProjectAutomata\ContentCreationModels\CCCBuildingEfficiencyModel.cs.
    pub fn upkeep(&self) -> f64 {
        self.production()
    }
}

pub struct BuildingInstance {
    pub id: BuildingId,
    pub modules: Option<(i64, ModuleId)>,
    pub recipe_id: RecipeId,
    pub efficiency: BuildingEfficiency,
}

impl BuildingInstance {
    pub fn upkeep_per_month(&self, data: &GameData) -> f64 {
        self.efficiency.upkeep() * self.purchase_price(data) as f64 * BASE_COST_TO_UPKEEP
    }

    pub fn purchase_price(&self, data: &GameData) -> f64 {
        data[self.id].base_cost as f64
            + self
                .modules
                .as_ref()
                .map_or(0, |&(count, module_id)| count * data[module_id].base_cost)
                as f64
    }

    pub fn productivity(&self) -> f64 {
        self.efficiency.production() * self.modules.as_ref().map_or(1, |&(count, _)| count) as f64
    }

    pub fn production_per_day_of(&self, data: &GameData, product_id: ProductId) -> Option<f64> {
        let recipe = &data[self.recipe_id];
        recipe
            .products(data)
            .find(|x| x.product.id == product_id)
            .map(|x| self.productivity() * x.amount as f64 / recipe.easy_chains_days())
    }
}

pub struct TransportKind {
    pub name: String,
    pub base_price: i64,
    pub tile_price: i64,
}

pub struct Transport {
    pub kind: Rc<TransportKind>,
    pub description: String,
    pub tiles: i64,
    pub amount_per_day: f64,
}

trait ExactlyOne: Iterator + Sized {
    fn exactly_one(self) -> Option<Self::Item>;
}

impl<T> ExactlyOne for T
where
    T: Iterator,
{
    fn exactly_one(mut self) -> Option<Self::Item> {
        match (self.next(), self.next()) {
            (Some(first), None) => Some(first),
            _ => None,
        }
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let data = &GameData::load(std::path::Path::new("data.json")).unwrap();

    let plantation = data.buildings().find(|&x| x.name == "PLANTATION").unwrap();
    let cocoa = data.products().find(|&x| x.name == "Cocoa").unwrap();
    let cocoa_recipe = plantation
        .available_recipes(data)
        .find(|&recipe| recipe.name == "Cocoas")
        .unwrap();
    let cocoa_field = cocoa_recipe.required_modules(data).exactly_one().unwrap();

    let water_siphon = data
        .buildings()
        .find(|&x| x.name == "WATER SIPHON")
        .unwrap();
    let water = data.products().find(|&x| x.name == "Water").unwrap();
    let water_recipe = water_siphon
        .available_recipes(data)
        .filter(|&recipe| {
            recipe
                .products(data)
                .any(|x| x.product.id == water.id && x.amount > 0)
        })
        .exactly_one()
        .unwrap();
    let water_well_harvester = water_recipe.required_modules(data).exactly_one().unwrap();

    let water_harvesters = (
        1i64,
        BuildingInstance {
            id: water_siphon.id,
            recipe_id: water_recipe.id,
            modules: Some((5, water_well_harvester.id)),
            efficiency: Default::default(),
        },
    );

    let cocoa_plantations: (i64, BuildingInstance) = (
        2i64,
        BuildingInstance {
            id: plantation.id,
            recipe_id: cocoa_recipe.id,
            modules: Some((5, cocoa_field.id)),
            efficiency: Default::default(),
        },
    );

    let truck = Rc::new(TransportKind {
        name: "Truck".to_string(),
        base_price: 250,
        tile_price: 10,
    });

    let transports = vec![
        Transport {
            kind: Rc::clone(&truck),
            description: "Water to Cocoa Plantations".to_string(),
            tiles: 10,
            amount_per_day: water_harvesters.0 as f64
                * water_harvesters
                    .1
                    .production_per_day_of(data, water.id)
                    .unwrap(),
        },
        Transport {
            kind: Rc::clone(&truck),
            description: "Cocoa Plantations to Farmers Market".to_string(),
            tiles: 100,
            amount_per_day: cocoa_plantations.0 as f64
                * cocoa_plantations
                    .1
                    .production_per_day_of(data, cocoa.id)
                    .unwrap(),
        },
    ];

    let building_groups = vec![water_harvesters, cocoa_plantations];

    simulate(data, building_groups, cocoa, transports);
}

fn simulate(
    data: &GameData,
    building_groups: Vec<(i64, BuildingInstance)>,
    cocoa: &Product,
    transports: Vec<Transport>,
) {
    let mut monthly_operational_costs = building_groups
        .iter()
        .map(|&(count, ref instance)| count as f64 * instance.upkeep_per_month(data))
        .sum::<f64>();

    let initial_costs = building_groups
        .iter()
        .map(|&(count, ref instance)| count as f64 * instance.purchase_price(data))
        .sum::<f64>();

    let mut production_map: HashMap<ProductId, f64> = HashMap::new();
    for &(count, ref instance) in &building_groups {
        let recipe = &data[instance.recipe_id];
        for ingredient in recipe.products(data) {
            *production_map.entry(ingredient.product.id).or_default() +=
                count as f64 * instance.productivity() * ingredient.amount as f64
                    / recipe.easy_chains_days();
        }
    }

    let monthly_revenue = production_map[&cocoa.id] * 30.0 * 12661.0;

    println!("production per month:");
    for (&product_id, &amount_per_day) in &production_map {
        println!(
            "| {:20} | {:7.1} units |",
            &data[product_id].name,
            amount_per_day * 30.0
        );
    }
    println!();

    println!("transportation costs per month:");
    for transport in &transports {
        let price_per_month = transport.amount_per_day
            * 30.0
            * (transport.kind.base_price + transport.tiles * transport.kind.tile_price) as f64;
        monthly_operational_costs += price_per_month;
        println!(
            "| {kind:7} | {amount:7.1} deliveries | {tiles:7} tiles | {total:7.1}k | {description:40} |",
            kind = &transport.kind.name,
            amount = transport.amount_per_day * 30.0,
            tiles = transport.tiles,
            total = price_per_month / 1000.0,
            description = transport.description,
        )
    }
    println!();

    let monthly_profit = monthly_revenue - monthly_operational_costs;

    let roi = monthly_profit / initial_costs - (0.275 / 120.0);

    println!(
        "operational costs:    {:9.0}k",
        monthly_operational_costs / 1000.0
    );
    println!("revenue:              {:9.0}k", monthly_revenue / 1000.0);
    println!("profit:               {:9.0}k", monthly_profit / 1000.0);
    println!("initial costs:        {:9.0}k", initial_costs / 1000.0);
    println!("return on investment: {:9.2}%", roi * 100.0);
}
