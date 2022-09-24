//this is where our generated item enum will go

//the items should be scraped from wikipedia commons
//a different strategy will be neccessary for each
//vegetable, cereal, fruit, meat categories
//and that doesnt even include weird thigs like rennet and yeast

//if a species name is found, keep the english name as well

//once an item is found, ALL of the images from its page
//should be copied down

//after the scrape, make a program that shows you the name of the
//item, and all the images for it, and you can select one image
//for each item, which will then be marked with SELECTED, so that
//the selection can be changed later

//the selections must then be normalized to a single resolution
//and file format, and then copied into images with names
//matching the items entry in items.rs
use strum::{Display, EnumString, EnumVariantNames};

#[repr(C)]
#[derive(EnumString, Display, Debug, EnumVariantNames)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Item {
    #[strum(serialize = "large chicken egg")]
    LargeChickenEgg,
    #[strum(serialize = "large chicken egg yolk")]
    LargeChickenEggYolk,
    #[strum(serialize = "large chicken egg white")]
    LargeChickenEggWhite,
    #[strum(serialize = "table salt")]
    TableSalt,
    #[strum(serialize = "table sugar")]
    TableSugar,
    #[strum(serialize = "water")]
    Water,
    #[strum(serialize = "wheat flour")]
    WheatFlour,
    #[strum(serialize = "active dry yeast")]
    ActiveDryYeast,
    #[strum(serialize = "cow butter")]
    CowButter,
    NoMatch
}
