
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

| tier |      category | RoI    | product                                                        |
| ---: | ------------: | ------ | -------------------------------------------------------------- |
|    0 |  Farm Produce | 3.35%  | [Apples](<./output.md#product-apples>)                        |
|    0 |  Farm Produce | 3.35%  | [Grapes](<./output.md#product-grapes>)                        |
|    0 |  Farm Produce | 3.35%  | [Olives](<./output.md#product-olives>)                        |
|    0 |  Farm Produce | 3.35%  | [Oranges](<./output.md#product-oranges>)                      |
|    0 |  Farm Produce | 3.35%  | [Raw Rubber](<./output.md#product-raw-rubber>)                |
|    0 |  Farm Produce | 3.60%  | [Hops](<./output.md#product-hops>)                            |
|    0 |  Farm Produce | 3.60%  | [Potato](<./output.md#product-potato>)                        |
|    0 |  Farm Produce | 3.60%  | [Vegetables](<./output.md#product-vegetables>)                |
|    0 |  Farm Produce | 3.60%  | [Wheat](<./output.md#product-wheat>)                          |
|    0 |  Farm Produce | 3.85%  | [Berries](<./output.md#product-berries>)                      |
|    0 |  Farm Produce | 3.85%  | [Cocoa](<./output.md#product-cocoa>)                          |
|    0 |  Farm Produce | 3.85%  | [Cotton](<./output.md#product-cotton>)                        |
|    0 |  Farm Produce | 3.85%  | [Sugar](<./output.md#product-sugar>)                          |
|    0 |    Prototypes | 2.88%  | [Premade Dinner](<./output.md#product-premade-dinner>)        |
|    0 |    Prototypes | 2.90%  | [First Computer](<./output.md#product-first-computer>)        |
|    0 |    Prototypes | 2.90%  | [Car Prototype](<./output.md#product-car-prototype>)          |
|    0 | Raw Resources | -1.37% | [Water](<./output.md#product-water>)                          |
|    0 | Raw Resources | -0.98% | [Sand](<./output.md#product-sand>)                            |
|    0 | Raw Resources | -0.98% | [Wood](<./output.md#product-wood>)                            |
|    0 | Raw Resources | -0.31% | [Fish](<./output.md#product-fish>)                            |
|    0 | Raw Resources | 3.00%  | [Coal](<./output.md#product-coal>)                            |
|    0 | Raw Resources | 3.00%  | [Copper](<./output.md#product-copper>)                        |
|    0 | Raw Resources | 3.00%  | [Iron Ore](<./output.md#product-iron-ore>)                    |
|    0 | Raw Resources | 3.29%  | [Gas](<./output.md#product-gas>)                              |
|    0 | Raw Resources | 3.29%  | [Oil](<./output.md#product-oil>)                              |
|    1 |     Livestock | -0.83% | [Eggs](<./output.md#product-eggs>)                            |
|    1 |     Livestock | -0.82% | [Chicken Meat](<./output.md#product-chicken-meat>)            |
|    1 |     Livestock | -0.52% | [Wool](<./output.md#product-wool>)                            |
|    1 |     Livestock | -0.45% | [Mutton](<./output.md#product-mutton>)                        |
|    1 |     Livestock | 1.85%  | [Beef](<./output.md#product-beef>)                            |
|    1 |     Livestock | 1.85%  | [Leather](<./output.md#product-leather>)                      |
|    1 |     Livestock | 1.85%  | [Milk](<./output.md#product-milk>)                            |
|    1 |         Tier1 | 4.27%  | [Apple Smoothie](<./output.md#product-apple-smoothie>)        |
|    1 |         Tier1 | 4.27%  | [Grape Juice](<./output.md#product-grape-juice>)              |
|    1 |         Tier1 | 4.27%  | [Orange Juice](<./output.md#product-orange-juice>)            |
|    1 |         Tier1 | 4.32%  | [Berry Smoothie](<./output.md#product-berry-smoothie>)        |
|    1 |         Tier1 | 4.32%  | [Soda Water](<./output.md#product-soda-water>)                |
|    1 |         Tier1 | 4.34%  | [Ink](<./output.md#product-ink>)                              |
|    1 |         Tier1 | 4.43%  | [Hard Cider](<./output.md#product-hard-cider>)                |
|    1 |         Tier1 | 4.45%  | [Olive Oil](<./output.md#product-olive-oil>)                  |
|    1 |         Tier1 | 4.50%  | [Wine](<./output.md#product-wine>)                            |
|    1 |         Tier1 | 4.52%  | [Bricks](<./output.md#product-bricks>)                        |
|    1 |         Tier1 | 4.58%  | [Flour](<./output.md#product-flour>)                          |
|    1 |         Tier1 | 4.83%  | [Soup](<./output.md#product-soup>)                            |
|    1 |         Tier1 | 4.93%  | [Stuffing](<./output.md#product-stuffing>)                    |
|    1 |         Tier1 | 5.30%  | [Glass](<./output.md#product-glass>)                          |
|    1 |         Tier1 | 6.23%  | [Wooden Planks](<./output.md#product-wooden-planks>)          |
|    1 |         Tier1 | 6.26%  | [Concrete](<./output.md#product-concrete>)                    |
|    1 |         Tier1 | 6.54%  | [Copper Tubing](<./output.md#product-copper-tubing>)          |
|    1 |         Tier1 | 6.54%  | [Copper Wire](<./output.md#product-copper-wire>)              |
|    1 |         Tier1 | 6.57%  | [Dye](<./output.md#product-dye>)                              |
|    1 |         Tier1 | 6.64%  | [Chemicals](<./output.md#product-chemicals>)                  |
|    1 |         Tier1 | 6.64%  | [Refined Oil](<./output.md#product-refined-oil>)              |
|    1 |         Tier1 | 6.67%  | [Heavy Pulp](<./output.md#product-heavy-pulp>)                |
|    1 |         Tier1 | 6.67%  | [Paper Roll](<./output.md#product-paper-roll>)                |
|    1 |         Tier1 | 6.72%  | [Yeast](<./output.md#product-yeast>)                          |
|    1 |         Tier1 | 6.84%  | [Fibers](<./output.md#product-fibers>)                        |
|    1 |         Tier1 | 7.13%  | [Rubber](<./output.md#product-rubber>)                        |
|    1 |         Tier1 | 7.57%  | [Plastic](<./output.md#product-plastic>)                      |
|    1 |         Tier1 | 7.65%  | [Wooden Train](<./output.md#product-wooden-train>)            |
|    1 |         Tier1 | 7.75%  | [Steel](<./output.md#product-steel>)                          |
|    1 |         Tier1 | 7.80%  | [Toy Furniture](<./output.md#product-toy-furniture>)          |
|    2 |    Components | 2.15%  | [Fried Chicken](<./output.md#product-fried-chicken>)          |
|    2 |    Components | 3.52%  | [Cooked Vegetables](<./output.md#product-cooked-vegetables>)  |
|    2 |    Components | 3.85%  | [Exterior Body](<./output.md#product-exterior-body>)          |
|    2 |    Components | 3.93%  | [Interior Body](<./output.md#product-interior-body>)          |
|    2 |    Components | 3.96%  | [Combustion Engine](<./output.md#product-combustion-engine>)  |
|    2 |    Components | 3.96%  | [Binary Switcher](<./output.md#product-binary-switcher>)      |
|    2 |    Components | 4.40%  | [Axles](<./output.md#product-axles>)                          |
|    2 |         Tier2 | 1.23%  | [Chicken Soup](<./output.md#product-chicken-soup>)            |
|    2 |         Tier2 | 2.98%  | [Beef Stew](<./output.md#product-beef-stew>)                  |
|    2 |         Tier2 | 3.05%  | [Cheese](<./output.md#product-cheese>)                        |
|    2 |         Tier2 | 3.09%  | [Chocolate Bar](<./output.md#product-chocolate-bar>)          |
|    2 |         Tier2 | 3.16%  | [Teddy Bear](<./output.md#product-teddy-bear>)                |
|    2 |         Tier2 | 3.48%  | [Chocolate Cake](<./output.md#product-chocolate-cake>)        |
|    2 |         Tier2 | 4.48%  | [Dough](<./output.md#product-dough>)                          |
|    2 |         Tier2 | 4.50%  | [Heavy Fabric](<./output.md#product-heavy-fabric>)            |
|    2 |         Tier2 | 4.52%  | [Ceramics](<./output.md#product-ceramics>)                    |
|    2 |         Tier2 | 5.04%  | [Light Bulbs](<./output.md#product-light-bulbs>)              |
|    2 |         Tier2 | 5.04%  | [Diodes](<./output.md#product-diodes>)                        |
|    2 |         Tier2 | 5.14%  | [Bottles](<./output.md#product-bottles>)                      |
|    2 |         Tier2 | 5.14%  | [Glass Tubes](<./output.md#product-glass-tubes>)              |
|    2 |         Tier2 | 5.18%  | [Furniture Base (L](<./output.md#product-furniture-base-l>)) |
|    2 |         Tier2 | 5.18%  | [Furniture Base (S](<./output.md#product-furniture-base-s>)) |
|    2 |         Tier2 | 5.18%  | [Wooden Barrels](<./output.md#product-wooden-barrels>)        |
|    2 |         Tier2 | 5.38%  | [Wall Panels](<./output.md#product-wall-panels>)              |
|    2 |         Tier2 | 5.42%  | [Biofuel](<./output.md#product-biofuel>)                      |
|    2 |         Tier2 | 5.48%  | [Printed Paper](<./output.md#product-printed-paper>)          |
|    2 |         Tier2 | 5.56%  | [Radiator](<./output.md#product-radiator>)                    |
|    2 |         Tier2 | 5.62%  | [Capacitors](<./output.md#product-capacitors>)                |
|    2 |         Tier2 | 5.78%  | [Refrigerator](<./output.md#product-refrigerator>)            |
|    2 |         Tier2 | 5.85%  | [Marbles](<./output.md#product-marbles>)                      |
|    2 |         Tier2 | 5.85%  | [Bag Of Chips](<./output.md#product-bag-of-chips>)            |
|    2 |         Tier2 | 5.90%  | [Cardboard](<./output.md#product-cardboard>)                  |
|    2 |         Tier2 | 6.00%  | [Doll](<./output.md#product-doll>)                            |
|    2 |         Tier2 | 6.04%  | [Stovetop](<./output.md#product-stovetop>)                    |
|    2 |         Tier2 | 6.11%  | [Steel Barrel](<./output.md#product-steel-barrel>)            |
|    2 |         Tier2 | 6.18%  | [Light Fabric](<./output.md#product-light-fabric>)            |
|    2 |         Tier2 | 6.30%  | [Punch Cards](<./output.md#product-punch-cards>)              |
|    2 |         Tier2 | 6.35%  | [Toy Train Set](<./output.md#product-toy-train-set>)          |
|    2 |         Tier2 | 6.40%  | [Buttons](<./output.md#product-buttons>)                      |
|    2 |         Tier2 | 6.64%  | [Rubber Tubes](<./output.md#product-rubber-tubes>)            |
|    2 |         Tier2 | 6.64%  | [Tire](<./output.md#product-tire>)                            |
|    2 |         Tier2 | 6.72%  | [Plastic Cutlery](<./output.md#product-plastic-cutlery>)      |
|    2 |         Tier2 | 6.83%  | [Paint](<./output.md#product-paint>)                          |
|    2 |         Tier2 | 6.91%  | [Cans](<./output.md#product-cans>)                            |
|    2 |         Tier2 | 6.91%  | [Steel Frame](<./output.md#product-steel-frame>)              |
|    2 |         Tier2 | 7.18%  | [Adhesive](<./output.md#product-adhesive>)                    |
|    3 |    Components | 3.20%  | [Chicken Dinner](<./output.md#product-chicken-dinner>)        |
|    3 |    Components | 3.94%  | [Body Chassis](<./output.md#product-body-chassis>)            |
|    3 |    Components | 3.97%  | [Computer Memory](<./output.md#product-computer-memory>)      |
|    3 |    Components | 3.97%  | [Processor](<./output.md#product-processor>)                  |
|    3 |    Components | 4.06%  | [Rolling Chassis](<./output.md#product-rolling-chassis>)      |
|    3 |    Components | 4.17%  | [Interface](<./output.md#product-interface>)                  |
|    3 |    Components | 4.28%  | [Dinner Container](<./output.md#product-dinner-container>)    |
|    3 |         Tier3 | 3.64%  | [Winter Clothes](<./output.md#product-winter-clothes>)        |
|    3 |         Tier3 | 3.80%  | [Pizza](<./output.md#product-pizza>)                          |
|    3 |         Tier3 | 3.94%  | [Burgers](<./output.md#product-burgers>)                      |
|    3 |         Tier3 | 4.06%  | [Work Clothes](<./output.md#product-work-clothes>)            |
|    3 |         Tier3 | 4.16%  | [Leather Furniture](<./output.md#product-leather-furniture>)  |
|    3 |         Tier3 | 4.19%  | [Canned Mutton](<./output.md#product-canned-mutton>)          |
|    3 |         Tier3 | 4.20%  | [Orange Soda](<./output.md#product-orange-soda>)              |
|    3 |         Tier3 | 4.28%  | [Oven](<./output.md#product-oven>)                            |
|    3 |         Tier3 | 4.29%  | [Waffles](<./output.md#product-waffles>)                      |
|    3 |         Tier3 | 4.31%  | [Headlights](<./output.md#product-headlights>)                |
|    3 |         Tier3 | 4.35%  | [Newspapers](<./output.md#product-newspapers>)                |
|    3 |         Tier3 | 4.38%  | [Berry Pie](<./output.md#product-berry-pie>)                  |
|    3 |         Tier3 | 4.42%  | [Couch](<./output.md#product-couch>)                          |
|    3 |         Tier3 | 4.44%  | [Hard Cider Donuts](<./output.md#product-hard-cider-donuts>)  |
|    3 |         Tier3 | 4.48%  | [Interior Lining](<./output.md#product-interior-lining>)      |
|    3 |         Tier3 | 4.51%  | [Radio Receiver](<./output.md#product-radio-receiver>)        |
|    3 |         Tier3 | 4.62%  | [Brandy](<./output.md#product-brandy>)                        |
|    3 |         Tier3 | 4.71%  | [Deluxe Books](<./output.md#product-deluxe-books>)            |
|    3 |         Tier3 | 4.78%  | [Quilt](<./output.md#product-quilt>)                          |
|    3 |         Tier3 | 4.93%  | [Fish and Chips](<./output.md#product-fish-and-chips>)        |
|    3 |         Tier3 | 4.96%  | [Telephone](<./output.md#product-telephone>)                  |
|    3 |         Tier3 | 5.00%  | [Car Seat](<./output.md#product-car-seat>)                    |
|    3 |         Tier3 | 5.03%  | [Canned Fish](<./output.md#product-canned-fish>)              |
|    3 |         Tier3 | 5.17%  | [Engine Block](<./output.md#product-engine-block>)            |
|    3 |         Tier3 | 5.25%  | [Beer](<./output.md#product-beer>)                            |
|    3 |         Tier3 | 5.25%  | [Grain Whiskey](<./output.md#product-grain-whiskey>)          |
|    3 |         Tier3 | 5.25%  | [Vodka](<./output.md#product-vodka>)                          |
|    3 |         Tier3 | 5.28%  | [Plastic Furniture](<./output.md#product-plastic-furniture>)  |
|    3 |         Tier3 | 5.30%  | [Office Furniture](<./output.md#product-office-furniture>)    |
|    3 |         Tier3 | 5.35%  | [Books](<./output.md#product-books>)                          |
|    3 |         Tier3 | 5.42%  | [Napkins](<./output.md#product-napkins>)                      |
|    3 |         Tier3 | 5.44%  | [Reinforced Wall](<./output.md#product-reinforced-wall>)      |
|    3 |         Tier3 | 5.47%  | [Summer Clothes](<./output.md#product-summer-clothes>)        |
|    3 |         Tier3 | 5.52%  | [Dollhouse](<./output.md#product-dollhouse>)                  |
|    3 |         Tier3 | 5.54%  | [Thin Cardboard](<./output.md#product-thin-cardboard>)        |
|    3 |         Tier3 | 5.64%  | [Easter Eggs](<./output.md#product-easter-eggs>)              |
