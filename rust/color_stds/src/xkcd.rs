//! https://xkcd.com/color/rgb.txt
//! Edited from the original file with Vim's visual block and regex-replace :)
//! Slashes within names were substituted by "_or_" to prevent snake case warnings.
//! Single quotes within names were simply removed.

use Rgb24;
#[cfg(any(feature="tables", feature="xkcd_table"))]
use Entry;

// This thing takes 36 secs to compile - this is crazy.
#[cfg(any(feature="tables", feature="xkcd_table"))]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const XKCD_COLORS : &[Entry] = &[
    Entry { ident: "cloudy_blue", value: hex24!(0xacc2d9) },
    Entry { ident: "dark_pastel_green", value: hex24!(0x56ae57) },
    Entry { ident: "dust", value: hex24!(0xb2996e) },
    Entry { ident: "electric_lime", value: hex24!(0xa8ff04) },
    Entry { ident: "fresh_green", value: hex24!(0x69d84f) },
    Entry { ident: "light_eggplant", value: hex24!(0x894585) },
    Entry { ident: "nasty_green", value: hex24!(0x70b23f) },
    Entry { ident: "really_light_blue", value: hex24!(0xd4ffff) },
    Entry { ident: "tea", value: hex24!(0x65ab7c) },
    Entry { ident: "warm_purple", value: hex24!(0x952e8f) },
    Entry { ident: "yellowish_tan", value: hex24!(0xfcfc81) },
    Entry { ident: "cement", value: hex24!(0xa5a391) },
    Entry { ident: "dark_grass_green", value: hex24!(0x388004) },
    Entry { ident: "dusty_teal", value: hex24!(0x4c9085) },
    Entry { ident: "grey_teal", value: hex24!(0x5e9b8a) },
    Entry { ident: "macaroni_and_cheese", value: hex24!(0xefb435) },
    Entry { ident: "pinkish_tan", value: hex24!(0xd99b82) },
    Entry { ident: "spruce", value: hex24!(0x0a5f38) },
    Entry { ident: "strong_blue", value: hex24!(0x0c06f7) },
    Entry { ident: "toxic_green", value: hex24!(0x61de2a) },
    Entry { ident: "windows_blue", value: hex24!(0x3778bf) },
    Entry { ident: "blue_blue", value: hex24!(0x2242c7) },
    Entry { ident: "blue_with_a_hint_of_purple", value: hex24!(0x533cc6) },
    Entry { ident: "booger", value: hex24!(0x9bb53c) },
    Entry { ident: "bright_sea_green", value: hex24!(0x05ffa6) },
    Entry { ident: "dark_green_blue", value: hex24!(0x1f6357) },
    Entry { ident: "deep_turquoise", value: hex24!(0x017374) },
    Entry { ident: "green_teal", value: hex24!(0x0cb577) },
    Entry { ident: "strong_pink", value: hex24!(0xff0789) },
    Entry { ident: "bland", value: hex24!(0xafa88b) },
    Entry { ident: "deep_aqua", value: hex24!(0x08787f) },
    Entry { ident: "lavender_pink", value: hex24!(0xdd85d7) },
    Entry { ident: "light_moss_green", value: hex24!(0xa6c875) },
    Entry { ident: "light_seafoam_green", value: hex24!(0xa7ffb5) },
    Entry { ident: "olive_yellow", value: hex24!(0xc2b709) },
    Entry { ident: "pig_pink", value: hex24!(0xe78ea5) },
    Entry { ident: "deep_lilac", value: hex24!(0x966ebd) },
    Entry { ident: "desert", value: hex24!(0xccad60) },
    Entry { ident: "dusty_lavender", value: hex24!(0xac86a8) },
    Entry { ident: "purpley_grey", value: hex24!(0x947e94) },
    Entry { ident: "purply", value: hex24!(0x983fb2) },
    Entry { ident: "candy_pink", value: hex24!(0xff63e9) },
    Entry { ident: "light_pastel_green", value: hex24!(0xb2fba5) },
    Entry { ident: "boring_green", value: hex24!(0x63b365) },
    Entry { ident: "kiwi_green", value: hex24!(0x8ee53f) },
    Entry { ident: "light_grey_green", value: hex24!(0xb7e1a1) },
    Entry { ident: "orange_pink", value: hex24!(0xff6f52) },
    Entry { ident: "tea_green", value: hex24!(0xbdf8a3) },
    Entry { ident: "very_light_brown", value: hex24!(0xd3b683) },
    Entry { ident: "egg_shell", value: hex24!(0xfffcc4) },
    Entry { ident: "eggplant_purple", value: hex24!(0x430541) },
    Entry { ident: "powder_pink", value: hex24!(0xffb2d0) },
    Entry { ident: "reddish_grey", value: hex24!(0x997570) },
    Entry { ident: "baby_shit_brown", value: hex24!(0xad900d) },
    Entry { ident: "liliac", value: hex24!(0xc48efd) },
    Entry { ident: "stormy_blue", value: hex24!(0x507b9c) },
    Entry { ident: "ugly_brown", value: hex24!(0x7d7103) },
    Entry { ident: "custard", value: hex24!(0xfffd78) },
    Entry { ident: "darkish_pink", value: hex24!(0xda467d) },
    Entry { ident: "deep_brown", value: hex24!(0x410200) },
    Entry { ident: "greenish_beige", value: hex24!(0xc9d179) },
    Entry { ident: "manilla", value: hex24!(0xfffa86) },
    Entry { ident: "off_blue", value: hex24!(0x5684ae) },
    Entry { ident: "battleship_grey", value: hex24!(0x6b7c85) },
    Entry { ident: "browny_green", value: hex24!(0x6f6c0a) },
    Entry { ident: "bruise", value: hex24!(0x7e4071) },
    Entry { ident: "kelley_green", value: hex24!(0x009337) },
    Entry { ident: "sickly_yellow", value: hex24!(0xd0e429) },
    Entry { ident: "sunny_yellow", value: hex24!(0xfff917) },
    Entry { ident: "azul", value: hex24!(0x1d5dec) },
    Entry { ident: "darkgreen", value: hex24!(0x054907) },
    Entry { ident: "green_or_yellow", value: hex24!(0xb5ce08) },
    Entry { ident: "lichen", value: hex24!(0x8fb67b) },
    Entry { ident: "light_light_green", value: hex24!(0xc8ffb0) },
    Entry { ident: "pale_gold", value: hex24!(0xfdde6c) },
    Entry { ident: "sun_yellow", value: hex24!(0xffdf22) },
    Entry { ident: "tan_green", value: hex24!(0xa9be70) },
    Entry { ident: "burple", value: hex24!(0x6832e3) },
    Entry { ident: "butterscotch", value: hex24!(0xfdb147) },
    Entry { ident: "toupe", value: hex24!(0xc7ac7d) },
    Entry { ident: "dark_cream", value: hex24!(0xfff39a) },
    Entry { ident: "indian_red", value: hex24!(0x850e04) },
    Entry { ident: "light_lavendar", value: hex24!(0xefc0fe) },
    Entry { ident: "poison_green", value: hex24!(0x40fd14) },
    Entry { ident: "baby_puke_green", value: hex24!(0xb6c406) },
    Entry { ident: "bright_yellow_green", value: hex24!(0x9dff00) },
    Entry { ident: "charcoal_grey", value: hex24!(0x3c4142) },
    Entry { ident: "squash", value: hex24!(0xf2ab15) },
    Entry { ident: "cinnamon", value: hex24!(0xac4f06) },
    Entry { ident: "light_pea_green", value: hex24!(0xc4fe82) },
    Entry { ident: "radioactive_green", value: hex24!(0x2cfa1f) },
    Entry { ident: "raw_sienna", value: hex24!(0x9a6200) },
    Entry { ident: "baby_purple", value: hex24!(0xca9bf7) },
    Entry { ident: "cocoa", value: hex24!(0x875f42) },
    Entry { ident: "light_royal_blue", value: hex24!(0x3a2efe) },
    Entry { ident: "orangeish", value: hex24!(0xfd8d49) },
    Entry { ident: "rust_brown", value: hex24!(0x8b3103) },
    Entry { ident: "sand_brown", value: hex24!(0xcba560) },
    Entry { ident: "swamp", value: hex24!(0x698339) },
    Entry { ident: "tealish_green", value: hex24!(0x0cdc73) },
    Entry { ident: "burnt_siena", value: hex24!(0xb75203) },
    Entry { ident: "camo", value: hex24!(0x7f8f4e) },
    Entry { ident: "dusk_blue", value: hex24!(0x26538d) },
    Entry { ident: "fern", value: hex24!(0x63a950) },
    Entry { ident: "old_rose", value: hex24!(0xc87f89) },
    Entry { ident: "pale_light_green", value: hex24!(0xb1fc99) },
    Entry { ident: "peachy_pink", value: hex24!(0xff9a8a) },
    Entry { ident: "rosy_pink", value: hex24!(0xf6688e) },
    Entry { ident: "light_bluish_green", value: hex24!(0x76fda8) },
    Entry { ident: "light_bright_green", value: hex24!(0x53fe5c) },
    Entry { ident: "light_neon_green", value: hex24!(0x4efd54) },
    Entry { ident: "light_seafoam", value: hex24!(0xa0febf) },
    Entry { ident: "tiffany_blue", value: hex24!(0x7bf2da) },
    Entry { ident: "washed_out_green", value: hex24!(0xbcf5a6) },
    Entry { ident: "browny_orange", value: hex24!(0xca6b02) },
    Entry { ident: "nice_blue", value: hex24!(0x107ab0) },
    Entry { ident: "sapphire", value: hex24!(0x2138ab) },
    Entry { ident: "greyish_teal", value: hex24!(0x719f91) },
    Entry { ident: "orangey_yellow", value: hex24!(0xfdb915) },
    Entry { ident: "parchment", value: hex24!(0xfefcaf) },
    Entry { ident: "straw", value: hex24!(0xfcf679) },
    Entry { ident: "very_dark_brown", value: hex24!(0x1d0200) },
    Entry { ident: "terracota", value: hex24!(0xcb6843) },
    Entry { ident: "ugly_blue", value: hex24!(0x31668a) },
    Entry { ident: "clear_blue", value: hex24!(0x247afd) },
    Entry { ident: "creme", value: hex24!(0xffffb6) },
    Entry { ident: "foam_green", value: hex24!(0x90fda9) },
    Entry { ident: "grey_or_green", value: hex24!(0x86a17d) },
    Entry { ident: "light_gold", value: hex24!(0xfddc5c) },
    Entry { ident: "seafoam_blue", value: hex24!(0x78d1b6) },
    Entry { ident: "topaz", value: hex24!(0x13bbaf) },
    Entry { ident: "violet_pink", value: hex24!(0xfb5ffc) },
    Entry { ident: "wintergreen", value: hex24!(0x20f986) },
    Entry { ident: "yellow_tan", value: hex24!(0xffe36e) },
    Entry { ident: "dark_fuchsia", value: hex24!(0x9d0759) },
    Entry { ident: "indigo_blue", value: hex24!(0x3a18b1) },
    Entry { ident: "light_yellowish_green", value: hex24!(0xc2ff89) },
    Entry { ident: "pale_magenta", value: hex24!(0xd767ad) },
    Entry { ident: "rich_purple", value: hex24!(0x720058) },
    Entry { ident: "sunflower_yellow", value: hex24!(0xffda03) },
    Entry { ident: "green_or_blue", value: hex24!(0x01c08d) },
    Entry { ident: "leather", value: hex24!(0xac7434) },
    Entry { ident: "racing_green", value: hex24!(0x014600) },
    Entry { ident: "vivid_purple", value: hex24!(0x9900fa) },
    Entry { ident: "dark_royal_blue", value: hex24!(0x02066f) },
    Entry { ident: "hazel", value: hex24!(0x8e7618) },
    Entry { ident: "muted_pink", value: hex24!(0xd1768f) },
    Entry { ident: "booger_green", value: hex24!(0x96b403) },
    Entry { ident: "canary", value: hex24!(0xfdff63) },
    Entry { ident: "cool_grey", value: hex24!(0x95a3a6) },
    Entry { ident: "dark_taupe", value: hex24!(0x7f684e) },
    Entry { ident: "darkish_purple", value: hex24!(0x751973) },
    Entry { ident: "true_green", value: hex24!(0x089404) },
    Entry { ident: "coral_pink", value: hex24!(0xff6163) },
    Entry { ident: "dark_sage", value: hex24!(0x598556) },
    Entry { ident: "dark_slate_blue", value: hex24!(0x214761) },
    Entry { ident: "flat_blue", value: hex24!(0x3c73a8) },
    Entry { ident: "mushroom", value: hex24!(0xba9e88) },
    Entry { ident: "rich_blue", value: hex24!(0x021bf9) },
    Entry { ident: "dirty_purple", value: hex24!(0x734a65) },
    Entry { ident: "greenblue", value: hex24!(0x23c48b) },
    Entry { ident: "icky_green", value: hex24!(0x8fae22) },
    Entry { ident: "light_khaki", value: hex24!(0xe6f2a2) },
    Entry { ident: "warm_blue", value: hex24!(0x4b57db) },
    Entry { ident: "dark_hot_pink", value: hex24!(0xd90166) },
    Entry { ident: "deep_sea_blue", value: hex24!(0x015482) },
    Entry { ident: "carmine", value: hex24!(0x9d0216) },
    Entry { ident: "dark_yellow_green", value: hex24!(0x728f02) },
    Entry { ident: "pale_peach", value: hex24!(0xffe5ad) },
    Entry { ident: "plum_purple", value: hex24!(0x4e0550) },
    Entry { ident: "golden_rod", value: hex24!(0xf9bc08) },
    Entry { ident: "neon_red", value: hex24!(0xff073a) },
    Entry { ident: "old_pink", value: hex24!(0xc77986) },
    Entry { ident: "very_pale_blue", value: hex24!(0xd6fffe) },
    Entry { ident: "blood_orange", value: hex24!(0xfe4b03) },
    Entry { ident: "grapefruit", value: hex24!(0xfd5956) },
    Entry { ident: "sand_yellow", value: hex24!(0xfce166) },
    Entry { ident: "clay_brown", value: hex24!(0xb2713d) },
    Entry { ident: "dark_blue_grey", value: hex24!(0x1f3b4d) },
    Entry { ident: "flat_green", value: hex24!(0x699d4c) },
    Entry { ident: "light_green_blue", value: hex24!(0x56fca2) },
    Entry { ident: "warm_pink", value: hex24!(0xfb5581) },
    Entry { ident: "dodger_blue", value: hex24!(0x3e82fc) },
    Entry { ident: "gross_green", value: hex24!(0xa0bf16) },
    Entry { ident: "ice", value: hex24!(0xd6fffa) },
    Entry { ident: "metallic_blue", value: hex24!(0x4f738e) },
    Entry { ident: "pale_salmon", value: hex24!(0xffb19a) },
    Entry { ident: "sap_green", value: hex24!(0x5c8b15) },
    Entry { ident: "algae", value: hex24!(0x54ac68) },
    Entry { ident: "bluey_grey", value: hex24!(0x89a0b0) },
    Entry { ident: "greeny_grey", value: hex24!(0x7ea07a) },
    Entry { ident: "highlighter_green", value: hex24!(0x1bfc06) },
    Entry { ident: "light_light_blue", value: hex24!(0xcafffb) },
    Entry { ident: "light_mint", value: hex24!(0xb6ffbb) },
    Entry { ident: "raw_umber", value: hex24!(0xa75e09) },
    Entry { ident: "vivid_blue", value: hex24!(0x152eff) },
    Entry { ident: "deep_lavender", value: hex24!(0x8d5eb7) },
    Entry { ident: "dull_teal", value: hex24!(0x5f9e8f) },
    Entry { ident: "light_greenish_blue", value: hex24!(0x63f7b4) },
    Entry { ident: "mud_green", value: hex24!(0x606602) },
    Entry { ident: "pinky", value: hex24!(0xfc86aa) },
    Entry { ident: "red_wine", value: hex24!(0x8c0034) },
    Entry { ident: "shit_green", value: hex24!(0x758000) },
    Entry { ident: "tan_brown", value: hex24!(0xab7e4c) },
    Entry { ident: "darkblue", value: hex24!(0x030764) },
    Entry { ident: "rosa", value: hex24!(0xfe86a4) },
    Entry { ident: "lipstick", value: hex24!(0xd5174e) },
    Entry { ident: "pale_mauve", value: hex24!(0xfed0fc) },
    Entry { ident: "claret", value: hex24!(0x680018) },
    Entry { ident: "dandelion", value: hex24!(0xfedf08) },
    Entry { ident: "orangered", value: hex24!(0xfe420f) },
    Entry { ident: "poop_green", value: hex24!(0x6f7c00) },
    Entry { ident: "ruby", value: hex24!(0xca0147) },
    Entry { ident: "dark", value: hex24!(0x1b2431) },
    Entry { ident: "greenish_turquoise", value: hex24!(0x00fbb0) },
    Entry { ident: "pastel_red", value: hex24!(0xdb5856) },
    Entry { ident: "piss_yellow", value: hex24!(0xddd618) },
    Entry { ident: "bright_cyan", value: hex24!(0x41fdfe) },
    Entry { ident: "dark_coral", value: hex24!(0xcf524e) },
    Entry { ident: "algae_green", value: hex24!(0x21c36f) },
    Entry { ident: "darkish_red", value: hex24!(0xa90308) },
    Entry { ident: "reddy_brown", value: hex24!(0x6e1005) },
    Entry { ident: "blush_pink", value: hex24!(0xfe828c) },
    Entry { ident: "camouflage_green", value: hex24!(0x4b6113) },
    Entry { ident: "lawn_green", value: hex24!(0x4da409) },
    Entry { ident: "putty", value: hex24!(0xbeae8a) },
    Entry { ident: "vibrant_blue", value: hex24!(0x0339f8) },
    Entry { ident: "dark_sand", value: hex24!(0xa88f59) },
    Entry { ident: "purple_or_blue", value: hex24!(0x5d21d0) },
    Entry { ident: "saffron", value: hex24!(0xfeb209) },
    Entry { ident: "twilight", value: hex24!(0x4e518b) },
    Entry { ident: "warm_brown", value: hex24!(0x964e02) },
    Entry { ident: "bluegrey", value: hex24!(0x85a3b2) },
    Entry { ident: "bubble_gum_pink", value: hex24!(0xff69af) },
    Entry { ident: "duck_egg_blue", value: hex24!(0xc3fbf4) },
    Entry { ident: "greenish_cyan", value: hex24!(0x2afeb7) },
    Entry { ident: "petrol", value: hex24!(0x005f6a) },
    Entry { ident: "royal", value: hex24!(0x0c1793) },
    Entry { ident: "butter", value: hex24!(0xffff81) },
    Entry { ident: "dusty_orange", value: hex24!(0xf0833a) },
    Entry { ident: "off_yellow", value: hex24!(0xf1f33f) },
    Entry { ident: "pale_olive_green", value: hex24!(0xb1d27b) },
    Entry { ident: "orangish", value: hex24!(0xfc824a) },
    Entry { ident: "leaf", value: hex24!(0x71aa34) },
    Entry { ident: "light_blue_grey", value: hex24!(0xb7c9e2) },
    Entry { ident: "dried_blood", value: hex24!(0x4b0101) },
    Entry { ident: "lightish_purple", value: hex24!(0xa552e6) },
    Entry { ident: "rusty_red", value: hex24!(0xaf2f0d) },
    Entry { ident: "lavender_blue", value: hex24!(0x8b88f8) },
    Entry { ident: "light_grass_green", value: hex24!(0x9af764) },
    Entry { ident: "light_mint_green", value: hex24!(0xa6fbb2) },
    Entry { ident: "sunflower", value: hex24!(0xffc512) },
    Entry { ident: "velvet", value: hex24!(0x750851) },
    Entry { ident: "brick_orange", value: hex24!(0xc14a09) },
    Entry { ident: "lightish_red", value: hex24!(0xfe2f4a) },
    Entry { ident: "pure_blue", value: hex24!(0x0203e2) },
    Entry { ident: "twilight_blue", value: hex24!(0x0a437a) },
    Entry { ident: "violet_red", value: hex24!(0xa50055) },
    Entry { ident: "yellowy_brown", value: hex24!(0xae8b0c) },
    Entry { ident: "carnation", value: hex24!(0xfd798f) },
    Entry { ident: "muddy_yellow", value: hex24!(0xbfac05) },
    Entry { ident: "dark_seafoam_green", value: hex24!(0x3eaf76) },
    Entry { ident: "deep_rose", value: hex24!(0xc74767) },
    Entry { ident: "dusty_red", value: hex24!(0xb9484e) },
    Entry { ident: "grey_or_blue", value: hex24!(0x647d8e) },
    Entry { ident: "lemon_lime", value: hex24!(0xbffe28) },
    Entry { ident: "purple_or_pink", value: hex24!(0xd725de) },
    Entry { ident: "brown_yellow", value: hex24!(0xb29705) },
    Entry { ident: "purple_brown", value: hex24!(0x673a3f) },
    Entry { ident: "wisteria", value: hex24!(0xa87dc2) },
    Entry { ident: "banana_yellow", value: hex24!(0xfafe4b) },
    Entry { ident: "lipstick_red", value: hex24!(0xc0022f) },
    Entry { ident: "water_blue", value: hex24!(0x0e87cc) },
    Entry { ident: "brown_grey", value: hex24!(0x8d8468) },
    Entry { ident: "vibrant_purple", value: hex24!(0xad03de) },
    Entry { ident: "baby_green", value: hex24!(0x8cff9e) },
    Entry { ident: "barf_green", value: hex24!(0x94ac02) },
    Entry { ident: "eggshell_blue", value: hex24!(0xc4fff7) },
    Entry { ident: "sandy_yellow", value: hex24!(0xfdee73) },
    Entry { ident: "cool_green", value: hex24!(0x33b864) },
    Entry { ident: "pale", value: hex24!(0xfff9d0) },
    Entry { ident: "blue_or_grey", value: hex24!(0x758da3) },
    Entry { ident: "hot_magenta", value: hex24!(0xf504c9) },
    Entry { ident: "greyblue", value: hex24!(0x77a1b5) },
    Entry { ident: "purpley", value: hex24!(0x8756e4) },
    Entry { ident: "baby_shit_green", value: hex24!(0x889717) },
    Entry { ident: "brownish_pink", value: hex24!(0xc27e79) },
    Entry { ident: "dark_aquamarine", value: hex24!(0x017371) },
    Entry { ident: "diarrhea", value: hex24!(0x9f8303) },
    Entry { ident: "light_mustard", value: hex24!(0xf7d560) },
    Entry { ident: "pale_sky_blue", value: hex24!(0xbdf6fe) },
    Entry { ident: "turtle_green", value: hex24!(0x75b84f) },
    Entry { ident: "bright_olive", value: hex24!(0x9cbb04) },
    Entry { ident: "dark_grey_blue", value: hex24!(0x29465b) },
    Entry { ident: "greeny_brown", value: hex24!(0x696006) },
    Entry { ident: "lemon_green", value: hex24!(0xadf802) },
    Entry { ident: "light_periwinkle", value: hex24!(0xc1c6fc) },
    Entry { ident: "seaweed_green", value: hex24!(0x35ad6b) },
    Entry { ident: "sunshine_yellow", value: hex24!(0xfffd37) },
    Entry { ident: "ugly_purple", value: hex24!(0xa442a0) },
    Entry { ident: "medium_pink", value: hex24!(0xf36196) },
    Entry { ident: "puke_brown", value: hex24!(0x947706) },
    Entry { ident: "very_light_pink", value: hex24!(0xfff4f2) },
    Entry { ident: "viridian", value: hex24!(0x1e9167) },
    Entry { ident: "bile", value: hex24!(0xb5c306) },
    Entry { ident: "faded_yellow", value: hex24!(0xfeff7f) },
    Entry { ident: "very_pale_green", value: hex24!(0xcffdbc) },
    Entry { ident: "vibrant_green", value: hex24!(0x0add08) },
    Entry { ident: "bright_lime", value: hex24!(0x87fd05) },
    Entry { ident: "spearmint", value: hex24!(0x1ef876) },
    Entry { ident: "light_aquamarine", value: hex24!(0x7bfdc7) },
    Entry { ident: "light_sage", value: hex24!(0xbcecac) },
    Entry { ident: "yellowgreen", value: hex24!(0xbbf90f) },
    Entry { ident: "baby_poo", value: hex24!(0xab9004) },
    Entry { ident: "dark_seafoam", value: hex24!(0x1fb57a) },
    Entry { ident: "deep_teal", value: hex24!(0x00555a) },
    Entry { ident: "heather", value: hex24!(0xa484ac) },
    Entry { ident: "rust_orange", value: hex24!(0xc45508) },
    Entry { ident: "dirty_blue", value: hex24!(0x3f829d) },
    Entry { ident: "fern_green", value: hex24!(0x548d44) },
    Entry { ident: "bright_lilac", value: hex24!(0xc95efb) },
    Entry { ident: "weird_green", value: hex24!(0x3ae57f) },
    Entry { ident: "peacock_blue", value: hex24!(0x016795) },
    Entry { ident: "avocado_green", value: hex24!(0x87a922) },
    Entry { ident: "faded_orange", value: hex24!(0xf0944d) },
    Entry { ident: "grape_purple", value: hex24!(0x5d1451) },
    Entry { ident: "hot_green", value: hex24!(0x25ff29) },
    Entry { ident: "lime_yellow", value: hex24!(0xd0fe1d) },
    Entry { ident: "mango", value: hex24!(0xffa62b) },
    Entry { ident: "shamrock", value: hex24!(0x01b44c) },
    Entry { ident: "bubblegum", value: hex24!(0xff6cb5) },
    Entry { ident: "purplish_brown", value: hex24!(0x6b4247) },
    Entry { ident: "vomit_yellow", value: hex24!(0xc7c10c) },
    Entry { ident: "pale_cyan", value: hex24!(0xb7fffa) },
    Entry { ident: "key_lime", value: hex24!(0xaeff6e) },
    Entry { ident: "tomato_red", value: hex24!(0xec2d01) },
    Entry { ident: "lightgreen", value: hex24!(0x76ff7b) },
    Entry { ident: "merlot", value: hex24!(0x730039) },
    Entry { ident: "night_blue", value: hex24!(0x040348) },
    Entry { ident: "purpleish_pink", value: hex24!(0xdf4ec8) },
    Entry { ident: "apple", value: hex24!(0x6ecb3c) },
    Entry { ident: "baby_poop_green", value: hex24!(0x8f9805) },
    Entry { ident: "green_apple", value: hex24!(0x5edc1f) },
    Entry { ident: "heliotrope", value: hex24!(0xd94ff5) },
    Entry { ident: "yellow_or_green", value: hex24!(0xc8fd3d) },
    Entry { ident: "almost_black", value: hex24!(0x070d0d) },
    Entry { ident: "cool_blue", value: hex24!(0x4984b8) },
    Entry { ident: "leafy_green", value: hex24!(0x51b73b) },
    Entry { ident: "mustard_brown", value: hex24!(0xac7e04) },
    Entry { ident: "dusk", value: hex24!(0x4e5481) },
    Entry { ident: "dull_brown", value: hex24!(0x876e4b) },
    Entry { ident: "frog_green", value: hex24!(0x58bc08) },
    Entry { ident: "vivid_green", value: hex24!(0x2fef10) },
    Entry { ident: "bright_light_green", value: hex24!(0x2dfe54) },
    Entry { ident: "fluro_green", value: hex24!(0x0aff02) },
    Entry { ident: "kiwi", value: hex24!(0x9cef43) },
    Entry { ident: "seaweed", value: hex24!(0x18d17b) },
    Entry { ident: "navy_green", value: hex24!(0x35530a) },
    Entry { ident: "ultramarine_blue", value: hex24!(0x1805db) },
    Entry { ident: "iris", value: hex24!(0x6258c4) },
    Entry { ident: "pastel_orange", value: hex24!(0xff964f) },
    Entry { ident: "yellowish_orange", value: hex24!(0xffab0f) },
    Entry { ident: "perrywinkle", value: hex24!(0x8f8ce7) },
    Entry { ident: "tealish", value: hex24!(0x24bca8) },
    Entry { ident: "dark_plum", value: hex24!(0x3f012c) },
    Entry { ident: "pear", value: hex24!(0xcbf85f) },
    Entry { ident: "pinkish_orange", value: hex24!(0xff724c) },
    Entry { ident: "midnight_purple", value: hex24!(0x280137) },
    Entry { ident: "light_urple", value: hex24!(0xb36ff6) },
    Entry { ident: "dark_mint", value: hex24!(0x48c072) },
    Entry { ident: "greenish_tan", value: hex24!(0xbccb7a) },
    Entry { ident: "light_burgundy", value: hex24!(0xa8415b) },
    Entry { ident: "turquoise_blue", value: hex24!(0x06b1c4) },
    Entry { ident: "ugly_pink", value: hex24!(0xcd7584) },
    Entry { ident: "sandy", value: hex24!(0xf1da7a) },
    Entry { ident: "electric_pink", value: hex24!(0xff0490) },
    Entry { ident: "muted_purple", value: hex24!(0x805b87) },
    Entry { ident: "mid_green", value: hex24!(0x50a747) },
    Entry { ident: "greyish", value: hex24!(0xa8a495) },
    Entry { ident: "neon_yellow", value: hex24!(0xcfff04) },
    Entry { ident: "banana", value: hex24!(0xffff7e) },
    Entry { ident: "carnation_pink", value: hex24!(0xff7fa7) },
    Entry { ident: "tomato", value: hex24!(0xef4026) },
    Entry { ident: "sea", value: hex24!(0x3c9992) },
    Entry { ident: "muddy_brown", value: hex24!(0x886806) },
    Entry { ident: "turquoise_green", value: hex24!(0x04f489) },
    Entry { ident: "buff", value: hex24!(0xfef69e) },
    Entry { ident: "fawn", value: hex24!(0xcfaf7b) },
    Entry { ident: "muted_blue", value: hex24!(0x3b719f) },
    Entry { ident: "pale_rose", value: hex24!(0xfdc1c5) },
    Entry { ident: "dark_mint_green", value: hex24!(0x20c073) },
    Entry { ident: "amethyst", value: hex24!(0x9b5fc0) },
    Entry { ident: "blue_or_green", value: hex24!(0x0f9b8e) },
    Entry { ident: "chestnut", value: hex24!(0x742802) },
    Entry { ident: "sick_green", value: hex24!(0x9db92c) },
    Entry { ident: "pea", value: hex24!(0xa4bf20) },
    Entry { ident: "rusty_orange", value: hex24!(0xcd5909) },
    Entry { ident: "stone", value: hex24!(0xada587) },
    Entry { ident: "rose_red", value: hex24!(0xbe013c) },
    Entry { ident: "pale_aqua", value: hex24!(0xb8ffeb) },
    Entry { ident: "deep_orange", value: hex24!(0xdc4d01) },
    Entry { ident: "earth", value: hex24!(0xa2653e) },
    Entry { ident: "mossy_green", value: hex24!(0x638b27) },
    Entry { ident: "grassy_green", value: hex24!(0x419c03) },
    Entry { ident: "pale_lime_green", value: hex24!(0xb1ff65) },
    Entry { ident: "light_grey_blue", value: hex24!(0x9dbcd4) },
    Entry { ident: "pale_grey", value: hex24!(0xfdfdfe) },
    Entry { ident: "asparagus", value: hex24!(0x77ab56) },
    Entry { ident: "blueberry", value: hex24!(0x464196) },
    Entry { ident: "purple_red", value: hex24!(0x990147) },
    Entry { ident: "pale_lime", value: hex24!(0xbefd73) },
    Entry { ident: "greenish_teal", value: hex24!(0x32bf84) },
    Entry { ident: "caramel", value: hex24!(0xaf6f09) },
    Entry { ident: "deep_magenta", value: hex24!(0xa0025c) },
    Entry { ident: "light_peach", value: hex24!(0xffd8b1) },
    Entry { ident: "milk_chocolate", value: hex24!(0x7f4e1e) },
    Entry { ident: "ocher", value: hex24!(0xbf9b0c) },
    Entry { ident: "off_green", value: hex24!(0x6ba353) },
    Entry { ident: "purply_pink", value: hex24!(0xf075e6) },
    Entry { ident: "lightblue", value: hex24!(0x7bc8f6) },
    Entry { ident: "dusky_blue", value: hex24!(0x475f94) },
    Entry { ident: "golden", value: hex24!(0xf5bf03) },
    Entry { ident: "light_beige", value: hex24!(0xfffeb6) },
    Entry { ident: "butter_yellow", value: hex24!(0xfffd74) },
    Entry { ident: "dusky_purple", value: hex24!(0x895b7b) },
    Entry { ident: "french_blue", value: hex24!(0x436bad) },
    Entry { ident: "ugly_yellow", value: hex24!(0xd0c101) },
    Entry { ident: "greeny_yellow", value: hex24!(0xc6f808) },
    Entry { ident: "orangish_red", value: hex24!(0xf43605) },
    Entry { ident: "shamrock_green", value: hex24!(0x02c14d) },
    Entry { ident: "orangish_brown", value: hex24!(0xb25f03) },
    Entry { ident: "tree_green", value: hex24!(0x2a7e19) },
    Entry { ident: "deep_violet", value: hex24!(0x490648) },
    Entry { ident: "gunmetal", value: hex24!(0x536267) },
    Entry { ident: "blue_or_purple", value: hex24!(0x5a06ef) },
    Entry { ident: "cherry", value: hex24!(0xcf0234) },
    Entry { ident: "sandy_brown", value: hex24!(0xc4a661) },
    Entry { ident: "warm_grey", value: hex24!(0x978a84) },
    Entry { ident: "dark_indigo", value: hex24!(0x1f0954) },
    Entry { ident: "midnight", value: hex24!(0x03012d) },
    Entry { ident: "bluey_green", value: hex24!(0x2bb179) },
    Entry { ident: "grey_pink", value: hex24!(0xc3909b) },
    Entry { ident: "soft_purple", value: hex24!(0xa66fb5) },
    Entry { ident: "blood", value: hex24!(0x770001) },
    Entry { ident: "brown_red", value: hex24!(0x922b05) },
    Entry { ident: "medium_grey", value: hex24!(0x7d7f7c) },
    Entry { ident: "berry", value: hex24!(0x990f4b) },
    Entry { ident: "poo", value: hex24!(0x8f7303) },
    Entry { ident: "purpley_pink", value: hex24!(0xc83cb9) },
    Entry { ident: "light_salmon", value: hex24!(0xfea993) },
    Entry { ident: "snot", value: hex24!(0xacbb0d) },
    Entry { ident: "easter_purple", value: hex24!(0xc071fe) },
    Entry { ident: "light_yellow_green", value: hex24!(0xccfd7f) },
    Entry { ident: "dark_navy_blue", value: hex24!(0x00022e) },
    Entry { ident: "drab", value: hex24!(0x828344) },
    Entry { ident: "light_rose", value: hex24!(0xffc5cb) },
    Entry { ident: "rouge", value: hex24!(0xab1239) },
    Entry { ident: "purplish_red", value: hex24!(0xb0054b) },
    Entry { ident: "slime_green", value: hex24!(0x99cc04) },
    Entry { ident: "baby_poop", value: hex24!(0x937c00) },
    Entry { ident: "irish_green", value: hex24!(0x019529) },
    Entry { ident: "pink_or_purple", value: hex24!(0xef1de7) },
    Entry { ident: "dark_navy", value: hex24!(0x000435) },
    Entry { ident: "greeny_blue", value: hex24!(0x42b395) },
    Entry { ident: "light_plum", value: hex24!(0x9d5783) },
    Entry { ident: "pinkish_grey", value: hex24!(0xc8aca9) },
    Entry { ident: "dirty_orange", value: hex24!(0xc87606) },
    Entry { ident: "rust_red", value: hex24!(0xaa2704) },
    Entry { ident: "pale_lilac", value: hex24!(0xe4cbff) },
    Entry { ident: "orangey_red", value: hex24!(0xfa4224) },
    Entry { ident: "primary_blue", value: hex24!(0x0804f9) },
    Entry { ident: "kermit_green", value: hex24!(0x5cb200) },
    Entry { ident: "brownish_purple", value: hex24!(0x76424e) },
    Entry { ident: "murky_green", value: hex24!(0x6c7a0e) },
    Entry { ident: "wheat", value: hex24!(0xfbdd7e) },
    Entry { ident: "very_dark_purple", value: hex24!(0x2a0134) },
    Entry { ident: "bottle_green", value: hex24!(0x044a05) },
    Entry { ident: "watermelon", value: hex24!(0xfd4659) },
    Entry { ident: "deep_sky_blue", value: hex24!(0x0d75f8) },
    Entry { ident: "fire_engine_red", value: hex24!(0xfe0002) },
    Entry { ident: "yellow_ochre", value: hex24!(0xcb9d06) },
    Entry { ident: "pumpkin_orange", value: hex24!(0xfb7d07) },
    Entry { ident: "pale_olive", value: hex24!(0xb9cc81) },
    Entry { ident: "light_lilac", value: hex24!(0xedc8ff) },
    Entry { ident: "lightish_green", value: hex24!(0x61e160) },
    Entry { ident: "carolina_blue", value: hex24!(0x8ab8fe) },
    Entry { ident: "mulberry", value: hex24!(0x920a4e) },
    Entry { ident: "shocking_pink", value: hex24!(0xfe02a2) },
    Entry { ident: "auburn", value: hex24!(0x9a3001) },
    Entry { ident: "bright_lime_green", value: hex24!(0x65fe08) },
    Entry { ident: "celadon", value: hex24!(0xbefdb7) },
    Entry { ident: "pinkish_brown", value: hex24!(0xb17261) },
    Entry { ident: "poo_brown", value: hex24!(0x885f01) },
    Entry { ident: "bright_sky_blue", value: hex24!(0x02ccfe) },
    Entry { ident: "celery", value: hex24!(0xc1fd95) },
    Entry { ident: "dirt_brown", value: hex24!(0x836539) },
    Entry { ident: "strawberry", value: hex24!(0xfb2943) },
    Entry { ident: "dark_lime", value: hex24!(0x84b701) },
    Entry { ident: "copper", value: hex24!(0xb66325) },
    Entry { ident: "medium_brown", value: hex24!(0x7f5112) },
    Entry { ident: "muted_green", value: hex24!(0x5fa052) },
    Entry { ident: "robins_egg", value: hex24!(0x6dedfd) },
    Entry { ident: "bright_aqua", value: hex24!(0x0bf9ea) },
    Entry { ident: "bright_lavender", value: hex24!(0xc760ff) },
    Entry { ident: "ivory", value: hex24!(0xffffcb) },
    Entry { ident: "very_light_purple", value: hex24!(0xf6cefc) },
    Entry { ident: "light_navy", value: hex24!(0x155084) },
    Entry { ident: "pink_red", value: hex24!(0xf5054f) },
    Entry { ident: "olive_brown", value: hex24!(0x645403) },
    Entry { ident: "poop_brown", value: hex24!(0x7a5901) },
    Entry { ident: "mustard_green", value: hex24!(0xa8b504) },
    Entry { ident: "ocean_green", value: hex24!(0x3d9973) },
    Entry { ident: "very_dark_blue", value: hex24!(0x000133) },
    Entry { ident: "dusty_green", value: hex24!(0x76a973) },
    Entry { ident: "light_navy_blue", value: hex24!(0x2e5a88) },
    Entry { ident: "minty_green", value: hex24!(0x0bf77d) },
    Entry { ident: "adobe", value: hex24!(0xbd6c48) },
    Entry { ident: "barney", value: hex24!(0xac1db8) },
    Entry { ident: "jade_green", value: hex24!(0x2baf6a) },
    Entry { ident: "bright_light_blue", value: hex24!(0x26f7fd) },
    Entry { ident: "light_lime", value: hex24!(0xaefd6c) },
    Entry { ident: "dark_khaki", value: hex24!(0x9b8f55) },
    Entry { ident: "orange_yellow", value: hex24!(0xffad01) },
    Entry { ident: "ocre", value: hex24!(0xc69c04) },
    Entry { ident: "maize", value: hex24!(0xf4d054) },
    Entry { ident: "faded_pink", value: hex24!(0xde9dac) },
    Entry { ident: "british_racing_green", value: hex24!(0x05480d) },
    Entry { ident: "sandstone", value: hex24!(0xc9ae74) },
    Entry { ident: "mud_brown", value: hex24!(0x60460f) },
    Entry { ident: "light_sea_green", value: hex24!(0x98f6b0) },
    Entry { ident: "robin_egg_blue", value: hex24!(0x8af1fe) },
    Entry { ident: "aqua_marine", value: hex24!(0x2ee8bb) },
    Entry { ident: "dark_sea_green", value: hex24!(0x11875d) },
    Entry { ident: "soft_pink", value: hex24!(0xfdb0c0) },
    Entry { ident: "orangey_brown", value: hex24!(0xb16002) },
    Entry { ident: "cherry_red", value: hex24!(0xf7022a) },
    Entry { ident: "burnt_yellow", value: hex24!(0xd5ab09) },
    Entry { ident: "brownish_grey", value: hex24!(0x86775f) },
    Entry { ident: "camel", value: hex24!(0xc69f59) },
    Entry { ident: "purplish_grey", value: hex24!(0x7a687f) },
    Entry { ident: "marine", value: hex24!(0x042e60) },
    Entry { ident: "greyish_pink", value: hex24!(0xc88d94) },
    Entry { ident: "pale_turquoise", value: hex24!(0xa5fbd5) },
    Entry { ident: "pastel_yellow", value: hex24!(0xfffe71) },
    Entry { ident: "bluey_purple", value: hex24!(0x6241c7) },
    Entry { ident: "canary_yellow", value: hex24!(0xfffe40) },
    Entry { ident: "faded_red", value: hex24!(0xd3494e) },
    Entry { ident: "sepia", value: hex24!(0x985e2b) },
    Entry { ident: "coffee", value: hex24!(0xa6814c) },
    Entry { ident: "bright_magenta", value: hex24!(0xff08e8) },
    Entry { ident: "mocha", value: hex24!(0x9d7651) },
    Entry { ident: "ecru", value: hex24!(0xfeffca) },
    Entry { ident: "purpleish", value: hex24!(0x98568d) },
    Entry { ident: "cranberry", value: hex24!(0x9e003a) },
    Entry { ident: "darkish_green", value: hex24!(0x287c37) },
    Entry { ident: "brown_orange", value: hex24!(0xb96902) },
    Entry { ident: "dusky_rose", value: hex24!(0xba6873) },
    Entry { ident: "melon", value: hex24!(0xff7855) },
    Entry { ident: "sickly_green", value: hex24!(0x94b21c) },
    Entry { ident: "silver", value: hex24!(0xc5c9c7) },
    Entry { ident: "purply_blue", value: hex24!(0x661aee) },
    Entry { ident: "purpleish_blue", value: hex24!(0x6140ef) },
    Entry { ident: "hospital_green", value: hex24!(0x9be5aa) },
    Entry { ident: "shit_brown", value: hex24!(0x7b5804) },
    Entry { ident: "mid_blue", value: hex24!(0x276ab3) },
    Entry { ident: "amber", value: hex24!(0xfeb308) },
    Entry { ident: "easter_green", value: hex24!(0x8cfd7e) },
    Entry { ident: "soft_blue", value: hex24!(0x6488ea) },
    Entry { ident: "cerulean_blue", value: hex24!(0x056eee) },
    Entry { ident: "golden_brown", value: hex24!(0xb27a01) },
    Entry { ident: "bright_turquoise", value: hex24!(0x0ffef9) },
    Entry { ident: "red_pink", value: hex24!(0xfa2a55) },
    Entry { ident: "red_purple", value: hex24!(0x820747) },
    Entry { ident: "greyish_brown", value: hex24!(0x7a6a4f) },
    Entry { ident: "vermillion", value: hex24!(0xf4320c) },
    Entry { ident: "russet", value: hex24!(0xa13905) },
    Entry { ident: "steel_grey", value: hex24!(0x6f828a) },
    Entry { ident: "lighter_purple", value: hex24!(0xa55af4) },
    Entry { ident: "bright_violet", value: hex24!(0xad0afd) },
    Entry { ident: "prussian_blue", value: hex24!(0x004577) },
    Entry { ident: "slate_green", value: hex24!(0x658d6d) },
    Entry { ident: "dirty_pink", value: hex24!(0xca7b80) },
    Entry { ident: "dark_blue_green", value: hex24!(0x005249) },
    Entry { ident: "pine", value: hex24!(0x2b5d34) },
    Entry { ident: "yellowy_green", value: hex24!(0xbff128) },
    Entry { ident: "dark_gold", value: hex24!(0xb59410) },
    Entry { ident: "bluish", value: hex24!(0x2976bb) },
    Entry { ident: "darkish_blue", value: hex24!(0x014182) },
    Entry { ident: "dull_red", value: hex24!(0xbb3f3f) },
    Entry { ident: "pinky_red", value: hex24!(0xfc2647) },
    Entry { ident: "bronze", value: hex24!(0xa87900) },
    Entry { ident: "pale_teal", value: hex24!(0x82cbb2) },
    Entry { ident: "military_green", value: hex24!(0x667c3e) },
    Entry { ident: "barbie_pink", value: hex24!(0xfe46a5) },
    Entry { ident: "bubblegum_pink", value: hex24!(0xfe83cc) },
    Entry { ident: "pea_soup_green", value: hex24!(0x94a617) },
    Entry { ident: "dark_mustard", value: hex24!(0xa88905) },
    Entry { ident: "shit", value: hex24!(0x7f5f00) },
    Entry { ident: "medium_purple", value: hex24!(0x9e43a2) },
    Entry { ident: "very_dark_green", value: hex24!(0x062e03) },
    Entry { ident: "dirt", value: hex24!(0x8a6e45) },
    Entry { ident: "dusky_pink", value: hex24!(0xcc7a8b) },
    Entry { ident: "red_violet", value: hex24!(0x9e0168) },
    Entry { ident: "lemon_yellow", value: hex24!(0xfdff38) },
    Entry { ident: "pistachio", value: hex24!(0xc0fa8b) },
    Entry { ident: "dull_yellow", value: hex24!(0xeedc5b) },
    Entry { ident: "dark_lime_green", value: hex24!(0x7ebd01) },
    Entry { ident: "denim_blue", value: hex24!(0x3b5b92) },
    Entry { ident: "teal_blue", value: hex24!(0x01889f) },
    Entry { ident: "lightish_blue", value: hex24!(0x3d7afd) },
    Entry { ident: "purpley_blue", value: hex24!(0x5f34e7) },
    Entry { ident: "light_indigo", value: hex24!(0x6d5acf) },
    Entry { ident: "swamp_green", value: hex24!(0x748500) },
    Entry { ident: "brown_green", value: hex24!(0x706c11) },
    Entry { ident: "dark_maroon", value: hex24!(0x3c0008) },
    Entry { ident: "hot_purple", value: hex24!(0xcb00f5) },
    Entry { ident: "dark_forest_green", value: hex24!(0x002d04) },
    Entry { ident: "faded_blue", value: hex24!(0x658cbb) },
    Entry { ident: "drab_green", value: hex24!(0x749551) },
    Entry { ident: "light_lime_green", value: hex24!(0xb9ff66) },
    Entry { ident: "snot_green", value: hex24!(0x9dc100) },
    Entry { ident: "yellowish", value: hex24!(0xfaee66) },
    Entry { ident: "light_blue_green", value: hex24!(0x7efbb3) },
    Entry { ident: "bordeaux", value: hex24!(0x7b002c) },
    Entry { ident: "light_mauve", value: hex24!(0xc292a1) },
    Entry { ident: "ocean", value: hex24!(0x017b92) },
    Entry { ident: "marigold", value: hex24!(0xfcc006) },
    Entry { ident: "muddy_green", value: hex24!(0x657432) },
    Entry { ident: "dull_orange", value: hex24!(0xd8863b) },
    Entry { ident: "steel", value: hex24!(0x738595) },
    Entry { ident: "electric_purple", value: hex24!(0xaa23ff) },
    Entry { ident: "fluorescent_green", value: hex24!(0x08ff08) },
    Entry { ident: "yellowish_brown", value: hex24!(0x9b7a01) },
    Entry { ident: "blush", value: hex24!(0xf29e8e) },
    Entry { ident: "soft_green", value: hex24!(0x6fc276) },
    Entry { ident: "bright_orange", value: hex24!(0xff5b00) },
    Entry { ident: "lemon", value: hex24!(0xfdff52) },
    Entry { ident: "purple_grey", value: hex24!(0x866f85) },
    Entry { ident: "acid_green", value: hex24!(0x8ffe09) },
    Entry { ident: "pale_lavender", value: hex24!(0xeecffe) },
    Entry { ident: "violet_blue", value: hex24!(0x510ac9) },
    Entry { ident: "light_forest_green", value: hex24!(0x4f9153) },
    Entry { ident: "burnt_red", value: hex24!(0x9f2305) },
    Entry { ident: "khaki_green", value: hex24!(0x728639) },
    Entry { ident: "cerise", value: hex24!(0xde0c62) },
    Entry { ident: "faded_purple", value: hex24!(0x916e99) },
    Entry { ident: "apricot", value: hex24!(0xffb16d) },
    Entry { ident: "dark_olive_green", value: hex24!(0x3c4d03) },
    Entry { ident: "grey_brown", value: hex24!(0x7f7053) },
    Entry { ident: "green_grey", value: hex24!(0x77926f) },
    Entry { ident: "true_blue", value: hex24!(0x010fcc) },
    Entry { ident: "pale_violet", value: hex24!(0xceaefa) },
    Entry { ident: "periwinkle_blue", value: hex24!(0x8f99fb) },
    Entry { ident: "light_sky_blue", value: hex24!(0xc6fcff) },
    Entry { ident: "blurple", value: hex24!(0x5539cc) },
    Entry { ident: "green_brown", value: hex24!(0x544e03) },
    Entry { ident: "bluegreen", value: hex24!(0x017a79) },
    Entry { ident: "bright_teal", value: hex24!(0x01f9c6) },
    Entry { ident: "brownish_yellow", value: hex24!(0xc9b003) },
    Entry { ident: "pea_soup", value: hex24!(0x929901) },
    Entry { ident: "forest", value: hex24!(0x0b5509) },
    Entry { ident: "barney_purple", value: hex24!(0xa00498) },
    Entry { ident: "ultramarine", value: hex24!(0x2000b1) },
    Entry { ident: "purplish", value: hex24!(0x94568c) },
    Entry { ident: "puke_yellow", value: hex24!(0xc2be0e) },
    Entry { ident: "bluish_grey", value: hex24!(0x748b97) },
    Entry { ident: "dark_periwinkle", value: hex24!(0x665fd1) },
    Entry { ident: "dark_lilac", value: hex24!(0x9c6da5) },
    Entry { ident: "reddish", value: hex24!(0xc44240) },
    Entry { ident: "light_maroon", value: hex24!(0xa24857) },
    Entry { ident: "dusty_purple", value: hex24!(0x825f87) },
    Entry { ident: "terra_cotta", value: hex24!(0xc9643b) },
    Entry { ident: "avocado", value: hex24!(0x90b134) },
    Entry { ident: "marine_blue", value: hex24!(0x01386a) },
    Entry { ident: "teal_green", value: hex24!(0x25a36f) },
    Entry { ident: "slate_grey", value: hex24!(0x59656d) },
    Entry { ident: "lighter_green", value: hex24!(0x75fd63) },
    Entry { ident: "electric_green", value: hex24!(0x21fc0d) },
    Entry { ident: "dusty_blue", value: hex24!(0x5a86ad) },
    Entry { ident: "golden_yellow", value: hex24!(0xfec615) },
    Entry { ident: "bright_yellow", value: hex24!(0xfffd01) },
    Entry { ident: "light_lavender", value: hex24!(0xdfc5fe) },
    Entry { ident: "umber", value: hex24!(0xb26400) },
    Entry { ident: "poop", value: hex24!(0x7f5e00) },
    Entry { ident: "dark_peach", value: hex24!(0xde7e5d) },
    Entry { ident: "jungle_green", value: hex24!(0x048243) },
    Entry { ident: "eggshell", value: hex24!(0xffffd4) },
    Entry { ident: "denim", value: hex24!(0x3b638c) },
    Entry { ident: "yellow_brown", value: hex24!(0xb79400) },
    Entry { ident: "dull_purple", value: hex24!(0x84597e) },
    Entry { ident: "chocolate_brown", value: hex24!(0x411900) },
    Entry { ident: "wine_red", value: hex24!(0x7b0323) },
    Entry { ident: "neon_blue", value: hex24!(0x04d9ff) },
    Entry { ident: "dirty_green", value: hex24!(0x667e2c) },
    Entry { ident: "light_tan", value: hex24!(0xfbeeac) },
    Entry { ident: "ice_blue", value: hex24!(0xd7fffe) },
    Entry { ident: "cadet_blue", value: hex24!(0x4e7496) },
    Entry { ident: "dark_mauve", value: hex24!(0x874c62) },
    Entry { ident: "very_light_blue", value: hex24!(0xd5ffff) },
    Entry { ident: "grey_purple", value: hex24!(0x826d8c) },
    Entry { ident: "pastel_pink", value: hex24!(0xffbacd) },
    Entry { ident: "very_light_green", value: hex24!(0xd1ffbd) },
    Entry { ident: "dark_sky_blue", value: hex24!(0x448ee4) },
    Entry { ident: "evergreen", value: hex24!(0x05472a) },
    Entry { ident: "dull_pink", value: hex24!(0xd5869d) },
    Entry { ident: "aubergine", value: hex24!(0x3d0734) },
    Entry { ident: "mahogany", value: hex24!(0x4a0100) },
    Entry { ident: "reddish_orange", value: hex24!(0xf8481c) },
    Entry { ident: "deep_green", value: hex24!(0x02590f) },
    Entry { ident: "vomit_green", value: hex24!(0x89a203) },
    Entry { ident: "purple_pink", value: hex24!(0xe03fd8) },
    Entry { ident: "dusty_pink", value: hex24!(0xd58a94) },
    Entry { ident: "faded_green", value: hex24!(0x7bb274) },
    Entry { ident: "camo_green", value: hex24!(0x526525) },
    Entry { ident: "pinky_purple", value: hex24!(0xc94cbe) },
    Entry { ident: "pink_purple", value: hex24!(0xdb4bda) },
    Entry { ident: "brownish_red", value: hex24!(0x9e3623) },
    Entry { ident: "dark_rose", value: hex24!(0xb5485d) },
    Entry { ident: "mud", value: hex24!(0x735c12) },
    Entry { ident: "brownish", value: hex24!(0x9c6d57) },
    Entry { ident: "emerald_green", value: hex24!(0x028f1e) },
    Entry { ident: "pale_brown", value: hex24!(0xb1916e) },
    Entry { ident: "dull_blue", value: hex24!(0x49759c) },
    Entry { ident: "burnt_umber", value: hex24!(0xa0450e) },
    Entry { ident: "medium_green", value: hex24!(0x39ad48) },
    Entry { ident: "clay", value: hex24!(0xb66a50) },
    Entry { ident: "light_aqua", value: hex24!(0x8cffdb) },
    Entry { ident: "light_olive_green", value: hex24!(0xa4be5c) },
    Entry { ident: "brownish_orange", value: hex24!(0xcb7723) },
    Entry { ident: "dark_aqua", value: hex24!(0x05696b) },
    Entry { ident: "purplish_pink", value: hex24!(0xce5dae) },
    Entry { ident: "dark_salmon", value: hex24!(0xc85a53) },
    Entry { ident: "greenish_grey", value: hex24!(0x96ae8d) },
    Entry { ident: "jade", value: hex24!(0x1fa774) },
    Entry { ident: "ugly_green", value: hex24!(0x7a9703) },
    Entry { ident: "dark_beige", value: hex24!(0xac9362) },
    Entry { ident: "emerald", value: hex24!(0x01a049) },
    Entry { ident: "pale_red", value: hex24!(0xd9544d) },
    Entry { ident: "light_magenta", value: hex24!(0xfa5ff7) },
    Entry { ident: "sky", value: hex24!(0x82cafc) },
    Entry { ident: "light_cyan", value: hex24!(0xacfffc) },
    Entry { ident: "yellow_orange", value: hex24!(0xfcb001) },
    Entry { ident: "reddish_purple", value: hex24!(0x910951) },
    Entry { ident: "reddish_pink", value: hex24!(0xfe2c54) },
    Entry { ident: "orchid", value: hex24!(0xc875c4) },
    Entry { ident: "dirty_yellow", value: hex24!(0xcdc50a) },
    Entry { ident: "orange_red", value: hex24!(0xfd411e) },
    Entry { ident: "deep_red", value: hex24!(0x9a0200) },
    Entry { ident: "orange_brown", value: hex24!(0xbe6400) },
    Entry { ident: "cobalt_blue", value: hex24!(0x030aa7) },
    Entry { ident: "neon_pink", value: hex24!(0xfe019a) },
    Entry { ident: "rose_pink", value: hex24!(0xf7879a) },
    Entry { ident: "greyish_purple", value: hex24!(0x887191) },
    Entry { ident: "raspberry", value: hex24!(0xb00149) },
    Entry { ident: "aqua_green", value: hex24!(0x12e193) },
    Entry { ident: "salmon_pink", value: hex24!(0xfe7b7c) },
    Entry { ident: "tangerine", value: hex24!(0xff9408) },
    Entry { ident: "brownish_green", value: hex24!(0x6a6e09) },
    Entry { ident: "red_brown", value: hex24!(0x8b2e16) },
    Entry { ident: "greenish_brown", value: hex24!(0x696112) },
    Entry { ident: "pumpkin", value: hex24!(0xe17701) },
    Entry { ident: "pine_green", value: hex24!(0x0a481e) },
    Entry { ident: "charcoal", value: hex24!(0x343837) },
    Entry { ident: "baby_pink", value: hex24!(0xffb7ce) },
    Entry { ident: "cornflower", value: hex24!(0x6a79f7) },
    Entry { ident: "blue_violet", value: hex24!(0x5d06e9) },
    Entry { ident: "chocolate", value: hex24!(0x3d1c02) },
    Entry { ident: "greyish_green", value: hex24!(0x82a67d) },
    Entry { ident: "scarlet", value: hex24!(0xbe0119) },
    Entry { ident: "green_yellow", value: hex24!(0xc9ff27) },
    Entry { ident: "dark_olive", value: hex24!(0x373e02) },
    Entry { ident: "sienna", value: hex24!(0xa9561e) },
    Entry { ident: "pastel_purple", value: hex24!(0xcaa0ff) },
    Entry { ident: "terracotta", value: hex24!(0xca6641) },
    Entry { ident: "aqua_blue", value: hex24!(0x02d8e9) },
    Entry { ident: "sage_green", value: hex24!(0x88b378) },
    Entry { ident: "blood_red", value: hex24!(0x980002) },
    Entry { ident: "deep_pink", value: hex24!(0xcb0162) },
    Entry { ident: "grass", value: hex24!(0x5cac2d) },
    Entry { ident: "moss", value: hex24!(0x769958) },
    Entry { ident: "pastel_blue", value: hex24!(0xa2bffe) },
    Entry { ident: "bluish_green", value: hex24!(0x10a674) },
    Entry { ident: "green_blue", value: hex24!(0x06b48b) },
    Entry { ident: "dark_tan", value: hex24!(0xaf884a) },
    Entry { ident: "greenish_blue", value: hex24!(0x0b8b87) },
    Entry { ident: "pale_orange", value: hex24!(0xffa756) },
    Entry { ident: "vomit", value: hex24!(0xa2a415) },
    Entry { ident: "forrest_green", value: hex24!(0x154406) },
    Entry { ident: "dark_lavender", value: hex24!(0x856798) },
    Entry { ident: "dark_violet", value: hex24!(0x34013f) },
    Entry { ident: "purple_blue", value: hex24!(0x632de9) },
    Entry { ident: "dark_cyan", value: hex24!(0x0a888a) },
    Entry { ident: "olive_drab", value: hex24!(0x6f7632) },
    Entry { ident: "pinkish", value: hex24!(0xd46a7e) },
    Entry { ident: "cobalt", value: hex24!(0x1e488f) },
    Entry { ident: "neon_purple", value: hex24!(0xbc13fe) },
    Entry { ident: "light_turquoise", value: hex24!(0x7ef4cc) },
    Entry { ident: "apple_green", value: hex24!(0x76cd26) },
    Entry { ident: "dull_green", value: hex24!(0x74a662) },
    Entry { ident: "wine", value: hex24!(0x80013f) },
    Entry { ident: "powder_blue", value: hex24!(0xb1d1fc) },
    Entry { ident: "off_white", value: hex24!(0xffffe4) },
    Entry { ident: "electric_blue", value: hex24!(0x0652ff) },
    Entry { ident: "dark_turquoise", value: hex24!(0x045c5a) },
    Entry { ident: "blue_purple", value: hex24!(0x5729ce) },
    Entry { ident: "azure", value: hex24!(0x069af3) },
    Entry { ident: "bright_red", value: hex24!(0xff000d) },
    Entry { ident: "pinkish_red", value: hex24!(0xf10c45) },
    Entry { ident: "cornflower_blue", value: hex24!(0x5170d7) },
    Entry { ident: "light_olive", value: hex24!(0xacbf69) },
    Entry { ident: "grape", value: hex24!(0x6c3461) },
    Entry { ident: "greyish_blue", value: hex24!(0x5e819d) },
    Entry { ident: "purplish_blue", value: hex24!(0x601ef9) },
    Entry { ident: "yellowish_green", value: hex24!(0xb0dd16) },
    Entry { ident: "greenish_yellow", value: hex24!(0xcdfd02) },
    Entry { ident: "medium_blue", value: hex24!(0x2c6fbb) },
    Entry { ident: "dusty_rose", value: hex24!(0xc0737a) },
    Entry { ident: "light_violet", value: hex24!(0xd6b4fc) },
    Entry { ident: "midnight_blue", value: hex24!(0x020035) },
    Entry { ident: "bluish_purple", value: hex24!(0x703be7) },
    Entry { ident: "red_orange", value: hex24!(0xfd3c06) },
    Entry { ident: "dark_magenta", value: hex24!(0x960056) },
    Entry { ident: "greenish", value: hex24!(0x40a368) },
    Entry { ident: "ocean_blue", value: hex24!(0x03719c) },
    Entry { ident: "coral", value: hex24!(0xfc5a50) },
    Entry { ident: "cream", value: hex24!(0xffffc2) },
    Entry { ident: "reddish_brown", value: hex24!(0x7f2b0a) },
    Entry { ident: "burnt_sienna", value: hex24!(0xb04e0f) },
    Entry { ident: "brick", value: hex24!(0xa03623) },
    Entry { ident: "sage", value: hex24!(0x87ae73) },
    Entry { ident: "grey_green", value: hex24!(0x789b73) },
    Entry { ident: "white", value: hex24!(0xffffff) },
    Entry { ident: "robins_egg_blue", value: hex24!(0x98eff9) },
    Entry { ident: "moss_green", value: hex24!(0x658b38) },
    Entry { ident: "steel_blue", value: hex24!(0x5a7d9a) },
    Entry { ident: "eggplant", value: hex24!(0x380835) },
    Entry { ident: "light_yellow", value: hex24!(0xfffe7a) },
    Entry { ident: "leaf_green", value: hex24!(0x5ca904) },
    Entry { ident: "light_grey", value: hex24!(0xd8dcd6) },
    Entry { ident: "puke", value: hex24!(0xa5a502) },
    Entry { ident: "pinkish_purple", value: hex24!(0xd648d7) },
    Entry { ident: "sea_blue", value: hex24!(0x047495) },
    Entry { ident: "pale_purple", value: hex24!(0xb790d4) },
    Entry { ident: "slate_blue", value: hex24!(0x5b7c99) },
    Entry { ident: "blue_grey", value: hex24!(0x607c8e) },
    Entry { ident: "hunter_green", value: hex24!(0x0b4008) },
    Entry { ident: "fuchsia", value: hex24!(0xed0dd9) },
    Entry { ident: "crimson", value: hex24!(0x8c000f) },
    Entry { ident: "pale_yellow", value: hex24!(0xffff84) },
    Entry { ident: "ochre", value: hex24!(0xbf9005) },
    Entry { ident: "mustard_yellow", value: hex24!(0xd2bd0a) },
    Entry { ident: "light_red", value: hex24!(0xff474c) },
    Entry { ident: "cerulean", value: hex24!(0x0485d1) },
    Entry { ident: "pale_pink", value: hex24!(0xffcfdc) },
    Entry { ident: "deep_blue", value: hex24!(0x040273) },
    Entry { ident: "rust", value: hex24!(0xa83c09) },
    Entry { ident: "light_teal", value: hex24!(0x90e4c1) },
    Entry { ident: "slate", value: hex24!(0x516572) },
    Entry { ident: "goldenrod", value: hex24!(0xfac205) },
    Entry { ident: "dark_yellow", value: hex24!(0xd5b60a) },
    Entry { ident: "dark_grey", value: hex24!(0x363737) },
    Entry { ident: "army_green", value: hex24!(0x4b5d16) },
    Entry { ident: "grey_blue", value: hex24!(0x6b8ba4) },
    Entry { ident: "seafoam", value: hex24!(0x80f9ad) },
    Entry { ident: "puce", value: hex24!(0xa57e52) },
    Entry { ident: "spring_green", value: hex24!(0xa9f971) },
    Entry { ident: "dark_orange", value: hex24!(0xc65102) },
    Entry { ident: "sand", value: hex24!(0xe2ca76) },
    Entry { ident: "pastel_green", value: hex24!(0xb0ff9d) },
    Entry { ident: "mint", value: hex24!(0x9ffeb0) },
    Entry { ident: "light_orange", value: hex24!(0xfdaa48) },
    Entry { ident: "bright_pink", value: hex24!(0xfe01b1) },
    Entry { ident: "chartreuse", value: hex24!(0xc1f80a) },
    Entry { ident: "deep_purple", value: hex24!(0x36013f) },
    Entry { ident: "dark_brown", value: hex24!(0x341c02) },
    Entry { ident: "taupe", value: hex24!(0xb9a281) },
    Entry { ident: "pea_green", value: hex24!(0x8eab12) },
    Entry { ident: "puke_green", value: hex24!(0x9aae07) },
    Entry { ident: "kelly_green", value: hex24!(0x02ab2e) },
    Entry { ident: "seafoam_green", value: hex24!(0x7af9ab) },
    Entry { ident: "blue_green", value: hex24!(0x137e6d) },
    Entry { ident: "khaki", value: hex24!(0xaaa662) },
    Entry { ident: "burgundy", value: hex24!(0x610023) },
    Entry { ident: "dark_teal", value: hex24!(0x014d4e) },
    Entry { ident: "brick_red", value: hex24!(0x8f1402) },
    Entry { ident: "royal_purple", value: hex24!(0x4b006e) },
    Entry { ident: "plum", value: hex24!(0x580f41) },
    Entry { ident: "mint_green", value: hex24!(0x8fff9f) },
    Entry { ident: "gold", value: hex24!(0xdbb40c) },
    Entry { ident: "baby_blue", value: hex24!(0xa2cffe) },
    Entry { ident: "yellow_green", value: hex24!(0xc0fb2d) },
    Entry { ident: "bright_purple", value: hex24!(0xbe03fd) },
    Entry { ident: "dark_red", value: hex24!(0x840000) },
    Entry { ident: "pale_blue", value: hex24!(0xd0fefe) },
    Entry { ident: "grass_green", value: hex24!(0x3f9b0b) },
    Entry { ident: "navy", value: hex24!(0x01153e) },
    Entry { ident: "aquamarine", value: hex24!(0x04d8b2) },
    Entry { ident: "burnt_orange", value: hex24!(0xc04e01) },
    Entry { ident: "neon_green", value: hex24!(0x0cff0c) },
    Entry { ident: "bright_blue", value: hex24!(0x0165fc) },
    Entry { ident: "rose", value: hex24!(0xcf6275) },
    Entry { ident: "light_pink", value: hex24!(0xffd1df) },
    Entry { ident: "mustard", value: hex24!(0xceb301) },
    Entry { ident: "indigo", value: hex24!(0x380282) },
    Entry { ident: "lime", value: hex24!(0xaaff32) },
    Entry { ident: "sea_green", value: hex24!(0x53fca1) },
    Entry { ident: "periwinkle", value: hex24!(0x8e82fe) },
    Entry { ident: "dark_pink", value: hex24!(0xcb416b) },
    Entry { ident: "olive_green", value: hex24!(0x677a04) },
    Entry { ident: "peach", value: hex24!(0xffb07c) },
    Entry { ident: "pale_green", value: hex24!(0xc7fdb5) },
    Entry { ident: "light_brown", value: hex24!(0xad8150) },
    Entry { ident: "hot_pink", value: hex24!(0xff028d) },
    Entry { ident: "black", value: hex24!(0x000000) },
    Entry { ident: "lilac", value: hex24!(0xcea2fd) },
    Entry { ident: "navy_blue", value: hex24!(0x001146) },
    Entry { ident: "royal_blue", value: hex24!(0x0504aa) },
    Entry { ident: "beige", value: hex24!(0xe6daa6) },
    Entry { ident: "salmon", value: hex24!(0xff796c) },
    Entry { ident: "olive", value: hex24!(0x6e750e) },
    Entry { ident: "maroon", value: hex24!(0x650021) },
    Entry { ident: "bright_green", value: hex24!(0x01ff07) },
    Entry { ident: "dark_purple", value: hex24!(0x35063e) },
    Entry { ident: "mauve", value: hex24!(0xae7181) },
    Entry { ident: "forest_green", value: hex24!(0x06470c) },
    Entry { ident: "aqua", value: hex24!(0x13eac9) },
    Entry { ident: "cyan", value: hex24!(0x00ffff) },
    Entry { ident: "tan", value: hex24!(0xd1b26f) },
    Entry { ident: "dark_blue", value: hex24!(0x00035b) },
    Entry { ident: "lavender", value: hex24!(0xc79fef) },
    Entry { ident: "turquoise", value: hex24!(0x06c2ac) },
    Entry { ident: "dark_green", value: hex24!(0x033500) },
    Entry { ident: "violet", value: hex24!(0x9a0eea) },
    Entry { ident: "light_purple", value: hex24!(0xbf77f6) },
    Entry { ident: "lime_green", value: hex24!(0x89fe05) },
    Entry { ident: "grey", value: hex24!(0x929591) },
    Entry { ident: "sky_blue", value: hex24!(0x75bbfd) },
    Entry { ident: "yellow", value: hex24!(0xffff14) },
    Entry { ident: "magenta", value: hex24!(0xc20078) },
    Entry { ident: "light_green", value: hex24!(0x96f97b) },
    Entry { ident: "orange", value: hex24!(0xf97306) },
    Entry { ident: "teal", value: hex24!(0x029386) },
    Entry { ident: "light_blue", value: hex24!(0x95d0fc) },
    Entry { ident: "red", value: hex24!(0xe50000) },
    Entry { ident: "brown", value: hex24!(0x653700) },
    Entry { ident: "pink", value: hex24!(0xff81c0) },
    Entry { ident: "blue", value: hex24!(0x0343df) },
    Entry { ident: "green", value: hex24!(0x15b01a) },
    Entry { ident: "purple", value: hex24!(0x7e1e9c) },
];


