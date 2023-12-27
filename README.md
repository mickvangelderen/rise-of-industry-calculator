
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

For builds and other details see [output.md](./output.md).

|    0 |  -2.03% | Raw Resources        | Water                | 
|    0 |  -1.84% | Raw Resources        | Sand                 | 
|    0 |  -1.84% | Raw Resources        | Wood                 | 
|    0 |  -0.98% | Raw Resources        | Fish                 | 
|    0 |   2.60% | Raw Resources        | Coal                 | 
|    0 |   2.60% | Raw Resources        | Copper               | 
|    0 |   2.60% | Raw Resources        | Iron Ore             | 
|    0 |   2.87% | Prototypes           | Premade Dinner       | 
|    0 |   2.90% | Prototypes           | First Computer       | 
|    0 |   2.90% | Prototypes           | Car Prototype        | 
|    0 |   2.92% | Farm Produce         | Apples               | 
|    0 |   2.92% | Farm Produce         | Grapes               | 
|    0 |   2.92% | Farm Produce         | Olives               | 
|    0 |   2.92% | Farm Produce         | Oranges              | 
|    0 |   2.92% | Farm Produce         | Raw Rubber           | 
|    0 |   2.97% | Raw Resources        | Gas                  | 
|    0 |   2.97% | Raw Resources        | Oil                  | 
|    0 |   3.10% | Farm Produce         | Hops                 | 
|    0 |   3.10% | Farm Produce         | Potato               | 
|    0 |   3.10% | Farm Produce         | Vegetables           | 
|    0 |   3.10% | Farm Produce         | Wheat                | 
|    0 |   3.46% | Farm Produce         | Berries              | 
|    0 |   3.46% | Farm Produce         | Cocoa                | 
|    0 |   3.46% | Farm Produce         | Cotton               | 
|    0 |   3.46% | Farm Produce         | Sugar                | 
|    1 |  -1.21% | Livestock            | Chicken Meat         | 
|    1 |  -1.19% | Livestock            | Eggs                 | 
|    1 |  -0.88% | Livestock            | Wool                 | 
|    1 |  -0.84% | Livestock            | Mutton               | 
|    1 |   1.46% | Livestock            | Beef                 | 
|    1 |   1.46% | Livestock            | Leather              | 
|    1 |   1.46% | Livestock            | Milk                 | 
|    1 |   4.13% | Tier1                | Apple Smoothie       | 
|    1 |   4.13% | Tier1                | Grape Juice          | 
|    1 |   4.13% | Tier1                | Orange Juice         | 
|    1 |   4.18% | Tier1                | Berry Smoothie       | 
|    1 |   4.18% | Tier1                | Soda Water           | 
|    1 |   4.22% | Tier1                | Ink                  | 
|    1 |   4.30% | Tier1                | Hard Cider           | 
|    1 |   4.30% | Tier1                | Olive Oil            | 
|    1 |   4.31% | Tier1                | Bricks               | 
|    1 |   4.37% | Tier1                | Wine                 | 
|    1 |   4.43% | Tier1                | Flour                | 
|    1 |   4.62% | Tier1                | Soup                 | 
|    1 |   4.72% | Tier1                | Stuffing             | 
|    1 |   5.12% | Tier1                | Glass                | 
|    1 |   6.00% | Tier1                | Wooden Planks        | 
|    1 |   6.07% | Tier1                | Concrete             | 
|    1 |   6.35% | Tier1                | Copper Tubing        | 
|    1 |   6.35% | Tier1                | Copper Wire          | 
|    1 |   6.38% | Tier1                | Dye                  | 
|    1 |   6.48% | Tier1                | Heavy Pulp           | 
|    1 |   6.48% | Tier1                | Paper Roll           | 
|    1 |   6.50% | Tier1                | Chemicals            | 
|    1 |   6.50% | Tier1                | Refined Oil          | 
|    1 |   6.52% | Tier1                | Yeast                | 
|    1 |   6.64% | Tier1                | Fibers               | 
|    1 |   6.93% | Tier1                | Rubber               | 
|    1 |   7.41% | Tier1                | Wooden Train         | 
|    1 |   7.43% | Tier1                | Plastic              | 
|    1 |   7.53% | Tier1                | Toy Furniture        | 
|    1 |   7.60% | Tier1                | Steel                | 
|    2 |   1.00% | Tier2                | Chicken Soup         | 
|    2 |   2.05% | Components           | Fried Chicken        | 
|    2 |   2.75% | Tier2                | Beef Stew            | 
|    2 |   2.82% | Tier2                | Cheese               | 
|    2 |   2.89% | Tier2                | Chocolate Bar        | 
|    2 |   3.04% | Tier2                | Teddy Bear           | 
|    2 |   3.39% | Tier2                | Chocolate Cake       | 
|    2 |   3.48% | Components           | Cooked Vegetables    | 
|    2 |   3.83% | Components           | Exterior Body        | 
|    2 |   3.91% | Components           | Interior Body        | 
|    2 |   3.93% | Components           | Binary Switcher      | 
|    2 |   3.94% | Components           | Combustion Engine    | 
|    2 |   4.33% | Tier2                | Heavy Fabric         | 
|    2 |   4.38% | Tier2                | Dough                | 
|    2 |   4.39% | Components           | Axles                | 
|    2 |   4.42% | Tier2                | Ceramics             | 
|    2 |   4.95% | Tier2                | Light Bulbs          | 
|    2 |   4.95% | Tier2                | Diodes               | 
|    2 |   5.04% | Tier2                | Bottles              | 
|    2 |   5.04% | Tier2                | Glass Tubes          | 
|    2 |   5.06% | Tier2                | Furniture Base (L)   | 
|    2 |   5.06% | Tier2                | Furniture Base (S)   | 
|    2 |   5.06% | Tier2                | Wooden Barrels       | 
|    2 |   5.31% | Tier2                | Wall Panels          | 
|    2 |   5.32% | Tier2                | Biofuel              | 
|    2 |   5.41% | Tier2                | Printed Paper        | 
|    2 |   5.47% | Tier2                | Radiator             | 
|    2 |   5.56% | Tier2                | Capacitors           | 
|    2 |   5.70% | Tier2                | Refrigerator         | 
|    2 |   5.76% | Tier2                | Bag Of Chips         | 
|    2 |   5.76% | Tier2                | Marbles              | 
|    2 |   5.79% | Tier2                | Cardboard            | 
|    2 |   5.91% | Tier2                | Doll                 | 
|    2 |   5.96% | Tier2                | Stovetop             | 
|    2 |   6.02% | Tier2                | Steel Barrel         | 
|    2 |   6.08% | Tier2                | Light Fabric         | 
|    2 |   6.26% | Tier2                | Toy Train Set        | 
|    2 |   6.26% | Tier2                | Punch Cards          | 
|    2 |   6.32% | Tier2                | Buttons              | 
|    2 |   6.52% | Tier2                | Rubber Tubes         | 
|    2 |   6.52% | Tier2                | Tire                 | 
|    2 |   6.64% | Tier2                | Plastic Cutlery      | 
|    2 |   6.75% | Tier2                | Paint                | 
|    2 |   6.82% | Tier2                | Cans                 | 
|    2 |   6.82% | Tier2                | Steel Frame          | 
|    2 |   7.11% | Tier2                | Adhesive             | 
|    3 |   3.16% | Components           | Chicken Dinner       | 
|    3 |   3.50% | Tier3                | Winter Clothes       | 
|    3 |   3.66% | Tier3                | Pizza                | 
|    3 |   3.84% | Tier3                | Burgers              | 
|    3 |   3.88% | Tier3                | Work Clothes         | 
|    3 |   3.93% | Components           | Body Chassis         | 
|    3 |   3.95% | Components           | Computer Memory      | 
|    3 |   3.96% | Components           | Processor            | 
|    3 |   4.01% | Tier3                | Leather Furniture    | 
|    3 |   4.03% | Tier3                | Canned Mutton        | 
|    3 |   4.06% | Components           | Rolling Chassis      | 
|    3 |   4.15% | Tier3                | Orange Soda          | 
|    3 |   4.16% | Components           | Interface            | 
|    3 |   4.23% | Tier3                | Oven                 | 
|    3 |   4.25% | Tier3                | Waffles              | 
|    3 |   4.25% | Tier3                | Headlights           | 
|    3 |   4.26% | Components           | Dinner Container     | 
|    3 |   4.30% | Tier3                | Newspapers           | 
|    3 |   4.33% | Tier3                | Berry Pie            | 
|    3 |   4.34% | Tier3                | Couch                | 
|    3 |   4.38% | Tier3                | Interior Lining      | 
|    3 |   4.40% | Tier3                | Hard Cider Donuts    | 
|    3 |   4.47% | Tier3                | Radio Receiver       | 
|    3 |   4.57% | Tier3                | Brandy               | 
|    3 |   4.62% | Tier3                | Deluxe Books         | 
|    3 |   4.71% | Tier3                | Quilt                | 
|    3 |   4.87% | Tier3                | Fish and Chips       | 
|    3 |   4.92% | Tier3                | Telephone            | 
|    3 |   4.92% | Tier3                | Car Seat             | 
|    3 |   4.97% | Tier3                | Canned Fish          | 
|    3 |   5.12% | Tier3                | Engine Block         | 
|    3 |   5.18% | Tier3                | Beer                 | 
|    3 |   5.18% | Tier3                | Grain Whiskey        | 
|    3 |   5.18% | Tier3                | Vodka                | 
|    3 |   5.23% | Tier3                | Plastic Furniture    | 
|    3 |   5.24% | Tier3                | Office Furniture     | 
|    3 |   5.29% | Tier3                | Books                | 
|    3 |   5.35% | Tier3                | Napkins              | 
|    3 |   5.37% | Tier3                | Reinforced Wall      | 
|    3 |   5.42% | Tier3                | Summer Clothes       | 
|    3 |   5.44% | Tier3                | Dollhouse            | 
|    3 |   5.50% | Tier3                | Thin Cardboard       | 
|    3 |   5.59% | Tier3                | Easter Eggs          | 
