#![allow(unused)]

use std::{collections::HashMap, rc::Rc};

use rise_of_industry_calculator::{
    Building, BuildingId, GameData, Module, ModuleId, Product, ProductId, Query, Recipe, RecipeId,
};

// Assets\Scripts\Assembly-CSharp\ProjectAutomata\Upkeep.cs
// This variable is called `buildingCostPercentage` and comes from the prefab files.
// While it varies for some logistic buildings, it's the same for everything else so I'll just hard code it.
/// The ratio of upkeep to base cost.
const BASE_COST_TO_UPKEEP: f64 = 0.025;

/*
ExportedProject\Assets\Resources\gamedata\formulas\product price\Factories.asset:
  16:   formula: (ingredientsValue + ((upkeep / 30) * recipeDays)) / recipeOutput

ExportedProject\Assets\Resources\gamedata\formulas\product price\FarmProduce.asset:
  16:   formula: ingredientsValue * 2.8

ExportedProject\Assets\Resources\gamedata\formulas\product price\Farms.asset:
  16:   formula: ((ingredientsValue * 3) + ((upkeep / 30) * recipeDays)) / (recipeOutput * 3)

ExportedProject\Assets\Resources\gamedata\formulas\product price\Gatherers.asset:
  16:   formula: upkeep / ((3 * recipeOutput) * (30 / recipeDays))

ExportedProject\Assets\Resources\gamedata\formulas\product price\ProductPriceUpkeepComponent.asset:
  16:   formula: hubUpkeep + (moduleUpkeep * 3)

ExportedProject\Assets\Resources\gamedata\formulas\product price\RawResources.asset:
  16:   formula: 75 * recipeDays * 3.25
*/

// private void InizializeRecipePricingInfo(Recipe recipe)
// {
//     float upkeepPriceComponent = GetUpkeepPriceComponent(recipe);
//     float ingredientsValue = ComputeIngredientsValue(recipe);
//     int num = 0;
//     foreach (Product entry in recipe.result.entries)
//     {
//         num += entry.amount;
//     }
//     foreach (Product entry2 in recipe.result.entries)
//     {
//         ComputeProductPrice(entry2.definition, upkeepPriceComponent, ingredientsValue, entry2.amount, num, recipe.gameDaysForPriceCalculation);
//     }
// }

type RecipeQuery<'a> = Query<'a, &'a Recipe>;

fn initialize_recipe_pricing_info(
    data: &GameData,
    recipe: &Recipe,
    prices: &mut HashMap<ProductId, f64>,
) {
    let recipe = data.query(recipe.id);
    let upkeep_price_component = get_upkeep_price_component(data, &*recipe);
    let ingredients_value = compute_ingredients_value(data, &*recipe, prices);
    let recipe_output_amount = recipe.outputs().map(|x| x.amount).sum::<i64>();
    for input in recipe.inputs() {
        compute_product_price(
            data,
            upkeep_price_component,
            ingredients_value,
            input.amount,
            recipe_output_amount,
            recipe.days,
            prices,
        );
    }
}

// ExportedProject\Assets\Resources\gamedata\formulas\product price\ProductPriceUpkeepComponent.asset:
//   16:   formula: hubUpkeep + (moduleUpkeep * 3)
// private float GetUpkeepPriceComponent(Recipe recipe)
// {
//     if (!upkeepComponentFormula)
//     {
//         return 0f;
//     }
//     ReadOnlyList<Building> originsOfRecipe = RecipeDatabase.instance.GetOriginsOfRecipe(recipe);
//     if (!originsOfRecipe.notNull || originsOfRecipe.count <= 0)
//     {
//         Debug.LogError("Cannot find origin of recipe '" + recipe.Title + "'!");
//         return 0f;
//     }
//     Upkeep component = originsOfRecipe[0].GetComponent<Upkeep>();
//     if (!component)
//     {
//         return 0f;
//     }
//     float monthlyUpkeep = component.GetMonthlyUpkeep(Upkeep.GetMonthlyUpkeepOptions.PURE);
//     Building building = recipe.requiredModules.FirstOrDefault();
//     float? obj;
//     if ((object)building == null)
//     {
//         obj = null;
//     }
//     else
//     {
//         Module component2 = building.GetComponent<Module>();
//         obj = (((object)component2 != null) ? new float?(component2.ModuleUpkeep()) : null);
//     }
//     float moduleUpkeep = obj ?? 0f;
//     ProductPriceUpkeepComponentFormulaArguments argumentsProvider = new ProductPriceUpkeepComponentFormulaArguments(monthlyUpkeep, moduleUpkeep);
//     return (float)upkeepComponentFormula.Evaluate(argumentsProvider);
// }

