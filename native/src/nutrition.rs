//this is where our generated nutritional structs will go
//after a suitable number of items have been generated
//the scraper for this can walk the enum in items.rs and
//get nutritional info for each one, and generate structs
//for each item, holding its nutritional info, if no field
//is found (say for trans fat), mark the field 0

//all items should be have a weight in grams, which will allow
//us to calculate its 1g nutritional values for the struct definitions

//any unit item without a weight should be considered before adding

//aqua-calc can help us get a weight/liter field for volume/weight convert
use crate::items::Item;
//use strum::{Display, EnumString, EnumVariantNames, VariantNames};

static IMAGE_EXTENSION: &str = "jpg";

pub trait Nutritional {
    fn with_nutrition(item: Item) -> Self;
}

#[repr(C)]
pub struct Nutrition {
    //dont think we actually use variant anywhere
    pub variant: Item,
    pub unit_weight: Option<f64>,
    pub liter_weight: f64,
    pub calories: f64,
    pub calories_from_fat: f64,
    pub total_fat_weight: f64,
    pub saturated_fat_weight: f64,
    pub polyunsaturated_fat_weight: f64,
    pub monounsaturated_fat_weight: f64,
    pub cholesterol_weight: f64,
    pub sodium_weight: f64,
    pub potassium_weight: f64,
    pub carbohydrate_weight: f64,
    pub fiber_weight: f64,
    pub sugar_weight: f64,
    pub protein_weight: f64,
}

#[repr(C)]
pub struct ItemInfo {
    pub nutrition: Option<Nutrition>,
    pub file_name: String,
    pub display_name: String,
}