#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait XkcdColors : From<Rgb24> {
    #[inline(always)] fn cloudy_blue	() -> Self { Self::from(Rgb24::from(0xacc2d9	)) }
    #[inline(always)] fn dark_pastel_green	() -> Self { Self::from(Rgb24::from(0x56ae57	)) }
    #[inline(always)] fn dust	() -> Self { Self::from(Rgb24::from(0xb2996e	)) }
    #[inline(always)] fn electric_lime	() -> Self { Self::from(Rgb24::from(0xa8ff04	)) }
    #[inline(always)] fn fresh_green	() -> Self { Self::from(Rgb24::from(0x69d84f	)) }
    #[inline(always)] fn light_eggplant	() -> Self { Self::from(Rgb24::from(0x894585	)) }
    #[inline(always)] fn nasty_green	() -> Self { Self::from(Rgb24::from(0x70b23f	)) }
    #[inline(always)] fn really_light_blue	() -> Self { Self::from(Rgb24::from(0xd4ffff	)) }
    #[inline(always)] fn tea	() -> Self { Self::from(Rgb24::from(0x65ab7c	)) }
    #[inline(always)] fn warm_purple	() -> Self { Self::from(Rgb24::from(0x952e8f	)) }
    #[inline(always)] fn yellowish_tan	() -> Self { Self::from(Rgb24::from(0xfcfc81	)) }
    #[inline(always)] fn cement	() -> Self { Self::from(Rgb24::from(0xa5a391	)) }
    #[inline(always)] fn dark_grass_green	() -> Self { Self::from(Rgb24::from(0x388004	)) }
    #[inline(always)] fn dusty_teal	() -> Self { Self::from(Rgb24::from(0x4c9085	)) }
    #[inline(always)] fn grey_teal	() -> Self { Self::from(Rgb24::from(0x5e9b8a	)) }
    #[inline(always)] fn macaroni_and_cheese	() -> Self { Self::from(Rgb24::from(0xefb435	)) }
    #[inline(always)] fn pinkish_tan	() -> Self { Self::from(Rgb24::from(0xd99b82	)) }
    #[inline(always)] fn spruce	() -> Self { Self::from(Rgb24::from(0x0a5f38	)) }
    #[inline(always)] fn strong_blue	() -> Self { Self::from(Rgb24::from(0x0c06f7	)) }
    #[inline(always)] fn toxic_green	() -> Self { Self::from(Rgb24::from(0x61de2a	)) }
    #[inline(always)] fn windows_blue	() -> Self { Self::from(Rgb24::from(0x3778bf	)) }
    #[inline(always)] fn blue_blue	() -> Self { Self::from(Rgb24::from(0x2242c7	)) }
    #[inline(always)] fn blue_with_a_hint_of_purple	() -> Self { Self::from(Rgb24::from(0x533cc6	)) }
    #[inline(always)] fn booger	() -> Self { Self::from(Rgb24::from(0x9bb53c	)) }
    #[inline(always)] fn bright_sea_green	() -> Self { Self::from(Rgb24::from(0x05ffa6	)) }
    #[inline(always)] fn dark_green_blue	() -> Self { Self::from(Rgb24::from(0x1f6357	)) }
    #[inline(always)] fn deep_turquoise	() -> Self { Self::from(Rgb24::from(0x017374	)) }
    #[inline(always)] fn green_teal	() -> Self { Self::from(Rgb24::from(0x0cb577	)) }
    #[inline(always)] fn strong_pink	() -> Self { Self::from(Rgb24::from(0xff0789	)) }
    #[inline(always)] fn bland	() -> Self { Self::from(Rgb24::from(0xafa88b	)) }
    #[inline(always)] fn deep_aqua	() -> Self { Self::from(Rgb24::from(0x08787f	)) }
    #[inline(always)] fn lavender_pink	() -> Self { Self::from(Rgb24::from(0xdd85d7	)) }
    #[inline(always)] fn light_moss_green	() -> Self { Self::from(Rgb24::from(0xa6c875	)) }
    #[inline(always)] fn light_seafoam_green	() -> Self { Self::from(Rgb24::from(0xa7ffb5	)) }
    #[inline(always)] fn olive_yellow	() -> Self { Self::from(Rgb24::from(0xc2b709	)) }
    #[inline(always)] fn pig_pink	() -> Self { Self::from(Rgb24::from(0xe78ea5	)) }
    #[inline(always)] fn deep_lilac	() -> Self { Self::from(Rgb24::from(0x966ebd	)) }
    #[inline(always)] fn desert	() -> Self { Self::from(Rgb24::from(0xccad60	)) }
    #[inline(always)] fn dusty_lavender	() -> Self { Self::from(Rgb24::from(0xac86a8	)) }
    #[inline(always)] fn purpley_grey	() -> Self { Self::from(Rgb24::from(0x947e94	)) }
    #[inline(always)] fn purply	() -> Self { Self::from(Rgb24::from(0x983fb2	)) }
    #[inline(always)] fn candy_pink	() -> Self { Self::from(Rgb24::from(0xff63e9	)) }
    #[inline(always)] fn light_pastel_green	() -> Self { Self::from(Rgb24::from(0xb2fba5	)) }
    #[inline(always)] fn boring_green	() -> Self { Self::from(Rgb24::from(0x63b365	)) }
    #[inline(always)] fn kiwi_green	() -> Self { Self::from(Rgb24::from(0x8ee53f	)) }
    #[inline(always)] fn light_grey_green	() -> Self { Self::from(Rgb24::from(0xb7e1a1	)) }
    #[inline(always)] fn orange_pink	() -> Self { Self::from(Rgb24::from(0xff6f52	)) }
    #[inline(always)] fn tea_green	() -> Self { Self::from(Rgb24::from(0xbdf8a3	)) }
    #[inline(always)] fn very_light_brown	() -> Self { Self::from(Rgb24::from(0xd3b683	)) }
    #[inline(always)] fn egg_shell	() -> Self { Self::from(Rgb24::from(0xfffcc4	)) }
    #[inline(always)] fn eggplant_purple	() -> Self { Self::from(Rgb24::from(0x430541	)) }
    #[inline(always)] fn powder_pink	() -> Self { Self::from(Rgb24::from(0xffb2d0	)) }
    #[inline(always)] fn reddish_grey	() -> Self { Self::from(Rgb24::from(0x997570	)) }
    #[inline(always)] fn baby_shit_brown	() -> Self { Self::from(Rgb24::from(0xad900d	)) }
    #[inline(always)] fn liliac	() -> Self { Self::from(Rgb24::from(0xc48efd	)) }
    #[inline(always)] fn stormy_blue	() -> Self { Self::from(Rgb24::from(0x507b9c	)) }
    #[inline(always)] fn ugly_brown	() -> Self { Self::from(Rgb24::from(0x7d7103	)) }
    #[inline(always)] fn custard	() -> Self { Self::from(Rgb24::from(0xfffd78	)) }
    #[inline(always)] fn darkish_pink	() -> Self { Self::from(Rgb24::from(0xda467d	)) }
    #[inline(always)] fn deep_brown	() -> Self { Self::from(Rgb24::from(0x410200	)) }
    #[inline(always)] fn greenish_beige	() -> Self { Self::from(Rgb24::from(0xc9d179	)) }
    #[inline(always)] fn manilla	() -> Self { Self::from(Rgb24::from(0xfffa86	)) }
    #[inline(always)] fn off_blue	() -> Self { Self::from(Rgb24::from(0x5684ae	)) }
    #[inline(always)] fn battleship_grey	() -> Self { Self::from(Rgb24::from(0x6b7c85	)) }
    #[inline(always)] fn browny_green	() -> Self { Self::from(Rgb24::from(0x6f6c0a	)) }
    #[inline(always)] fn bruise	() -> Self { Self::from(Rgb24::from(0x7e4071	)) }
    #[inline(always)] fn kelley_green	() -> Self { Self::from(Rgb24::from(0x009337	)) }
    #[inline(always)] fn sickly_yellow	() -> Self { Self::from(Rgb24::from(0xd0e429	)) }
    #[inline(always)] fn sunny_yellow	() -> Self { Self::from(Rgb24::from(0xfff917	)) }
    #[inline(always)] fn azul	() -> Self { Self::from(Rgb24::from(0x1d5dec	)) }
    #[inline(always)] fn darkgreen	() -> Self { Self::from(Rgb24::from(0x054907	)) }
    #[inline(always)] fn green_or_yellow	() -> Self { Self::from(Rgb24::from(0xb5ce08	)) }
    #[inline(always)] fn lichen	() -> Self { Self::from(Rgb24::from(0x8fb67b	)) }
    #[inline(always)] fn light_light_green	() -> Self { Self::from(Rgb24::from(0xc8ffb0	)) }
    #[inline(always)] fn pale_gold	() -> Self { Self::from(Rgb24::from(0xfdde6c	)) }
    #[inline(always)] fn sun_yellow	() -> Self { Self::from(Rgb24::from(0xffdf22	)) }
    #[inline(always)] fn tan_green	() -> Self { Self::from(Rgb24::from(0xa9be70	)) }
    #[inline(always)] fn burple	() -> Self { Self::from(Rgb24::from(0x6832e3	)) }
    #[inline(always)] fn butterscotch	() -> Self { Self::from(Rgb24::from(0xfdb147	)) }
    #[inline(always)] fn toupe	() -> Self { Self::from(Rgb24::from(0xc7ac7d	)) }
    #[inline(always)] fn dark_cream	() -> Self { Self::from(Rgb24::from(0xfff39a	)) }
    #[inline(always)] fn indian_red	() -> Self { Self::from(Rgb24::from(0x850e04	)) }
    #[inline(always)] fn light_lavendar	() -> Self { Self::from(Rgb24::from(0xefc0fe	)) }
    #[inline(always)] fn poison_green	() -> Self { Self::from(Rgb24::from(0x40fd14	)) }
    #[inline(always)] fn baby_puke_green	() -> Self { Self::from(Rgb24::from(0xb6c406	)) }
    #[inline(always)] fn bright_yellow_green	() -> Self { Self::from(Rgb24::from(0x9dff00	)) }
    #[inline(always)] fn charcoal_grey	() -> Self { Self::from(Rgb24::from(0x3c4142	)) }
    #[inline(always)] fn squash	() -> Self { Self::from(Rgb24::from(0xf2ab15	)) }
    #[inline(always)] fn cinnamon	() -> Self { Self::from(Rgb24::from(0xac4f06	)) }
    #[inline(always)] fn light_pea_green	() -> Self { Self::from(Rgb24::from(0xc4fe82	)) }
    #[inline(always)] fn radioactive_green	() -> Self { Self::from(Rgb24::from(0x2cfa1f	)) }
    #[inline(always)] fn raw_sienna	() -> Self { Self::from(Rgb24::from(0x9a6200	)) }
    #[inline(always)] fn baby_purple	() -> Self { Self::from(Rgb24::from(0xca9bf7	)) }
    #[inline(always)] fn cocoa	() -> Self { Self::from(Rgb24::from(0x875f42	)) }
    #[inline(always)] fn light_royal_blue	() -> Self { Self::from(Rgb24::from(0x3a2efe	)) }
    #[inline(always)] fn orangeish	() -> Self { Self::from(Rgb24::from(0xfd8d49	)) }
    #[inline(always)] fn rust_brown	() -> Self { Self::from(Rgb24::from(0x8b3103	)) }
    #[inline(always)] fn sand_brown	() -> Self { Self::from(Rgb24::from(0xcba560	)) }
    #[inline(always)] fn swamp	() -> Self { Self::from(Rgb24::from(0x698339	)) }
    #[inline(always)] fn tealish_green	() -> Self { Self::from(Rgb24::from(0x0cdc73	)) }
    #[inline(always)] fn burnt_siena	() -> Self { Self::from(Rgb24::from(0xb75203	)) }
    #[inline(always)] fn camo	() -> Self { Self::from(Rgb24::from(0x7f8f4e	)) }
    #[inline(always)] fn dusk_blue	() -> Self { Self::from(Rgb24::from(0x26538d	)) }
    #[inline(always)] fn fern	() -> Self { Self::from(Rgb24::from(0x63a950	)) }
    #[inline(always)] fn old_rose	() -> Self { Self::from(Rgb24::from(0xc87f89	)) }
    #[inline(always)] fn pale_light_green	() -> Self { Self::from(Rgb24::from(0xb1fc99	)) }
    #[inline(always)] fn peachy_pink	() -> Self { Self::from(Rgb24::from(0xff9a8a	)) }
    #[inline(always)] fn rosy_pink	() -> Self { Self::from(Rgb24::from(0xf6688e	)) }
    #[inline(always)] fn light_bluish_green	() -> Self { Self::from(Rgb24::from(0x76fda8	)) }
    #[inline(always)] fn light_bright_green	() -> Self { Self::from(Rgb24::from(0x53fe5c	)) }
    #[inline(always)] fn light_neon_green	() -> Self { Self::from(Rgb24::from(0x4efd54	)) }
    #[inline(always)] fn light_seafoam	() -> Self { Self::from(Rgb24::from(0xa0febf	)) }
    #[inline(always)] fn tiffany_blue	() -> Self { Self::from(Rgb24::from(0x7bf2da	)) }
    #[inline(always)] fn washed_out_green	() -> Self { Self::from(Rgb24::from(0xbcf5a6	)) }
    #[inline(always)] fn browny_orange	() -> Self { Self::from(Rgb24::from(0xca6b02	)) }
    #[inline(always)] fn nice_blue	() -> Self { Self::from(Rgb24::from(0x107ab0	)) }
    #[inline(always)] fn sapphire	() -> Self { Self::from(Rgb24::from(0x2138ab	)) }
    #[inline(always)] fn greyish_teal	() -> Self { Self::from(Rgb24::from(0x719f91	)) }
    #[inline(always)] fn orangey_yellow	() -> Self { Self::from(Rgb24::from(0xfdb915	)) }
    #[inline(always)] fn parchment	() -> Self { Self::from(Rgb24::from(0xfefcaf	)) }
    #[inline(always)] fn straw	() -> Self { Self::from(Rgb24::from(0xfcf679	)) }
    #[inline(always)] fn very_dark_brown	() -> Self { Self::from(Rgb24::from(0x1d0200	)) }
    #[inline(always)] fn terracota	() -> Self { Self::from(Rgb24::from(0xcb6843	)) }
    #[inline(always)] fn ugly_blue	() -> Self { Self::from(Rgb24::from(0x31668a	)) }
    #[inline(always)] fn clear_blue	() -> Self { Self::from(Rgb24::from(0x247afd	)) }
    #[inline(always)] fn creme	() -> Self { Self::from(Rgb24::from(0xffffb6	)) }
    #[inline(always)] fn foam_green	() -> Self { Self::from(Rgb24::from(0x90fda9	)) }
    #[inline(always)] fn grey_or_green	() -> Self { Self::from(Rgb24::from(0x86a17d	)) }
    #[inline(always)] fn light_gold	() -> Self { Self::from(Rgb24::from(0xfddc5c	)) }
    #[inline(always)] fn seafoam_blue	() -> Self { Self::from(Rgb24::from(0x78d1b6	)) }
    #[inline(always)] fn topaz	() -> Self { Self::from(Rgb24::from(0x13bbaf	)) }
    #[inline(always)] fn violet_pink	() -> Self { Self::from(Rgb24::from(0xfb5ffc	)) }
    #[inline(always)] fn wintergreen	() -> Self { Self::from(Rgb24::from(0x20f986	)) }
    #[inline(always)] fn yellow_tan	() -> Self { Self::from(Rgb24::from(0xffe36e	)) }
    #[inline(always)] fn dark_fuchsia	() -> Self { Self::from(Rgb24::from(0x9d0759	)) }
    #[inline(always)] fn indigo_blue	() -> Self { Self::from(Rgb24::from(0x3a18b1	)) }
    #[inline(always)] fn light_yellowish_green	() -> Self { Self::from(Rgb24::from(0xc2ff89	)) }
    #[inline(always)] fn pale_magenta	() -> Self { Self::from(Rgb24::from(0xd767ad	)) }
    #[inline(always)] fn rich_purple	() -> Self { Self::from(Rgb24::from(0x720058	)) }
    #[inline(always)] fn sunflower_yellow	() -> Self { Self::from(Rgb24::from(0xffda03	)) }
    #[inline(always)] fn green_or_blue	() -> Self { Self::from(Rgb24::from(0x01c08d	)) }
    #[inline(always)] fn leather	() -> Self { Self::from(Rgb24::from(0xac7434	)) }
    #[inline(always)] fn racing_green	() -> Self { Self::from(Rgb24::from(0x014600	)) }
    #[inline(always)] fn vivid_purple	() -> Self { Self::from(Rgb24::from(0x9900fa	)) }
    #[inline(always)] fn dark_royal_blue	() -> Self { Self::from(Rgb24::from(0x02066f	)) }
    #[inline(always)] fn hazel	() -> Self { Self::from(Rgb24::from(0x8e7618	)) }
    #[inline(always)] fn muted_pink	() -> Self { Self::from(Rgb24::from(0xd1768f	)) }
    #[inline(always)] fn booger_green	() -> Self { Self::from(Rgb24::from(0x96b403	)) }
    #[inline(always)] fn canary	() -> Self { Self::from(Rgb24::from(0xfdff63	)) }
    #[inline(always)] fn cool_grey	() -> Self { Self::from(Rgb24::from(0x95a3a6	)) }
    #[inline(always)] fn dark_taupe	() -> Self { Self::from(Rgb24::from(0x7f684e	)) }
    #[inline(always)] fn darkish_purple	() -> Self { Self::from(Rgb24::from(0x751973	)) }
    #[inline(always)] fn true_green	() -> Self { Self::from(Rgb24::from(0x089404	)) }
    #[inline(always)] fn coral_pink	() -> Self { Self::from(Rgb24::from(0xff6163	)) }
    #[inline(always)] fn dark_sage	() -> Self { Self::from(Rgb24::from(0x598556	)) }
    #[inline(always)] fn dark_slate_blue	() -> Self { Self::from(Rgb24::from(0x214761	)) }
    #[inline(always)] fn flat_blue	() -> Self { Self::from(Rgb24::from(0x3c73a8	)) }
    #[inline(always)] fn mushroom	() -> Self { Self::from(Rgb24::from(0xba9e88	)) }
    #[inline(always)] fn rich_blue	() -> Self { Self::from(Rgb24::from(0x021bf9	)) }
    #[inline(always)] fn dirty_purple	() -> Self { Self::from(Rgb24::from(0x734a65	)) }
    #[inline(always)] fn greenblue	() -> Self { Self::from(Rgb24::from(0x23c48b	)) }
    #[inline(always)] fn icky_green	() -> Self { Self::from(Rgb24::from(0x8fae22	)) }
    #[inline(always)] fn light_khaki	() -> Self { Self::from(Rgb24::from(0xe6f2a2	)) }
    #[inline(always)] fn warm_blue	() -> Self { Self::from(Rgb24::from(0x4b57db	)) }
    #[inline(always)] fn dark_hot_pink	() -> Self { Self::from(Rgb24::from(0xd90166	)) }
    #[inline(always)] fn deep_sea_blue	() -> Self { Self::from(Rgb24::from(0x015482	)) }
    #[inline(always)] fn carmine	() -> Self { Self::from(Rgb24::from(0x9d0216	)) }
    #[inline(always)] fn dark_yellow_green	() -> Self { Self::from(Rgb24::from(0x728f02	)) }
    #[inline(always)] fn pale_peach	() -> Self { Self::from(Rgb24::from(0xffe5ad	)) }
    #[inline(always)] fn plum_purple	() -> Self { Self::from(Rgb24::from(0x4e0550	)) }
    #[inline(always)] fn golden_rod	() -> Self { Self::from(Rgb24::from(0xf9bc08	)) }
    #[inline(always)] fn neon_red	() -> Self { Self::from(Rgb24::from(0xff073a	)) }
    #[inline(always)] fn old_pink	() -> Self { Self::from(Rgb24::from(0xc77986	)) }
    #[inline(always)] fn very_pale_blue	() -> Self { Self::from(Rgb24::from(0xd6fffe	)) }
    #[inline(always)] fn blood_orange	() -> Self { Self::from(Rgb24::from(0xfe4b03	)) }
    #[inline(always)] fn grapefruit	() -> Self { Self::from(Rgb24::from(0xfd5956	)) }
    #[inline(always)] fn sand_yellow	() -> Self { Self::from(Rgb24::from(0xfce166	)) }
    #[inline(always)] fn clay_brown	() -> Self { Self::from(Rgb24::from(0xb2713d	)) }
    #[inline(always)] fn dark_blue_grey	() -> Self { Self::from(Rgb24::from(0x1f3b4d	)) }
    #[inline(always)] fn flat_green	() -> Self { Self::from(Rgb24::from(0x699d4c	)) }
    #[inline(always)] fn light_green_blue	() -> Self { Self::from(Rgb24::from(0x56fca2	)) }
    #[inline(always)] fn warm_pink	() -> Self { Self::from(Rgb24::from(0xfb5581	)) }
    #[inline(always)] fn dodger_blue	() -> Self { Self::from(Rgb24::from(0x3e82fc	)) }
    #[inline(always)] fn gross_green	() -> Self { Self::from(Rgb24::from(0xa0bf16	)) }
    #[inline(always)] fn ice	() -> Self { Self::from(Rgb24::from(0xd6fffa	)) }
    #[inline(always)] fn metallic_blue	() -> Self { Self::from(Rgb24::from(0x4f738e	)) }
    #[inline(always)] fn pale_salmon	() -> Self { Self::from(Rgb24::from(0xffb19a	)) }
    #[inline(always)] fn sap_green	() -> Self { Self::from(Rgb24::from(0x5c8b15	)) }
    #[inline(always)] fn algae	() -> Self { Self::from(Rgb24::from(0x54ac68	)) }
    #[inline(always)] fn bluey_grey	() -> Self { Self::from(Rgb24::from(0x89a0b0	)) }
    #[inline(always)] fn greeny_grey	() -> Self { Self::from(Rgb24::from(0x7ea07a	)) }
    #[inline(always)] fn highlighter_green	() -> Self { Self::from(Rgb24::from(0x1bfc06	)) }
    #[inline(always)] fn light_light_blue	() -> Self { Self::from(Rgb24::from(0xcafffb	)) }
    #[inline(always)] fn light_mint	() -> Self { Self::from(Rgb24::from(0xb6ffbb	)) }
    #[inline(always)] fn raw_umber	() -> Self { Self::from(Rgb24::from(0xa75e09	)) }
    #[inline(always)] fn vivid_blue	() -> Self { Self::from(Rgb24::from(0x152eff	)) }
    #[inline(always)] fn deep_lavender	() -> Self { Self::from(Rgb24::from(0x8d5eb7	)) }
    #[inline(always)] fn dull_teal	() -> Self { Self::from(Rgb24::from(0x5f9e8f	)) }
    #[inline(always)] fn light_greenish_blue	() -> Self { Self::from(Rgb24::from(0x63f7b4	)) }
    #[inline(always)] fn mud_green	() -> Self { Self::from(Rgb24::from(0x606602	)) }
    #[inline(always)] fn pinky	() -> Self { Self::from(Rgb24::from(0xfc86aa	)) }
    #[inline(always)] fn red_wine	() -> Self { Self::from(Rgb24::from(0x8c0034	)) }
    #[inline(always)] fn shit_green	() -> Self { Self::from(Rgb24::from(0x758000	)) }
    #[inline(always)] fn tan_brown	() -> Self { Self::from(Rgb24::from(0xab7e4c	)) }
    #[inline(always)] fn darkblue	() -> Self { Self::from(Rgb24::from(0x030764	)) }
    #[inline(always)] fn rosa	() -> Self { Self::from(Rgb24::from(0xfe86a4	)) }
    #[inline(always)] fn lipstick	() -> Self { Self::from(Rgb24::from(0xd5174e	)) }
    #[inline(always)] fn pale_mauve	() -> Self { Self::from(Rgb24::from(0xfed0fc	)) }
    #[inline(always)] fn claret	() -> Self { Self::from(Rgb24::from(0x680018	)) }
    #[inline(always)] fn dandelion	() -> Self { Self::from(Rgb24::from(0xfedf08	)) }
    #[inline(always)] fn orangered	() -> Self { Self::from(Rgb24::from(0xfe420f	)) }
    #[inline(always)] fn poop_green	() -> Self { Self::from(Rgb24::from(0x6f7c00	)) }
    #[inline(always)] fn ruby	() -> Self { Self::from(Rgb24::from(0xca0147	)) }
    #[inline(always)] fn dark	() -> Self { Self::from(Rgb24::from(0x1b2431	)) }
    #[inline(always)] fn greenish_turquoise	() -> Self { Self::from(Rgb24::from(0x00fbb0	)) }
    #[inline(always)] fn pastel_red	() -> Self { Self::from(Rgb24::from(0xdb5856	)) }
    #[inline(always)] fn piss_yellow	() -> Self { Self::from(Rgb24::from(0xddd618	)) }
    #[inline(always)] fn bright_cyan	() -> Self { Self::from(Rgb24::from(0x41fdfe	)) }
    #[inline(always)] fn dark_coral	() -> Self { Self::from(Rgb24::from(0xcf524e	)) }
    #[inline(always)] fn algae_green	() -> Self { Self::from(Rgb24::from(0x21c36f	)) }
    #[inline(always)] fn darkish_red	() -> Self { Self::from(Rgb24::from(0xa90308	)) }
    #[inline(always)] fn reddy_brown	() -> Self { Self::from(Rgb24::from(0x6e1005	)) }
    #[inline(always)] fn blush_pink	() -> Self { Self::from(Rgb24::from(0xfe828c	)) }
    #[inline(always)] fn camouflage_green	() -> Self { Self::from(Rgb24::from(0x4b6113	)) }
    #[inline(always)] fn lawn_green	() -> Self { Self::from(Rgb24::from(0x4da409	)) }
    #[inline(always)] fn putty	() -> Self { Self::from(Rgb24::from(0xbeae8a	)) }
    #[inline(always)] fn vibrant_blue	() -> Self { Self::from(Rgb24::from(0x0339f8	)) }
    #[inline(always)] fn dark_sand	() -> Self { Self::from(Rgb24::from(0xa88f59	)) }
    #[inline(always)] fn purple_or_blue	() -> Self { Self::from(Rgb24::from(0x5d21d0	)) }
    #[inline(always)] fn saffron	() -> Self { Self::from(Rgb24::from(0xfeb209	)) }
    #[inline(always)] fn twilight	() -> Self { Self::from(Rgb24::from(0x4e518b	)) }
    #[inline(always)] fn warm_brown	() -> Self { Self::from(Rgb24::from(0x964e02	)) }
    #[inline(always)] fn bluegrey	() -> Self { Self::from(Rgb24::from(0x85a3b2	)) }
    #[inline(always)] fn bubble_gum_pink	() -> Self { Self::from(Rgb24::from(0xff69af	)) }
    #[inline(always)] fn duck_egg_blue	() -> Self { Self::from(Rgb24::from(0xc3fbf4	)) }
    #[inline(always)] fn greenish_cyan	() -> Self { Self::from(Rgb24::from(0x2afeb7	)) }
    #[inline(always)] fn petrol	() -> Self { Self::from(Rgb24::from(0x005f6a	)) }
    #[inline(always)] fn royal	() -> Self { Self::from(Rgb24::from(0x0c1793	)) }
    #[inline(always)] fn butter	() -> Self { Self::from(Rgb24::from(0xffff81	)) }
    #[inline(always)] fn dusty_orange	() -> Self { Self::from(Rgb24::from(0xf0833a	)) }
    #[inline(always)] fn off_yellow	() -> Self { Self::from(Rgb24::from(0xf1f33f	)) }
    #[inline(always)] fn pale_olive_green	() -> Self { Self::from(Rgb24::from(0xb1d27b	)) }
    #[inline(always)] fn orangish	() -> Self { Self::from(Rgb24::from(0xfc824a	)) }
    #[inline(always)] fn leaf	() -> Self { Self::from(Rgb24::from(0x71aa34	)) }
    #[inline(always)] fn light_blue_grey	() -> Self { Self::from(Rgb24::from(0xb7c9e2	)) }
    #[inline(always)] fn dried_blood	() -> Self { Self::from(Rgb24::from(0x4b0101	)) }
    #[inline(always)] fn lightish_purple	() -> Self { Self::from(Rgb24::from(0xa552e6	)) }
    #[inline(always)] fn rusty_red	() -> Self { Self::from(Rgb24::from(0xaf2f0d	)) }
    #[inline(always)] fn lavender_blue	() -> Self { Self::from(Rgb24::from(0x8b88f8	)) }
    #[inline(always)] fn light_grass_green	() -> Self { Self::from(Rgb24::from(0x9af764	)) }
    #[inline(always)] fn light_mint_green	() -> Self { Self::from(Rgb24::from(0xa6fbb2	)) }
    #[inline(always)] fn sunflower	() -> Self { Self::from(Rgb24::from(0xffc512	)) }
    #[inline(always)] fn velvet	() -> Self { Self::from(Rgb24::from(0x750851	)) }
    #[inline(always)] fn brick_orange	() -> Self { Self::from(Rgb24::from(0xc14a09	)) }
    #[inline(always)] fn lightish_red	() -> Self { Self::from(Rgb24::from(0xfe2f4a	)) }
    #[inline(always)] fn pure_blue	() -> Self { Self::from(Rgb24::from(0x0203e2	)) }
    #[inline(always)] fn twilight_blue	() -> Self { Self::from(Rgb24::from(0x0a437a	)) }
    #[inline(always)] fn violet_red	() -> Self { Self::from(Rgb24::from(0xa50055	)) }
    #[inline(always)] fn yellowy_brown	() -> Self { Self::from(Rgb24::from(0xae8b0c	)) }
    #[inline(always)] fn carnation	() -> Self { Self::from(Rgb24::from(0xfd798f	)) }
    #[inline(always)] fn muddy_yellow	() -> Self { Self::from(Rgb24::from(0xbfac05	)) }
    #[inline(always)] fn dark_seafoam_green	() -> Self { Self::from(Rgb24::from(0x3eaf76	)) }
    #[inline(always)] fn deep_rose	() -> Self { Self::from(Rgb24::from(0xc74767	)) }
    #[inline(always)] fn dusty_red	() -> Self { Self::from(Rgb24::from(0xb9484e	)) }
    #[inline(always)] fn grey_or_blue	() -> Self { Self::from(Rgb24::from(0x647d8e	)) }
    #[inline(always)] fn lemon_lime	() -> Self { Self::from(Rgb24::from(0xbffe28	)) }
    #[inline(always)] fn purple_or_pink	() -> Self { Self::from(Rgb24::from(0xd725de	)) }
    #[inline(always)] fn brown_yellow	() -> Self { Self::from(Rgb24::from(0xb29705	)) }
    #[inline(always)] fn purple_brown	() -> Self { Self::from(Rgb24::from(0x673a3f	)) }
    #[inline(always)] fn wisteria	() -> Self { Self::from(Rgb24::from(0xa87dc2	)) }
    #[inline(always)] fn banana_yellow	() -> Self { Self::from(Rgb24::from(0xfafe4b	)) }
    #[inline(always)] fn lipstick_red	() -> Self { Self::from(Rgb24::from(0xc0022f	)) }
    #[inline(always)] fn water_blue	() -> Self { Self::from(Rgb24::from(0x0e87cc	)) }
    #[inline(always)] fn brown_grey	() -> Self { Self::from(Rgb24::from(0x8d8468	)) }
    #[inline(always)] fn vibrant_purple	() -> Self { Self::from(Rgb24::from(0xad03de	)) }
    #[inline(always)] fn baby_green	() -> Self { Self::from(Rgb24::from(0x8cff9e	)) }
    #[inline(always)] fn barf_green	() -> Self { Self::from(Rgb24::from(0x94ac02	)) }
    #[inline(always)] fn eggshell_blue	() -> Self { Self::from(Rgb24::from(0xc4fff7	)) }
    #[inline(always)] fn sandy_yellow	() -> Self { Self::from(Rgb24::from(0xfdee73	)) }
    #[inline(always)] fn cool_green	() -> Self { Self::from(Rgb24::from(0x33b864	)) }
    #[inline(always)] fn pale	() -> Self { Self::from(Rgb24::from(0xfff9d0	)) }
    #[inline(always)] fn blue_or_grey	() -> Self { Self::from(Rgb24::from(0x758da3	)) }
    #[inline(always)] fn hot_magenta	() -> Self { Self::from(Rgb24::from(0xf504c9	)) }
    #[inline(always)] fn greyblue	() -> Self { Self::from(Rgb24::from(0x77a1b5	)) }
    #[inline(always)] fn purpley	() -> Self { Self::from(Rgb24::from(0x8756e4	)) }
    #[inline(always)] fn baby_shit_green	() -> Self { Self::from(Rgb24::from(0x889717	)) }
    #[inline(always)] fn brownish_pink	() -> Self { Self::from(Rgb24::from(0xc27e79	)) }
    #[inline(always)] fn dark_aquamarine	() -> Self { Self::from(Rgb24::from(0x017371	)) }
    #[inline(always)] fn diarrhea	() -> Self { Self::from(Rgb24::from(0x9f8303	)) }
    #[inline(always)] fn light_mustard	() -> Self { Self::from(Rgb24::from(0xf7d560	)) }
    #[inline(always)] fn pale_sky_blue	() -> Self { Self::from(Rgb24::from(0xbdf6fe	)) }
    #[inline(always)] fn turtle_green	() -> Self { Self::from(Rgb24::from(0x75b84f	)) }
    #[inline(always)] fn bright_olive	() -> Self { Self::from(Rgb24::from(0x9cbb04	)) }
    #[inline(always)] fn dark_grey_blue	() -> Self { Self::from(Rgb24::from(0x29465b	)) }
    #[inline(always)] fn greeny_brown	() -> Self { Self::from(Rgb24::from(0x696006	)) }
    #[inline(always)] fn lemon_green	() -> Self { Self::from(Rgb24::from(0xadf802	)) }
    #[inline(always)] fn light_periwinkle	() -> Self { Self::from(Rgb24::from(0xc1c6fc	)) }
    #[inline(always)] fn seaweed_green	() -> Self { Self::from(Rgb24::from(0x35ad6b	)) }
    #[inline(always)] fn sunshine_yellow	() -> Self { Self::from(Rgb24::from(0xfffd37	)) }
    #[inline(always)] fn ugly_purple	() -> Self { Self::from(Rgb24::from(0xa442a0	)) }
    #[inline(always)] fn medium_pink	() -> Self { Self::from(Rgb24::from(0xf36196	)) }
    #[inline(always)] fn puke_brown	() -> Self { Self::from(Rgb24::from(0x947706	)) }
    #[inline(always)] fn very_light_pink	() -> Self { Self::from(Rgb24::from(0xfff4f2	)) }
    #[inline(always)] fn viridian	() -> Self { Self::from(Rgb24::from(0x1e9167	)) }
    #[inline(always)] fn bile	() -> Self { Self::from(Rgb24::from(0xb5c306	)) }
    #[inline(always)] fn faded_yellow	() -> Self { Self::from(Rgb24::from(0xfeff7f	)) }
    #[inline(always)] fn very_pale_green	() -> Self { Self::from(Rgb24::from(0xcffdbc	)) }
    #[inline(always)] fn vibrant_green	() -> Self { Self::from(Rgb24::from(0x0add08	)) }
    #[inline(always)] fn bright_lime	() -> Self { Self::from(Rgb24::from(0x87fd05	)) }
    #[inline(always)] fn spearmint	() -> Self { Self::from(Rgb24::from(0x1ef876	)) }
    #[inline(always)] fn light_aquamarine	() -> Self { Self::from(Rgb24::from(0x7bfdc7	)) }
    #[inline(always)] fn light_sage	() -> Self { Self::from(Rgb24::from(0xbcecac	)) }
    #[inline(always)] fn yellowgreen	() -> Self { Self::from(Rgb24::from(0xbbf90f	)) }
    #[inline(always)] fn baby_poo	() -> Self { Self::from(Rgb24::from(0xab9004	)) }
    #[inline(always)] fn dark_seafoam	() -> Self { Self::from(Rgb24::from(0x1fb57a	)) }
    #[inline(always)] fn deep_teal	() -> Self { Self::from(Rgb24::from(0x00555a	)) }
    #[inline(always)] fn heather	() -> Self { Self::from(Rgb24::from(0xa484ac	)) }
    #[inline(always)] fn rust_orange	() -> Self { Self::from(Rgb24::from(0xc45508	)) }
    #[inline(always)] fn dirty_blue	() -> Self { Self::from(Rgb24::from(0x3f829d	)) }
    #[inline(always)] fn fern_green	() -> Self { Self::from(Rgb24::from(0x548d44	)) }
    #[inline(always)] fn bright_lilac	() -> Self { Self::from(Rgb24::from(0xc95efb	)) }
    #[inline(always)] fn weird_green	() -> Self { Self::from(Rgb24::from(0x3ae57f	)) }
    #[inline(always)] fn peacock_blue	() -> Self { Self::from(Rgb24::from(0x016795	)) }
    #[inline(always)] fn avocado_green	() -> Self { Self::from(Rgb24::from(0x87a922	)) }
    #[inline(always)] fn faded_orange	() -> Self { Self::from(Rgb24::from(0xf0944d	)) }
    #[inline(always)] fn grape_purple	() -> Self { Self::from(Rgb24::from(0x5d1451	)) }
    #[inline(always)] fn hot_green	() -> Self { Self::from(Rgb24::from(0x25ff29	)) }
    #[inline(always)] fn lime_yellow	() -> Self { Self::from(Rgb24::from(0xd0fe1d	)) }
    #[inline(always)] fn mango	() -> Self { Self::from(Rgb24::from(0xffa62b	)) }
    #[inline(always)] fn shamrock	() -> Self { Self::from(Rgb24::from(0x01b44c	)) }
    #[inline(always)] fn bubblegum	() -> Self { Self::from(Rgb24::from(0xff6cb5	)) }
    #[inline(always)] fn purplish_brown	() -> Self { Self::from(Rgb24::from(0x6b4247	)) }
    #[inline(always)] fn vomit_yellow	() -> Self { Self::from(Rgb24::from(0xc7c10c	)) }
    #[inline(always)] fn pale_cyan	() -> Self { Self::from(Rgb24::from(0xb7fffa	)) }
    #[inline(always)] fn key_lime	() -> Self { Self::from(Rgb24::from(0xaeff6e	)) }
    #[inline(always)] fn tomato_red	() -> Self { Self::from(Rgb24::from(0xec2d01	)) }
    #[inline(always)] fn lightgreen	() -> Self { Self::from(Rgb24::from(0x76ff7b	)) }
    #[inline(always)] fn merlot	() -> Self { Self::from(Rgb24::from(0x730039	)) }
    #[inline(always)] fn night_blue	() -> Self { Self::from(Rgb24::from(0x040348	)) }
    #[inline(always)] fn purpleish_pink	() -> Self { Self::from(Rgb24::from(0xdf4ec8	)) }
    #[inline(always)] fn apple	() -> Self { Self::from(Rgb24::from(0x6ecb3c	)) }
    #[inline(always)] fn baby_poop_green	() -> Self { Self::from(Rgb24::from(0x8f9805	)) }
    #[inline(always)] fn green_apple	() -> Self { Self::from(Rgb24::from(0x5edc1f	)) }
    #[inline(always)] fn heliotrope	() -> Self { Self::from(Rgb24::from(0xd94ff5	)) }
    #[inline(always)] fn yellow_or_green	() -> Self { Self::from(Rgb24::from(0xc8fd3d	)) }
    #[inline(always)] fn almost_black	() -> Self { Self::from(Rgb24::from(0x070d0d	)) }
    #[inline(always)] fn cool_blue	() -> Self { Self::from(Rgb24::from(0x4984b8	)) }
    #[inline(always)] fn leafy_green	() -> Self { Self::from(Rgb24::from(0x51b73b	)) }
    #[inline(always)] fn mustard_brown	() -> Self { Self::from(Rgb24::from(0xac7e04	)) }
    #[inline(always)] fn dusk	() -> Self { Self::from(Rgb24::from(0x4e5481	)) }
    #[inline(always)] fn dull_brown	() -> Self { Self::from(Rgb24::from(0x876e4b	)) }
    #[inline(always)] fn frog_green	() -> Self { Self::from(Rgb24::from(0x58bc08	)) }
    #[inline(always)] fn vivid_green	() -> Self { Self::from(Rgb24::from(0x2fef10	)) }
    #[inline(always)] fn bright_light_green	() -> Self { Self::from(Rgb24::from(0x2dfe54	)) }
    #[inline(always)] fn fluro_green	() -> Self { Self::from(Rgb24::from(0x0aff02	)) }
    #[inline(always)] fn kiwi	() -> Self { Self::from(Rgb24::from(0x9cef43	)) }
    #[inline(always)] fn seaweed	() -> Self { Self::from(Rgb24::from(0x18d17b	)) }
    #[inline(always)] fn navy_green	() -> Self { Self::from(Rgb24::from(0x35530a	)) }
    #[inline(always)] fn ultramarine_blue	() -> Self { Self::from(Rgb24::from(0x1805db	)) }
    #[inline(always)] fn iris	() -> Self { Self::from(Rgb24::from(0x6258c4	)) }
    #[inline(always)] fn pastel_orange	() -> Self { Self::from(Rgb24::from(0xff964f	)) }
    #[inline(always)] fn yellowish_orange	() -> Self { Self::from(Rgb24::from(0xffab0f	)) }
    #[inline(always)] fn perrywinkle	() -> Self { Self::from(Rgb24::from(0x8f8ce7	)) }
    #[inline(always)] fn tealish	() -> Self { Self::from(Rgb24::from(0x24bca8	)) }
    #[inline(always)] fn dark_plum	() -> Self { Self::from(Rgb24::from(0x3f012c	)) }
    #[inline(always)] fn pear	() -> Self { Self::from(Rgb24::from(0xcbf85f	)) }
    #[inline(always)] fn pinkish_orange	() -> Self { Self::from(Rgb24::from(0xff724c	)) }
    #[inline(always)] fn midnight_purple	() -> Self { Self::from(Rgb24::from(0x280137	)) }
    #[inline(always)] fn light_urple	() -> Self { Self::from(Rgb24::from(0xb36ff6	)) }
    #[inline(always)] fn dark_mint	() -> Self { Self::from(Rgb24::from(0x48c072	)) }
    #[inline(always)] fn greenish_tan	() -> Self { Self::from(Rgb24::from(0xbccb7a	)) }
    #[inline(always)] fn light_burgundy	() -> Self { Self::from(Rgb24::from(0xa8415b	)) }
    #[inline(always)] fn turquoise_blue	() -> Self { Self::from(Rgb24::from(0x06b1c4	)) }
    #[inline(always)] fn ugly_pink	() -> Self { Self::from(Rgb24::from(0xcd7584	)) }
    #[inline(always)] fn sandy	() -> Self { Self::from(Rgb24::from(0xf1da7a	)) }
    #[inline(always)] fn electric_pink	() -> Self { Self::from(Rgb24::from(0xff0490	)) }
    #[inline(always)] fn muted_purple	() -> Self { Self::from(Rgb24::from(0x805b87	)) }
    #[inline(always)] fn mid_green	() -> Self { Self::from(Rgb24::from(0x50a747	)) }
    #[inline(always)] fn greyish	() -> Self { Self::from(Rgb24::from(0xa8a495	)) }
    #[inline(always)] fn neon_yellow	() -> Self { Self::from(Rgb24::from(0xcfff04	)) }
    #[inline(always)] fn banana	() -> Self { Self::from(Rgb24::from(0xffff7e	)) }
    #[inline(always)] fn carnation_pink	() -> Self { Self::from(Rgb24::from(0xff7fa7	)) }
    #[inline(always)] fn tomato	() -> Self { Self::from(Rgb24::from(0xef4026	)) }
    #[inline(always)] fn sea	() -> Self { Self::from(Rgb24::from(0x3c9992	)) }
    #[inline(always)] fn muddy_brown	() -> Self { Self::from(Rgb24::from(0x886806	)) }
    #[inline(always)] fn turquoise_green	() -> Self { Self::from(Rgb24::from(0x04f489	)) }
    #[inline(always)] fn buff	() -> Self { Self::from(Rgb24::from(0xfef69e	)) }
    #[inline(always)] fn fawn	() -> Self { Self::from(Rgb24::from(0xcfaf7b	)) }
    #[inline(always)] fn muted_blue	() -> Self { Self::from(Rgb24::from(0x3b719f	)) }
    #[inline(always)] fn pale_rose	() -> Self { Self::from(Rgb24::from(0xfdc1c5	)) }
    #[inline(always)] fn dark_mint_green	() -> Self { Self::from(Rgb24::from(0x20c073	)) }
    #[inline(always)] fn amethyst	() -> Self { Self::from(Rgb24::from(0x9b5fc0	)) }
    #[inline(always)] fn blue_or_green	() -> Self { Self::from(Rgb24::from(0x0f9b8e	)) }
    #[inline(always)] fn chestnut	() -> Self { Self::from(Rgb24::from(0x742802	)) }
    #[inline(always)] fn sick_green	() -> Self { Self::from(Rgb24::from(0x9db92c	)) }
    #[inline(always)] fn pea	() -> Self { Self::from(Rgb24::from(0xa4bf20	)) }
    #[inline(always)] fn rusty_orange	() -> Self { Self::from(Rgb24::from(0xcd5909	)) }
    #[inline(always)] fn stone	() -> Self { Self::from(Rgb24::from(0xada587	)) }
    #[inline(always)] fn rose_red	() -> Self { Self::from(Rgb24::from(0xbe013c	)) }
    #[inline(always)] fn pale_aqua	() -> Self { Self::from(Rgb24::from(0xb8ffeb	)) }
    #[inline(always)] fn deep_orange	() -> Self { Self::from(Rgb24::from(0xdc4d01	)) }
    #[inline(always)] fn earth	() -> Self { Self::from(Rgb24::from(0xa2653e	)) }
    #[inline(always)] fn mossy_green	() -> Self { Self::from(Rgb24::from(0x638b27	)) }
    #[inline(always)] fn grassy_green	() -> Self { Self::from(Rgb24::from(0x419c03	)) }
    #[inline(always)] fn pale_lime_green	() -> Self { Self::from(Rgb24::from(0xb1ff65	)) }
    #[inline(always)] fn light_grey_blue	() -> Self { Self::from(Rgb24::from(0x9dbcd4	)) }
    #[inline(always)] fn pale_grey	() -> Self { Self::from(Rgb24::from(0xfdfdfe	)) }
    #[inline(always)] fn asparagus	() -> Self { Self::from(Rgb24::from(0x77ab56	)) }
    #[inline(always)] fn blueberry	() -> Self { Self::from(Rgb24::from(0x464196	)) }
    #[inline(always)] fn purple_red	() -> Self { Self::from(Rgb24::from(0x990147	)) }
    #[inline(always)] fn pale_lime	() -> Self { Self::from(Rgb24::from(0xbefd73	)) }
    #[inline(always)] fn greenish_teal	() -> Self { Self::from(Rgb24::from(0x32bf84	)) }
    #[inline(always)] fn caramel	() -> Self { Self::from(Rgb24::from(0xaf6f09	)) }
    #[inline(always)] fn deep_magenta	() -> Self { Self::from(Rgb24::from(0xa0025c	)) }
    #[inline(always)] fn light_peach	() -> Self { Self::from(Rgb24::from(0xffd8b1	)) }
    #[inline(always)] fn milk_chocolate	() -> Self { Self::from(Rgb24::from(0x7f4e1e	)) }
    #[inline(always)] fn ocher	() -> Self { Self::from(Rgb24::from(0xbf9b0c	)) }
    #[inline(always)] fn off_green	() -> Self { Self::from(Rgb24::from(0x6ba353	)) }
    #[inline(always)] fn purply_pink	() -> Self { Self::from(Rgb24::from(0xf075e6	)) }
    #[inline(always)] fn lightblue	() -> Self { Self::from(Rgb24::from(0x7bc8f6	)) }
    #[inline(always)] fn dusky_blue	() -> Self { Self::from(Rgb24::from(0x475f94	)) }
    #[inline(always)] fn golden	() -> Self { Self::from(Rgb24::from(0xf5bf03	)) }
    #[inline(always)] fn light_beige	() -> Self { Self::from(Rgb24::from(0xfffeb6	)) }
    #[inline(always)] fn butter_yellow	() -> Self { Self::from(Rgb24::from(0xfffd74	)) }
    #[inline(always)] fn dusky_purple	() -> Self { Self::from(Rgb24::from(0x895b7b	)) }
    #[inline(always)] fn french_blue	() -> Self { Self::from(Rgb24::from(0x436bad	)) }
    #[inline(always)] fn ugly_yellow	() -> Self { Self::from(Rgb24::from(0xd0c101	)) }
    #[inline(always)] fn greeny_yellow	() -> Self { Self::from(Rgb24::from(0xc6f808	)) }
    #[inline(always)] fn orangish_red	() -> Self { Self::from(Rgb24::from(0xf43605	)) }
    #[inline(always)] fn shamrock_green	() -> Self { Self::from(Rgb24::from(0x02c14d	)) }
    #[inline(always)] fn orangish_brown	() -> Self { Self::from(Rgb24::from(0xb25f03	)) }
    #[inline(always)] fn tree_green	() -> Self { Self::from(Rgb24::from(0x2a7e19	)) }
    #[inline(always)] fn deep_violet	() -> Self { Self::from(Rgb24::from(0x490648	)) }
    #[inline(always)] fn gunmetal	() -> Self { Self::from(Rgb24::from(0x536267	)) }
    #[inline(always)] fn blue_or_purple	() -> Self { Self::from(Rgb24::from(0x5a06ef	)) }
    #[inline(always)] fn cherry	() -> Self { Self::from(Rgb24::from(0xcf0234	)) }
    #[inline(always)] fn sandy_brown	() -> Self { Self::from(Rgb24::from(0xc4a661	)) }
    #[inline(always)] fn warm_grey	() -> Self { Self::from(Rgb24::from(0x978a84	)) }
    #[inline(always)] fn dark_indigo	() -> Self { Self::from(Rgb24::from(0x1f0954	)) }
    #[inline(always)] fn midnight	() -> Self { Self::from(Rgb24::from(0x03012d	)) }
    #[inline(always)] fn bluey_green	() -> Self { Self::from(Rgb24::from(0x2bb179	)) }
    #[inline(always)] fn grey_pink	() -> Self { Self::from(Rgb24::from(0xc3909b	)) }
    #[inline(always)] fn soft_purple	() -> Self { Self::from(Rgb24::from(0xa66fb5	)) }
    #[inline(always)] fn blood	() -> Self { Self::from(Rgb24::from(0x770001	)) }
    #[inline(always)] fn brown_red	() -> Self { Self::from(Rgb24::from(0x922b05	)) }
    #[inline(always)] fn medium_grey	() -> Self { Self::from(Rgb24::from(0x7d7f7c	)) }
    #[inline(always)] fn berry	() -> Self { Self::from(Rgb24::from(0x990f4b	)) }
    #[inline(always)] fn poo	() -> Self { Self::from(Rgb24::from(0x8f7303	)) }
    #[inline(always)] fn purpley_pink	() -> Self { Self::from(Rgb24::from(0xc83cb9	)) }
    #[inline(always)] fn light_salmon	() -> Self { Self::from(Rgb24::from(0xfea993	)) }
    #[inline(always)] fn snot	() -> Self { Self::from(Rgb24::from(0xacbb0d	)) }
    #[inline(always)] fn easter_purple	() -> Self { Self::from(Rgb24::from(0xc071fe	)) }
    #[inline(always)] fn light_yellow_green	() -> Self { Self::from(Rgb24::from(0xccfd7f	)) }
    #[inline(always)] fn dark_navy_blue	() -> Self { Self::from(Rgb24::from(0x00022e	)) }
    #[inline(always)] fn drab	() -> Self { Self::from(Rgb24::from(0x828344	)) }
    #[inline(always)] fn light_rose	() -> Self { Self::from(Rgb24::from(0xffc5cb	)) }
    #[inline(always)] fn rouge	() -> Self { Self::from(Rgb24::from(0xab1239	)) }
    #[inline(always)] fn purplish_red	() -> Self { Self::from(Rgb24::from(0xb0054b	)) }
    #[inline(always)] fn slime_green	() -> Self { Self::from(Rgb24::from(0x99cc04	)) }
    #[inline(always)] fn baby_poop	() -> Self { Self::from(Rgb24::from(0x937c00	)) }
    #[inline(always)] fn irish_green	() -> Self { Self::from(Rgb24::from(0x019529	)) }
    #[inline(always)] fn pink_or_purple	() -> Self { Self::from(Rgb24::from(0xef1de7	)) }
    #[inline(always)] fn dark_navy	() -> Self { Self::from(Rgb24::from(0x000435	)) }
    #[inline(always)] fn greeny_blue	() -> Self { Self::from(Rgb24::from(0x42b395	)) }
    #[inline(always)] fn light_plum	() -> Self { Self::from(Rgb24::from(0x9d5783	)) }
    #[inline(always)] fn pinkish_grey	() -> Self { Self::from(Rgb24::from(0xc8aca9	)) }
    #[inline(always)] fn dirty_orange	() -> Self { Self::from(Rgb24::from(0xc87606	)) }
    #[inline(always)] fn rust_red	() -> Self { Self::from(Rgb24::from(0xaa2704	)) }
    #[inline(always)] fn pale_lilac	() -> Self { Self::from(Rgb24::from(0xe4cbff	)) }
    #[inline(always)] fn orangey_red	() -> Self { Self::from(Rgb24::from(0xfa4224	)) }
    #[inline(always)] fn primary_blue	() -> Self { Self::from(Rgb24::from(0x0804f9	)) }
    #[inline(always)] fn kermit_green	() -> Self { Self::from(Rgb24::from(0x5cb200	)) }
    #[inline(always)] fn brownish_purple	() -> Self { Self::from(Rgb24::from(0x76424e	)) }
    #[inline(always)] fn murky_green	() -> Self { Self::from(Rgb24::from(0x6c7a0e	)) }
    #[inline(always)] fn wheat	() -> Self { Self::from(Rgb24::from(0xfbdd7e	)) }
    #[inline(always)] fn very_dark_purple	() -> Self { Self::from(Rgb24::from(0x2a0134	)) }
    #[inline(always)] fn bottle_green	() -> Self { Self::from(Rgb24::from(0x044a05	)) }
    #[inline(always)] fn watermelon	() -> Self { Self::from(Rgb24::from(0xfd4659	)) }
    #[inline(always)] fn deep_sky_blue	() -> Self { Self::from(Rgb24::from(0x0d75f8	)) }
    #[inline(always)] fn fire_engine_red	() -> Self { Self::from(Rgb24::from(0xfe0002	)) }
    #[inline(always)] fn yellow_ochre	() -> Self { Self::from(Rgb24::from(0xcb9d06	)) }
    #[inline(always)] fn pumpkin_orange	() -> Self { Self::from(Rgb24::from(0xfb7d07	)) }
    #[inline(always)] fn pale_olive	() -> Self { Self::from(Rgb24::from(0xb9cc81	)) }
    #[inline(always)] fn light_lilac	() -> Self { Self::from(Rgb24::from(0xedc8ff	)) }
    #[inline(always)] fn lightish_green	() -> Self { Self::from(Rgb24::from(0x61e160	)) }
    #[inline(always)] fn carolina_blue	() -> Self { Self::from(Rgb24::from(0x8ab8fe	)) }
    #[inline(always)] fn mulberry	() -> Self { Self::from(Rgb24::from(0x920a4e	)) }
    #[inline(always)] fn shocking_pink	() -> Self { Self::from(Rgb24::from(0xfe02a2	)) }
    #[inline(always)] fn auburn	() -> Self { Self::from(Rgb24::from(0x9a3001	)) }
    #[inline(always)] fn bright_lime_green	() -> Self { Self::from(Rgb24::from(0x65fe08	)) }
    #[inline(always)] fn celadon	() -> Self { Self::from(Rgb24::from(0xbefdb7	)) }
    #[inline(always)] fn pinkish_brown	() -> Self { Self::from(Rgb24::from(0xb17261	)) }
    #[inline(always)] fn poo_brown	() -> Self { Self::from(Rgb24::from(0x885f01	)) }
    #[inline(always)] fn bright_sky_blue	() -> Self { Self::from(Rgb24::from(0x02ccfe	)) }
    #[inline(always)] fn celery	() -> Self { Self::from(Rgb24::from(0xc1fd95	)) }
    #[inline(always)] fn dirt_brown	() -> Self { Self::from(Rgb24::from(0x836539	)) }
    #[inline(always)] fn strawberry	() -> Self { Self::from(Rgb24::from(0xfb2943	)) }
    #[inline(always)] fn dark_lime	() -> Self { Self::from(Rgb24::from(0x84b701	)) }
    #[inline(always)] fn copper	() -> Self { Self::from(Rgb24::from(0xb66325	)) }
    #[inline(always)] fn medium_brown	() -> Self { Self::from(Rgb24::from(0x7f5112	)) }
    #[inline(always)] fn muted_green	() -> Self { Self::from(Rgb24::from(0x5fa052	)) }
    #[inline(always)] fn robins_egg	() -> Self { Self::from(Rgb24::from(0x6dedfd	)) }
    #[inline(always)] fn bright_aqua	() -> Self { Self::from(Rgb24::from(0x0bf9ea	)) }
    #[inline(always)] fn bright_lavender	() -> Self { Self::from(Rgb24::from(0xc760ff	)) }
    #[inline(always)] fn ivory	() -> Self { Self::from(Rgb24::from(0xffffcb	)) }
    #[inline(always)] fn very_light_purple	() -> Self { Self::from(Rgb24::from(0xf6cefc	)) }
    #[inline(always)] fn light_navy	() -> Self { Self::from(Rgb24::from(0x155084	)) }
    #[inline(always)] fn pink_red	() -> Self { Self::from(Rgb24::from(0xf5054f	)) }
    #[inline(always)] fn olive_brown	() -> Self { Self::from(Rgb24::from(0x645403	)) }
    #[inline(always)] fn poop_brown	() -> Self { Self::from(Rgb24::from(0x7a5901	)) }
    #[inline(always)] fn mustard_green	() -> Self { Self::from(Rgb24::from(0xa8b504	)) }
    #[inline(always)] fn ocean_green	() -> Self { Self::from(Rgb24::from(0x3d9973	)) }
    #[inline(always)] fn very_dark_blue	() -> Self { Self::from(Rgb24::from(0x000133	)) }
    #[inline(always)] fn dusty_green	() -> Self { Self::from(Rgb24::from(0x76a973	)) }
    #[inline(always)] fn light_navy_blue	() -> Self { Self::from(Rgb24::from(0x2e5a88	)) }
    #[inline(always)] fn minty_green	() -> Self { Self::from(Rgb24::from(0x0bf77d	)) }
    #[inline(always)] fn adobe	() -> Self { Self::from(Rgb24::from(0xbd6c48	)) }
    #[inline(always)] fn barney	() -> Self { Self::from(Rgb24::from(0xac1db8	)) }
    #[inline(always)] fn jade_green	() -> Self { Self::from(Rgb24::from(0x2baf6a	)) }
    #[inline(always)] fn bright_light_blue	() -> Self { Self::from(Rgb24::from(0x26f7fd	)) }
    #[inline(always)] fn light_lime	() -> Self { Self::from(Rgb24::from(0xaefd6c	)) }
    #[inline(always)] fn dark_khaki	() -> Self { Self::from(Rgb24::from(0x9b8f55	)) }
    #[inline(always)] fn orange_yellow	() -> Self { Self::from(Rgb24::from(0xffad01	)) }
    #[inline(always)] fn ocre	() -> Self { Self::from(Rgb24::from(0xc69c04	)) }
    #[inline(always)] fn maize	() -> Self { Self::from(Rgb24::from(0xf4d054	)) }
    #[inline(always)] fn faded_pink	() -> Self { Self::from(Rgb24::from(0xde9dac	)) }
    #[inline(always)] fn british_racing_green	() -> Self { Self::from(Rgb24::from(0x05480d	)) }
    #[inline(always)] fn sandstone	() -> Self { Self::from(Rgb24::from(0xc9ae74	)) }
    #[inline(always)] fn mud_brown	() -> Self { Self::from(Rgb24::from(0x60460f	)) }
    #[inline(always)] fn light_sea_green	() -> Self { Self::from(Rgb24::from(0x98f6b0	)) }
    #[inline(always)] fn robin_egg_blue	() -> Self { Self::from(Rgb24::from(0x8af1fe	)) }
    #[inline(always)] fn aqua_marine	() -> Self { Self::from(Rgb24::from(0x2ee8bb	)) }
    #[inline(always)] fn dark_sea_green	() -> Self { Self::from(Rgb24::from(0x11875d	)) }
    #[inline(always)] fn soft_pink	() -> Self { Self::from(Rgb24::from(0xfdb0c0	)) }
    #[inline(always)] fn orangey_brown	() -> Self { Self::from(Rgb24::from(0xb16002	)) }
    #[inline(always)] fn cherry_red	() -> Self { Self::from(Rgb24::from(0xf7022a	)) }
    #[inline(always)] fn burnt_yellow	() -> Self { Self::from(Rgb24::from(0xd5ab09	)) }
    #[inline(always)] fn brownish_grey	() -> Self { Self::from(Rgb24::from(0x86775f	)) }
    #[inline(always)] fn camel	() -> Self { Self::from(Rgb24::from(0xc69f59	)) }
    #[inline(always)] fn purplish_grey	() -> Self { Self::from(Rgb24::from(0x7a687f	)) }
    #[inline(always)] fn marine	() -> Self { Self::from(Rgb24::from(0x042e60	)) }
    #[inline(always)] fn greyish_pink	() -> Self { Self::from(Rgb24::from(0xc88d94	)) }
    #[inline(always)] fn pale_turquoise	() -> Self { Self::from(Rgb24::from(0xa5fbd5	)) }
    #[inline(always)] fn pastel_yellow	() -> Self { Self::from(Rgb24::from(0xfffe71	)) }
    #[inline(always)] fn bluey_purple	() -> Self { Self::from(Rgb24::from(0x6241c7	)) }
    #[inline(always)] fn canary_yellow	() -> Self { Self::from(Rgb24::from(0xfffe40	)) }
    #[inline(always)] fn faded_red	() -> Self { Self::from(Rgb24::from(0xd3494e	)) }
    #[inline(always)] fn sepia	() -> Self { Self::from(Rgb24::from(0x985e2b	)) }
    #[inline(always)] fn coffee	() -> Self { Self::from(Rgb24::from(0xa6814c	)) }
    #[inline(always)] fn bright_magenta	() -> Self { Self::from(Rgb24::from(0xff08e8	)) }
    #[inline(always)] fn mocha	() -> Self { Self::from(Rgb24::from(0x9d7651	)) }
    #[inline(always)] fn ecru	() -> Self { Self::from(Rgb24::from(0xfeffca	)) }
    #[inline(always)] fn purpleish	() -> Self { Self::from(Rgb24::from(0x98568d	)) }
    #[inline(always)] fn cranberry	() -> Self { Self::from(Rgb24::from(0x9e003a	)) }
    #[inline(always)] fn darkish_green	() -> Self { Self::from(Rgb24::from(0x287c37	)) }
    #[inline(always)] fn brown_orange	() -> Self { Self::from(Rgb24::from(0xb96902	)) }
    #[inline(always)] fn dusky_rose	() -> Self { Self::from(Rgb24::from(0xba6873	)) }
    #[inline(always)] fn melon	() -> Self { Self::from(Rgb24::from(0xff7855	)) }
    #[inline(always)] fn sickly_green	() -> Self { Self::from(Rgb24::from(0x94b21c	)) }
    #[inline(always)] fn silver	() -> Self { Self::from(Rgb24::from(0xc5c9c7	)) }
    #[inline(always)] fn purply_blue	() -> Self { Self::from(Rgb24::from(0x661aee	)) }
    #[inline(always)] fn purpleish_blue	() -> Self { Self::from(Rgb24::from(0x6140ef	)) }
    #[inline(always)] fn hospital_green	() -> Self { Self::from(Rgb24::from(0x9be5aa	)) }
    #[inline(always)] fn shit_brown	() -> Self { Self::from(Rgb24::from(0x7b5804	)) }
    #[inline(always)] fn mid_blue	() -> Self { Self::from(Rgb24::from(0x276ab3	)) }
    #[inline(always)] fn amber	() -> Self { Self::from(Rgb24::from(0xfeb308	)) }
    #[inline(always)] fn easter_green	() -> Self { Self::from(Rgb24::from(0x8cfd7e	)) }
    #[inline(always)] fn soft_blue	() -> Self { Self::from(Rgb24::from(0x6488ea	)) }
    #[inline(always)] fn cerulean_blue	() -> Self { Self::from(Rgb24::from(0x056eee	)) }
    #[inline(always)] fn golden_brown	() -> Self { Self::from(Rgb24::from(0xb27a01	)) }
    #[inline(always)] fn bright_turquoise	() -> Self { Self::from(Rgb24::from(0x0ffef9	)) }
    #[inline(always)] fn red_pink	() -> Self { Self::from(Rgb24::from(0xfa2a55	)) }
    #[inline(always)] fn red_purple	() -> Self { Self::from(Rgb24::from(0x820747	)) }
    #[inline(always)] fn greyish_brown	() -> Self { Self::from(Rgb24::from(0x7a6a4f	)) }
    #[inline(always)] fn vermillion	() -> Self { Self::from(Rgb24::from(0xf4320c	)) }
    #[inline(always)] fn russet	() -> Self { Self::from(Rgb24::from(0xa13905	)) }
    #[inline(always)] fn steel_grey	() -> Self { Self::from(Rgb24::from(0x6f828a	)) }
    #[inline(always)] fn lighter_purple	() -> Self { Self::from(Rgb24::from(0xa55af4	)) }
    #[inline(always)] fn bright_violet	() -> Self { Self::from(Rgb24::from(0xad0afd	)) }
    #[inline(always)] fn prussian_blue	() -> Self { Self::from(Rgb24::from(0x004577	)) }
    #[inline(always)] fn slate_green	() -> Self { Self::from(Rgb24::from(0x658d6d	)) }
    #[inline(always)] fn dirty_pink	() -> Self { Self::from(Rgb24::from(0xca7b80	)) }
    #[inline(always)] fn dark_blue_green	() -> Self { Self::from(Rgb24::from(0x005249	)) }
    #[inline(always)] fn pine	() -> Self { Self::from(Rgb24::from(0x2b5d34	)) }
    #[inline(always)] fn yellowy_green	() -> Self { Self::from(Rgb24::from(0xbff128	)) }
    #[inline(always)] fn dark_gold	() -> Self { Self::from(Rgb24::from(0xb59410	)) }
    #[inline(always)] fn bluish	() -> Self { Self::from(Rgb24::from(0x2976bb	)) }
    #[inline(always)] fn darkish_blue	() -> Self { Self::from(Rgb24::from(0x014182	)) }
    #[inline(always)] fn dull_red	() -> Self { Self::from(Rgb24::from(0xbb3f3f	)) }
    #[inline(always)] fn pinky_red	() -> Self { Self::from(Rgb24::from(0xfc2647	)) }
    #[inline(always)] fn bronze	() -> Self { Self::from(Rgb24::from(0xa87900	)) }
    #[inline(always)] fn pale_teal	() -> Self { Self::from(Rgb24::from(0x82cbb2	)) }
    #[inline(always)] fn military_green	() -> Self { Self::from(Rgb24::from(0x667c3e	)) }
    #[inline(always)] fn barbie_pink	() -> Self { Self::from(Rgb24::from(0xfe46a5	)) }
    #[inline(always)] fn bubblegum_pink	() -> Self { Self::from(Rgb24::from(0xfe83cc	)) }
    #[inline(always)] fn pea_soup_green	() -> Self { Self::from(Rgb24::from(0x94a617	)) }
    #[inline(always)] fn dark_mustard	() -> Self { Self::from(Rgb24::from(0xa88905	)) }
    #[inline(always)] fn shit	() -> Self { Self::from(Rgb24::from(0x7f5f00	)) }
    #[inline(always)] fn medium_purple	() -> Self { Self::from(Rgb24::from(0x9e43a2	)) }
    #[inline(always)] fn very_dark_green	() -> Self { Self::from(Rgb24::from(0x062e03	)) }
    #[inline(always)] fn dirt	() -> Self { Self::from(Rgb24::from(0x8a6e45	)) }
    #[inline(always)] fn dusky_pink	() -> Self { Self::from(Rgb24::from(0xcc7a8b	)) }
    #[inline(always)] fn red_violet	() -> Self { Self::from(Rgb24::from(0x9e0168	)) }
    #[inline(always)] fn lemon_yellow	() -> Self { Self::from(Rgb24::from(0xfdff38	)) }
    #[inline(always)] fn pistachio	() -> Self { Self::from(Rgb24::from(0xc0fa8b	)) }
    #[inline(always)] fn dull_yellow	() -> Self { Self::from(Rgb24::from(0xeedc5b	)) }
    #[inline(always)] fn dark_lime_green	() -> Self { Self::from(Rgb24::from(0x7ebd01	)) }
    #[inline(always)] fn denim_blue	() -> Self { Self::from(Rgb24::from(0x3b5b92	)) }
    #[inline(always)] fn teal_blue	() -> Self { Self::from(Rgb24::from(0x01889f	)) }
    #[inline(always)] fn lightish_blue	() -> Self { Self::from(Rgb24::from(0x3d7afd	)) }
    #[inline(always)] fn purpley_blue	() -> Self { Self::from(Rgb24::from(0x5f34e7	)) }
    #[inline(always)] fn light_indigo	() -> Self { Self::from(Rgb24::from(0x6d5acf	)) }
    #[inline(always)] fn swamp_green	() -> Self { Self::from(Rgb24::from(0x748500	)) }
    #[inline(always)] fn brown_green	() -> Self { Self::from(Rgb24::from(0x706c11	)) }
    #[inline(always)] fn dark_maroon	() -> Self { Self::from(Rgb24::from(0x3c0008	)) }
    #[inline(always)] fn hot_purple	() -> Self { Self::from(Rgb24::from(0xcb00f5	)) }
    #[inline(always)] fn dark_forest_green	() -> Self { Self::from(Rgb24::from(0x002d04	)) }
    #[inline(always)] fn faded_blue	() -> Self { Self::from(Rgb24::from(0x658cbb	)) }
    #[inline(always)] fn drab_green	() -> Self { Self::from(Rgb24::from(0x749551	)) }
    #[inline(always)] fn light_lime_green	() -> Self { Self::from(Rgb24::from(0xb9ff66	)) }
    #[inline(always)] fn snot_green	() -> Self { Self::from(Rgb24::from(0x9dc100	)) }
    #[inline(always)] fn yellowish	() -> Self { Self::from(Rgb24::from(0xfaee66	)) }
    #[inline(always)] fn light_blue_green	() -> Self { Self::from(Rgb24::from(0x7efbb3	)) }
    #[inline(always)] fn bordeaux	() -> Self { Self::from(Rgb24::from(0x7b002c	)) }
    #[inline(always)] fn light_mauve	() -> Self { Self::from(Rgb24::from(0xc292a1	)) }
    #[inline(always)] fn ocean	() -> Self { Self::from(Rgb24::from(0x017b92	)) }
    #[inline(always)] fn marigold	() -> Self { Self::from(Rgb24::from(0xfcc006	)) }
    #[inline(always)] fn muddy_green	() -> Self { Self::from(Rgb24::from(0x657432	)) }
    #[inline(always)] fn dull_orange	() -> Self { Self::from(Rgb24::from(0xd8863b	)) }
    #[inline(always)] fn steel	() -> Self { Self::from(Rgb24::from(0x738595	)) }
    #[inline(always)] fn electric_purple	() -> Self { Self::from(Rgb24::from(0xaa23ff	)) }
    #[inline(always)] fn fluorescent_green	() -> Self { Self::from(Rgb24::from(0x08ff08	)) }
    #[inline(always)] fn yellowish_brown	() -> Self { Self::from(Rgb24::from(0x9b7a01	)) }
    #[inline(always)] fn blush	() -> Self { Self::from(Rgb24::from(0xf29e8e	)) }
    #[inline(always)] fn soft_green	() -> Self { Self::from(Rgb24::from(0x6fc276	)) }
    #[inline(always)] fn bright_orange	() -> Self { Self::from(Rgb24::from(0xff5b00	)) }
    #[inline(always)] fn lemon	() -> Self { Self::from(Rgb24::from(0xfdff52	)) }
    #[inline(always)] fn purple_grey	() -> Self { Self::from(Rgb24::from(0x866f85	)) }
    #[inline(always)] fn acid_green	() -> Self { Self::from(Rgb24::from(0x8ffe09	)) }
    #[inline(always)] fn pale_lavender	() -> Self { Self::from(Rgb24::from(0xeecffe	)) }
    #[inline(always)] fn violet_blue	() -> Self { Self::from(Rgb24::from(0x510ac9	)) }
    #[inline(always)] fn light_forest_green	() -> Self { Self::from(Rgb24::from(0x4f9153	)) }
    #[inline(always)] fn burnt_red	() -> Self { Self::from(Rgb24::from(0x9f2305	)) }
    #[inline(always)] fn khaki_green	() -> Self { Self::from(Rgb24::from(0x728639	)) }
    #[inline(always)] fn cerise	() -> Self { Self::from(Rgb24::from(0xde0c62	)) }
    #[inline(always)] fn faded_purple	() -> Self { Self::from(Rgb24::from(0x916e99	)) }
    #[inline(always)] fn apricot	() -> Self { Self::from(Rgb24::from(0xffb16d	)) }
    #[inline(always)] fn dark_olive_green	() -> Self { Self::from(Rgb24::from(0x3c4d03	)) }
    #[inline(always)] fn grey_brown	() -> Self { Self::from(Rgb24::from(0x7f7053	)) }
    #[inline(always)] fn green_grey	() -> Self { Self::from(Rgb24::from(0x77926f	)) }
    #[inline(always)] fn true_blue	() -> Self { Self::from(Rgb24::from(0x010fcc	)) }
    #[inline(always)] fn pale_violet	() -> Self { Self::from(Rgb24::from(0xceaefa	)) }
    #[inline(always)] fn periwinkle_blue	() -> Self { Self::from(Rgb24::from(0x8f99fb	)) }
    #[inline(always)] fn light_sky_blue	() -> Self { Self::from(Rgb24::from(0xc6fcff	)) }
    #[inline(always)] fn blurple	() -> Self { Self::from(Rgb24::from(0x5539cc	)) }
    #[inline(always)] fn green_brown	() -> Self { Self::from(Rgb24::from(0x544e03	)) }
    #[inline(always)] fn bluegreen	() -> Self { Self::from(Rgb24::from(0x017a79	)) }
    #[inline(always)] fn bright_teal	() -> Self { Self::from(Rgb24::from(0x01f9c6	)) }
    #[inline(always)] fn brownish_yellow	() -> Self { Self::from(Rgb24::from(0xc9b003	)) }
    #[inline(always)] fn pea_soup	() -> Self { Self::from(Rgb24::from(0x929901	)) }
    #[inline(always)] fn forest	() -> Self { Self::from(Rgb24::from(0x0b5509	)) }
    #[inline(always)] fn barney_purple	() -> Self { Self::from(Rgb24::from(0xa00498	)) }
    #[inline(always)] fn ultramarine	() -> Self { Self::from(Rgb24::from(0x2000b1	)) }
    #[inline(always)] fn purplish	() -> Self { Self::from(Rgb24::from(0x94568c	)) }
    #[inline(always)] fn puke_yellow	() -> Self { Self::from(Rgb24::from(0xc2be0e	)) }
    #[inline(always)] fn bluish_grey	() -> Self { Self::from(Rgb24::from(0x748b97	)) }
    #[inline(always)] fn dark_periwinkle	() -> Self { Self::from(Rgb24::from(0x665fd1	)) }
    #[inline(always)] fn dark_lilac	() -> Self { Self::from(Rgb24::from(0x9c6da5	)) }
    #[inline(always)] fn reddish	() -> Self { Self::from(Rgb24::from(0xc44240	)) }
    #[inline(always)] fn light_maroon	() -> Self { Self::from(Rgb24::from(0xa24857	)) }
    #[inline(always)] fn dusty_purple	() -> Self { Self::from(Rgb24::from(0x825f87	)) }
    #[inline(always)] fn terra_cotta	() -> Self { Self::from(Rgb24::from(0xc9643b	)) }
    #[inline(always)] fn avocado	() -> Self { Self::from(Rgb24::from(0x90b134	)) }
    #[inline(always)] fn marine_blue	() -> Self { Self::from(Rgb24::from(0x01386a	)) }
    #[inline(always)] fn teal_green	() -> Self { Self::from(Rgb24::from(0x25a36f	)) }
    #[inline(always)] fn slate_grey	() -> Self { Self::from(Rgb24::from(0x59656d	)) }
    #[inline(always)] fn lighter_green	() -> Self { Self::from(Rgb24::from(0x75fd63	)) }
    #[inline(always)] fn electric_green	() -> Self { Self::from(Rgb24::from(0x21fc0d	)) }
    #[inline(always)] fn dusty_blue	() -> Self { Self::from(Rgb24::from(0x5a86ad	)) }
    #[inline(always)] fn golden_yellow	() -> Self { Self::from(Rgb24::from(0xfec615	)) }
    #[inline(always)] fn bright_yellow	() -> Self { Self::from(Rgb24::from(0xfffd01	)) }
    #[inline(always)] fn light_lavender	() -> Self { Self::from(Rgb24::from(0xdfc5fe	)) }
    #[inline(always)] fn umber	() -> Self { Self::from(Rgb24::from(0xb26400	)) }
    #[inline(always)] fn poop	() -> Self { Self::from(Rgb24::from(0x7f5e00	)) }
    #[inline(always)] fn dark_peach	() -> Self { Self::from(Rgb24::from(0xde7e5d	)) }
    #[inline(always)] fn jungle_green	() -> Self { Self::from(Rgb24::from(0x048243	)) }
    #[inline(always)] fn eggshell	() -> Self { Self::from(Rgb24::from(0xffffd4	)) }
    #[inline(always)] fn denim	() -> Self { Self::from(Rgb24::from(0x3b638c	)) }
    #[inline(always)] fn yellow_brown	() -> Self { Self::from(Rgb24::from(0xb79400	)) }
    #[inline(always)] fn dull_purple	() -> Self { Self::from(Rgb24::from(0x84597e	)) }
    #[inline(always)] fn chocolate_brown	() -> Self { Self::from(Rgb24::from(0x411900	)) }
    #[inline(always)] fn wine_red	() -> Self { Self::from(Rgb24::from(0x7b0323	)) }
    #[inline(always)] fn neon_blue	() -> Self { Self::from(Rgb24::from(0x04d9ff	)) }
    #[inline(always)] fn dirty_green	() -> Self { Self::from(Rgb24::from(0x667e2c	)) }
    #[inline(always)] fn light_tan	() -> Self { Self::from(Rgb24::from(0xfbeeac	)) }
    #[inline(always)] fn ice_blue	() -> Self { Self::from(Rgb24::from(0xd7fffe	)) }
    #[inline(always)] fn cadet_blue	() -> Self { Self::from(Rgb24::from(0x4e7496	)) }
    #[inline(always)] fn dark_mauve	() -> Self { Self::from(Rgb24::from(0x874c62	)) }
    #[inline(always)] fn very_light_blue	() -> Self { Self::from(Rgb24::from(0xd5ffff	)) }
    #[inline(always)] fn grey_purple	() -> Self { Self::from(Rgb24::from(0x826d8c	)) }
    #[inline(always)] fn pastel_pink	() -> Self { Self::from(Rgb24::from(0xffbacd	)) }
    #[inline(always)] fn very_light_green	() -> Self { Self::from(Rgb24::from(0xd1ffbd	)) }
    #[inline(always)] fn dark_sky_blue	() -> Self { Self::from(Rgb24::from(0x448ee4	)) }
    #[inline(always)] fn evergreen	() -> Self { Self::from(Rgb24::from(0x05472a	)) }
    #[inline(always)] fn dull_pink	() -> Self { Self::from(Rgb24::from(0xd5869d	)) }
    #[inline(always)] fn aubergine	() -> Self { Self::from(Rgb24::from(0x3d0734	)) }
    #[inline(always)] fn mahogany	() -> Self { Self::from(Rgb24::from(0x4a0100	)) }
    #[inline(always)] fn reddish_orange	() -> Self { Self::from(Rgb24::from(0xf8481c	)) }
    #[inline(always)] fn deep_green	() -> Self { Self::from(Rgb24::from(0x02590f	)) }
    #[inline(always)] fn vomit_green	() -> Self { Self::from(Rgb24::from(0x89a203	)) }
    #[inline(always)] fn purple_pink	() -> Self { Self::from(Rgb24::from(0xe03fd8	)) }
    #[inline(always)] fn dusty_pink	() -> Self { Self::from(Rgb24::from(0xd58a94	)) }
    #[inline(always)] fn faded_green	() -> Self { Self::from(Rgb24::from(0x7bb274	)) }
    #[inline(always)] fn camo_green	() -> Self { Self::from(Rgb24::from(0x526525	)) }
    #[inline(always)] fn pinky_purple	() -> Self { Self::from(Rgb24::from(0xc94cbe	)) }
    #[inline(always)] fn pink_purple	() -> Self { Self::from(Rgb24::from(0xdb4bda	)) }
    #[inline(always)] fn brownish_red	() -> Self { Self::from(Rgb24::from(0x9e3623	)) }
    #[inline(always)] fn dark_rose	() -> Self { Self::from(Rgb24::from(0xb5485d	)) }
    #[inline(always)] fn mud	() -> Self { Self::from(Rgb24::from(0x735c12	)) }
    #[inline(always)] fn brownish	() -> Self { Self::from(Rgb24::from(0x9c6d57	)) }
    #[inline(always)] fn emerald_green	() -> Self { Self::from(Rgb24::from(0x028f1e	)) }
    #[inline(always)] fn pale_brown	() -> Self { Self::from(Rgb24::from(0xb1916e	)) }
    #[inline(always)] fn dull_blue	() -> Self { Self::from(Rgb24::from(0x49759c	)) }
    #[inline(always)] fn burnt_umber	() -> Self { Self::from(Rgb24::from(0xa0450e	)) }
    #[inline(always)] fn medium_green	() -> Self { Self::from(Rgb24::from(0x39ad48	)) }
    #[inline(always)] fn clay	() -> Self { Self::from(Rgb24::from(0xb66a50	)) }
    #[inline(always)] fn light_aqua	() -> Self { Self::from(Rgb24::from(0x8cffdb	)) }
    #[inline(always)] fn light_olive_green	() -> Self { Self::from(Rgb24::from(0xa4be5c	)) }
    #[inline(always)] fn brownish_orange	() -> Self { Self::from(Rgb24::from(0xcb7723	)) }
    #[inline(always)] fn dark_aqua	() -> Self { Self::from(Rgb24::from(0x05696b	)) }
    #[inline(always)] fn purplish_pink	() -> Self { Self::from(Rgb24::from(0xce5dae	)) }
    #[inline(always)] fn dark_salmon	() -> Self { Self::from(Rgb24::from(0xc85a53	)) }
    #[inline(always)] fn greenish_grey	() -> Self { Self::from(Rgb24::from(0x96ae8d	)) }
    #[inline(always)] fn jade	() -> Self { Self::from(Rgb24::from(0x1fa774	)) }
    #[inline(always)] fn ugly_green	() -> Self { Self::from(Rgb24::from(0x7a9703	)) }
    #[inline(always)] fn dark_beige	() -> Self { Self::from(Rgb24::from(0xac9362	)) }
    #[inline(always)] fn emerald	() -> Self { Self::from(Rgb24::from(0x01a049	)) }
    #[inline(always)] fn pale_red	() -> Self { Self::from(Rgb24::from(0xd9544d	)) }
    #[inline(always)] fn light_magenta	() -> Self { Self::from(Rgb24::from(0xfa5ff7	)) }
    #[inline(always)] fn sky	() -> Self { Self::from(Rgb24::from(0x82cafc	)) }
    #[inline(always)] fn light_cyan	() -> Self { Self::from(Rgb24::from(0xacfffc	)) }
    #[inline(always)] fn yellow_orange	() -> Self { Self::from(Rgb24::from(0xfcb001	)) }
    #[inline(always)] fn reddish_purple	() -> Self { Self::from(Rgb24::from(0x910951	)) }
    #[inline(always)] fn reddish_pink	() -> Self { Self::from(Rgb24::from(0xfe2c54	)) }
    #[inline(always)] fn orchid	() -> Self { Self::from(Rgb24::from(0xc875c4	)) }
    #[inline(always)] fn dirty_yellow	() -> Self { Self::from(Rgb24::from(0xcdc50a	)) }
    #[inline(always)] fn orange_red	() -> Self { Self::from(Rgb24::from(0xfd411e	)) }
    #[inline(always)] fn deep_red	() -> Self { Self::from(Rgb24::from(0x9a0200	)) }
    #[inline(always)] fn orange_brown	() -> Self { Self::from(Rgb24::from(0xbe6400	)) }
    #[inline(always)] fn cobalt_blue	() -> Self { Self::from(Rgb24::from(0x030aa7	)) }
    #[inline(always)] fn neon_pink	() -> Self { Self::from(Rgb24::from(0xfe019a	)) }
    #[inline(always)] fn rose_pink	() -> Self { Self::from(Rgb24::from(0xf7879a	)) }
    #[inline(always)] fn greyish_purple	() -> Self { Self::from(Rgb24::from(0x887191	)) }
    #[inline(always)] fn raspberry	() -> Self { Self::from(Rgb24::from(0xb00149	)) }
    #[inline(always)] fn aqua_green	() -> Self { Self::from(Rgb24::from(0x12e193	)) }
    #[inline(always)] fn salmon_pink	() -> Self { Self::from(Rgb24::from(0xfe7b7c	)) }
    #[inline(always)] fn tangerine	() -> Self { Self::from(Rgb24::from(0xff9408	)) }
    #[inline(always)] fn brownish_green	() -> Self { Self::from(Rgb24::from(0x6a6e09	)) }
    #[inline(always)] fn red_brown	() -> Self { Self::from(Rgb24::from(0x8b2e16	)) }
    #[inline(always)] fn greenish_brown	() -> Self { Self::from(Rgb24::from(0x696112	)) }
    #[inline(always)] fn pumpkin	() -> Self { Self::from(Rgb24::from(0xe17701	)) }
    #[inline(always)] fn pine_green	() -> Self { Self::from(Rgb24::from(0x0a481e	)) }
    #[inline(always)] fn charcoal	() -> Self { Self::from(Rgb24::from(0x343837	)) }
    #[inline(always)] fn baby_pink	() -> Self { Self::from(Rgb24::from(0xffb7ce	)) }
    #[inline(always)] fn cornflower	() -> Self { Self::from(Rgb24::from(0x6a79f7	)) }
    #[inline(always)] fn blue_violet	() -> Self { Self::from(Rgb24::from(0x5d06e9	)) }
    #[inline(always)] fn chocolate	() -> Self { Self::from(Rgb24::from(0x3d1c02	)) }
    #[inline(always)] fn greyish_green	() -> Self { Self::from(Rgb24::from(0x82a67d	)) }
    #[inline(always)] fn scarlet	() -> Self { Self::from(Rgb24::from(0xbe0119	)) }
    #[inline(always)] fn green_yellow	() -> Self { Self::from(Rgb24::from(0xc9ff27	)) }
    #[inline(always)] fn dark_olive	() -> Self { Self::from(Rgb24::from(0x373e02	)) }
    #[inline(always)] fn sienna	() -> Self { Self::from(Rgb24::from(0xa9561e	)) }
    #[inline(always)] fn pastel_purple	() -> Self { Self::from(Rgb24::from(0xcaa0ff	)) }
    #[inline(always)] fn terracotta	() -> Self { Self::from(Rgb24::from(0xca6641	)) }
    #[inline(always)] fn aqua_blue	() -> Self { Self::from(Rgb24::from(0x02d8e9	)) }
    #[inline(always)] fn sage_green	() -> Self { Self::from(Rgb24::from(0x88b378	)) }
    #[inline(always)] fn blood_red	() -> Self { Self::from(Rgb24::from(0x980002	)) }
    #[inline(always)] fn deep_pink	() -> Self { Self::from(Rgb24::from(0xcb0162	)) }
    #[inline(always)] fn grass	() -> Self { Self::from(Rgb24::from(0x5cac2d	)) }
    #[inline(always)] fn moss	() -> Self { Self::from(Rgb24::from(0x769958	)) }
    #[inline(always)] fn pastel_blue	() -> Self { Self::from(Rgb24::from(0xa2bffe	)) }
    #[inline(always)] fn bluish_green	() -> Self { Self::from(Rgb24::from(0x10a674	)) }
    #[inline(always)] fn green_blue	() -> Self { Self::from(Rgb24::from(0x06b48b	)) }
    #[inline(always)] fn dark_tan	() -> Self { Self::from(Rgb24::from(0xaf884a	)) }
    #[inline(always)] fn greenish_blue	() -> Self { Self::from(Rgb24::from(0x0b8b87	)) }
    #[inline(always)] fn pale_orange	() -> Self { Self::from(Rgb24::from(0xffa756	)) }
    #[inline(always)] fn vomit	() -> Self { Self::from(Rgb24::from(0xa2a415	)) }
    #[inline(always)] fn forrest_green	() -> Self { Self::from(Rgb24::from(0x154406	)) }
    #[inline(always)] fn dark_lavender	() -> Self { Self::from(Rgb24::from(0x856798	)) }
    #[inline(always)] fn dark_violet	() -> Self { Self::from(Rgb24::from(0x34013f	)) }
    #[inline(always)] fn purple_blue	() -> Self { Self::from(Rgb24::from(0x632de9	)) }
    #[inline(always)] fn dark_cyan	() -> Self { Self::from(Rgb24::from(0x0a888a	)) }
    #[inline(always)] fn olive_drab	() -> Self { Self::from(Rgb24::from(0x6f7632	)) }
    #[inline(always)] fn pinkish	() -> Self { Self::from(Rgb24::from(0xd46a7e	)) }
    #[inline(always)] fn cobalt	() -> Self { Self::from(Rgb24::from(0x1e488f	)) }
    #[inline(always)] fn neon_purple	() -> Self { Self::from(Rgb24::from(0xbc13fe	)) }
    #[inline(always)] fn light_turquoise	() -> Self { Self::from(Rgb24::from(0x7ef4cc	)) }
    #[inline(always)] fn apple_green	() -> Self { Self::from(Rgb24::from(0x76cd26	)) }
    #[inline(always)] fn dull_green	() -> Self { Self::from(Rgb24::from(0x74a662	)) }
    #[inline(always)] fn wine	() -> Self { Self::from(Rgb24::from(0x80013f	)) }
    #[inline(always)] fn powder_blue	() -> Self { Self::from(Rgb24::from(0xb1d1fc	)) }
    #[inline(always)] fn off_white	() -> Self { Self::from(Rgb24::from(0xffffe4	)) }
    #[inline(always)] fn electric_blue	() -> Self { Self::from(Rgb24::from(0x0652ff	)) }
    #[inline(always)] fn dark_turquoise	() -> Self { Self::from(Rgb24::from(0x045c5a	)) }
    #[inline(always)] fn blue_purple	() -> Self { Self::from(Rgb24::from(0x5729ce	)) }
    #[inline(always)] fn azure	() -> Self { Self::from(Rgb24::from(0x069af3	)) }
    #[inline(always)] fn bright_red	() -> Self { Self::from(Rgb24::from(0xff000d)) }
    #[inline(always)] fn pinkish_red	() -> Self { Self::from(Rgb24::from(0xf10c45	)) }
    #[inline(always)] fn cornflower_blue	() -> Self { Self::from(Rgb24::from(0x5170d7	)) }
    #[inline(always)] fn light_olive	() -> Self { Self::from(Rgb24::from(0xacbf69	)) }
    #[inline(always)] fn grape	() -> Self { Self::from(Rgb24::from(0x6c3461	)) }
    #[inline(always)] fn greyish_blue	() -> Self { Self::from(Rgb24::from(0x5e819d	)) }
    #[inline(always)] fn purplish_blue	() -> Self { Self::from(Rgb24::from(0x601ef9	)) }
    #[inline(always)] fn yellowish_green	() -> Self { Self::from(Rgb24::from(0xb0dd16	)) }
    #[inline(always)] fn greenish_yellow	() -> Self { Self::from(Rgb24::from(0xcdfd02	)) }
    #[inline(always)] fn medium_blue	() -> Self { Self::from(Rgb24::from(0x2c6fbb	)) }
    #[inline(always)] fn dusty_rose	() -> Self { Self::from(Rgb24::from(0xc0737a	)) }
    #[inline(always)] fn light_violet	() -> Self { Self::from(Rgb24::from(0xd6b4fc	)) }
    #[inline(always)] fn midnight_blue	() -> Self { Self::from(Rgb24::from(0x020035	)) }
    #[inline(always)] fn bluish_purple	() -> Self { Self::from(Rgb24::from(0x703be7	)) }
    #[inline(always)] fn red_orange	() -> Self { Self::from(Rgb24::from(0xfd3c06	)) }
    #[inline(always)] fn dark_magenta	() -> Self { Self::from(Rgb24::from(0x960056	)) }
    #[inline(always)] fn greenish	() -> Self { Self::from(Rgb24::from(0x40a368	)) }
    #[inline(always)] fn ocean_blue	() -> Self { Self::from(Rgb24::from(0x03719c	)) }
    #[inline(always)] fn coral	() -> Self { Self::from(Rgb24::from(0xfc5a50	)) }
    #[inline(always)] fn cream	() -> Self { Self::from(Rgb24::from(0xffffc2	)) }
    #[inline(always)] fn reddish_brown	() -> Self { Self::from(Rgb24::from(0x7f2b0a	)) }
    #[inline(always)] fn burnt_sienna	() -> Self { Self::from(Rgb24::from(0xb04e0f	)) }
    #[inline(always)] fn brick	() -> Self { Self::from(Rgb24::from(0xa03623	)) }
    #[inline(always)] fn sage	() -> Self { Self::from(Rgb24::from(0x87ae73	)) }
    #[inline(always)] fn grey_green	() -> Self { Self::from(Rgb24::from(0x789b73	)) }
    #[inline(always)] fn white	() -> Self { Self::from(Rgb24::from(0xffffff	)) }
    #[inline(always)] fn robins_egg_blue	() -> Self { Self::from(Rgb24::from(0x98eff9	)) }
    #[inline(always)] fn moss_green	() -> Self { Self::from(Rgb24::from(0x658b38	)) }
    #[inline(always)] fn steel_blue	() -> Self { Self::from(Rgb24::from(0x5a7d9a	)) }
    #[inline(always)] fn eggplant	() -> Self { Self::from(Rgb24::from(0x380835	)) }
    #[inline(always)] fn light_yellow	() -> Self { Self::from(Rgb24::from(0xfffe7a	)) }
    #[inline(always)] fn leaf_green	() -> Self { Self::from(Rgb24::from(0x5ca904	)) }
    #[inline(always)] fn light_grey	() -> Self { Self::from(Rgb24::from(0xd8dcd6	)) }
    #[inline(always)] fn puke	() -> Self { Self::from(Rgb24::from(0xa5a502	)) }
    #[inline(always)] fn pinkish_purple	() -> Self { Self::from(Rgb24::from(0xd648d7	)) }
    #[inline(always)] fn sea_blue	() -> Self { Self::from(Rgb24::from(0x047495	)) }
    #[inline(always)] fn pale_purple	() -> Self { Self::from(Rgb24::from(0xb790d4	)) }
    #[inline(always)] fn slate_blue	() -> Self { Self::from(Rgb24::from(0x5b7c99	)) }
    #[inline(always)] fn blue_grey	() -> Self { Self::from(Rgb24::from(0x607c8e	)) }
    #[inline(always)] fn hunter_green	() -> Self { Self::from(Rgb24::from(0x0b4008	)) }
    #[inline(always)] fn fuchsia	() -> Self { Self::from(Rgb24::from(0xed0dd9	)) }
    #[inline(always)] fn crimson	() -> Self { Self::from(Rgb24::from(0x8c000f	)) }
    #[inline(always)] fn pale_yellow	() -> Self { Self::from(Rgb24::from(0xffff84	)) }
    #[inline(always)] fn ochre	() -> Self { Self::from(Rgb24::from(0xbf9005	)) }
    #[inline(always)] fn mustard_yellow	() -> Self { Self::from(Rgb24::from(0xd2bd0a	)) }
    #[inline(always)] fn light_red	() -> Self { Self::from(Rgb24::from(0xff474c	)) }
    #[inline(always)] fn cerulean	() -> Self { Self::from(Rgb24::from(0x0485d1	)) }
    #[inline(always)] fn pale_pink	() -> Self { Self::from(Rgb24::from(0xffcfdc	)) }
    #[inline(always)] fn deep_blue	() -> Self { Self::from(Rgb24::from(0x040273	)) }
    #[inline(always)] fn rust	() -> Self { Self::from(Rgb24::from(0xa83c09	)) }
    #[inline(always)] fn light_teal	() -> Self { Self::from(Rgb24::from(0x90e4c1	)) }
    #[inline(always)] fn slate	() -> Self { Self::from(Rgb24::from(0x516572	)) }
    #[inline(always)] fn goldenrod	() -> Self { Self::from(Rgb24::from(0xfac205	)) }
    #[inline(always)] fn dark_yellow	() -> Self { Self::from(Rgb24::from(0xd5b60a	)) }
    #[inline(always)] fn dark_grey	() -> Self { Self::from(Rgb24::from(0x363737	)) }
    #[inline(always)] fn army_green	() -> Self { Self::from(Rgb24::from(0x4b5d16	)) }
    #[inline(always)] fn grey_blue	() -> Self { Self::from(Rgb24::from(0x6b8ba4	)) }
    #[inline(always)] fn seafoam	() -> Self { Self::from(Rgb24::from(0x80f9ad	)) }
    #[inline(always)] fn puce	() -> Self { Self::from(Rgb24::from(0xa57e52	)) }
    #[inline(always)] fn spring_green	() -> Self { Self::from(Rgb24::from(0xa9f971	)) }
    #[inline(always)] fn dark_orange	() -> Self { Self::from(Rgb24::from(0xc65102	)) }
    #[inline(always)] fn sand	() -> Self { Self::from(Rgb24::from(0xe2ca76	)) }
    #[inline(always)] fn pastel_green	() -> Self { Self::from(Rgb24::from(0xb0ff9d	)) }
    #[inline(always)] fn mint	() -> Self { Self::from(Rgb24::from(0x9ffeb0	)) }
    #[inline(always)] fn light_orange	() -> Self { Self::from(Rgb24::from(0xfdaa48	)) }
    #[inline(always)] fn bright_pink	() -> Self { Self::from(Rgb24::from(0xfe01b1	)) }
    #[inline(always)] fn chartreuse	() -> Self { Self::from(Rgb24::from(0xc1f80a	)) }
    #[inline(always)] fn deep_purple	() -> Self { Self::from(Rgb24::from(0x36013f	)) }
    #[inline(always)] fn dark_brown	() -> Self { Self::from(Rgb24::from(0x341c02	)) }
    #[inline(always)] fn taupe	() -> Self { Self::from(Rgb24::from(0xb9a281	)) }
    #[inline(always)] fn pea_green	() -> Self { Self::from(Rgb24::from(0x8eab12	)) }
    #[inline(always)] fn puke_green	() -> Self { Self::from(Rgb24::from(0x9aae07	)) }
    #[inline(always)] fn kelly_green	() -> Self { Self::from(Rgb24::from(0x02ab2e	)) }
    #[inline(always)] fn seafoam_green	() -> Self { Self::from(Rgb24::from(0x7af9ab	)) }
    #[inline(always)] fn blue_green	() -> Self { Self::from(Rgb24::from(0x137e6d	)) }
    #[inline(always)] fn khaki	() -> Self { Self::from(Rgb24::from(0xaaa662	)) }
    #[inline(always)] fn burgundy	() -> Self { Self::from(Rgb24::from(0x610023	)) }
    #[inline(always)] fn dark_teal	() -> Self { Self::from(Rgb24::from(0x014d4e	)) }
    #[inline(always)] fn brick_red	() -> Self { Self::from(Rgb24::from(0x8f1402	)) }
    #[inline(always)] fn royal_purple	() -> Self { Self::from(Rgb24::from(0x4b006e	)) }
    #[inline(always)] fn plum	() -> Self { Self::from(Rgb24::from(0x580f41	)) }
    #[inline(always)] fn mint_green	() -> Self { Self::from(Rgb24::from(0x8fff9f	)) }
    #[inline(always)] fn gold	() -> Self { Self::from(Rgb24::from(0xdbb40c	)) }
    #[inline(always)] fn baby_blue	() -> Self { Self::from(Rgb24::from(0xa2cffe	)) }
    #[inline(always)] fn yellow_green	() -> Self { Self::from(Rgb24::from(0xc0fb2d	)) }
    #[inline(always)] fn bright_purple	() -> Self { Self::from(Rgb24::from(0xbe03fd	)) }
    #[inline(always)] fn dark_red	() -> Self { Self::from(Rgb24::from(0x840000	)) }
    #[inline(always)] fn pale_blue	() -> Self { Self::from(Rgb24::from(0xd0fefe	)) }
    #[inline(always)] fn grass_green	() -> Self { Self::from(Rgb24::from(0x3f9b0b	)) }
    #[inline(always)] fn navy	() -> Self { Self::from(Rgb24::from(0x01153e	)) }
    #[inline(always)] fn aquamarine	() -> Self { Self::from(Rgb24::from(0x04d8b2	)) }
    #[inline(always)] fn burnt_orange	() -> Self { Self::from(Rgb24::from(0xc04e01	)) }
    #[inline(always)] fn neon_green	() -> Self { Self::from(Rgb24::from(0x0cff0c	)) }
    #[inline(always)] fn bright_blue	() -> Self { Self::from(Rgb24::from(0x0165fc	)) }
    #[inline(always)] fn rose	() -> Self { Self::from(Rgb24::from(0xcf6275	)) }
    #[inline(always)] fn light_pink	() -> Self { Self::from(Rgb24::from(0xffd1df	)) }
    #[inline(always)] fn mustard	() -> Self { Self::from(Rgb24::from(0xceb301	)) }
    #[inline(always)] fn indigo	() -> Self { Self::from(Rgb24::from(0x380282	)) }
    #[inline(always)] fn lime	() -> Self { Self::from(Rgb24::from(0xaaff32	)) }
    #[inline(always)] fn sea_green	() -> Self { Self::from(Rgb24::from(0x53fca1	)) }
    #[inline(always)] fn periwinkle	() -> Self { Self::from(Rgb24::from(0x8e82fe	)) }
    #[inline(always)] fn dark_pink	() -> Self { Self::from(Rgb24::from(0xcb416b	)) }
    #[inline(always)] fn olive_green	() -> Self { Self::from(Rgb24::from(0x677a04	)) }
    #[inline(always)] fn peach	() -> Self { Self::from(Rgb24::from(0xffb07c	)) }
    #[inline(always)] fn pale_green	() -> Self { Self::from(Rgb24::from(0xc7fdb5	)) }
    #[inline(always)] fn light_brown	() -> Self { Self::from(Rgb24::from(0xad8150	)) }
    #[inline(always)] fn hot_pink	() -> Self { Self::from(Rgb24::from(0xff028d	)) }
    #[inline(always)] fn black	() -> Self { Self::from(Rgb24::from(0x000000	)) }
    #[inline(always)] fn lilac	() -> Self { Self::from(Rgb24::from(0xcea2fd	)) }
    #[inline(always)] fn navy_blue	() -> Self { Self::from(Rgb24::from(0x001146	)) }
    #[inline(always)] fn royal_blue	() -> Self { Self::from(Rgb24::from(0x0504aa	)) }
    #[inline(always)] fn beige	() -> Self { Self::from(Rgb24::from(0xe6daa6	)) }
    #[inline(always)] fn salmon	() -> Self { Self::from(Rgb24::from(0xff796c	)) }
    #[inline(always)] fn olive	() -> Self { Self::from(Rgb24::from(0x6e750e	)) }
    #[inline(always)] fn maroon	() -> Self { Self::from(Rgb24::from(0x650021	)) }
    #[inline(always)] fn bright_green	() -> Self { Self::from(Rgb24::from(0x01ff07	)) }
    #[inline(always)] fn dark_purple	() -> Self { Self::from(Rgb24::from(0x35063e	)) }
    #[inline(always)] fn mauve	() -> Self { Self::from(Rgb24::from(0xae7181	)) }
    #[inline(always)] fn forest_green	() -> Self { Self::from(Rgb24::from(0x06470c	)) }
    #[inline(always)] fn aqua	() -> Self { Self::from(Rgb24::from(0x13eac9	)) }
    #[inline(always)] fn cyan	() -> Self { Self::from(Rgb24::from(0x00ffff	)) }
    #[inline(always)] fn tan	() -> Self { Self::from(Rgb24::from(0xd1b26f	)) }
    #[inline(always)] fn dark_blue	() -> Self { Self::from(Rgb24::from(0x00035b	)) }
    #[inline(always)] fn lavender	() -> Self { Self::from(Rgb24::from(0xc79fef	)) }
    #[inline(always)] fn turquoise	() -> Self { Self::from(Rgb24::from(0x06c2ac	)) }
    #[inline(always)] fn dark_green	() -> Self { Self::from(Rgb24::from(0x033500	)) }
    #[inline(always)] fn violet	() -> Self { Self::from(Rgb24::from(0x9a0eea	)) }
    #[inline(always)] fn light_purple	() -> Self { Self::from(Rgb24::from(0xbf77f6	)) }
    #[inline(always)] fn lime_green	() -> Self { Self::from(Rgb24::from(0x89fe05	)) }
    #[inline(always)] fn grey	() -> Self { Self::from(Rgb24::from(0x929591	)) }
    #[inline(always)] fn sky_blue	() -> Self { Self::from(Rgb24::from(0x75bbfd	)) }
    #[inline(always)] fn yellow	() -> Self { Self::from(Rgb24::from(0xffff14	)) }
    #[inline(always)] fn magenta	() -> Self { Self::from(Rgb24::from(0xc20078	)) }
    #[inline(always)] fn light_green	() -> Self { Self::from(Rgb24::from(0x96f97b	)) }
    #[inline(always)] fn orange	() -> Self { Self::from(Rgb24::from(0xf97306	)) }
    #[inline(always)] fn teal	() -> Self { Self::from(Rgb24::from(0x029386	)) }
    #[inline(always)] fn light_blue	() -> Self { Self::from(Rgb24::from(0x95d0fc	)) }
    #[inline(always)] fn red	() -> Self { Self::from(Rgb24::from(0xe50000	)) }
    #[inline(always)] fn brown	() -> Self { Self::from(Rgb24::from(0x653700	)) }
    #[inline(always)] fn pink	() -> Self { Self::from(Rgb24::from(0xff81c0	)) }
    #[inline(always)] fn blue	() -> Self { Self::from(Rgb24::from(0x0343df	)) }
    #[inline(always)] fn green	() -> Self { Self::from(Rgb24::from(0x15b01a	)) }
    #[inline(always)] fn purple	() -> Self { Self::from(Rgb24::from(0x7e1e9c	)) }
}
