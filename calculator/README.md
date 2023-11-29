Formula search results. 

```
/*
31 results - 31 files

ExportedProject\Assets\Resources\gamedata\formulas\2130ShopDemandFormula.asset:
  15    _category: {fileID: 11400000, guid: 1838e8fed5c29e541b34e7acf83a0c78, type: 2}
  16:   formula: shopModifier + ceil(0.035 * ceil(population / 500000) * (30 / timeToProduce) * producedAmount * tierValue * difficulty)

ExportedProject\Assets\Resources\gamedata\formulas\ContractSpawnChance.asset:
  15    _category: {fileID: 0}
  16:   formula: 0.001 + min(failedSpawnCount * 0.0002, 0.05)

ExportedProject\Assets\Resources\gamedata\formulas\Final Score.asset:
  15    _category: {fileID: 0}
  16:   formula: (assets / (months ^ 1.5)) * difficulty

ExportedProject\Assets\Resources\gamedata\formulas\GlobalMarket.asset:
  15    _category: {fileID: 0}
  16:   formula: 2 * sign(demand - sold - stored) * sqrt(abs(demand - sold - stored))

ExportedProject\Assets\Resources\gamedata\formulas\Pollution Rate Efficiency.asset:
  15    _category: {fileID: 0}
  16:   formula: ifmore(efficiency, 1, efficiency ^ 2, 1)

ExportedProject\Assets\Resources\gamedata\formulas\Settlement Distance Restriction.asset:
  15    _category: {fileID: 0}
  16:   formula: max(abs(x0 - x1), abs(y0 - y1))

ExportedProject\Assets\Resources\gamedata\formulas\Shop Demand.asset:
  15    _category: {fileID: 11400000, guid: 1838e8fed5c29e541b34e7acf83a0c78, type: 2}
  16:   formula: shopModifier + ceil(0.05 * ceil(population / 50000) * (30 / timeToProduce) * producedAmount * tierValue * difficulty)

ExportedProject\Assets\Resources\gamedata\formulas\Upkeep.asset:
  15    _category: {fileID: 0}
  16:   formula: buildingCost * 0.025

ExportedProject\Assets\Resources\gamedata\formulas\dispatch cost\AwhDispatchCost.asset:
  15    _category: {fileID: 0}
  16:   formula: (250 + distance * 10) * difficulty * actor

ExportedProject\Assets\Resources\gamedata\formulas\dispatch cost\BoatDepotDispatchCost.asset:
  15    _category: {fileID: 0}
  16:   formula: (4000 + distance * 20) * difficulty * actor

ExportedProject\Assets\Resources\gamedata\formulas\dispatch cost\ManualDestinationDispatchCost.asset:
  15    _category: {fileID: 0}
  16:   formula: (250 + distance * 10) * difficulty * actor

ExportedProject\Assets\Resources\gamedata\formulas\dispatch cost\TrainTerminalDispatchCost.asset:
  15    _category: {fileID: 0}
  16:   formula: (2250 + distance * 25) * difficulty * actor

ExportedProject\Assets\Resources\gamedata\formulas\dispatch cost\TruckDepotDispatchCost.asset:
  15    _category: {fileID: 0}
  16:   formula: (350 + distance * 15) * difficulty * actor

ExportedProject\Assets\Resources\gamedata\formulas\dispatch cost\ZeppelinFieldDispatchCost.asset:
  15    _category: {fileID: 0}
  16:   formula: (7500 + distance * 40) * difficulty * actor

ExportedProject\Assets\Resources\gamedata\formulas\hq\Hq Change Visuals Cost.asset:
  15    _category: {fileID: 0}
  16:   formula: 1000000 + (tier - 1) * 1500000

ExportedProject\Assets\Resources\gamedata\formulas\hq\Hq Change Visuals Required Unlocks.asset:
  15    _category: {fileID: 0}
  16:   formula: 20 + (tier - 1) * 10

ExportedProject\Assets\Resources\gamedata\formulas\product price\Factories.asset:
  15    _category: {fileID: 0}
  16:   formula: (ingredientsValue + ((upkeep / 30) * recipeDays)) / recipeOutput

ExportedProject\Assets\Resources\gamedata\formulas\product price\FarmProduce.asset:
  15    _category: {fileID: 0}
  16:   formula: ingredientsValue * 2.8

ExportedProject\Assets\Resources\gamedata\formulas\product price\Farms.asset:
  14    _category: {fileID: 0}
  15    _category: {fileID: 0}
  16:   formula: ((ingredientsValue * 3) + ((upkeep / 30) * recipeDays)) / (recipeOutput * 3)
  16  
  17  

ExportedProject\Assets\Resources\gamedata\formulas\product price\Gatherers.asset:
  15    _category: {fileID: 0}
  16:   formula: upkeep / ((3 * recipeOutput) * (30 / recipeDays))

ExportedProject\Assets\Resources\gamedata\formulas\product price\Livestock.asset:
  14    _category: {fileID: 0}
  15    _category: {fileID: 0}
  16:   formula: ((((ingredientsValue * 3) + ((upkeep / 30) * recipeDays)) / (recipeOutput * 3)) * (recipeOutput - productOutput)) / recipeOutput
  16  
  17  

ExportedProject\Assets\Resources\gamedata\formulas\product price\ProductPriceUpkeepComponent.asset:
  15    _category: {fileID: 0}
  16:   formula: hubUpkeep + (moduleUpkeep * 3)

ExportedProject\Assets\Resources\gamedata\formulas\product price\RawResources.asset:
  15    _category: {fileID: 0}
  16:   formula: 75 * recipeDays * 3.25

ExportedProject\Assets\Resources\gamedata\formulas\recipetime\FactoryRecipeTime.asset:
  15    _category: {fileID: 0}
  16:   formula: tier * 15

ExportedProject\Assets\Resources\gamedata\formulas\research\Research Cost.asset:
  15    _category: {fileID: 0}
  16:   formula: 3333.333333 * efficiency

ExportedProject\Assets\Resources\gamedata\formulas\research\Research Time.asset:
  15    _category: {fileID: 0}
  16:   formula: (60 + (tier ^ 3) * 60) / efficiency

ExportedProject\Assets\Resources\gamedata\help\loan\LoanAmount.asset:
  15    _category: {fileID: 0}
  16:   formula: ceil(population / 100000) * 5000000

ExportedProject\Assets\Resources\gamedata\help\loan\LoanAmount2130.asset:
  15    _category: {fileID: 0}
  16:   formula: ceil(population / 1000000) * 5000000

ExportedProject\Assets\Resources\gamedata\help\loan\LoanApr.asset:
  15    _category: {fileID: 0}
  16:   formula: max(0.05, 0.01 * (10 + 2.5 * (duration / 12 - 3)))

ExportedProject\Assets\Resources\gamedata\help\loan\LoanDuration.asset:
  15    _category: {fileID: 0}
  16:   formula: 120

ExportedProject\Assets\Scripts\Assembly-CSharp\ProjectAutomata\TechTreeGenericUnlockEffectAction.cs:
  120  				break;
  121: 			case UnlockEffectActionType.FORMULA:
  122  			{
```

