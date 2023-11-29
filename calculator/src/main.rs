#![allow(unused)]

use std::{collections::HashMap, rc::Rc};

// Assets\Scripts\Assembly-CSharp\ProjectAutomata\Upkeep.cs
// This variable is called `buildingCostPercentage` and comes from the prefab files.
// While it varies for some logistic buildings, it's the same for everything else so I'll just hard code it.
/// The ratio of upkeep to base cost.
const BASE_COST_TO_UPKEEP: f64 = 0.025;

type Currency = f64;

// Assets\Scripts\Assembly-CSharp\ProjectAutomata\BuildingEfficiency.cs
enum BuildingEfficiency {
    P025,
    P050,
    P075,
    P100,
    P125,
    P150,
    P200,
}

impl BuildingEfficiency {
    fn production(&self) -> f64 {
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
    fn upkeep(&self) -> f64 {
        self.production()
    }
}

impl Default for BuildingEfficiency {
    fn default() -> Self {
        BuildingEfficiency::P100
    }
}

struct ModuleKind {
    name: String,
    base_cost: Currency,
}

struct BuildingKind {
    name: String,
    base_cost: Currency,
    modules: Vec<Rc<ModuleKind>>,
}

#[derive(Hash, Eq, PartialEq)]
struct ProductKind {
    name: String,
}

struct Ingredient {
    kind: Rc<ProductKind>,
    amount_per_day: f64,
}

struct BuildingInstance {
    kind: Rc<BuildingKind>,
    modules: Option<(u64, Rc<ModuleKind>)>,
    recipe: Vec<Ingredient>,
    efficiency: BuildingEfficiency,
}

impl BuildingInstance {
    fn upkeep_per_month(&self) -> Currency {
        self.efficiency.upkeep() * self.purchase_price() * BASE_COST_TO_UPKEEP
    }

    fn purchase_price(&self) -> Currency {
        self.kind.base_cost
            + self
                .modules
                .as_ref()
                .map_or(0.0, |&(count, ref kind)| count as f64 * kind.base_cost)
    }

    fn productivity(&self) -> f64 {
        self.efficiency.production() * self.modules.as_ref().map_or(1, |&(count, _)| count) as f64
    }

    fn production_per_day_of(&self, product: &Rc<ProductKind>) -> Option<f64> {
        self.recipe
            .iter()
            .find(|&x| &x.kind == product)
            .map(|ingredient| self.productivity() * ingredient.amount_per_day)
    }
}

struct TransportKind {
    name: String,
    base_price: Currency,
    tile_price: Currency,
}

struct Transport {
    kind: Rc<TransportKind>,
    description: String,
    tiles: i64,
    amount_per_day: f64,
}

fn main() {
    let plantation_kind = Rc::new(BuildingKind {
        name: "Plantation".to_string(),
        base_cost: 175000.0,
        modules: vec![], // TODO
    });

    let cocoa_field_kind = Rc::new(ModuleKind {
        name: "Cocoa Field".to_string(),
        base_cost: 125000.0,
    });

    let water_harvester_kind = Rc::new(BuildingKind {
        name: "Water Harvester".to_string(),
        base_cost: 150000.0,
        modules: vec![], // TODO
    });

    let water_harvester_silo_kind = Rc::new(ModuleKind {
        name: "Silo".to_string(),
        base_cost: 100000.0,
    });

    let water = Rc::new(ProductKind {
        name: "Water".to_string(),
    });

    let cocoa = Rc::new(ProductKind {
        name: "Cocoa".to_string(),
    });

    let water_harvesters = (
        1i64,
        BuildingInstance {
            kind: Rc::clone(&water_harvester_kind),
            recipe: vec![Ingredient {
                kind: Rc::clone(&water),
                amount_per_day: 1.0 / 15.0,
            }],
            modules: Some((5, Rc::clone(&water_harvester_silo_kind))),
            efficiency: Default::default(),
        },
    );

    let cocoa_plantations = (
        2i64,
        BuildingInstance {
            kind: Rc::clone(&plantation_kind),
            recipe: vec![
                Ingredient {
                    kind: Rc::clone(&water),
                    amount_per_day: -1.0 / 30.0,
                },
                Ingredient {
                    kind: Rc::clone(&cocoa),
                    amount_per_day: 2.0 / 30.0,
                },
            ],
            modules: Some((5, Rc::clone(&cocoa_field_kind))),
            efficiency: Default::default(),
        },
    );

    let truck = Rc::new(TransportKind {
        name: "Truck".to_string(),
        base_price: 250.0,
        tile_price: 10.0,
    });

    let transports = vec![
        Transport {
            kind: Rc::clone(&truck),
            description: "Water to Cocoa Plantations".to_string(),
            tiles: 10,
            amount_per_day: water_harvesters.0 as f64
                * water_harvesters.1.production_per_day_of(&water).unwrap(),
        },
        Transport {
            kind: Rc::clone(&truck),
            description: "Cocoa Plantations to Farmers Market".to_string(),
            tiles: 100,
            amount_per_day: cocoa_plantations.0 as f64
                * cocoa_plantations.1.production_per_day_of(&cocoa).unwrap(),
        },
    ];

    let building_groups = vec![water_harvesters, cocoa_plantations];

    let mut monthly_operational_costs = building_groups
        .iter()
        .map(|&(count, ref instance)| count as f64 * instance.upkeep_per_month())
        .sum::<Currency>() as f64;

    let initial_costs = building_groups
        .iter()
        .map(|&(count, ref instance)| count as f64 * instance.purchase_price())
        .sum::<Currency>() as f64;

    let mut production_map: HashMap<Rc<ProductKind>, f64> = HashMap::new();
    for &(count, ref instance) in &building_groups {
        for ingredient in &instance.recipe {
            *production_map.entry(ingredient.kind.clone()).or_default() +=
                count as f64 * instance.productivity() * ingredient.amount_per_day;
        }
    }

    let monthly_revenue = production_map[&cocoa] * 30.0 * 12661.0;

    println!("production:");
    for (ingredient_kind, &amount_per_day) in &production_map {
        println!(
            "| {:20} | {:7.1} per month |",
            ingredient_kind.name,
            amount_per_day * 30.0
        );
    }
    println!();

    println!("transportation:");
    for transport in &transports {
        let total_per_month = transport.amount_per_day
            * 30.0
            * (transport.kind.base_price + transport.tiles as f64 * transport.kind.tile_price)
                as f64;
        monthly_operational_costs += total_per_month;
        println!(
            "| {kind:7} | {amount:7.1} | {tiles:7} | {total:7}k per month | {description:40} |",
            kind = &transport.kind.name,
            amount = transport.amount_per_day * 30.0,
            tiles = transport.tiles,
            total = total_per_month / 1000.0,
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