impl Nutritional for ItemInfo {
    fn with_nutrition(item: Item) -> Self {
        match item {
            Item::LargeChickenEgg => {
                let nutrition = Nutrition {
                    variant: Item::LargeChickenEgg,
                    unit_weight: Some(50.0),
                    liter_weight: 1028.0,
                    calories: 1.44,
                    calories_from_fat: 0.86,
                    total_fat_weight: 0.096,
                    saturated_fat_weight: 0.032,
                    polyunsaturated_fat_weight: 0.02,
                    monounsaturated_fat_weight: 0.036,
                    cholesterol_weight: 0.00372,
                    sodium_weight: 0.00142,
                    potassium_weight: 0.000138,
                    carbohydrate_weight: 0.008,
                    fiber_weight: 0.0,
                    sugar_weight: 0.004,
                    protein_weight: 0.128,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("large_chicken_egg.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::LargeChickenEggYolk => {
                let nutrition = Nutrition {
                    variant: Item::LargeChickenEggYolk,
                    unit_weight: Some(17.0),
                    liter_weight: 1028.0,
                    calories: 3.235294117647059,
                    calories_from_fat: 2.411764705882353,
                    total_fat_weight: 0.2647058823529412,
                    saturated_fat_weight: 0.0941176470588235,
                    polyunsaturated_fat_weight: 0.0411764705882353,
                    monounsaturated_fat_weight: 0.1176470588235294,
                    cholesterol_weight: 0.0108235294117647,
                    sodium_weight: 0.00048235294117647065,
                    potassium_weight: 0.0011176470588235294,
                    carbohydrate_weight: 0.03529411764705882,
                    fiber_weight: 0.0,
                    sugar_weight: 0.0058823529411764705,
                    protein_weight: 0.15882352941176472,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("large_chicken_egg_yolk.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::LargeChickenEggWhite => {
                let nutrition = Nutrition {
                    variant: Item::LargeChickenEggWhite,
                    unit_weight: Some(33.0),
                    liter_weight: 1028.0,
                    calories: 0.5151515151515151,
                    calories_from_fat: 0.015151515151515152,
                    total_fat_weight: 0.0030303030303030303,
                    saturated_fat_weight: 0.0,
                    polyunsaturated_fat_weight: 0.0,
                    monounsaturated_fat_weight: 0.0,
                    cholesterol_weight: 0.0,
                    sodium_weight: 0.0016666666666666668,
                    potassium_weight: 0.0016363636363636363,
                    carbohydrate_weight: 0.006060606060606061,
                    fiber_weight: 0.0,
                    sugar_weight: 0.006060606060606061,
                    protein_weight: 0.1090909090909091,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("large_chicken_egg_white.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::TableSalt => {
                let nutrition = Nutrition {
                    variant: Item::TableSalt,
                    unit_weight: None,
                    liter_weight: 1150.0,
                    calories: 0.0,
                    calories_from_fat: 0.0,
                    total_fat_weight: 0.0,
                    saturated_fat_weight: 0.0,
                    polyunsaturated_fat_weight: 0.0,
                    monounsaturated_fat_weight: 0.0,
                    cholesterol_weight: 0.0,
                    sodium_weight: 0.3876,
                    potassium_weight: 0.0,
                    carbohydrate_weight: 0.0,
                    fiber_weight: 0.0,
                    sugar_weight: 0.0,
                    protein_weight: 0.0,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("table_salt.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::TableSugar => {
                let nutrition = Nutrition {
                    variant: Item::TableSugar,
                    unit_weight: None,
                    liter_weight: 844.0,
                    calories: 3.9,
                    calories_from_fat: 0.0,
                    total_fat_weight: 0.0,
                    saturated_fat_weight: 0.0,
                    polyunsaturated_fat_weight: 0.0,
                    monounsaturated_fat_weight: 0.0,
                    cholesterol_weight: 0.0,
                    sodium_weight: 0.0095,
                    potassium_weight: 0.0,
                    carbohydrate_weight: 1.0,
                    fiber_weight: 0.0,
                    sugar_weight: 1.0,
                    protein_weight: 0.0,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("table_sugar.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::Water => {
                let nutrition = Nutrition {
                    variant: Item::Water,
                    unit_weight: None,
                    liter_weight: 1000.0,
                    calories: 0.0,
                    calories_from_fat: 0.0,
                    total_fat_weight: 0.0,
                    saturated_fat_weight: 0.0,
                    polyunsaturated_fat_weight: 0.0,
                    monounsaturated_fat_weight: 0.0,
                    cholesterol_weight: 0.0,
                    sodium_weight: 0.00004008438818565401,
                    potassium_weight: 0.0,
                    carbohydrate_weight: 0.0,
                    fiber_weight: 0.0,
                    sugar_weight: 0.0,
                    protein_weight: 0.0,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("water.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::WheatFlour => {
                let nutrition = Nutrition {
                    variant: Item::WheatFlour,
                    unit_weight: None,
                    liter_weight: 508.0,
                    calories: 3.64,
                    calories_from_fat: 0.088,
                    total_fat_weight: 0.0096,
                    saturated_fat_weight: 0.0016,
                    polyunsaturated_fat_weight: 0.004,
                    monounsaturated_fat_weight: 0.0008,
                    cholesterol_weight: 0.0,
                    sodium_weight: 0.00002,
                    potassium_weight: 0.001072,
                    carbohydrate_weight: 0.76,
                    fiber_weight: 0.0272,
                    sugar_weight: 0.0024,
                    protein_weight: 0.104,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("wheat_flour.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::ActiveDryYeast => {
                let nutrition = Nutrition {
                    variant: Item::ActiveDryYeast,
                    unit_weight: None,
                    liter_weight: 588.2352941176471,
                    calories: 3.3,
                    calories_from_fat: 0.7,
                    total_fat_weight: 0.1,
                    saturated_fat_weight: 0.0,
                    polyunsaturated_fat_weight: 0.0,
                    monounsaturated_fat_weight: 0.0,
                    cholesterol_weight: 0.0,
                    sodium_weight: 0.0005,
                    potassium_weight: 0.0096,
                    carbohydrate_weight: 0.4,
                    fiber_weight: 0.3,
                    sugar_weight: 0.0,
                    protein_weight: 0.4,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("active_dry_yeast.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::CowButter => {
                let nutrition = Nutrition {
                    variant: Item::CowButter,
                    unit_weight: None,
                    liter_weight: 948.0,
                    calories: 7.2,
                    calories_from_fat: 7.3,
                    total_fat_weight: 0.8,
                    saturated_fat_weight: 0.5,
                    polyunsaturated_fat_weight: 0.0,
                    monounsaturated_fat_weight: 0.02,
                    cholesterol_weight: 0.0022,
                    sodium_weight: 0.0064,
                    potassium_weight: 0.0002,
                    carbohydrate_weight: 0.0,
                    fiber_weight: 0.0,
                    sugar_weight: 0.0,
                    protein_weight: 0.0,
                };
                ItemInfo {
                    nutrition: Some(nutrition),
                    file_name: format!("cow_butter.{IMAGE_EXTENSION}"),
                    display_name: item.to_string(),
                }
            }
            Item::NoMatch => ItemInfo {
                nutrition: None,
                file_name: "no_image.{IMAGE_EXTENSION}".into(),
                display_name: "Item not matched in native/src/items.rs".into(),
            },
        }
    }
}
