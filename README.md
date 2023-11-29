
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
