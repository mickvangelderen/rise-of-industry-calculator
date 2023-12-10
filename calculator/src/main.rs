#![allow(unused)]

use std::{collections::HashMap, rc::Rc};

use rise_of_industry_calculator::{
    serialization::ProductPriceFormula, BuildingIndex, GameData, ModuleIndex, ProductIndex,
    ProductVec, Query, RecipeIndex, RecipeVec,
};

// Assets\Scripts\Assembly-CSharp\ProjectAutomata\Upkeep.cs
// This variable is called `buildingCostPercentage` and comes from the prefab files.
// While it varies for some logistic buildings, it's the same for everything else so I'll just hard code it.
/// The ratio of upkeep to base cost.
const BASE_COST_TO_UPKEEP: f64 = 0.025;

fn get_upkeep_price_component(recipe: Query<'_, RecipeIndex>) -> f64 {
    let processor = recipe
        .data()
        .buildings()
        .filter(|building| building.available_recipes().any(|x| x == recipe))
        .exactly_one()
        .unwrap();
    let upkeep = processor.base_cost() as f64 * BASE_COST_TO_UPKEEP;
    let module = recipe.required_module();
    let module_upkeep = module
        .map(|module| module.base_cost() as f64 * BASE_COST_TO_UPKEEP)
        .unwrap_or(0.0);
    upkeep + module_upkeep * 3.0
}

fn compute_product_prices(data: &GameData) -> ProductVec<f64> {
    // Counts how many recipes still need to be evaluated for a product.
    let mut product_recipe_counts: ProductVec<usize> = data
        .products()
        .map(|product| product.producing_recipes().count())
        .collect();
    let mut product_values: ProductVec<Option<f64>> = data.products().map(|_| None).collect();

    let mut recipe_upkeep: RecipeVec<f64> = data
        .recipes()
        .map(|recipe| get_upkeep_price_component(recipe))
        .collect();

    let mut todo_recipes: Vec<RecipeIndex> = data.recipe.indices().collect();
    let mut temp_recipes = vec![];
    while !todo_recipes.is_empty() {
        std::mem::swap(&mut todo_recipes, &mut temp_recipes);
        for recipe in temp_recipes.drain(..) {
            let recipe = data.query(recipe);

            let Some(ingredients_value) = recipe.inputs().try_fold(0.0, |sum, input| {
                if product_recipe_counts[input.product_index] > 0 {
                    return None;
                }
                let price = product_values[input.product_index].unwrap();
                Some(sum + price * -input.amount as f64)
            }) else {
                todo_recipes.push(recipe.index());
                continue;
            };

            let upkeep = recipe_upkeep[recipe.index()];

            let recipe_output = recipe.outputs().map(|entry| entry.amount).sum::<i64>() as f64;
            let recipe_days = recipe.days() as f64;

            for entry in recipe.outputs() {
                let value = match entry.product().price_formula() {
                    ProductPriceFormula::Factories => {
                        (ingredients_value + ((upkeep / 30.0) * recipe_days)) / recipe_output
                    }
                    ProductPriceFormula::FarmProduce => ingredients_value * 2.8,
                    ProductPriceFormula::Farms => {
                        ((ingredients_value * 3.0) + ((upkeep / 30.0) * recipe_days))
                            / (recipe_output * 3.0)
                    }
                    ProductPriceFormula::Gatherers => {
                        upkeep / ((3.0 * recipe_output) * (30.0 / recipe_days))
                    }
                    ProductPriceFormula::Livestock => {
                        ((((ingredients_value * 3.0) + ((upkeep / 30.0) * recipe_days))
                            / (recipe_output * 3.0))
                            * (recipe_output - entry.amount as f64))
                            / recipe_output
                    }
                    ProductPriceFormula::RawResources => 75.0 * recipe_days * 3.25,
                };

                product_recipe_counts[entry.product_index] -= 1;
                product_values[entry.product_index] =
                    Some(match product_values[entry.product_index] {
                        Some(existing) => f64::min(existing, value),
                        None => value,
                    });
            }
        }
    }

    data.products()
        .map(|product| {
            product_values[product.index()].unwrap() * product.category().price_modifier()
        })
        .collect()
}

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
    pub id: BuildingIndex,
    pub modules: Option<(i64, ModuleIndex)>,
    pub recipe_index: RecipeIndex,
    pub efficiency: BuildingEfficiency,
}

impl BuildingInstance {
    pub fn upkeep_per_month(&self, data: &GameData) -> f64 {
        self.efficiency.upkeep() * self.purchase_price(data) * BASE_COST_TO_UPKEEP
    }

    pub fn purchase_price(&self, data: &GameData) -> f64 {
        data.query(self.id).base_cost() as f64
            + self.modules.as_ref().map_or(0, |&(count, module_index)| {
                count * data.query(module_index).base_cost()
            }) as f64
    }

    pub fn productivity(&self) -> f64 {
        self.efficiency.production() * self.modules.as_ref().map_or(1, |&(count, _)| count) as f64
    }

