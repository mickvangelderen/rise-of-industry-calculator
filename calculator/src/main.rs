#![allow(unused)]

use std::{collections::HashMap, rc::Rc};

use rise_of_industry_calculator::{
    serialization::ProductPriceFormula, BuildingEfficiency, BuildingIndex, GameData, ModuleIndex,
    ProductIndex, ProductVec, Query, RecipeIndex, RecipeVec, TransportKind, BASE_COST_TO_UPKEEP,
};

pub(crate) mod iter_ext;
use iter_ext::ExactlyOne;

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

pub struct Transport {
    pub kind: Rc<TransportKind>,
    pub description: String,
    pub tiles: i64,
    pub amount_per_day: f64,
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
            modules: Some((4, *cotton_field)),
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
            modules: Some((2, *berry_field)),
            efficiency: Default::default(),
        },
    );

    let water_siphons = (
        1,
        BuildingInstance {
            id: *water_siphon,
            recipe_index: *water_siphon_water_recipe,
            modules: Some((4, *water_siphon_water_recipe.required_module().unwrap())),
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
            tiles: 20,
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

    simulate(data, &context, building_groups, transports);

    let (building_groups, transports) = auto_build(data, truck, *napkins, 4.0);
    simulate(data, &context, building_groups, transports);
}

fn auto_build(
    data: &GameData,
    truck: Rc<TransportKind>,
    product_index: ProductIndex,
    target_sales_per_month: f64,
) -> (Vec<(i64, BuildingInstance)>, Vec<Transport>) {
    let mut buildings = vec![];
    let mut transports = vec![];

    let mut production_map: ProductVec<f64> = data.products().map(|_| 0.0).collect();
    let mut consumption_map: ProductVec<f64> = data.products().map(|_| 0.0).collect();
    let mut sales_map: ProductVec<f64> = data
        .products()
        .map(|p| {
            if *p == product_index {
                target_sales_per_month / 30.0
            } else {
                0.0
            }
        })
        .collect();

    'outer: loop {
        for product in data.products() {
            let production = production_map[*product];
            let sales = sales_map[*product];

            let mut add_building = |count: i64, instance: BuildingInstance| {
                let building = data.query(instance.id);
                let recipe = data.query(instance.recipe_index);
                for entry in recipe.entries() {
                    let amount_per_day =
                        count as f64 * instance.productivity() * entry.amount as f64
                            / recipe.easy_chains_days();
                    production_map[entry.product_index] += amount_per_day;
                    if amount_per_day < 0.0 {
                        transports.push(Transport {
                            kind: Rc::clone(&truck),
                            description: format!(
                                "{} to {} for {}",
                                entry.product().name(),
                                building.name(),
                                recipe.name()
                            ),
                            tiles: 20,
                            amount_per_day: -amount_per_day,
                        })
                    }
                }
                buildings.push((count, instance));
            };

            if production < sales {
                let recipe = product.producing_recipes().next().unwrap();
                let entry = recipe
                    .outputs()
                    .find(|e| e.product_index == *product)
                    .unwrap();
                let production_per_day = entry.amount as f64 / recipe.easy_chains_days();
                let recipe_count = ((sales - production) / production_per_day).ceil() as i64;

                let building_index = *data
                    .buildings()
                    .filter(|b| b.available_recipes().any(|r| *r == *recipe))
                    .exactly_one()
                    .unwrap();
                if let Some(module) = recipe.required_module() {
                    if recipe_count / 5 > 0 {
                        add_building(
                            recipe_count / 5,
                            BuildingInstance {
                                id: building_index,
                                modules: Some((5, *module)),
                                recipe_index: *recipe,
                                efficiency: Default::default(),
                            },
                        );
                    }

                    if recipe_count % 5 != 0 {
                        add_building(
                            1,
                            BuildingInstance {
                                id: building_index,
                                modules: Some((recipe_count % 5, *module)),
                                recipe_index: *recipe,
                                efficiency: Default::default(),
                            },
                        );
                    }
                } else {
                    add_building(
                        recipe_count,
                        BuildingInstance {
                            id: building_index,
                            modules: None,
                            recipe_index: *recipe,
                            efficiency: Default::default(),
                        },
                    );
                }

                continue 'outer;
            }
        }

        for product in data.products() {
            let sales = production_map[*product];
            if sales > 0.0 {
                transports.push(Transport {
                    kind: Rc::clone(&truck),
                    description: format!("sales of {}", product.name()),
                    tiles: 200,
                    amount_per_day: sales,
                });
            }
        }
        break;
    }

    (buildings, transports)
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

    println!("setup:");
    for (count, building) in &building_groups {
        println!(
            "  | {count:4}x | {:20} | {:4} | {:20} |",
            data.query(building.id).name(),
            building.modules.as_ref().map_or(1, |&(count, _)| count),
            data.query(building.recipe_index).name(),
        );
    }
    println!();

    println!("sales per month:");
    let mut monthly_revenue = 0.0;
    for (&product_index, &production_per_day) in &production_map {
        let units_per_month = production_per_day * 30.0;
        let price = data.query(product_index).price() * 1.5;
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
