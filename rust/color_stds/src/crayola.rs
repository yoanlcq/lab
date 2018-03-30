//! https://www.w3schools.com/colors/colors_crayola.asp
use Rgb24;

#[cfg(any(feature="tables", feature="crayola_table"))]
use Entry;

#[cfg(any(feature="tables", feature="crayola_table"))]
pub const CRAYOLA_COLORS : &[Entry] = &[
    Entry { ident: "red", value: hex24!(0xed0a3f) },
    Entry { ident: "maroon", value: hex24!(0xc32148) },
    Entry { ident: "scarlet", value: hex24!(0xfd0e35) },
    Entry { ident: "brick_red", value: hex24!(0xc62d42) },
    Entry { ident: "english_vermilion", value: hex24!(0xcc474b) },
    Entry { ident: "madder_lake", value: hex24!(0xcc3336) },
    Entry { ident: "permanent_geranium_lake", value: hex24!(0xe12c2c) },
    Entry { ident: "maximum_red", value: hex24!(0xd92121) },
    Entry { ident: "indian_red", value: hex24!(0xb94e48) },
    Entry { ident: "orange_red", value: hex24!(0xff5349) },
    Entry { ident: "sunset_orange", value: hex24!(0xfe4c40) },
    Entry { ident: "bittersweet", value: hex24!(0xfe6f5e) },
    Entry { ident: "dark_venetian_red", value: hex24!(0xb33b24) },
    Entry { ident: "venetian_red", value: hex24!(0xcc553d) },
    Entry { ident: "light_venetian_red", value: hex24!(0xe6735c) },
    Entry { ident: "vivid_tangerine", value: hex24!(0xff9980) },
    Entry { ident: "middle_red", value: hex24!(0xe58e73) },
    Entry { ident: "burnt_orange", value: hex24!(0xff7f49) },
    Entry { ident: "red_orange", value: hex24!(0xff681f) },
    Entry { ident: "orange", value: hex24!(0xff8833) },
    Entry { ident: "macaroni_and_cheese", value: hex24!(0xffb97b) },
    Entry { ident: "middle_yellow_red", value: hex24!(0xecb176) },
    Entry { ident: "mango_tango", value: hex24!(0xe77200) },
    Entry { ident: "yellow_orange", value: hex24!(0xffae42) },
    Entry { ident: "maximum_yellow_red", value: hex24!(0xf2ba49) },
    Entry { ident: "banana_mania", value: hex24!(0xfbe7b2) },
    Entry { ident: "maize", value: hex24!(0xf2c649) },
    Entry { ident: "orange_yellow", value: hex24!(0xf8d568) },
    Entry { ident: "goldenrod", value: hex24!(0xfcd667) },
    Entry { ident: "dandelion", value: hex24!(0xfed85d) },
    Entry { ident: "yellow", value: hex24!(0xfbe870) },
    Entry { ident: "green_yellow", value: hex24!(0xf1e788) },
    Entry { ident: "middle_yellow", value: hex24!(0xffeb00) },
    Entry { ident: "olive_green", value: hex24!(0xb5b35c) },
    Entry { ident: "spring_green", value: hex24!(0xecebbd) },
    Entry { ident: "maximum_yellow", value: hex24!(0xfafa37) },
    Entry { ident: "canary", value: hex24!(0xffff99) },
    Entry { ident: "lemon_yellow", value: hex24!(0xffff9f) },
    Entry { ident: "maximum_green_yellow", value: hex24!(0xd9e650) },
    Entry { ident: "middle_green_yellow", value: hex24!(0xacbf60) },
    Entry { ident: "inchworm", value: hex24!(0xafe313) },
    Entry { ident: "light_chrome_green", value: hex24!(0xbee64b) },
    Entry { ident: "yellow_green", value: hex24!(0xc5e17a) },
    Entry { ident: "maximum_green", value: hex24!(0x5e8c31) },
    Entry { ident: "asparagus", value: hex24!(0x7ba05b) },
    Entry { ident: "granny_smith_apple", value: hex24!(0x9de093) },
    Entry { ident: "fern", value: hex24!(0x63b76c) },
    Entry { ident: "middle_green", value: hex24!(0x4d8c57) },
    Entry { ident: "green", value: hex24!(0x3aa655) },
    Entry { ident: "medium_chrome_green", value: hex24!(0x6ca67c) },
    Entry { ident: "forest_green", value: hex24!(0x5fa777) },
    Entry { ident: "sea_green", value: hex24!(0x93dfb8) },
    Entry { ident: "shamrock", value: hex24!(0x33cc99) },
    Entry { ident: "mountain_meadow", value: hex24!(0x1ab385) },
    Entry { ident: "jungle_green", value: hex24!(0x29ab87) },
    Entry { ident: "caribbean_green", value: hex24!(0x00cc99) },
    Entry { ident: "tropical_rain_forest", value: hex24!(0x00755e) },
    Entry { ident: "middle_blue_green", value: hex24!(0x8dd9cc) },
    Entry { ident: "pine_green", value: hex24!(0x01786f) },
    Entry { ident: "maximum_blue_green", value: hex24!(0x30bfbf) },
    Entry { ident: "robins_egg_blue", value: hex24!(0x00cccc) },
    Entry { ident: "teal_blue", value: hex24!(0x008080) },
    Entry { ident: "light_blue", value: hex24!(0x8fd8d8) },
    Entry { ident: "aquamarine", value: hex24!(0x95e0e8) },
    Entry { ident: "turquoise_blue", value: hex24!(0x6cdae7) },
    Entry { ident: "outer_space", value: hex24!(0x2d383a) },
    Entry { ident: "sky_blue", value: hex24!(0x76d7ea) },
    Entry { ident: "middle_blue", value: hex24!(0x7ed4e6) },
    Entry { ident: "blue_green", value: hex24!(0x0095b7) },
    Entry { ident: "pacific_blue", value: hex24!(0x009dc4) },
    Entry { ident: "cerulean", value: hex24!(0x02a4d3) },
    Entry { ident: "maximum_blue", value: hex24!(0x47abcc) },
    Entry { ident: "blue1", value: hex24!(0x4997d0) },
    Entry { ident: "cerulean_blue", value: hex24!(0x339acc) },
    Entry { ident: "cornflower", value: hex24!(0x93ccea) },
    Entry { ident: "green_blue", value: hex24!(0x2887c8) },
    Entry { ident: "midnight_blue", value: hex24!(0x00468c) },
    Entry { ident: "navy_blue", value: hex24!(0x0066cc) },
    Entry { ident: "denim", value: hex24!(0x1560bd) },
    Entry { ident: "blue3", value: hex24!(0x0066ff) },
    Entry { ident: "cadet_blue", value: hex24!(0xa9b2c3) },
    Entry { ident: "periwinkle", value: hex24!(0xc3cde6) },
    Entry { ident: "blue2", value: hex24!(0x4570e6) },
    Entry { ident: "wild_blue_yonder", value: hex24!(0x7a89b8) },
    Entry { ident: "indigo", value: hex24!(0x4f69c6) },
    Entry { ident: "manatee", value: hex24!(0x8d90a1) },
    Entry { ident: "cobalt_blue", value: hex24!(0x8c90c8) },
    Entry { ident: "celestial_blue", value: hex24!(0x7070cc) },
    Entry { ident: "blue_bell", value: hex24!(0x9999cc) },
    Entry { ident: "maximum_blue_purple", value: hex24!(0xacace6) },
    Entry { ident: "violet_blue", value: hex24!(0x766ec8) },
    Entry { ident: "blue_violet", value: hex24!(0x6456b7) },
    Entry { ident: "ultramarine_blue", value: hex24!(0x3f26bf) },
    Entry { ident: "middle_blue_purple", value: hex24!(0x8b72be) },
    Entry { ident: "purple_heart", value: hex24!(0x652dc1) },
    Entry { ident: "royal_purple", value: hex24!(0x6b3fa0) },
    Entry { ident: "violet2", value: hex24!(0x8359a3) },
    Entry { ident: "medium_violet", value: hex24!(0x8f47b3) },
    Entry { ident: "wisteria", value: hex24!(0xc9a0dc) },
    Entry { ident: "lavender1", value: hex24!(0xbf8fcc) },
    Entry { ident: "vivid_violet", value: hex24!(0x803790) },
    Entry { ident: "maximum_purple", value: hex24!(0x733380) },
    Entry { ident: "purple_mountains_majesty", value: hex24!(0xd6aedd) },
    Entry { ident: "fuchsia", value: hex24!(0xc154c1) },
    Entry { ident: "pink_flamingo", value: hex24!(0xfc74fd) },
    Entry { ident: "violet1", value: hex24!(0x732e6c) },
    Entry { ident: "brilliant_rose", value: hex24!(0xe667ce) },
    Entry { ident: "orchid", value: hex24!(0xe29cd2) },
    Entry { ident: "plum", value: hex24!(0x8e3179) },
    Entry { ident: "medium_rose", value: hex24!(0xd96cbe) },
    Entry { ident: "thistle", value: hex24!(0xebb0d7) },
    Entry { ident: "mulberry", value: hex24!(0xc8509b) },
    Entry { ident: "red_violet", value: hex24!(0xbb3385) },
    Entry { ident: "middle_purple", value: hex24!(0xd982b5) },
    Entry { ident: "maximum_red_purple", value: hex24!(0xa63a79) },
    Entry { ident: "jazzberry_jam", value: hex24!(0xa50b5e) },
    Entry { ident: "eggplant", value: hex24!(0x614051) },
    Entry { ident: "magenta", value: hex24!(0xf653a6) },
    Entry { ident: "cerise", value: hex24!(0xda3287) },
    Entry { ident: "wild_strawberry", value: hex24!(0xff3399) },
    Entry { ident: "lavender2", value: hex24!(0xfbaed2) },
    Entry { ident: "cotton_candy", value: hex24!(0xffb7d5) },
    Entry { ident: "carnation_pink", value: hex24!(0xffa6c9) },
    Entry { ident: "violet_red", value: hex24!(0xf7468a) },
    Entry { ident: "razzmatazz", value: hex24!(0xe30b5c) },
    Entry { ident: "pig_pink", value: hex24!(0xfdd7e4) },
    Entry { ident: "carmine", value: hex24!(0xe62e6b) },
    Entry { ident: "blush", value: hex24!(0xdb5079) },
    Entry { ident: "tickle_me_pink", value: hex24!(0xfc80a5) },
    Entry { ident: "mauvelous", value: hex24!(0xf091a9) },
    Entry { ident: "salmon", value: hex24!(0xff91a4) },
    Entry { ident: "middle_red_purple", value: hex24!(0xa55353) },
    Entry { ident: "mahogany", value: hex24!(0xca3435) },
    Entry { ident: "melon", value: hex24!(0xfebaad) },
    Entry { ident: "pink_sherbert", value: hex24!(0xf7a38e) },
    Entry { ident: "burnt_sienna", value: hex24!(0xe97451) },
    Entry { ident: "brown", value: hex24!(0xaf593e) },
    Entry { ident: "sepia", value: hex24!(0x9e5b40) },
    Entry { ident: "fuzzy_wuzzy", value: hex24!(0x87421f) },
    Entry { ident: "beaver", value: hex24!(0x926f5b) },
    Entry { ident: "tumbleweed", value: hex24!(0xdea681) },
    Entry { ident: "raw_sienna", value: hex24!(0xd27d46) },
    Entry { ident: "van_dyke_brown", value: hex24!(0x664228) },
    Entry { ident: "tan", value: hex24!(0xd99a6c) },
    Entry { ident: "desert_sand", value: hex24!(0xedc9af) },
    Entry { ident: "peach", value: hex24!(0xffcba4) },
    Entry { ident: "burnt_umber", value: hex24!(0x805533) },
    Entry { ident: "apricot", value: hex24!(0xfdd5b1) },
    Entry { ident: "almond", value: hex24!(0xeed9c4) },
    Entry { ident: "raw_umber", value: hex24!(0x665233) },
    Entry { ident: "shadow", value: hex24!(0x837050) },
    Entry { ident: "raw_sienna1", value: hex24!(0xe6bc5c) },
    Entry { ident: "timberwolf", value: hex24!(0xd9d6cf) },
    Entry { ident: "gold1", value: hex24!(0x92926e) },
    Entry { ident: "gold2", value: hex24!(0xe6be8a) },
    Entry { ident: "silver", value: hex24!(0xc9c0bb) },
    Entry { ident: "copper", value: hex24!(0xda8a67) },
    Entry { ident: "antique_brass", value: hex24!(0xc88a65) },
    Entry { ident: "black", value: hex24!(0x000000) },
    Entry { ident: "charcoal_gray", value: hex24!(0x736a62) },
    Entry { ident: "gray", value: hex24!(0x8b8680) },
    Entry { ident: "blue_gray", value: hex24!(0xc8c8cd) },
    Entry { ident: "radical_red", value: hex24!(0xff355e) },
    Entry { ident: "wild_watermelon", value: hex24!(0xfd5b78) },
    Entry { ident: "outrageous_orange", value: hex24!(0xff6037) },
    Entry { ident: "atomic_tangerine", value: hex24!(0xff9966) },
    Entry { ident: "neon_carrot", value: hex24!(0xff9933) },
    Entry { ident: "sunglow", value: hex24!(0xffcc33) },
    Entry { ident: "laser_lemon", value: hex24!(0xffff66) },
    Entry { ident: "unmellow_yellow", value: hex24!(0xffff66) },
    Entry { ident: "electric_lime", value: hex24!(0xccff00) },
    Entry { ident: "screamin_green", value: hex24!(0x66ff66) },
    Entry { ident: "magic_mint", value: hex24!(0xaaf0d1) },
    Entry { ident: "blizzard_blue", value: hex24!(0x50bfe6) },
    Entry { ident: "shocking_pink", value: hex24!(0xff6eff) },
    Entry { ident: "razzle_dazzle_rose", value: hex24!(0xee34d2) },
    Entry { ident: "hot_magenta", value: hex24!(0xff00cc) },
    Entry { ident: "purple_pizzazz", value: hex24!(0xff00cc) },
    Entry { ident: "sizzling_red", value: hex24!(0xff3855) },
    Entry { ident: "red_salsa", value: hex24!(0xfd3a4a) },
    Entry { ident: "tart_orange", value: hex24!(0xfb4d46) },
    Entry { ident: "orange_soda", value: hex24!(0xfa5b3d) },
    Entry { ident: "bright_yellow", value: hex24!(0xffaa1d) },
    Entry { ident: "yellow_sunshine", value: hex24!(0xfff700) },
    Entry { ident: "slimy_green", value: hex24!(0x299617) },
    Entry { ident: "green_lizard", value: hex24!(0xa7f432) },
    Entry { ident: "denim_blue", value: hex24!(0x2243b6) },
    Entry { ident: "blue_jeans", value: hex24!(0x5dadec) },
    Entry { ident: "plump_purple", value: hex24!(0x5946b2) },
    Entry { ident: "purple_plum", value: hex24!(0x9c51b6) },
    Entry { ident: "sweet_brown", value: hex24!(0xa83731) },
    Entry { ident: "brown_sugar", value: hex24!(0xaf6e4d) },
    Entry { ident: "eerie_black", value: hex24!(0x1b1b1b) },
    Entry { ident: "black_shadows", value: hex24!(0xbfafb2) },
    Entry { ident: "fiery_rose", value: hex24!(0xff5470) },
    Entry { ident: "sizzling_sunrise", value: hex24!(0xffdb00) },
    Entry { ident: "heat_wave", value: hex24!(0xff7a00) },
    Entry { ident: "lemon_glacier", value: hex24!(0xfdff00) },
    Entry { ident: "spring_frost", value: hex24!(0x87ff2a) },
    Entry { ident: "absolute_zero", value: hex24!(0x0048ba) },
    Entry { ident: "winter_sky", value: hex24!(0xff007c) },
    Entry { ident: "frostbite", value: hex24!(0xe936a7) },
    Entry { ident: "alloy_orange", value: hex24!(0xc46210) },
    Entry { ident: "bdazzled_blue", value: hex24!(0x2e5894) },
    Entry { ident: "big_dip_o_ruby", value: hex24!(0x9c2542) },
    Entry { ident: "bittersweet_shimmer", value: hex24!(0xbf4f51) },
    Entry { ident: "blast_off_bronze", value: hex24!(0xa57164) },
    Entry { ident: "cyber_grape", value: hex24!(0x58427c) },
    Entry { ident: "deep_space_sparkle", value: hex24!(0x4a646c) },
    Entry { ident: "gold_fusion", value: hex24!(0x85754e) },
    Entry { ident: "illuminating_emerald", value: hex24!(0x319177) },
    Entry { ident: "metallic_seaweed", value: hex24!(0x0a7e8c) },
    Entry { ident: "metallic_sunburst", value: hex24!(0x9c7c38) },
    Entry { ident: "razzmic_berry", value: hex24!(0x8d4e85) },
    Entry { ident: "sheen_green", value: hex24!(0x8fd400) },
    Entry { ident: "shimmering_blush", value: hex24!(0xd98695) },
    Entry { ident: "sonic_silver", value: hex24!(0x757575) },
    Entry { ident: "steel_blue", value: hex24!(0x0081ab) },
    Entry { ident: "aztec_gold", value: hex24!(0xc39953) },
    Entry { ident: "burnished_brown", value: hex24!(0xa17a74) },
    Entry { ident: "cerulean_frost", value: hex24!(0x6d9bc3) },
    Entry { ident: "cinnamon_satin", value: hex24!(0xcd607e) },
    Entry { ident: "copper_penny", value: hex24!(0xad6f69) },
    Entry { ident: "cosmic_cobalt", value: hex24!(0x2e2d88) },
    Entry { ident: "glossy_grape", value: hex24!(0xab92b3) },
    Entry { ident: "granite_gray", value: hex24!(0x676767) },
    Entry { ident: "green_sheen", value: hex24!(0x6eaea1) },
    Entry { ident: "lilac_luster", value: hex24!(0xae98aa) },
    Entry { ident: "misty_moss", value: hex24!(0xbbb477) },
    Entry { ident: "mystic_maroon", value: hex24!(0xad4379) },
    Entry { ident: "pearly_purple", value: hex24!(0xb768a2) },
    Entry { ident: "pewter_blue", value: hex24!(0x8ba8b7) },
    Entry { ident: "polished_pine", value: hex24!(0x5da493) },
    Entry { ident: "quick_silver", value: hex24!(0xa6a6a6) },
    Entry { ident: "rose_dust", value: hex24!(0x9e5e6f) },
    Entry { ident: "rusty_red", value: hex24!(0xda2c43) },
    Entry { ident: "shadow_blue", value: hex24!(0x778ba5) },
    Entry { ident: "shiny_shamrock", value: hex24!(0x5fa778) },
    Entry { ident: "steel_teal", value: hex24!(0x5f8a8b) },
    Entry { ident: "sugar_plum", value: hex24!(0x914e75) },
    Entry { ident: "twilight_lavender", value: hex24!(0x8a496b) },
    Entry { ident: "wintergreen_dream", value: hex24!(0x56887d) },
    Entry { ident: "baby_powder", value: hex24!(0xfefefa) },
    Entry { ident: "banana", value: hex24!(0xffd12a) },
    Entry { ident: "blueberry", value: hex24!(0x4f86f7) },
    Entry { ident: "bubble_gum", value: hex24!(0xffd3f8) },
    Entry { ident: "cedar_chest", value: hex24!(0xc95a49) },
    Entry { ident: "cherry", value: hex24!(0xda2647) },
    Entry { ident: "chocolate", value: hex24!(0xbd8260) },
    Entry { ident: "coconut", value: hex24!(0xfefefe) },
    Entry { ident: "daffodil", value: hex24!(0xffff31) },
    Entry { ident: "dirt", value: hex24!(0x9b7653) },
    Entry { ident: "eucalyptus", value: hex24!(0x44d7a8) },
    Entry { ident: "fresh_air", value: hex24!(0xa6e7ff) },
    Entry { ident: "grape", value: hex24!(0x6f2da8) },
    Entry { ident: "jelly_bean", value: hex24!(0xda614e) },
    Entry { ident: "leather_jacket", value: hex24!(0x253529) },
    Entry { ident: "lemon", value: hex24!(0xffff38) },
    Entry { ident: "licorice", value: hex24!(0x1a1110) },
    Entry { ident: "lilac", value: hex24!(0xdb91ef) },
    Entry { ident: "lime", value: hex24!(0xb2f302) },
    Entry { ident: "lumber", value: hex24!(0xffe4cd) },
    Entry { ident: "new_car", value: hex24!(0x214fc6) },
    Entry { ident: "orange_fragrance", value: hex24!(0xff8866) },
    Entry { ident: "peach_fragrance", value: hex24!(0xffd0b9) },
    Entry { ident: "pine", value: hex24!(0x45a27d) },
    Entry { ident: "rose", value: hex24!(0xff5050) },
    Entry { ident: "shampoo", value: hex24!(0xffcff1) },
    Entry { ident: "smoke", value: hex24!(0x738276) },
    Entry { ident: "soap", value: hex24!(0xcec8ef) },
    Entry { ident: "strawberry", value: hex24!(0xfc5a8d) },
    Entry { ident: "tulip", value: hex24!(0xff878d) },
    Entry { ident: "amethyst", value: hex24!(0x64609a) },
    Entry { ident: "citrine", value: hex24!(0x933709) },
    Entry { ident: "emerald", value: hex24!(0x14a989) },
    Entry { ident: "jade", value: hex24!(0x469a84) },
    Entry { ident: "jasper", value: hex24!(0xd05340) },
    Entry { ident: "lapis_lazuli", value: hex24!(0x436cb9) },
    Entry { ident: "malachite", value: hex24!(0x469496) },
    Entry { ident: "moonstone", value: hex24!(0x3aa8c1) },
    Entry { ident: "onyx", value: hex24!(0x353839) },
    Entry { ident: "peridot", value: hex24!(0xabad48) },
    Entry { ident: "pink_pearl", value: hex24!(0xb07080) },
    Entry { ident: "rose_quartz", value: hex24!(0xbd559c) },
    Entry { ident: "ruby", value: hex24!(0xaa4069) },
    Entry { ident: "sapphire", value: hex24!(0x2d5da1) },
    Entry { ident: "smokey_topaz", value: hex24!(0x832a0d) },
    Entry { ident: "tigers_eye", value: hex24!(0xb56917) },
    Entry { ident: "baseball_mittburnt_sienna", value: hex24!(0xe97451) },
    Entry { ident: "bubble_bathtickle_me_pink", value: hex24!(0xfc80a5) },
    Entry { ident: "earthwormbrick_red", value: hex24!(0xc62d42) },
    Entry { ident: "flower_shopwisteria", value: hex24!(0xc9a0dc) },
    Entry { ident: "fresh_airsky_blue", value: hex24!(0x76d7ea) },
    Entry { ident: "grandmas_perfumeorange", value: hex24!(0xff8833) },
    Entry { ident: "koala_treejungle_green", value: hex24!(0x29ab87) },
    Entry { ident: "pet_shopbrown", value: hex24!(0xaf593e) },
    Entry { ident: "pine_treepine_green", value: hex24!(0x01786f) },
    Entry { ident: "saw_dustpeach", value: hex24!(0xffcba4) },
    Entry { ident: "sharpening_pencilsgoldenrod", value: hex24!(0xfcd667) },
    Entry { ident: "smell_the_rosesred", value: hex24!(0xed0a3f) },
    Entry { ident: "sunny_dayyellow", value: hex24!(0xfbe870) },
    Entry { ident: "wash_the_dogdandelion", value: hex24!(0xfed85d) },
    Entry { ident: "alien_armpit", value: hex24!(0x84de02) },
    Entry { ident: "big_foot_feet", value: hex24!(0xe88e5a) },
    Entry { ident: "booger_buster", value: hex24!(0xdde26a) },
    Entry { ident: "dingy_dungeon", value: hex24!(0xc53151) },
    Entry { ident: "gargoyle_gas", value: hex24!(0xffdf46) },
    Entry { ident: "giants_club", value: hex24!(0xb05c52) },
    Entry { ident: "magic_potion", value: hex24!(0xff4466) },
    Entry { ident: "mummys_tomb", value: hex24!(0x828e84) },
    Entry { ident: "ogre_odor", value: hex24!(0xfd5240) },
    Entry { ident: "pixie_powder", value: hex24!(0x391285) },
    Entry { ident: "princess_perfume", value: hex24!(0xff85cf) },
    Entry { ident: "sasquatch_socks", value: hex24!(0xff4681) },
    Entry { ident: "sea_serpent", value: hex24!(0x4bc7cf) },
    Entry { ident: "smashed_pumpkin", value: hex24!(0xff6d3a) },
    Entry { ident: "sunburnt_cyclops", value: hex24!(0xff404c) },
    Entry { ident: "winter_wizard", value: hex24!(0xa0e6ff) },
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait CrayolaColors : From<Rgb24> {
    #[inline(always)] fn red () -> Self { Self::from(Rgb24::from(0xed0a3f)) }
    #[inline(always)] fn maroon () -> Self { Self::from(Rgb24::from(0xc32148)) }
    #[inline(always)] fn scarlet () -> Self { Self::from(Rgb24::from(0xfd0e35)) }
    #[inline(always)] fn brick_red () -> Self { Self::from(Rgb24::from(0xc62d42)) }
    #[inline(always)] fn english_vermilion () -> Self { Self::from(Rgb24::from(0xcc474b)) }
    #[inline(always)] fn madder_lake () -> Self { Self::from(Rgb24::from(0xcc3336)) }
    #[inline(always)] fn permanent_geranium_lake () -> Self { Self::from(Rgb24::from(0xe12c2c)) }
    #[inline(always)] fn maximum_red () -> Self { Self::from(Rgb24::from(0xd92121)) }
    #[inline(always)] fn indian_red () -> Self { Self::from(Rgb24::from(0xb94e48)) }
    #[inline(always)] fn orange_red () -> Self { Self::from(Rgb24::from(0xff5349)) }
    #[inline(always)] fn sunset_orange () -> Self { Self::from(Rgb24::from(0xfe4c40)) }
    #[inline(always)] fn bittersweet () -> Self { Self::from(Rgb24::from(0xfe6f5e)) }
    #[inline(always)] fn dark_venetian_red () -> Self { Self::from(Rgb24::from(0xb33b24)) }
    #[inline(always)] fn venetian_red () -> Self { Self::from(Rgb24::from(0xcc553d)) }
    #[inline(always)] fn light_venetian_red () -> Self { Self::from(Rgb24::from(0xe6735c)) }
    #[inline(always)] fn vivid_tangerine () -> Self { Self::from(Rgb24::from(0xff9980)) }
    #[inline(always)] fn middle_red () -> Self { Self::from(Rgb24::from(0xe58e73)) }
    #[inline(always)] fn burnt_orange () -> Self { Self::from(Rgb24::from(0xff7f49)) }
    #[inline(always)] fn red_orange () -> Self { Self::from(Rgb24::from(0xff681f)) }
    #[inline(always)] fn orange () -> Self { Self::from(Rgb24::from(0xff8833)) }
    #[inline(always)] fn macaroni_and_cheese () -> Self { Self::from(Rgb24::from(0xffb97b)) }
    #[inline(always)] fn middle_yellow_red () -> Self { Self::from(Rgb24::from(0xecb176)) }
    #[inline(always)] fn mango_tango () -> Self { Self::from(Rgb24::from(0xe77200)) }
    #[inline(always)] fn yellow_orange () -> Self { Self::from(Rgb24::from(0xffae42)) }
    #[inline(always)] fn maximum_yellow_red () -> Self { Self::from(Rgb24::from(0xf2ba49)) }
    #[inline(always)] fn banana_mania () -> Self { Self::from(Rgb24::from(0xfbe7b2)) }
    #[inline(always)] fn maize () -> Self { Self::from(Rgb24::from(0xf2c649)) }
    #[inline(always)] fn orange_yellow () -> Self { Self::from(Rgb24::from(0xf8d568)) }
    #[inline(always)] fn goldenrod () -> Self { Self::from(Rgb24::from(0xfcd667)) }
    #[inline(always)] fn dandelion () -> Self { Self::from(Rgb24::from(0xfed85d)) }
    #[inline(always)] fn yellow () -> Self { Self::from(Rgb24::from(0xfbe870)) }
    #[inline(always)] fn green_yellow () -> Self { Self::from(Rgb24::from(0xf1e788)) }
    #[inline(always)] fn middle_yellow () -> Self { Self::from(Rgb24::from(0xffeb00)) }
    #[inline(always)] fn olive_green () -> Self { Self::from(Rgb24::from(0xb5b35c)) }
    #[inline(always)] fn spring_green () -> Self { Self::from(Rgb24::from(0xecebbd)) }
    #[inline(always)] fn maximum_yellow () -> Self { Self::from(Rgb24::from(0xfafa37)) }
    #[inline(always)] fn canary () -> Self { Self::from(Rgb24::from(0xffff99)) }
    #[inline(always)] fn lemon_yellow () -> Self { Self::from(Rgb24::from(0xffff9f)) }
    #[inline(always)] fn maximum_green_yellow () -> Self { Self::from(Rgb24::from(0xd9e650)) }
    #[inline(always)] fn middle_green_yellow () -> Self { Self::from(Rgb24::from(0xacbf60)) }
    #[inline(always)] fn inchworm () -> Self { Self::from(Rgb24::from(0xafe313)) }
    #[inline(always)] fn light_chrome_green () -> Self { Self::from(Rgb24::from(0xbee64b)) }
    #[inline(always)] fn yellow_green () -> Self { Self::from(Rgb24::from(0xc5e17a)) }
    #[inline(always)] fn maximum_green () -> Self { Self::from(Rgb24::from(0x5e8c31)) }
    #[inline(always)] fn asparagus () -> Self { Self::from(Rgb24::from(0x7ba05b)) }
    #[inline(always)] fn granny_smith_apple () -> Self { Self::from(Rgb24::from(0x9de093)) }
    #[inline(always)] fn fern () -> Self { Self::from(Rgb24::from(0x63b76c)) }
    #[inline(always)] fn middle_green () -> Self { Self::from(Rgb24::from(0x4d8c57)) }
    #[inline(always)] fn green () -> Self { Self::from(Rgb24::from(0x3aa655)) }
    #[inline(always)] fn medium_chrome_green () -> Self { Self::from(Rgb24::from(0x6ca67c)) }
    #[inline(always)] fn forest_green () -> Self { Self::from(Rgb24::from(0x5fa777)) }
    #[inline(always)] fn sea_green () -> Self { Self::from(Rgb24::from(0x93dfb8)) }
    #[inline(always)] fn shamrock () -> Self { Self::from(Rgb24::from(0x33cc99)) }
    #[inline(always)] fn mountain_meadow () -> Self { Self::from(Rgb24::from(0x1ab385)) }
    #[inline(always)] fn jungle_green () -> Self { Self::from(Rgb24::from(0x29ab87)) }
    #[inline(always)] fn caribbean_green () -> Self { Self::from(Rgb24::from(0x00cc99)) }
    #[inline(always)] fn tropical_rain_forest () -> Self { Self::from(Rgb24::from(0x00755e)) }
    #[inline(always)] fn middle_blue_green () -> Self { Self::from(Rgb24::from(0x8dd9cc)) }
    #[inline(always)] fn pine_green () -> Self { Self::from(Rgb24::from(0x01786f)) }
    #[inline(always)] fn maximum_blue_green () -> Self { Self::from(Rgb24::from(0x30bfbf)) }
    #[inline(always)] fn robins_egg_blue () -> Self { Self::from(Rgb24::from(0x00cccc)) }
    #[inline(always)] fn teal_blue () -> Self { Self::from(Rgb24::from(0x008080)) }
    #[inline(always)] fn light_blue () -> Self { Self::from(Rgb24::from(0x8fd8d8)) }
    #[inline(always)] fn aquamarine () -> Self { Self::from(Rgb24::from(0x95e0e8)) }
    #[inline(always)] fn turquoise_blue () -> Self { Self::from(Rgb24::from(0x6cdae7)) }
    #[inline(always)] fn outer_space () -> Self { Self::from(Rgb24::from(0x2d383a)) }
    #[inline(always)] fn sky_blue () -> Self { Self::from(Rgb24::from(0x76d7ea)) }
    #[inline(always)] fn middle_blue () -> Self { Self::from(Rgb24::from(0x7ed4e6)) }
    #[inline(always)] fn blue_green () -> Self { Self::from(Rgb24::from(0x0095b7)) }
    #[inline(always)] fn pacific_blue () -> Self { Self::from(Rgb24::from(0x009dc4)) }
    #[inline(always)] fn cerulean () -> Self { Self::from(Rgb24::from(0x02a4d3)) }
    #[inline(always)] fn maximum_blue () -> Self { Self::from(Rgb24::from(0x47abcc)) }
    #[inline(always)] fn blue1 () -> Self { Self::from(Rgb24::from(0x4997d0)) }
    #[inline(always)] fn cerulean_blue () -> Self { Self::from(Rgb24::from(0x339acc)) }
    #[inline(always)] fn cornflower () -> Self { Self::from(Rgb24::from(0x93ccea)) }
    #[inline(always)] fn green_blue () -> Self { Self::from(Rgb24::from(0x2887c8)) }
    #[inline(always)] fn midnight_blue () -> Self { Self::from(Rgb24::from(0x00468c)) }
    #[inline(always)] fn navy_blue () -> Self { Self::from(Rgb24::from(0x0066cc)) }
    #[inline(always)] fn denim () -> Self { Self::from(Rgb24::from(0x1560bd)) }
    #[inline(always)] fn blue3 () -> Self { Self::from(Rgb24::from(0x0066ff)) }
    #[inline(always)] fn cadet_blue () -> Self { Self::from(Rgb24::from(0xa9b2c3)) }
    #[inline(always)] fn periwinkle () -> Self { Self::from(Rgb24::from(0xc3cde6)) }
    #[inline(always)] fn blue2 () -> Self { Self::from(Rgb24::from(0x4570e6)) }
    #[inline(always)] fn wild_blue_yonder () -> Self { Self::from(Rgb24::from(0x7a89b8)) }
    #[inline(always)] fn indigo () -> Self { Self::from(Rgb24::from(0x4f69c6)) }
    #[inline(always)] fn manatee () -> Self { Self::from(Rgb24::from(0x8d90a1)) }
    #[inline(always)] fn cobalt_blue () -> Self { Self::from(Rgb24::from(0x8c90c8)) }
    #[inline(always)] fn celestial_blue () -> Self { Self::from(Rgb24::from(0x7070cc)) }
    #[inline(always)] fn blue_bell () -> Self { Self::from(Rgb24::from(0x9999cc)) }
    #[inline(always)] fn maximum_blue_purple () -> Self { Self::from(Rgb24::from(0xacace6)) }
    #[inline(always)] fn violet_blue () -> Self { Self::from(Rgb24::from(0x766ec8)) }
    #[inline(always)] fn blue_violet () -> Self { Self::from(Rgb24::from(0x6456b7)) }
    #[inline(always)] fn ultramarine_blue () -> Self { Self::from(Rgb24::from(0x3f26bf)) }
    #[inline(always)] fn middle_blue_purple () -> Self { Self::from(Rgb24::from(0x8b72be)) }
    #[inline(always)] fn purple_heart () -> Self { Self::from(Rgb24::from(0x652dc1)) }
    #[inline(always)] fn royal_purple () -> Self { Self::from(Rgb24::from(0x6b3fa0)) }
    #[inline(always)] fn violet2 () -> Self { Self::from(Rgb24::from(0x8359a3)) }
    #[inline(always)] fn medium_violet () -> Self { Self::from(Rgb24::from(0x8f47b3)) }
    #[inline(always)] fn wisteria () -> Self { Self::from(Rgb24::from(0xc9a0dc)) }
    #[inline(always)] fn lavender1 () -> Self { Self::from(Rgb24::from(0xbf8fcc)) }
    #[inline(always)] fn vivid_violet () -> Self { Self::from(Rgb24::from(0x803790)) }
    #[inline(always)] fn maximum_purple () -> Self { Self::from(Rgb24::from(0x733380)) }
    #[inline(always)] fn purple_mountains_majesty () -> Self { Self::from(Rgb24::from(0xd6aedd)) }
    #[inline(always)] fn fuchsia () -> Self { Self::from(Rgb24::from(0xc154c1)) }
    #[inline(always)] fn pink_flamingo () -> Self { Self::from(Rgb24::from(0xfc74fd)) }
    #[inline(always)] fn violet1 () -> Self { Self::from(Rgb24::from(0x732e6c)) }
    #[inline(always)] fn brilliant_rose () -> Self { Self::from(Rgb24::from(0xe667ce)) }
    #[inline(always)] fn orchid () -> Self { Self::from(Rgb24::from(0xe29cd2)) }
    #[inline(always)] fn plum () -> Self { Self::from(Rgb24::from(0x8e3179)) }
    #[inline(always)] fn medium_rose () -> Self { Self::from(Rgb24::from(0xd96cbe)) }
    #[inline(always)] fn thistle () -> Self { Self::from(Rgb24::from(0xebb0d7)) }
    #[inline(always)] fn mulberry () -> Self { Self::from(Rgb24::from(0xc8509b)) }
    #[inline(always)] fn red_violet () -> Self { Self::from(Rgb24::from(0xbb3385)) }
    #[inline(always)] fn middle_purple () -> Self { Self::from(Rgb24::from(0xd982b5)) }
    #[inline(always)] fn maximum_red_purple () -> Self { Self::from(Rgb24::from(0xa63a79)) }
    #[inline(always)] fn jazzberry_jam () -> Self { Self::from(Rgb24::from(0xa50b5e)) }
    #[inline(always)] fn eggplant () -> Self { Self::from(Rgb24::from(0x614051)) }
    #[inline(always)] fn magenta () -> Self { Self::from(Rgb24::from(0xf653a6)) }
    #[inline(always)] fn cerise () -> Self { Self::from(Rgb24::from(0xda3287)) }
    #[inline(always)] fn wild_strawberry () -> Self { Self::from(Rgb24::from(0xff3399)) }
    #[inline(always)] fn lavender2 () -> Self { Self::from(Rgb24::from(0xfbaed2)) }
    #[inline(always)] fn cotton_candy () -> Self { Self::from(Rgb24::from(0xffb7d5)) }
    #[inline(always)] fn carnation_pink () -> Self { Self::from(Rgb24::from(0xffa6c9)) }
    #[inline(always)] fn violet_red () -> Self { Self::from(Rgb24::from(0xf7468a)) }
    #[inline(always)] fn razzmatazz () -> Self { Self::from(Rgb24::from(0xe30b5c)) }
    #[inline(always)] fn pig_pink () -> Self { Self::from(Rgb24::from(0xfdd7e4)) }
    #[inline(always)] fn carmine () -> Self { Self::from(Rgb24::from(0xe62e6b)) }
    #[inline(always)] fn blush () -> Self { Self::from(Rgb24::from(0xdb5079)) }
    #[inline(always)] fn tickle_me_pink () -> Self { Self::from(Rgb24::from(0xfc80a5)) }
    #[inline(always)] fn mauvelous () -> Self { Self::from(Rgb24::from(0xf091a9)) }
    #[inline(always)] fn salmon () -> Self { Self::from(Rgb24::from(0xff91a4)) }
    #[inline(always)] fn middle_red_purple () -> Self { Self::from(Rgb24::from(0xa55353)) }
    #[inline(always)] fn mahogany () -> Self { Self::from(Rgb24::from(0xca3435)) }
    #[inline(always)] fn melon () -> Self { Self::from(Rgb24::from(0xfebaad)) }
    #[inline(always)] fn pink_sherbert () -> Self { Self::from(Rgb24::from(0xf7a38e)) }
    #[inline(always)] fn burnt_sienna () -> Self { Self::from(Rgb24::from(0xe97451)) }
    #[inline(always)] fn brown () -> Self { Self::from(Rgb24::from(0xaf593e)) }
    #[inline(always)] fn sepia () -> Self { Self::from(Rgb24::from(0x9e5b40)) }
    #[inline(always)] fn fuzzy_wuzzy () -> Self { Self::from(Rgb24::from(0x87421f)) }
    #[inline(always)] fn beaver () -> Self { Self::from(Rgb24::from(0x926f5b)) }
    #[inline(always)] fn tumbleweed () -> Self { Self::from(Rgb24::from(0xdea681)) }
    #[inline(always)] fn raw_sienna () -> Self { Self::from(Rgb24::from(0xd27d46)) }
    #[inline(always)] fn van_dyke_brown () -> Self { Self::from(Rgb24::from(0x664228)) }
    #[inline(always)] fn tan () -> Self { Self::from(Rgb24::from(0xd99a6c)) }
    #[inline(always)] fn desert_sand () -> Self { Self::from(Rgb24::from(0xedc9af)) }
    #[inline(always)] fn peach () -> Self { Self::from(Rgb24::from(0xffcba4)) }
    #[inline(always)] fn burnt_umber () -> Self { Self::from(Rgb24::from(0x805533)) }
    #[inline(always)] fn apricot () -> Self { Self::from(Rgb24::from(0xfdd5b1)) }
    #[inline(always)] fn almond () -> Self { Self::from(Rgb24::from(0xeed9c4)) }
    #[inline(always)] fn raw_umber () -> Self { Self::from(Rgb24::from(0x665233)) }
    #[inline(always)] fn shadow () -> Self { Self::from(Rgb24::from(0x837050)) }
    #[inline(always)] fn raw_sienna1 () -> Self { Self::from(Rgb24::from(0xe6bc5c)) }
    #[inline(always)] fn timberwolf () -> Self { Self::from(Rgb24::from(0xd9d6cf)) }
    #[inline(always)] fn gold1 () -> Self { Self::from(Rgb24::from(0x92926e)) }
    #[inline(always)] fn gold2 () -> Self { Self::from(Rgb24::from(0xe6be8a)) }
    #[inline(always)] fn silver () -> Self { Self::from(Rgb24::from(0xc9c0bb)) }
    #[inline(always)] fn copper () -> Self { Self::from(Rgb24::from(0xda8a67)) }
    #[inline(always)] fn antique_brass () -> Self { Self::from(Rgb24::from(0xc88a65)) }
    #[inline(always)] fn black () -> Self { Self::from(Rgb24::from(0x000000)) }
    #[inline(always)] fn charcoal_gray () -> Self { Self::from(Rgb24::from(0x736a62)) }
    #[inline(always)] fn gray () -> Self { Self::from(Rgb24::from(0x8b8680)) }
    #[inline(always)] fn blue_gray () -> Self { Self::from(Rgb24::from(0xc8c8cd)) }
    #[inline(always)] fn radical_red () -> Self { Self::from(Rgb24::from(0xff355e)) }
    #[inline(always)] fn wild_watermelon () -> Self { Self::from(Rgb24::from(0xfd5b78)) }
    #[inline(always)] fn outrageous_orange () -> Self { Self::from(Rgb24::from(0xff6037)) }
    #[inline(always)] fn atomic_tangerine () -> Self { Self::from(Rgb24::from(0xff9966)) }
    #[inline(always)] fn neon_carrot () -> Self { Self::from(Rgb24::from(0xff9933)) }
    #[inline(always)] fn sunglow () -> Self { Self::from(Rgb24::from(0xffcc33)) }
    #[inline(always)] fn laser_lemon () -> Self { Self::from(Rgb24::from(0xffff66)) }
    #[inline(always)] fn unmellow_yellow () -> Self { Self::from(Rgb24::from(0xffff66)) }
    #[inline(always)] fn electric_lime () -> Self { Self::from(Rgb24::from(0xccff00)) }
    #[inline(always)] fn screamin_green () -> Self { Self::from(Rgb24::from(0x66ff66)) }
    #[inline(always)] fn magic_mint () -> Self { Self::from(Rgb24::from(0xaaf0d1)) }
    #[inline(always)] fn blizzard_blue () -> Self { Self::from(Rgb24::from(0x50bfe6)) }
    #[inline(always)] fn shocking_pink () -> Self { Self::from(Rgb24::from(0xff6eff)) }
    #[inline(always)] fn razzle_dazzle_rose () -> Self { Self::from(Rgb24::from(0xee34d2)) }
    #[inline(always)] fn hot_magenta () -> Self { Self::from(Rgb24::from(0xff00cc)) }
    #[inline(always)] fn purple_pizzazz () -> Self { Self::from(Rgb24::from(0xff00cc)) }
    #[inline(always)] fn sizzling_red () -> Self { Self::from(Rgb24::from(0xff3855)) }
    #[inline(always)] fn red_salsa () -> Self { Self::from(Rgb24::from(0xfd3a4a)) }
    #[inline(always)] fn tart_orange () -> Self { Self::from(Rgb24::from(0xfb4d46)) }
    #[inline(always)] fn orange_soda () -> Self { Self::from(Rgb24::from(0xfa5b3d)) }
    #[inline(always)] fn bright_yellow () -> Self { Self::from(Rgb24::from(0xffaa1d)) }
    #[inline(always)] fn yellow_sunshine () -> Self { Self::from(Rgb24::from(0xfff700)) }
    #[inline(always)] fn slimy_green () -> Self { Self::from(Rgb24::from(0x299617)) }
    #[inline(always)] fn green_lizard () -> Self { Self::from(Rgb24::from(0xa7f432)) }
    #[inline(always)] fn denim_blue () -> Self { Self::from(Rgb24::from(0x2243b6)) }
    #[inline(always)] fn blue_jeans () -> Self { Self::from(Rgb24::from(0x5dadec)) }
    #[inline(always)] fn plump_purple () -> Self { Self::from(Rgb24::from(0x5946b2)) }
    #[inline(always)] fn purple_plum () -> Self { Self::from(Rgb24::from(0x9c51b6)) }
    #[inline(always)] fn sweet_brown () -> Self { Self::from(Rgb24::from(0xa83731)) }
    #[inline(always)] fn brown_sugar () -> Self { Self::from(Rgb24::from(0xaf6e4d)) }
    #[inline(always)] fn eerie_black () -> Self { Self::from(Rgb24::from(0x1b1b1b)) }
    #[inline(always)] fn black_shadows () -> Self { Self::from(Rgb24::from(0xbfafb2)) }
    #[inline(always)] fn fiery_rose () -> Self { Self::from(Rgb24::from(0xff5470)) }
    #[inline(always)] fn sizzling_sunrise () -> Self { Self::from(Rgb24::from(0xffdb00)) }
    #[inline(always)] fn heat_wave () -> Self { Self::from(Rgb24::from(0xff7a00)) }
    #[inline(always)] fn lemon_glacier () -> Self { Self::from(Rgb24::from(0xfdff00)) }
    #[inline(always)] fn spring_frost () -> Self { Self::from(Rgb24::from(0x87ff2a)) }
    #[inline(always)] fn absolute_zero () -> Self { Self::from(Rgb24::from(0x0048ba)) }
    #[inline(always)] fn winter_sky () -> Self { Self::from(Rgb24::from(0xff007c)) }
    #[inline(always)] fn frostbite () -> Self { Self::from(Rgb24::from(0xe936a7)) }
    #[inline(always)] fn alloy_orange () -> Self { Self::from(Rgb24::from(0xc46210)) }
    #[inline(always)] fn bdazzled_blue () -> Self { Self::from(Rgb24::from(0x2e5894)) }
    #[inline(always)] fn big_dip_o_ruby () -> Self { Self::from(Rgb24::from(0x9c2542)) }
    #[inline(always)] fn bittersweet_shimmer () -> Self { Self::from(Rgb24::from(0xbf4f51)) }
    #[inline(always)] fn blast_off_bronze () -> Self { Self::from(Rgb24::from(0xa57164)) }
    #[inline(always)] fn cyber_grape () -> Self { Self::from(Rgb24::from(0x58427c)) }
    #[inline(always)] fn deep_space_sparkle () -> Self { Self::from(Rgb24::from(0x4a646c)) }
    #[inline(always)] fn gold_fusion () -> Self { Self::from(Rgb24::from(0x85754e)) }
    #[inline(always)] fn illuminating_emerald () -> Self { Self::from(Rgb24::from(0x319177)) }
    #[inline(always)] fn metallic_seaweed () -> Self { Self::from(Rgb24::from(0x0a7e8c)) }
    #[inline(always)] fn metallic_sunburst () -> Self { Self::from(Rgb24::from(0x9c7c38)) }
    #[inline(always)] fn razzmic_berry () -> Self { Self::from(Rgb24::from(0x8d4e85)) }
    #[inline(always)] fn sheen_green () -> Self { Self::from(Rgb24::from(0x8fd400)) }
    #[inline(always)] fn shimmering_blush () -> Self { Self::from(Rgb24::from(0xd98695)) }
    #[inline(always)] fn sonic_silver () -> Self { Self::from(Rgb24::from(0x757575)) }
    #[inline(always)] fn steel_blue () -> Self { Self::from(Rgb24::from(0x0081ab)) }
    #[inline(always)] fn aztec_gold () -> Self { Self::from(Rgb24::from(0xc39953)) }
    #[inline(always)] fn burnished_brown () -> Self { Self::from(Rgb24::from(0xa17a74)) }
    #[inline(always)] fn cerulean_frost () -> Self { Self::from(Rgb24::from(0x6d9bc3)) }
    #[inline(always)] fn cinnamon_satin () -> Self { Self::from(Rgb24::from(0xcd607e)) }
    #[inline(always)] fn copper_penny () -> Self { Self::from(Rgb24::from(0xad6f69)) }
    #[inline(always)] fn cosmic_cobalt () -> Self { Self::from(Rgb24::from(0x2e2d88)) }
    #[inline(always)] fn glossy_grape () -> Self { Self::from(Rgb24::from(0xab92b3)) }
    #[inline(always)] fn granite_gray () -> Self { Self::from(Rgb24::from(0x676767)) }
    #[inline(always)] fn green_sheen () -> Self { Self::from(Rgb24::from(0x6eaea1)) }
    #[inline(always)] fn lilac_luster () -> Self { Self::from(Rgb24::from(0xae98aa)) }
    #[inline(always)] fn misty_moss () -> Self { Self::from(Rgb24::from(0xbbb477)) }
    #[inline(always)] fn mystic_maroon () -> Self { Self::from(Rgb24::from(0xad4379)) }
    #[inline(always)] fn pearly_purple () -> Self { Self::from(Rgb24::from(0xb768a2)) }
    #[inline(always)] fn pewter_blue () -> Self { Self::from(Rgb24::from(0x8ba8b7)) }
    #[inline(always)] fn polished_pine () -> Self { Self::from(Rgb24::from(0x5da493)) }
    #[inline(always)] fn quick_silver () -> Self { Self::from(Rgb24::from(0xa6a6a6)) }
    #[inline(always)] fn rose_dust () -> Self { Self::from(Rgb24::from(0x9e5e6f)) }
    #[inline(always)] fn rusty_red () -> Self { Self::from(Rgb24::from(0xda2c43)) }
    #[inline(always)] fn shadow_blue () -> Self { Self::from(Rgb24::from(0x778ba5)) }
    #[inline(always)] fn shiny_shamrock () -> Self { Self::from(Rgb24::from(0x5fa778)) }
    #[inline(always)] fn steel_teal () -> Self { Self::from(Rgb24::from(0x5f8a8b)) }
    #[inline(always)] fn sugar_plum () -> Self { Self::from(Rgb24::from(0x914e75)) }
    #[inline(always)] fn twilight_lavender () -> Self { Self::from(Rgb24::from(0x8a496b)) }
    #[inline(always)] fn wintergreen_dream () -> Self { Self::from(Rgb24::from(0x56887d)) }
    #[inline(always)] fn baby_powder () -> Self { Self::from(Rgb24::from(0xfefefa)) }
    #[inline(always)] fn banana () -> Self { Self::from(Rgb24::from(0xffd12a)) }
    #[inline(always)] fn blueberry () -> Self { Self::from(Rgb24::from(0x4f86f7)) }
    #[inline(always)] fn bubble_gum () -> Self { Self::from(Rgb24::from(0xffd3f8)) }
    #[inline(always)] fn cedar_chest () -> Self { Self::from(Rgb24::from(0xc95a49)) }
    #[inline(always)] fn cherry () -> Self { Self::from(Rgb24::from(0xda2647)) }
    #[inline(always)] fn chocolate () -> Self { Self::from(Rgb24::from(0xbd8260)) }
    #[inline(always)] fn coconut () -> Self { Self::from(Rgb24::from(0xfefefe)) }
    #[inline(always)] fn daffodil () -> Self { Self::from(Rgb24::from(0xffff31)) }
    #[inline(always)] fn dirt () -> Self { Self::from(Rgb24::from(0x9b7653)) }
    #[inline(always)] fn eucalyptus () -> Self { Self::from(Rgb24::from(0x44d7a8)) }
    #[inline(always)] fn fresh_air () -> Self { Self::from(Rgb24::from(0xa6e7ff)) }
    #[inline(always)] fn grape () -> Self { Self::from(Rgb24::from(0x6f2da8)) }
    #[inline(always)] fn jelly_bean () -> Self { Self::from(Rgb24::from(0xda614e)) }
    #[inline(always)] fn leather_jacket () -> Self { Self::from(Rgb24::from(0x253529)) }
    #[inline(always)] fn lemon () -> Self { Self::from(Rgb24::from(0xffff38)) }
    #[inline(always)] fn licorice () -> Self { Self::from(Rgb24::from(0x1a1110)) }
    #[inline(always)] fn lilac () -> Self { Self::from(Rgb24::from(0xdb91ef)) }
    #[inline(always)] fn lime () -> Self { Self::from(Rgb24::from(0xb2f302)) }
    #[inline(always)] fn lumber () -> Self { Self::from(Rgb24::from(0xffe4cd)) }
    #[inline(always)] fn new_car () -> Self { Self::from(Rgb24::from(0x214fc6)) }
    #[inline(always)] fn orange_fragrance () -> Self { Self::from(Rgb24::from(0xff8866)) }
    #[inline(always)] fn peach_fragrance () -> Self { Self::from(Rgb24::from(0xffd0b9)) }
    #[inline(always)] fn pine () -> Self { Self::from(Rgb24::from(0x45a27d)) }
    #[inline(always)] fn rose () -> Self { Self::from(Rgb24::from(0xff5050)) }
    #[inline(always)] fn shampoo () -> Self { Self::from(Rgb24::from(0xffcff1)) }
    #[inline(always)] fn smoke () -> Self { Self::from(Rgb24::from(0x738276)) }
    #[inline(always)] fn soap () -> Self { Self::from(Rgb24::from(0xcec8ef)) }
    #[inline(always)] fn strawberry () -> Self { Self::from(Rgb24::from(0xfc5a8d)) }
    #[inline(always)] fn tulip () -> Self { Self::from(Rgb24::from(0xff878d)) }
    #[inline(always)] fn amethyst () -> Self { Self::from(Rgb24::from(0x64609a)) }
    #[inline(always)] fn citrine () -> Self { Self::from(Rgb24::from(0x933709)) }
    #[inline(always)] fn emerald () -> Self { Self::from(Rgb24::from(0x14a989)) }
    #[inline(always)] fn jade () -> Self { Self::from(Rgb24::from(0x469a84)) }
    #[inline(always)] fn jasper () -> Self { Self::from(Rgb24::from(0xd05340)) }
    #[inline(always)] fn lapis_lazuli () -> Self { Self::from(Rgb24::from(0x436cb9)) }
    #[inline(always)] fn malachite () -> Self { Self::from(Rgb24::from(0x469496)) }
    #[inline(always)] fn moonstone () -> Self { Self::from(Rgb24::from(0x3aa8c1)) }
    #[inline(always)] fn onyx () -> Self { Self::from(Rgb24::from(0x353839)) }
    #[inline(always)] fn peridot () -> Self { Self::from(Rgb24::from(0xabad48)) }
    #[inline(always)] fn pink_pearl () -> Self { Self::from(Rgb24::from(0xb07080)) }
    #[inline(always)] fn rose_quartz () -> Self { Self::from(Rgb24::from(0xbd559c)) }
    #[inline(always)] fn ruby () -> Self { Self::from(Rgb24::from(0xaa4069)) }
    #[inline(always)] fn sapphire () -> Self { Self::from(Rgb24::from(0x2d5da1)) }
    #[inline(always)] fn smokey_topaz () -> Self { Self::from(Rgb24::from(0x832a0d)) }
    #[inline(always)] fn tigers_eye () -> Self { Self::from(Rgb24::from(0xb56917)) }
    #[inline(always)] fn baseball_mittburnt_sienna () -> Self { Self::from(Rgb24::from(0xe97451)) }
    #[inline(always)] fn bubble_bathtickle_me_pink () -> Self { Self::from(Rgb24::from(0xfc80a5)) }
    #[inline(always)] fn earthwormbrick_red () -> Self { Self::from(Rgb24::from(0xc62d42)) }
    #[inline(always)] fn flower_shopwisteria () -> Self { Self::from(Rgb24::from(0xc9a0dc)) }
    #[inline(always)] fn fresh_airsky_blue () -> Self { Self::from(Rgb24::from(0x76d7ea)) }
    #[inline(always)] fn grandmas_perfumeorange () -> Self { Self::from(Rgb24::from(0xff8833)) }
    #[inline(always)] fn koala_treejungle_green () -> Self { Self::from(Rgb24::from(0x29ab87)) }
    #[inline(always)] fn pet_shopbrown () -> Self { Self::from(Rgb24::from(0xaf593e)) }
    #[inline(always)] fn pine_treepine_green () -> Self { Self::from(Rgb24::from(0x01786f)) }
    #[inline(always)] fn saw_dustpeach () -> Self { Self::from(Rgb24::from(0xffcba4)) }
    #[inline(always)] fn sharpening_pencilsgoldenrod () -> Self { Self::from(Rgb24::from(0xfcd667)) }
    #[inline(always)] fn smell_the_rosesred () -> Self { Self::from(Rgb24::from(0xed0a3f)) }
    #[inline(always)] fn sunny_dayyellow () -> Self { Self::from(Rgb24::from(0xfbe870)) }
    #[inline(always)] fn wash_the_dogdandelion () -> Self { Self::from(Rgb24::from(0xfed85d)) }
    #[inline(always)] fn alien_armpit () -> Self { Self::from(Rgb24::from(0x84de02)) }
    #[inline(always)] fn big_foot_feet () -> Self { Self::from(Rgb24::from(0xe88e5a)) }
    #[inline(always)] fn booger_buster () -> Self { Self::from(Rgb24::from(0xdde26a)) }
    #[inline(always)] fn dingy_dungeon () -> Self { Self::from(Rgb24::from(0xc53151)) }
    #[inline(always)] fn gargoyle_gas () -> Self { Self::from(Rgb24::from(0xffdf46)) }
    #[inline(always)] fn giants_club () -> Self { Self::from(Rgb24::from(0xb05c52)) }
    #[inline(always)] fn magic_potion () -> Self { Self::from(Rgb24::from(0xff4466)) }
    #[inline(always)] fn mummys_tomb () -> Self { Self::from(Rgb24::from(0x828e84)) }
    #[inline(always)] fn ogre_odor () -> Self { Self::from(Rgb24::from(0xfd5240)) }
    #[inline(always)] fn pixie_powder () -> Self { Self::from(Rgb24::from(0x391285)) }
    #[inline(always)] fn princess_perfume () -> Self { Self::from(Rgb24::from(0xff85cf)) }
    #[inline(always)] fn sasquatch_socks () -> Self { Self::from(Rgb24::from(0xff4681)) }
    #[inline(always)] fn sea_serpent () -> Self { Self::from(Rgb24::from(0x4bc7cf)) }
    #[inline(always)] fn smashed_pumpkin () -> Self { Self::from(Rgb24::from(0xff6d3a)) }
    #[inline(always)] fn sunburnt_cyclops () -> Self { Self::from(Rgb24::from(0xff404c)) }
    #[inline(always)] fn winter_wizard () -> Self { Self::from(Rgb24::from(0xa0e6ff)) }
}