    pub fn production_per_day_of(
        &self,
        data: &GameData,
        product_index: ProductIndex,
    ) -> Option<f64> {
        let recipe = data.query(self.recipe_index);
        recipe
            .entries()
            .find(|x| x.product_index == product_index)
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

struct Context<'data> {
    data: &'data GameData,

    cocoa: ProductIndex,
    water: ProductIndex,
    cotton: ProductIndex,
    fibers: ProductIndex,
    napkins: ProductIndex,
    berries: ProductIndex,
    light_fabric: ProductIndex,

    farm: BuildingIndex,
    plantation: BuildingIndex,
    water_siphon: BuildingIndex,
    water_well: BuildingIndex,

    cocoa_recipe: RecipeIndex,
    cotton_recipe: RecipeIndex,
    fibers_recipe: RecipeIndex,
    napkins_recipe: RecipeIndex,
    berry_recipe: RecipeIndex,
    water_well_water_recipe: RecipeIndex,
    water_siphon_water_recipe: RecipeIndex,

    cocoa_field: ModuleIndex,
    cotton_field: ModuleIndex,
    berry_field: ModuleIndex,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let data = &GameData::load(std::path::Path::new("data.json")).unwrap();

    let cocoa = data.product_by_name("Cocoa");
    let water = data.product_by_name("Water");
    let cotton = data.product_by_name("Cotton");
    let fibers = data.product_by_name("Fibers");
    let light_fabric = data.product_by_name("Light Fabric");
    let napkins = data.product_by_name("Napkins");
    let berries = data.product_by_name("Berries");
    let dye = data.product_by_name("Dye");

    let plantation = data.building_by_name("PLANTATION");
    let farm = data.building_by_name("CROP FARM");
    let water_siphon = data.building_by_name("WATER SIPHON");
    let water_well = data.building_by_name("WATER WELL");
    let textile_factory = data.building_by_name("TEXTILE FACTORY");

    let cocoa_recipe = data.recipe_by_name("Cocoas");
    let cotton_recipe = data.recipe_by_name("Cotton");
    let fibers_recipe = data.recipe_by_name("Fibers");
    let light_fabric_recipe = data.recipe_by_name("Light Fabric");
    let napkins_recipe = data.recipe_by_name("Napkins");
    let berry_recipe = data.recipe_by_name("Berries");
    let dye_recipe = data.recipe_by_name("Dye");

    let water_well_water_recipe = water_well.recipe_by_name("Water");
    let water_siphon_water_recipe = water_siphon.recipe_by_name("Water");

    let cocoa_field = cocoa_recipe.required_module().unwrap();
    let cotton_field = cotton_recipe.required_module().unwrap();
    let berry_field = berry_recipe.required_module().unwrap();

    let context = Context {
        data,

        cocoa: *cocoa,
        water: *water,
        cotton: *cotton,
        fibers: *fibers,
        napkins: *napkins,
        berries: *berries,
        light_fabric: *light_fabric,

        farm: *farm,
        plantation: *plantation,
        water_siphon: *water_siphon,
        water_well: *water_well,

        cocoa_recipe: *cocoa_recipe,
        cotton_recipe: *cotton_recipe,
        fibers_recipe: *fibers_recipe,
        napkins_recipe: *napkins_recipe,
        berry_recipe: *berry_recipe,
        water_well_water_recipe: *water_well_water_recipe,
        water_siphon_water_recipe: *water_siphon_water_recipe,

        cocoa_field: *cocoa_field,
        cotton_field: *cotton_field,
        berry_field: *berry_field,
    };

    let napkin_factories = (
        3i64,
        BuildingInstance {
            id: *textile_factory,
            recipe_index: *napkins_recipe,
            modules: None,
            efficiency: Default::default(),
        },
    );

    let light_fabric_factories = (
        3i64,
        BuildingInstance {
            id: *textile_factory,
            recipe_index: *light_fabric_recipe,
            modules: None,
            efficiency: Default::default(),
        },
    );

    // -2 cotton/+2 fiber per 15 days per factory
    let fibers_factories = (
        2i64,
        BuildingInstance {
            id: *textile_factory,
            recipe_index: *fibers_recipe,
            modules: None,
            efficiency: Default::default(),
        },
    );

    // -1 water/2 cotton per 30 days per field
    let cotton_farms = (
        1,
        BuildingInstance {
            id: *farm,
            recipe_index: *cotton_recipe,
            modules: Some((5, *cotton_field)),
            efficiency: Default::default(),
        },
    );

    // -2 berries/-1 water/+2 dye per 15 days per factory
    let dye_factories = (
        1i64,
        BuildingInstance {
            id: *textile_factory,
            recipe_index: *dye_recipe,
            modules: None,
            efficiency: Default::default(),
        },
    );

    // -1 water/2 cotton per 30 days per field
    let berry_farms = (
        1,
        BuildingInstance {
            id: *farm,
            recipe_index: *berry_recipe,
            modules: Some((5, *berry_field)),
            efficiency: Default::default(),
        },
    );

    let water_siphons = (
        2,
        BuildingInstance {
            id: *water_siphon,
            recipe_index: *water_siphon_water_recipe,
            modules: Some((3, *water_siphon_water_recipe.required_module().unwrap())),
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
            description: "Local Transport".to_string(),
            tiles: 15,
            amount_per_day: water_siphons.0 as f64
                * water_siphons.1.production_per_day_of(data, *water).unwrap()
                + berry_farms.0 as f64
                    * berry_farms.1.production_per_day_of(data, *berries).unwrap()
                + dye_factories.0 as f64
                    * dye_factories.1.production_per_day_of(data, *dye).unwrap()
                + cotton_farms.0 as f64
                    * cotton_farms.1.production_per_day_of(data, *cotton).unwrap(),
        },
        Transport {
            kind: Rc::clone(&truck),
            description: "Sale Transport".to_string(),
            tiles: 200,
            amount_per_day: light_fabric_factories.0 as f64
                * light_fabric_factories
                    .1
                    .production_per_day_of(data, *light_fabric)
                    .unwrap()
                + napkin_factories.0 as f64
                    * napkin_factories
                        .1
                        .production_per_day_of(data, *napkins)
                        .unwrap()
                + fibers_factories.0 as f64
                    * fibers_factories
                        .1
                        .production_per_day_of(data, *fibers)
                        .unwrap(),
        },
    ];

    let building_groups = vec![
        napkin_factories,
        light_fabric_factories,
        fibers_factories,
        cotton_farms,
        dye_factories,
        berry_farms,
        water_siphons,
    ];

    // let water_harvesters = (
    //     1i64,
    //     BuildingInstance {
    //         id: water_siphon.id,
    //         recipe_index: water_siphon_water_recipe.id,
    //         modules: Some((5, data.recipe_module(water_siphon_water_recipe).id)),
    //         efficiency: Default::default(),
    //     },
    // );

    // let cocoa_plantations: (i64, BuildingInstance) = (
    //     2i64,
    //     BuildingInstance {
    //         id: plantation.id,
    //         recipe_index: cocoa_recipe.id,
    //         modules: Some((5, cocoa_field.id)),
    //         efficiency: Default::default(),
    //     },
    // );

    // let building_groups = vec![water_harvesters, cocoa_plantations];

    // let truck = Rc::new(TransportKind {
    //     name: "Truck".to_string(),
    //     base_price: 250,
    //     tile_price: 10,
    // });

    // let transports = vec![
    //     Transport {
    //         kind: Rc::clone(&truck),
    //         description: "Water to Cocoa Plantations".to_string(),
    //         tiles: 10,
    //         amount_per_day: water_harvesters.0 as f64
    //             * water_harvesters
    //                 .1
    //                 .production_per_day_of(data, water.id)
    //                 .unwrap(),
    //     },
    //     Transport {
    //         kind: Rc::clone(&truck),
    //         description: "Cocoa Plantations to Farmers Market".to_string(),
    //         tiles: 100,
    //         amount_per_day: cocoa_plantations.0 as f64
    //             * cocoa_plantations
    //                 .1
    //                 .production_per_day_of(data, cocoa.id)
    //                 .unwrap(),
    //     },
    // ];

    simulate(data, &context, building_groups, transports);
}

fn simulate(
    data: &GameData,
    context: &Context<'_>,
    building_groups: Vec<(i64, BuildingInstance)>,
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

    let mut production_map: HashMap<ProductIndex, f64> = HashMap::new();
    for &(count, ref instance) in &building_groups {
        let recipe = data.query(instance.recipe_index);
        for ingredient in recipe.entries() {
            *production_map.entry(ingredient.product_index).or_default() +=
                count as f64 * instance.productivity() * ingredient.amount as f64
                    / recipe.easy_chains_days();
        }
    }

    {
        let prices = compute_product_prices(data);
        for product in data.products() {
            println!(
                "  {} ({:?}): {}",
                product.name(),
                *product,
                prices[*product]
            );
        }
    }

    let prices = [
        (context.berries, 12660.0),
        (context.light_fabric, 55650.0),
        (context.cotton, 13560.0),
        (context.fibers, 27220.0),
        (context.napkins, 109830.0),
    ];

    println!("sales per month:");
    let mut monthly_revenue = 0.0;
    for (product_index, price) in prices {
        let units_per_month = production_map
            .get(&product_index)
            .copied()
            .unwrap_or_default()
            * 30.0;
        let total = units_per_month * price;
        println!(
            "  | {name:20} | {units:7.1} units | {price:7.1}k $/unit | {total:7.1}k $ |",
            name = data.query(product_index).name(),
            units = units_per_month,
            price = price / 1000.0,
            total = total / 1000.0
        );
        monthly_revenue += total;
    }
    println!();

    println!("production per month:");
    for (&product_index, &amount_per_day) in &production_map {
        println!(
            "  | {:20} | {:7.1} units |",
            data.query(product_index).name(),
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
            "  | {kind:7} | {amount:7.1} deliveries | {tiles:7} tiles | {total:7.1}k | {description:40} |",
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
