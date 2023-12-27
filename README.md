
The [importer](./importer/) deserializes the game data and serializes the useful
parts to [`data.json`](./data.json). The game data is obtained by running
[AssetRipper](https://github.com/AssetRipper/AssetRipper) on an owned copy of
[Rise of Industry](https://www.riseofindustry.com/). 

The [calculator](./calculator/) uses the data in [`data.json`](./data.json) to calculate the total investment cost and return on investment for a particular setup.

The following setup (omitting obtaining the references and transportation):

```rust
let water_harvesters = (
    1i64,
    BuildingInstance {
        id: water_siphon.id,
        recipe_id: water_recipe.id,
        modules: Some((5, water_well_harvester.id)),
        efficiency: Default::default(),
    },
);

let cocoa_plantations = (
    2i64,
    BuildingInstance {
        id: plantation.id,
        recipe_id: cocoa_recipe.id,
        modules: Some((5, cocoa_field.id)),
        efficiency: Default::default(),
    },
);
```

generates the following output:

```
production per month:
| Water                |     0.0 units |
| Cocoa                |    20.0 units |

transportation costs per month:
| Truck   |    10.0 deliveries |      10 tiles |     3.5k | Water to Cocoa Plantations               |
| Truck   |    20.0 deliveries |     100 tiles |    25.0k | Cocoa Plantations to Farmers Market      |

operational costs:           81k
revenue:                    253k
profit:                     172k
initial costs:             2100k
return on investment:      7.97%
```

## Return on Investment Overview

Some assumptions have to be made, for example:

- which recipe to use for what product, particularly water and coal,
- how to transport products from one place to another,
- how many modules to use.

In this simulation we distinguish between local transport and sales transport. Local transport is
for intermediary goods between production buildings and is set to 20 tiles on average. Sales
transport for goods that are sold and the distance is set to 150 tiles. This explains why raw
resources have a low return on investment in the overview below.

For builds and other details see [output.md](./output.md).

| tier | category             |     RoI | product              |
|-----:|--------:|----------------------|----------------------|
|    0 | Farm Produce         |   3.35% | Apples               | 
|    0 | Farm Produce         |   3.35% | Grapes               | 
|    0 | Farm Produce         |   3.35% | Olives               | 
|    0 | Farm Produce         |   3.35% | Oranges              | 
|    0 | Farm Produce         |   3.35% | Raw Rubber           | 
|    0 | Farm Produce         |   3.60% | Hops                 | 
|    0 | Farm Produce         |   3.60% | Potato               | 
|    0 | Farm Produce         |   3.60% | Vegetables           | 
|    0 | Farm Produce         |   3.60% | Wheat                | 
|    0 | Farm Produce         |   3.85% | Berries              | 
|    0 | Farm Produce         |   3.85% | Cocoa                | 
|    0 | Farm Produce         |   3.85% | Cotton               | 
|    0 | Farm Produce         |   3.85% | Sugar                | 
|    0 | Prototypes           |   2.88% | Premade Dinner       | 
|    0 | Prototypes           |   2.90% | First Computer       | 
|    0 | Prototypes           |   2.90% | Car Prototype        | 
|    0 | Raw Resources        |  -1.37% | Water                | 
|    0 | Raw Resources        |  -0.98% | Sand                 | 
|    0 | Raw Resources        |  -0.98% | Wood                 | 
|    0 | Raw Resources        |  -0.31% | Fish                 | 
|    0 | Raw Resources        |   3.00% | Coal                 | 
|    0 | Raw Resources        |   3.00% | Copper               | 
|    0 | Raw Resources        |   3.00% | Iron Ore             | 
|    0 | Raw Resources        |   3.29% | Gas                  | 
|    0 | Raw Resources        |   3.29% | Oil                  | 
|    1 | Livestock            |  -0.83% | Eggs                 | 
|    1 | Livestock            |  -0.82% | Chicken Meat         | 
|    1 | Livestock            |  -0.52% | Wool                 | 
|    1 | Livestock            |  -0.45% | Mutton               | 
|    1 | Livestock            |   1.85% | Beef                 | 
|    1 | Livestock            |   1.85% | Leather              | 
|    1 | Livestock            |   1.85% | Milk                 | 
|    1 | Tier1                |   4.27% | Apple Smoothie       | 
|    1 | Tier1                |   4.27% | Grape Juice          | 
|    1 | Tier1                |   4.27% | Orange Juice         | 
|    1 | Tier1                |   4.32% | Berry Smoothie       | 
|    1 | Tier1                |   4.32% | Soda Water           | 
|    1 | Tier1                |   4.34% | Ink                  | 
|    1 | Tier1                |   4.43% | Hard Cider           | 
|    1 | Tier1                |   4.45% | Olive Oil            | 
|    1 | Tier1                |   4.50% | Wine                 | 
|    1 | Tier1                |   4.52% | Bricks               | 
|    1 | Tier1                |   4.58% | Flour                | 
|    1 | Tier1                |   4.83% | Soup                 | 
|    1 | Tier1                |   4.93% | Stuffing             | 
|    1 | Tier1                |   5.30% | Glass                | 
|    1 | Tier1                |   6.23% | Wooden Planks        | 
|    1 | Tier1                |   6.26% | Concrete             | 
|    1 | Tier1                |   6.54% | Copper Tubing        | 
|    1 | Tier1                |   6.54% | Copper Wire          | 
|    1 | Tier1                |   6.57% | Dye                  | 
|    1 | Tier1                |   6.64% | Chemicals            | 
|    1 | Tier1                |   6.64% | Refined Oil          | 
|    1 | Tier1                |   6.67% | Heavy Pulp           | 
|    1 | Tier1                |   6.67% | Paper Roll           | 
|    1 | Tier1                |   6.72% | Yeast                | 
|    1 | Tier1                |   6.84% | Fibers               | 
|    1 | Tier1                |   7.13% | Rubber               | 
|    1 | Tier1                |   7.57% | Plastic              | 
|    1 | Tier1                |   7.65% | Wooden Train         | 
|    1 | Tier1                |   7.75% | Steel                | 
|    1 | Tier1                |   7.80% | Toy Furniture        | 
|    2 | Components           |   2.15% | Fried Chicken        | 
|    2 | Components           |   3.52% | Cooked Vegetables    | 
|    2 | Components           |   3.85% | Exterior Body        | 
|    2 | Components           |   3.93% | Interior Body        | 
|    2 | Components           |   3.96% | Combustion Engine    | 
|    2 | Components           |   3.96% | Binary Switcher      | 
|    2 | Components           |   4.40% | Axles                | 
|    2 | Tier2                |   1.23% | Chicken Soup         | 
|    2 | Tier2                |   2.98% | Beef Stew            | 
|    2 | Tier2                |   3.05% | Cheese               | 
|    2 | Tier2                |   3.09% | Chocolate Bar        | 
|    2 | Tier2                |   3.16% | Teddy Bear           | 
|    2 | Tier2                |   3.48% | Chocolate Cake       | 
|    2 | Tier2                |   4.48% | Dough                | 
|    2 | Tier2                |   4.50% | Heavy Fabric         | 
|    2 | Tier2                |   4.52% | Ceramics             | 
|    2 | Tier2                |   5.04% | Light Bulbs          | 
|    2 | Tier2                |   5.04% | Diodes               | 
|    2 | Tier2                |   5.14% | Bottles              | 
|    2 | Tier2                |   5.14% | Glass Tubes          | 
|    2 | Tier2                |   5.18% | Furniture Base (L)   | 
|    2 | Tier2                |   5.18% | Furniture Base (S)   | 
|    2 | Tier2                |   5.18% | Wooden Barrels       | 
|    2 | Tier2                |   5.38% | Wall Panels          | 
|    2 | Tier2                |   5.42% | Biofuel              | 
|    2 | Tier2                |   5.48% | Printed Paper        | 
|    2 | Tier2                |   5.56% | Radiator             | 
|    2 | Tier2                |   5.62% | Capacitors           | 
|    2 | Tier2                |   5.78% | Refrigerator         | 
|    2 | Tier2                |   5.85% | Marbles              | 
|    2 | Tier2                |   5.85% | Bag Of Chips         | 
|    2 | Tier2                |   5.90% | Cardboard            | 
|    2 | Tier2                |   6.00% | Doll                 | 
|    2 | Tier2                |   6.04% | Stovetop             | 
|    2 | Tier2                |   6.11% | Steel Barrel         | 
|    2 | Tier2                |   6.18% | Light Fabric         | 
|    2 | Tier2                |   6.30% | Punch Cards          | 
|    2 | Tier2                |   6.35% | Toy Train Set        | 
|    2 | Tier2                |   6.40% | Buttons              | 
|    2 | Tier2                |   6.64% | Rubber Tubes         | 
|    2 | Tier2                |   6.64% | Tire                 | 
|    2 | Tier2                |   6.72% | Plastic Cutlery      | 
|    2 | Tier2                |   6.83% | Paint                | 
|    2 | Tier2                |   6.91% | Cans                 | 
|    2 | Tier2                |   6.91% | Steel Frame          | 
|    2 | Tier2                |   7.18% | Adhesive             | 
|    3 | Components           |   3.20% | Chicken Dinner       | 
|    3 | Components           |   3.94% | Body Chassis         | 
|    3 | Components           |   3.97% | Computer Memory      | 
|    3 | Components           |   3.97% | Processor            | 
|    3 | Components           |   4.06% | Rolling Chassis      | 
|    3 | Components           |   4.17% | Interface            | 
|    3 | Components           |   4.28% | Dinner Container     | 
|    3 | Tier3                |   3.64% | Winter Clothes       | 
|    3 | Tier3                |   3.80% | Pizza                | 
|    3 | Tier3                |   3.94% | Burgers              | 
|    3 | Tier3                |   4.06% | Work Clothes         | 
|    3 | Tier3                |   4.16% | Leather Furniture    | 
|    3 | Tier3                |   4.19% | Canned Mutton        | 
|    3 | Tier3                |   4.20% | Orange Soda          | 
|    3 | Tier3                |   4.28% | Oven                 | 
|    3 | Tier3                |   4.29% | Waffles              | 
|    3 | Tier3                |   4.31% | Headlights           | 
|    3 | Tier3                |   4.35% | Newspapers           | 
|    3 | Tier3                |   4.38% | Berry Pie            | 
|    3 | Tier3                |   4.42% | Couch                | 
|    3 | Tier3                |   4.44% | Hard Cider Donuts    | 
|    3 | Tier3                |   4.48% | Interior Lining      | 
|    3 | Tier3                |   4.51% | Radio Receiver       | 
|    3 | Tier3                |   4.62% | Brandy               | 
|    3 | Tier3                |   4.71% | Deluxe Books         | 
|    3 | Tier3                |   4.78% | Quilt                | 
|    3 | Tier3                |   4.93% | Fish and Chips       | 
|    3 | Tier3                |   4.96% | Telephone            | 
|    3 | Tier3                |   5.00% | Car Seat             | 
|    3 | Tier3                |   5.03% | Canned Fish          | 
|    3 | Tier3                |   5.17% | Engine Block         | 
|    3 | Tier3                |   5.25% | Beer                 | 
|    3 | Tier3                |   5.25% | Grain Whiskey        | 
|    3 | Tier3                |   5.25% | Vodka                | 
|    3 | Tier3                |   5.28% | Plastic Furniture    | 
|    3 | Tier3                |   5.30% | Office Furniture     | 
|    3 | Tier3                |   5.35% | Books                | 
|    3 | Tier3                |   5.42% | Napkins              | 
|    3 | Tier3                |   5.44% | Reinforced Wall      | 
|    3 | Tier3                |   5.47% | Summer Clothes       | 
|    3 | Tier3                |   5.52% | Dollhouse            | 
|    3 | Tier3                |   5.54% | Thin Cardboard       | 
|    3 | Tier3                |   5.64% | Easter Eggs          | 
