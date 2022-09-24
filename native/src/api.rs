// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
use crate::items::Item;
use crate::nutrition::{ItemInfo, Nutritional};
//use flutter_rust_bridge::ZeroCopyBuffer;
use std::vec::Vec;

pub enum Platform {
    Unknown,
    Android,
    Ios,
    Windows,
    Unix,
    MacIntel,
    MacApple,
    Wasm,
}

// A function definition in Rust. Similar to Dart, the return type must always be named
// and is never inferred.
pub fn platform() -> Platform {
    // This is a macro, a special expression that expands into code. In Rust, all macros
    // end with an exclamation mark and can be invoked with all kinds of brackets (parentheses,
    // brackets and curly braces). However, certain conventions exist, for example the
    // vector macro is almost always invoked as vec![..].
    //
    // The cfg!() macro returns a boolean value based on the current compiler configuration.
    // When attached to expressions (#[cfg(..)] form), they show or hide the expression at compile time.
    // Here, however, they evaluate to runtime values, which may or may not be optimized out
    // by the compiler. A variety of configurations are demonstrated here which cover most of
    // the modern oeprating systems. Try running the Flutter application on different machines
    // and see if it matches your expected OS.
    //
    // Furthermore, in Rust, the last expression in a function is the return value and does
    // not have the trailing semicolon. This entire if-else chain forms a single expression.
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::Ios
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Platform::MacApple
    } else if cfg!(target_os = "macos") {
        Platform::MacIntel
    } else if cfg!(target_family = "wasm") {
        Platform::Wasm
    } else if cfg!(unix) {
        Platform::Unix
    } else {
        Platform::Unknown
    }
}

// The convention for Rust identifiers is the snake_case,
// and they are automatically converted to camelCase on the Dart side.
pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}

pub fn get_nutrition(item: Item) -> ItemInfo {
    ItemInfo::with_nutrition(item)
}

pub fn search_local_items(input: String) -> Vec<Item> {
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;
    use std::str::FromStr;
    use strum::VariantNames;
    let matcher = SkimMatcherV2::default();
    let mut display_items: Vec<Item> = vec![];
    for item_string in Item::VARIANTS.iter() {
        if matcher.fuzzy_match(item_string, &input).is_some() {
            match Item::from_str(item_string) {
                Ok(item) => display_items.push(item),
                Err(err) => {
                    panic!(
                        "item string was not in Item enum despite being generated from Item: {err}"
                    )
                }
            };
        }
    }
    display_items
}

/*
pub enum NewItem {
    LargeChickenEgg,
    LargeChickenEggYolk,
    LargeChickenEggWhite,
    TableSalt,
    TableSugar,
    Water,
    WheatFlour,
    ActiveDryYeast,
    CowButter,
}
pub fn search_local_items(input: String) -> Vec<NewItem> {
    vec![NewItem::LargeChickenEgg]
}
*/