fn get_upkeep_price_component(data: &GameData, recipe: &Recipe) -> f64 {
    let processor = data
        .buildings()
        .filter(|building| building.available_recipes().any(|x| x.id == recipe.id))
        .exactly_one()
        .unwrap();
    let upkeep = processor.base_cost as f64 * BASE_COST_TO_UPKEEP;
    let module = data.query(recipe.id).required_module();
    let module_upkeep = module
        .map(|module| module.base_cost as f64 * BASE_COST_TO_UPKEEP)
        .unwrap_or(0.0);
    upkeep + module_upkeep * 3.0
}

// private float ComputeIngredientsValue(Recipe recipe)
// {
//     float num = 0f;
//     foreach (Product entry in recipe.ingredients.entries)
//     {
//         if (!entry.definition)
//         {
//             Debug.LogErrorFormat("Null ingredient in recipe '{0}'.", recipe.name);
//             continue;
//         }
//         ProductPricingInfo value;
//         if (!_pricingInfoByProduct.TryGetValue(entry.definition, out value))
//         {
//             Recipe productRecipe = GetProductRecipe(entry.definition);
//             if (!productRecipe)
//             {
//                 Debug.LogErrorFormat("No recipe for product {0}.", entry.definition.productName);
//                 continue;
//             }
//             InizializeRecipePricingInfo(productRecipe);
//             if (!_pricingInfoByProduct.TryGetValue(entry.definition, out value))
//             {
//                 Debug.LogErrorFormat("Could not compute price for product {0}.", entry.definition.productName);
//                 continue;
//             }
//         }
//         num += value.value * (float)entry.amount;
//     }
//     return num;
// }
fn compute_ingredients_value<'d>(
    data: &GameData,
    recipe: &Recipe,
    prices: &mut HashMap<ProductId, f64>,
) -> f64 {
    let mut sum = 0.0;
    for entry in data.query(recipe.id).inputs() {
        let price = match prices.get(&entry.product_id).copied() {
            Some(price) => price,
            None => {
                for recipe in data.recipes_with_output(entry.product_id) {
                    initialize_recipe_pricing_info(data, &*recipe, prices);
                }
                prices[&entry.product_id]
            }
        };
        sum += entry.amount as f64 * price;
    }
    sum
}

// private Recipe GetProductRecipe(ProductDefinition product)
// {
//     ReadOnlyList<Recipe> recipes = RecipeDatabase.instance.GetRecipes(product);
//     if (!recipes.notNull)
//     {
//         return null;
//     }
//     return recipes.FirstOrDefault();
// }

// private ProductPricingInfo ComputeProductPrice(ProductDefinition product, float upkeepComponent, float ingredientsValue, int productOutput, int recipeOutput, float recipeDays)
// {
//     ProductPricingInfo value;
//     if (_pricingInfoByProduct.TryGetValue(product, out value))
//     {
//         return value;
//     }
//     Formula value2;
//     if (!_formulasByProduct.TryGetValue(product, out value2) || value2 == null)
//     {
//         Debug.LogErrorFormat("No price formula for product '{0}'.", product.productName);
//         return new ProductPricingInfo();
//     }
//     ProductPriceFormulaArguments argumentsProvider = new ProductPriceFormulaArguments(upkeepComponent, ingredientsValue, productOutput, recipeOutput, recipeDays);
//     float num = (float)value2.Evaluate(argumentsProvider);
//     value = new ProductPricingInfo
//     {
//         value = num,
//         price = num * (product.productCategory ? product.productCategory.priceMultiplier : 1f)
//     };
//     _pricingInfoByProduct[product] = value;
//     return value;
// }

fn compute_product_price(
    data: &GameData,
    upkeep_component: f64,
    ingredients_value: f64,
    entry_amount: i64,
    recipe_output_amount: i64,
    days: i64,
    prices: &mut HashMap<ProductId, f64>,
) {
    // ExportedProject\Assets\Resources\gamedata\formulas\product price\Factories.asset:
    // 16:   formula: (ingredientsValue + ((upkeep / 30) * recipeDays)) / recipeOutput

    // ExportedProject\Assets\Resources\gamedata\formulas\product price\FarmProduce.asset:
    // 16:   formula: ingredientsValue * 2.8

    // ExportedProject\Assets\Resources\gamedata\formulas\product price\Farms.asset:
    // 16:   formula: ((ingredientsValue * 3) + ((upkeep / 30) * recipeDays)) / (recipeOutput * 3)

    // ExportedProject\Assets\Resources\gamedata\formulas\product price\Gatherers.asset:
    // 16:   formula: upkeep / ((3 * recipeOutput) * (30 / recipeDays))

    // ExportedProject\Assets\Resources\gamedata\formulas\product price\ProductPriceUpkeepComponent.asset:
    // 16:   formula: hubUpkeep + (moduleUpkeep * 3)

    // ExportedProject\Assets\Resources\gamedata\formulas\product price\RawResources.asset:
    // 16:   formula: 75 * recipeDays * 3.25
}

