use wf_api::{manifest::Category, weapons::get_weapons};

mod wf_api;

fn main() {
    let manifest: String = wf_api::manifest::get_manifest().expect("Could not download manifest");

    // Map each category to its corresponding schema
    for line in manifest.lines() {
        if let Some(category) = Category::get_match(line) {
            println!("{:?}", category);
            // map to schema
            match category {
                Category::Skins => {}
                Category::Extractors => {}
                Category::Flavors => {}
                Category::ModBundles => {}
                Category::Gear => {}
                Category::MissionKeys => {}
                Category::Blueprints => {}
                Category::StarChartNodes => {}
                Category::RelicsAndArcanes => {}
                Category::ResourcesScenesAndDecor => {}
                Category::Companions => {}
                Category::Rewards => {}
                Category::Mods => {}
                Category::Warframes => {}
                Category::Weapons => println!("{:?}", get_weapons(line).first()),
                Category::Images => {}
            }
        }
    }
}
