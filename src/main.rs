use serde::de::DeserializeOwned;

mod manifest;
mod weapons;

fn main() {
    let manifest: String = match manifest::get_manifest() {
        Ok(val) => val,
        Err(_) => todo!(), // Handle error, probably crash?
    };

    // Map each category to its corresponding schema
    for line in manifest.lines() {
        if let Some(category) = Category::get_match(line) {
            
        }
    }

    
    

    println!("{}", manifest);

    //println!("{}", download_json::<serde_json::Value>("http://content.warframe.com/PublicExport/Manifest/ExportWeapons_en.json!00_3syhuTxikRhJ8-abXbtF2Q"));
}

// Each category available through the manifest
#[derive(Debug)]
enum Category {
    Skins,
    Extractors,
    Flavors,
    ModBundles,
    Gear,
    MissionKeys,
    Blueprints,
    StarChartNodes,
    RelicsAndArcanes,
    ResourcesScenesAndDecor,
    Companions,
    Rewards,
    Mods,
    Warframes,
    Weapons
}

impl Category {
    // String representation of this category, for use in reading the manifest
    const fn as_str(&self) -> &str {
        match self {
            Category::Skins => "ExportCustoms",
            Category::Extractors => "ExportDrones",
            Category::Flavors => "ExportFlavour",
            Category::ModBundles => "ExportFusionBundles",
            Category::Gear => "ExportGear",
            Category::MissionKeys => "ExportKeys",
            Category::Blueprints => "ExportRecipes",
            Category::StarChartNodes => "ExportRegions",
            Category::RelicsAndArcanes => "ExportRelicArcane",
            Category::ResourcesScenesAndDecor => "ExportResources",
            Category::Companions => "ExportSentinels",
            Category::Rewards => "ExportSortieRewards",
            Category::Mods => "ExportUpgrades",
            Category::Warframes => "ExportWarframes",
            Category::Weapons => "ExportWeapons"
        }
    }

    // Returns the "first" category so we can iterate through all of them
    const fn iter() -> Category {
        Category::Skins
    }

    // Finds the first matching category, if any
    fn get_match(other: &str) -> Option<Category> {
        for category in Category::iter() {
            if category.matches(other) {
                return Some(category);
            }
        }

        return None;
    }

    // Whether or not the string contains this category's string representation
    fn matches(&self, other: &str) -> bool {
        other.contains(self.as_str())
    }
}

impl Iterator for Category {
    type Item = Category;

    // Goes through the enum by increasing ordinal
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Category::Skins => Some(Category::Extractors),
            Category::Extractors => Some(Category::Flavors),
            Category::Flavors => Some(Category::ModBundles),
            Category::ModBundles => Some(Category::Gear),
            Category::Gear => Some(Category::MissionKeys),
            Category::MissionKeys => Some(Category::Blueprints),
            Category::Blueprints => Some(Category::StarChartNodes),
            Category::StarChartNodes => Some(Category::RelicsAndArcanes),
            Category::RelicsAndArcanes => Some(Category::ResourcesScenesAndDecor),
            Category::ResourcesScenesAndDecor => Some(Category::Companions),
            Category::Companions => Some(Category::Rewards),
            Category::Rewards => Some(Category::Mods),
            Category::Mods => Some(Category::Warframes),
            Category::Warframes => Some(Category::Weapons),
            Category::Weapons => None
        }
    }
}

pub fn download_json<T: DeserializeOwned>(url: &str) -> T {
    let body = reqwest::blocking::get(url).expect(&format!("Could not download from {url}")).text().expect(&format!("Could not download from {url}"));
    serde_json::from_str::<T>(&body.replace("\r", "\\r").replace("\n", "\\n")).expect("Could not deserialize response body")
}