`ExportedProject\Assets\Scripts\Assembly-CSharp\ProjectAutomata\GlobalMarket.cs`

```csharp

private float GetUpkeepPriceComponent(Recipe recipe)
{
    if (!upkeepComponentFormula)
    {
        return 0f;
    }
    ReadOnlyList<Building> originsOfRecipe = RecipeDatabase.instance.GetOriginsOfRecipe(recipe);
    if (!originsOfRecipe.notNull || originsOfRecipe.count <= 0)
    {
        Debug.LogError("Cannot find origin of recipe '" + recipe.Title + "'!");
        return 0f;
    }
    Upkeep component = originsOfRecipe[0].GetComponent<Upkeep>();
    if (!component)
    {
        return 0f;
    }
    float monthlyUpkeep = component.GetMonthlyUpkeep(Upkeep.GetMonthlyUpkeepOptions.PURE);
    Building building = recipe.requiredModules.FirstOrDefault();
    float? obj;
    if ((object)building == null)
    {
        obj = null;
    }
    else
    {
        Module component2 = building.GetComponent<Module>();
        obj = (((object)component2 != null) ? new float?(component2.ModuleUpkeep()) : null);
    }
    float moduleUpkeep = obj ?? 0f;
    ProductPriceUpkeepComponentFormulaArguments argumentsProvider = new ProductPriceUpkeepComponentFormulaArguments(monthlyUpkeep, moduleUpkeep);
    return (float)upkeepComponentFormula.Evaluate(argumentsProvider);
}

private float ComputeIngredientsValue(Recipe recipe)
{
    float num = 0f;
    foreach (Product entry in recipe.ingredients.entries)
    {
        if (!entry.definition)
        {
            Debug.LogErrorFormat("Null ingredient in recipe '{0}'.", recipe.name);
            continue;
        }
        ProductPricingInfo value;
        if (!_pricingInfoByProduct.TryGetValue(entry.definition, out value))
        {
            Recipe productRecipe = GetProductRecipe(entry.definition);
            if (!productRecipe)
            {
                Debug.LogErrorFormat("No recipe for product {0}.", entry.definition.productName);
                continue;
            }
            InizializeRecipePricingInfo(productRecipe);
            if (!_pricingInfoByProduct.TryGetValue(entry.definition, out value))
            {
                Debug.LogErrorFormat("Could not compute price for product {0}.", entry.definition.productName);
                continue;
            }
        }
        num += value.value * (float)entry.amount;
    }
    return num;
}

private Recipe GetProductRecipe(ProductDefinition product)
{
    ReadOnlyList<Recipe> recipes = RecipeDatabase.instance.GetRecipes(product);
    if (!recipes.notNull)
    {
        return null;
    }
    return recipes.FirstOrDefault();
}

private ProductPricingInfo ComputeProductPrice(ProductDefinition product, float upkeepComponent, float ingredientsValue, int productOutput, int recipeOutput, float recipeDays)
{
    ProductPricingInfo value;
    if (_pricingInfoByProduct.TryGetValue(product, out value))
    {
        return value;
    }
    Formula value2;
    if (!_formulasByProduct.TryGetValue(product, out value2) || value2 == null)
    {
        Debug.LogErrorFormat("No price formula for product '{0}'.", product.productName);
        return new ProductPricingInfo();
    }
    ProductPriceFormulaArguments argumentsProvider = new ProductPriceFormulaArguments(upkeepComponent, ingredientsValue, productOutput, recipeOutput, recipeDays);
    float num = (float)value2.Evaluate(argumentsProvider);
    value = new ProductPricingInfo
    {
        value = num,
        price = num * (product.productCategory ? product.productCategory.priceMultiplier : 1f)
    };
    _pricingInfoByProduct[product] = value;
    return value;
}
```