fn compute_prices(data: &GameData) -> HashMap<ProductId, f64> {
    let mut prices: HashMap<ProductId, HashMap<RecipeId, Option<f64>>> = data
        .products()
        .map(|product| {
            (
                product.id,
                data.recipes_with_output(product.id)
                    .map(|recipe| (recipe.id, None))
                    .collect(),
            )
        })
        .collect();

    let mut changed = true;
    while (changed) {
        changed = false;

        for product in data.products() {
            for recipe in data.recipes_with_output(product.id) {
                if prices[&product.id][&recipe.id].is_some() {
                    continue;
                }

                let Some(inputs_price) =
                    data.query(recipe.id)
                        .inputs()
                        .fold(Some(0.0), |sum, input| {
                            let min_price = prices[&input.product_id]
                                .values()
                                .copied()
                                .reduce(|min, price| match (min, price) {
                                    (Some(min), Some(price)) => Some(f64::min(min, price)),
                                    _ => None,
                                })
                                .unwrap_or_default();
                            match (sum, min_price) {
                                (Some(sum), Some(min_price)) => {
                                    Some(sum + min_price * -input.amount as f64)
                                }
                                _ => None,
                            }
                        })
                else {
                    continue;
                };

                for output in data.query(recipe.id).outputs() {
                    *prices
                        .get_mut(&output.product_id)
                        .unwrap()
                        .get_mut(&recipe.id)
                        .unwrap() = Some(inputs_price / output.amount as f64 * 10.0 + 1.0);
                    changed = true;
                }
            }
        }
    }

    prices
        .into_iter()
        .map(|(product_id, recipes)| {
            (
                product_id,
                recipes
                    .values()
                    .map(|price| price.unwrap())
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or_default(),
            )
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
        let recipe = data.query(self.recipe_id);
        recipe
            .entries()
            .find(|x| x.product_id == product_id)
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

struct Context<'a> {
    cocoa: &'a Product,
    water: &'a Product,
    cotton: &'a Product,
    fibers: &'a Product,
    napkins: &'a Product,
    berries: &'a Product,
    light_fabric: &'a Product,

    farm: &'a Building,
    plantation: &'a Building,
    water_siphon: &'a Building,
    water_well: &'a Building,

    cocoa_recipe: &'a Recipe,
    cotton_recipe: &'a Recipe,
    fibers_recipe: &'a Recipe,
    napkins_recipe: &'a Recipe,
    berry_recipe: &'a Recipe,
    water_well_water_recipe: &'a Recipe,
    water_siphon_water_recipe: &'a Recipe,

    cocoa_field: &'a Module,
    cotton_field: &'a Module,
    berry_field: &'a Module,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let data = &GameData::load(std::path::Path::new("data.json")).unwrap();

    let cocoa = data.product("Cocoa");
    let water = data.product("Water");
    let cotton = data.product("Cotton");
    let fibers = data.product("Fibers");
    let light_fabric = data.product("Light Fabric");
    let napkins = data.product("Napkins");
    let berries = data.product("Berries");
    let dye = data.product("Dye");

    let plantation = data.building("PLANTATION");
    let farm = data.building("CROP FARM");
    let water_siphon = data.building("WATER SIPHON");
    let water_well = data.building("WATER WELL");
    let textile_factory = data.building("TEXTILE FACTORY");

    let cocoa_recipe = data.recipe("Cocoas");
    let cotton_recipe = data.recipe("Cotton");
    let fibers_recipe = data.recipe("Fibers");
    let light_fabric_recipe = data.recipe("Light Fabric");
    let napkins_recipe = data.recipe("Napkins");
    let berry_recipe = data.recipe("Berries");
    let dye_recipe = data.recipe("Dye");

    let water_well_water_recipe = water_well.building_recipe("Water");
    let water_siphon_water_recipe = water_siphon.building_recipe("Water");

    let cocoa_field = cocoa_recipe.required_module().unwrap();
    let cotton_field = cotton_recipe.required_module().unwrap();
    let berry_field = berry_recipe.required_module().unwrap();

    let context = Context {
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
            id: textile_factory.id,
            recipe_id: napkins_recipe.id,
            modules: None,
            efficiency: Default::default(),
        },
    );

    let light_fabric_factories = (
        3i64,
        BuildingInstance {
            id: textile_factory.id,
            recipe_id: light_fabric_recipe.id,
            modules: None,
            efficiency: Default::default(),
        },
    );

    // -2 cotton/+2 fiber per 15 days per factory
    let fibers_factories = (
        2i64,
        BuildingInstance {
            id: textile_factory.id,
            recipe_id: fibers_recipe.id,
            modules: None,
            efficiency: Default::default(),
        },
    );

    // -1 water/2 cotton per 30 days per field
    let cotton_farms = (
        1,
        BuildingInstance {
            id: farm.id,
            recipe_id: cotton_recipe.id,
            modules: Some((5, cotton_field.id)),
            efficiency: Default::default(),
        },
    );

    // -2 berries/-1 water/+2 dye per 15 days per factory
    let dye_factories = (
        1i64,
        BuildingInstance {
            id: textile_factory.id,
            recipe_id: dye_recipe.id,
            modules: None,
            efficiency: Default::default(),
        },
    );

    // -1 water/2 cotton per 30 days per field
    let berry_farms = (
        1,
        BuildingInstance {
            id: farm.id,
            recipe_id: berry_recipe.id,
            modules: Some((5, berry_field.id)),
            efficiency: Default::default(),
        },
    );

    let water_siphons = (
        2,
        BuildingInstance {
            id: water_siphon.id,
            recipe_id: water_siphon_water_recipe.id,
            modules: Some((3, water_siphon_water_recipe.required_module().unwrap().id)),
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
                * water_siphons
                    .1
                    .production_per_day_of(data, water.id)
                    .unwrap()
                + berry_farms.0 as f64
                    * berry_farms
                        .1
                        .production_per_day_of(data, berries.id)
                        .unwrap()
                + dye_factories.0 as f64
                    * dye_factories.1.production_per_day_of(data, dye.id).unwrap()
                + cotton_farms.0 as f64
                    * cotton_farms
                        .1
                        .production_per_day_of(data, cotton.id)
                        .unwrap(),
        },
        Transport {
            kind: Rc::clone(&truck),
            description: "Sale Transport".to_string(),
            tiles: 200,
            amount_per_day: light_fabric_factories.0 as f64
                * light_fabric_factories
                    .1
                    .production_per_day_of(data, light_fabric.id)
                    .unwrap()
                + napkin_factories.0 as f64
                    * napkin_factories
                        .1
                        .production_per_day_of(data, napkins.id)
                        .unwrap()
                + fibers_factories.0 as f64
                    * fibers_factories
                        .1
                        .production_per_day_of(data, fibers.id)
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
    //         recipe_id: water_siphon_water_recipe.id,
    //         modules: Some((5, data.recipe_module(water_siphon_water_recipe).id)),
    //         efficiency: Default::default(),
    //     },
    // );

    // let cocoa_plantations: (i64, BuildingInstance) = (
    //     2i64,
    //     BuildingInstance {
    //         id: plantation.id,
    //         recipe_id: cocoa_recipe.id,
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

    let mut production_map: HashMap<ProductId, f64> = HashMap::new();
    for &(count, ref instance) in &building_groups {
        let recipe = data.query(instance.recipe_id);
        for ingredient in recipe.entries() {
            *production_map.entry(ingredient.product_id).or_default() +=
                count as f64 * instance.productivity() * ingredient.amount as f64
                    / recipe.easy_chains_days();
        }
    }

    {
        let prices = compute_prices(data);
        for product in data.products() {
            println!(
                "  {} ({:?}): {}",
                product.name, product.id, prices[&product.id]
            );
        }
    }

    let prices = [
        (context.berries.id, 12660.0),
        (context.light_fabric.id, 55650.0),
        (context.cotton.id, 13560.0),
        (context.fibers.id, 27220.0),
        (context.napkins.id, 109830.0),
    ];

    println!("sales per month:");
    let mut monthly_revenue = 0.0;
    for (product_id, price) in prices {
        let units_per_month = production_map.get(&product_id).copied().unwrap_or_default() * 30.0;
        let total = units_per_month * price;
        println!(
            "  | {name:20} | {units:7.1} units | {price:7.1}k $/unit | {total:7.1}k $ |",
            name = &data[product_id].name,
            units = units_per_month,
            price = price / 1000.0,
            total = total / 1000.0
        );
        monthly_revenue += total;
    }
    println!();

    println!("production per month:");
    for (&product_id, &amount_per_day) in &production_map {
        println!(
            "  | {:20} | {:7.1} units |",
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
