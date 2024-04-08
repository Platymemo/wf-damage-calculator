use lzma;
use std::{
    io::{self, Cursor, Read},
    path::Path,
    slice::Iter,
};

const MANIFEST_URL: &str = "https://origin.warframe.com/PublicExport/index_en.txt.lzma";
const MANIFEST_PATH: &str = "./index_en.txt.lzma";

// Returns the newline separated list of all exportable categories.
pub fn get_manifest() -> Result<String, BinError> {
    let lzma = Path::new(MANIFEST_PATH);
    if !lzma.exists() {
        download_binary(MANIFEST_URL, lzma)?;
    }

    let mut output = String::new();
    let _ = lzma::open(lzma)?.read_to_string(&mut output);

    Ok(output)
}

#[derive(Debug)]
pub enum BinError {
    Reqwest(reqwest::Error),
    Io(io::Error),
    Lzma(lzma::Error),
}

impl From<reqwest::Error> for BinError {
    fn from(err: reqwest::Error) -> Self {
        BinError::Reqwest(err)
    }
}

impl From<io::Error> for BinError {
    fn from(err: io::Error) -> Self {
        BinError::Io(err)
    }
}

impl From<lzma::Error> for BinError {
    fn from(err: lzma::Error) -> Self {
        BinError::Lzma(err)
    }
}

// DANGEROUS!!!
// Make sure the URL is trusted!
fn download_binary<P: AsRef<Path>>(url: &str, path: P) -> Result<(), BinError> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(path)?;
    let mut content = Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

// Each category available through the manifest
#[derive(Debug, Clone, Copy)]
pub enum Category {
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
    Weapons,
    Images,
}

impl Category {
    // String representation of this category, for use in reading the manifest
    pub const fn as_str(&self) -> &str {
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
            Category::Weapons => "ExportWeapons",
            Category::Images => "ExportManifest",
        }
    }

    fn iter() -> Iter<'static, Category> {
        static CATEGORIES: [Category; 16] = [
            Category::Skins,
            Category::Extractors,
            Category::Flavors,
            Category::ModBundles,
            Category::Gear,
            Category::MissionKeys,
            Category::Blueprints,
            Category::StarChartNodes,
            Category::RelicsAndArcanes,
            Category::ResourcesScenesAndDecor,
            Category::Companions,
            Category::Rewards,
            Category::Mods,
            Category::Warframes,
            Category::Weapons,
            Category::Images,
        ];
        CATEGORIES.iter()
    }

    // Finds the first matching category, if any
    pub fn get_match(other: &str) -> Option<Category> {
        for &category in Category::iter() {
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
