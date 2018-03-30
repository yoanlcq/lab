//! https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
use Rgb24;

#[cfg(any(feature="tables", feature="css_table"))]
use Entry;

#[cfg(any(feature="tables", feature="css_table"))]
pub const CSS_COLORS : &[Entry] = &[
    Entry { ident: "black", value: hex24!(0x000000  ) },
    Entry { ident: "silver", value: hex24!(0xc0c0c0  ) },
    Entry { ident: "gray", value: hex24!(0x808080  ) },
    Entry { ident: "white", value: hex24!(0xffffff  ) },
    Entry { ident: "maroon", value: hex24!(0x800000  ) },
    Entry { ident: "red", value: hex24!(0xff0000  ) },
    Entry { ident: "purple", value: hex24!(0x800080  ) },
    Entry { ident: "fuchsia", value: hex24!(0xff00ff  ) },
    Entry { ident: "green", value: hex24!(0x008000  ) },
    Entry { ident: "lime", value: hex24!(0x00ff00  ) },
    Entry { ident: "olive", value: hex24!(0x808000  ) },
    Entry { ident: "yellow", value: hex24!(0xffff00  ) },
    Entry { ident: "navy", value: hex24!(0x000080  ) },
    Entry { ident: "blue", value: hex24!(0x0000ff  ) },
    Entry { ident: "teal", value: hex24!(0x008080  ) },
    Entry { ident: "aqua", value: hex24!(0x00ffff  ) },
    Entry { ident: "orange", value: hex24!(0xffa500  ) },
    Entry { ident: "aliceblue", value: hex24!(0xf0f8ff  ) },
    Entry { ident: "antiquewhite", value: hex24!(0xfaebd7  ) },
    Entry { ident: "aquamarine", value: hex24!(0x7fffd4  ) },
    Entry { ident: "azure", value: hex24!(0xf0ffff  ) },
    Entry { ident: "beige", value: hex24!(0xf5f5dc  ) },
    Entry { ident: "bisque", value: hex24!(0xffe4c4  ) },
    Entry { ident: "blanchedalmond", value: hex24!(0xffebcd  ) },
    Entry { ident: "blueviolet", value: hex24!(0x8a2be2  ) },
    Entry { ident: "brown", value: hex24!(0xa52a2a     ) },
    Entry { ident: "burlywood", value: hex24!(0xdeb887  ) },
    Entry { ident: "cadetblue", value: hex24!(0x5f9ea0  ) },
    Entry { ident: "chartreuse", value: hex24!(0x7fff00  ) },
    Entry { ident: "chocolate", value: hex24!(0xd2691e  ) },
    Entry { ident: "coral", value: hex24!(0xff7f50  ) },
    Entry { ident: "cornflowerblue", value: hex24!(0x6495ed  ) },
    Entry { ident: "cornsilk", value: hex24!(0xfff8dc  ) },
    Entry { ident: "crimson", value: hex24!(0xdc143c  ) },
    Entry { ident: "cyan", value: hex24!(0x00ffff  ) },
    Entry { ident: "darkblue", value: hex24!(0x00008b  ) },
    Entry { ident: "darkcyan", value: hex24!(0x008b8b  ) },
    Entry { ident: "darkgoldenrod", value: hex24!(0xb8860b  ) },
    Entry { ident: "darkgray", value: hex24!(0xa9a9a9  ) },
    Entry { ident: "darkgreen", value: hex24!(0x006400  ) },
    Entry { ident: "darkgrey", value: hex24!(0xa9a9a9  ) },
    Entry { ident: "darkkhaki", value: hex24!(0xbdb76b  ) },
    Entry { ident: "darkmagenta", value: hex24!(0x8b008b  ) },
    Entry { ident: "darkolivegreen", value: hex24!(0x556b2f  ) },
    Entry { ident: "darkorange", value: hex24!(0xff8c00  ) },
    Entry { ident: "darkorchid", value: hex24!(0x9932cc  ) },
    Entry { ident: "darkred", value: hex24!(0x8b0000  ) },
    Entry { ident: "darksalmon", value: hex24!(0xe9967a    ) },
    Entry { ident: "darkseagreen", value: hex24!(0x8fbc8f  ) },
    Entry { ident: "darkslateblue", value: hex24!(0x483d8b  ) },
    Entry { ident: "darkslategray", value: hex24!(0x2f4f4f  ) },
    Entry { ident: "darkslategrey", value: hex24!(0x2f4f4f  ) },
    Entry { ident: "darkturquoise", value: hex24!(0x00ced1  ) },
    Entry { ident: "darkviolet", value: hex24!(0x9400d3  ) },
    Entry { ident: "deeppink", value: hex24!(0xff1493  ) },
    Entry { ident: "deepskyblue", value: hex24!(0x00bfff  ) },
    Entry { ident: "dimgray", value: hex24!(0x696969  ) },
    Entry { ident: "dimgrey", value: hex24!(0x696969  ) },
    Entry { ident: "dodgerblue", value: hex24!(0x1e90ff  ) },
    Entry { ident: "firebrick", value: hex24!(0xb22222  ) },
    Entry { ident: "floralwhite", value: hex24!(0xfffaf0  ) },
    Entry { ident: "forestgreen", value: hex24!(0x228b22  ) },
    Entry { ident: "gainsboro", value: hex24!(0xdcdcdc  ) },
    Entry { ident: "ghostwhite", value: hex24!(0xf8f8ff  ) },
    Entry { ident: "gold", value: hex24!(0xffd700  ) },
    Entry { ident: "goldenrod", value: hex24!(0xdaa520  ) },
    Entry { ident: "greenyellow", value: hex24!(0xadff2f  ) },
    Entry { ident: "grey", value: hex24!(0x808080  ) },
    Entry { ident: "honeydew", value: hex24!(0xf0fff0  ) },
    Entry { ident: "hotpink", value: hex24!(0xff69b4  ) },
    Entry { ident: "indianred", value: hex24!(0xcd5c5c  ) },
    Entry { ident: "indigo", value: hex24!(0x4b0082  ) },
    Entry { ident: "ivory", value: hex24!(0xfffff0   ) },
    Entry { ident: "khaki", value: hex24!(0xf0e68c   ) },
    Entry { ident: "lavender", value: hex24!(0xe6e6fa  ) },
    Entry { ident: "lavenderblush", value: hex24!(0xfff0f5  ) },
    Entry { ident: "lawngreen", value: hex24!(0x7cfc00  ) },
    Entry { ident: "lemonchiffon", value: hex24!(0xfffacd  ) },
    Entry { ident: "lightblue", value: hex24!(0xadd8e6  ) },
    Entry { ident: "lightcoral", value: hex24!(0xf08080  ) },
    Entry { ident: "lightcyan", value: hex24!(0xe0ffff  ) },
    Entry { ident: "lightgoldenrodyellow", value: hex24!(0xfafad2  ) },
    Entry { ident: "lightgray", value: hex24!(0xd3d3d3  ) },
    Entry { ident: "lightgreen", value: hex24!(0x90ee90  ) },
    Entry { ident: "lightgrey", value: hex24!(0xd3d3d3  ) },
    Entry { ident: "lightpink", value: hex24!(0xffb6c1  ) },
    Entry { ident: "lightsalmon", value: hex24!(0xffa07a  ) },
    Entry { ident: "lightseagreen", value: hex24!(0x20b2aa  ) },
    Entry { ident: "lightskyblue", value: hex24!(0x87cefa  ) },
    Entry { ident: "lightslategray", value: hex24!(0x778899  ) },
    Entry { ident: "lightslategrey", value: hex24!(0x778899  ) },
    Entry { ident: "lightsteelblue", value: hex24!(0xb0c4de  ) },
    Entry { ident: "lightyellow", value: hex24!(0xffffe0  ) },
    Entry { ident: "limegreen", value: hex24!(0x32cd32  ) },
    Entry { ident: "linen", value: hex24!(0xfaf0e6  ) },
    Entry { ident: "mediumaquamarine", value: hex24!(0x66cdaa  ) },
    Entry { ident: "mediumblue", value: hex24!(0x0000cd  ) },
    Entry { ident: "mediumorchid", value: hex24!(0xba55d3  ) },
    Entry { ident: "mediumpurple", value: hex24!(0x9370db  ) },
    Entry { ident: "mediumseagreen", value: hex24!(0x3cb371  ) },
    Entry { ident: "mediumslateblue", value: hex24!(0x7b68ee  ) },
    Entry { ident: "mediumspringgreen", value: hex24!(0x00fa9a  ) },
    Entry { ident: "mediumturquoise", value: hex24!(0x48d1cc  ) },
    Entry { ident: "mediumvioletred", value: hex24!(0xc71585  ) },
    Entry { ident: "midnightblue", value: hex24!(0x191970  ) },
    Entry { ident: "mintcream", value: hex24!(0xf5fffa  ) },
    Entry { ident: "mistyrose", value: hex24!(0xffe4e1  ) },
    Entry { ident: "moccasin", value: hex24!(0xffe4b5  ) },
    Entry { ident: "navajowhite", value: hex24!(0xffdead  ) },
    Entry { ident: "oldlace", value: hex24!(0xfdf5e6  ) },
    Entry { ident: "olivedrab", value: hex24!(0x6b8e23  ) },
    Entry { ident: "orangered", value: hex24!(0xff4500  ) },
    Entry { ident: "orchid", value: hex24!(0xda70d6  ) },
    Entry { ident: "palegoldenrod", value: hex24!(0xeee8aa  ) },
    Entry { ident: "palegreen", value: hex24!(0x98fb98  ) },
    Entry { ident: "paleturquoise", value: hex24!(0xafeeee  ) },
    Entry { ident: "palevioletred", value: hex24!(0xdb7093  ) },
    Entry { ident: "papayawhip", value: hex24!(0xffefd5  ) },
    Entry { ident: "peachpuff", value: hex24!(0xffdab9  ) },
    Entry { ident: "peru", value: hex24!(0xcd853f  ) },
    Entry { ident: "pink", value: hex24!(0xffc0cb     ) },
    Entry { ident: "plum", value: hex24!(0xdda0dd     ) },
    Entry { ident: "powderblue", value: hex24!(0xb0e0e6  ) },
    Entry { ident: "rosybrown", value: hex24!(0xbc8f8f  ) },
    Entry { ident: "royalblue", value: hex24!(0x4169e1  ) },
    Entry { ident: "saddlebrown", value: hex24!(0x8b4513  ) },
    Entry { ident: "salmon", value: hex24!(0xfa8072  ) },
    Entry { ident: "sandybrown", value: hex24!(0xf4a460  ) },
    Entry { ident: "seagreen", value: hex24!(0x2e8b57  ) },
    Entry { ident: "seashell", value: hex24!(0xfff5ee  ) },
    Entry { ident: "sienna", value: hex24!(0xa0522d  ) },
    Entry { ident: "skyblue", value: hex24!(0x87ceeb  ) },
    Entry { ident: "slateblue", value: hex24!(0x6a5acd  ) },
    Entry { ident: "slategray", value: hex24!(0x708090  ) },
    Entry { ident: "slategrey", value: hex24!(0x708090  ) },
    Entry { ident: "snow", value: hex24!(0xfffafa  ) },
    Entry { ident: "springgreen", value: hex24!(0x00ff7f  ) },
    Entry { ident: "steelblue", value: hex24!(0x4682b4  ) },
    Entry { ident: "tan", value: hex24!(0xd2b48c  ) },
    Entry { ident: "thistle", value: hex24!(0xd8bfd8  ) },
    Entry { ident: "tomato", value: hex24!(0xff6347  ) },
    Entry { ident: "turquoise", value: hex24!(0x40e0d0  ) },
    Entry { ident: "violet", value: hex24!(0xee82ee  ) },
    Entry { ident: "wheat", value: hex24!(0xf5deb3  ) },
    Entry { ident: "whitesmoke", value: hex24!(0xf5f5f5    ) },
    Entry { ident: "yellowgreen", value: hex24!(0x9acd32  ) },
    Entry { ident: "rebeccapurple", value: hex24!(0x663399) },
];


#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait CssColors : From<Rgb24> {
    #[inline(always)] fn black   () -> Self { Self::from(Rgb24::from(0x000000  )) }
    #[inline(always)] fn silver  () -> Self { Self::from(Rgb24::from(0xc0c0c0  )) }
    #[inline(always)] fn gray    () -> Self { Self::from(Rgb24::from(0x808080  )) }
    #[inline(always)] fn white   () -> Self { Self::from(Rgb24::from(0xffffff  )) }
    #[inline(always)] fn maroon  () -> Self { Self::from(Rgb24::from(0x800000  )) }
    #[inline(always)] fn red     () -> Self { Self::from(Rgb24::from(0xff0000  )) }
    #[inline(always)] fn purple  () -> Self { Self::from(Rgb24::from(0x800080  )) }
    #[inline(always)] fn fuchsia () -> Self { Self::from(Rgb24::from(0xff00ff  )) }
    #[inline(always)] fn green   () -> Self { Self::from(Rgb24::from(0x008000  )) }
    #[inline(always)] fn lime    () -> Self { Self::from(Rgb24::from(0x00ff00  )) }
    #[inline(always)] fn olive   () -> Self { Self::from(Rgb24::from(0x808000  )) }
    #[inline(always)] fn yellow  () -> Self { Self::from(Rgb24::from(0xffff00  )) }
    #[inline(always)] fn navy    () -> Self { Self::from(Rgb24::from(0x000080  )) }
    #[inline(always)] fn blue    () -> Self { Self::from(Rgb24::from(0x0000ff  )) }
    #[inline(always)] fn teal    () -> Self { Self::from(Rgb24::from(0x008080  )) }
    #[inline(always)] fn aqua    () -> Self { Self::from(Rgb24::from(0x00ffff  )) }
    #[inline(always)] fn orange () -> Self { Self::from(Rgb24::from(0xffa500  )) }
    #[inline(always)] fn aliceblue  () -> Self { Self::from(Rgb24::from(0xf0f8ff  )) }
    #[inline(always)] fn antiquewhite    () -> Self { Self::from(Rgb24::from(0xfaebd7  )) }
    #[inline(always)] fn aquamarine  () -> Self { Self::from(Rgb24::from(0x7fffd4  )) }
    #[inline(always)] fn azure   () -> Self { Self::from(Rgb24::from(0xf0ffff  )) }
    #[inline(always)] fn beige   () -> Self { Self::from(Rgb24::from(0xf5f5dc  )) }
    #[inline(always)] fn bisque  () -> Self { Self::from(Rgb24::from(0xffe4c4  )) }
    #[inline(always)] fn blanchedalmond () -> Self { Self::from(Rgb24::from(0xffebcd  )) }
    #[inline(always)] fn blueviolet () -> Self { Self::from(Rgb24::from(0x8a2be2  )) }
    #[inline(always)] fn brown   () -> Self { Self::from(Rgb24::from(0xa52a2a     )) }
    #[inline(always)] fn burlywood  () -> Self { Self::from(Rgb24::from(0xdeb887  )) }
    #[inline(always)] fn cadetblue   () -> Self { Self::from(Rgb24::from(0x5f9ea0  )) }
    #[inline(always)] fn chartreuse  () -> Self { Self::from(Rgb24::from(0x7fff00  )) }
    #[inline(always)] fn chocolate   () -> Self { Self::from(Rgb24::from(0xd2691e  )) }
    #[inline(always)] fn coral   () -> Self { Self::from(Rgb24::from(0xff7f50  )) }
    #[inline(always)] fn cornflowerblue  () -> Self { Self::from(Rgb24::from(0x6495ed  )) }
    #[inline(always)] fn cornsilk    () -> Self { Self::from(Rgb24::from(0xfff8dc  )) }
    #[inline(always)] fn crimson () -> Self { Self::from(Rgb24::from(0xdc143c  )) }
    #[inline(always)] fn cyan    () -> Self { Self::from(Rgb24::from(0x00ffff  )) }
    #[inline(always)] fn darkblue    () -> Self { Self::from(Rgb24::from(0x00008b  )) }
    #[inline(always)] fn darkcyan    () -> Self { Self::from(Rgb24::from(0x008b8b  )) }
    #[inline(always)] fn darkgoldenrod   () -> Self { Self::from(Rgb24::from(0xb8860b  )) }
    #[inline(always)] fn darkgray    () -> Self { Self::from(Rgb24::from(0xa9a9a9  )) }
    #[inline(always)] fn darkgreen   () -> Self { Self::from(Rgb24::from(0x006400  )) }
    #[inline(always)] fn darkgrey    () -> Self { Self::from(Rgb24::from(0xa9a9a9  )) }
    #[inline(always)] fn darkkhaki   () -> Self { Self::from(Rgb24::from(0xbdb76b  )) }
    #[inline(always)] fn darkmagenta () -> Self { Self::from(Rgb24::from(0x8b008b  )) }
    #[inline(always)] fn darkolivegreen  () -> Self { Self::from(Rgb24::from(0x556b2f  )) }
    #[inline(always)] fn darkorange  () -> Self { Self::from(Rgb24::from(0xff8c00  )) }
    #[inline(always)] fn darkorchid  () -> Self { Self::from(Rgb24::from(0x9932cc  )) }
    #[inline(always)] fn darkred () -> Self { Self::from(Rgb24::from(0x8b0000  )) }
    #[inline(always)] fn darksalmon  () -> Self { Self::from(Rgb24::from(0xe9967a    )) }
    #[inline(always)] fn darkseagreen  () -> Self { Self::from(Rgb24::from(0x8fbc8f  )) }
    #[inline(always)] fn darkslateblue () -> Self { Self::from(Rgb24::from(0x483d8b  )) }
    #[inline(always)] fn darkslategray () -> Self { Self::from(Rgb24::from(0x2f4f4f  )) }
    #[inline(always)] fn darkslategrey   () -> Self { Self::from(Rgb24::from(0x2f4f4f  )) }
    #[inline(always)] fn darkturquoise   () -> Self { Self::from(Rgb24::from(0x00ced1  )) }
    #[inline(always)] fn darkviolet  () -> Self { Self::from(Rgb24::from(0x9400d3  )) }
    #[inline(always)] fn deeppink    () -> Self { Self::from(Rgb24::from(0xff1493  )) }
    #[inline(always)] fn deepskyblue () -> Self { Self::from(Rgb24::from(0x00bfff  )) }
    #[inline(always)] fn dimgray () -> Self { Self::from(Rgb24::from(0x696969  )) }
    #[inline(always)] fn dimgrey () -> Self { Self::from(Rgb24::from(0x696969  )) }
    #[inline(always)] fn dodgerblue  () -> Self { Self::from(Rgb24::from(0x1e90ff  )) }
    #[inline(always)] fn firebrick   () -> Self { Self::from(Rgb24::from(0xb22222  )) }
    #[inline(always)] fn floralwhite () -> Self { Self::from(Rgb24::from(0xfffaf0  )) }
    #[inline(always)] fn forestgreen () -> Self { Self::from(Rgb24::from(0x228b22  )) }
    #[inline(always)] fn gainsboro   () -> Self { Self::from(Rgb24::from(0xdcdcdc  )) }
    #[inline(always)] fn ghostwhite  () -> Self { Self::from(Rgb24::from(0xf8f8ff  )) }
    #[inline(always)] fn gold    () -> Self { Self::from(Rgb24::from(0xffd700  )) }
    #[inline(always)] fn goldenrod   () -> Self { Self::from(Rgb24::from(0xdaa520  )) }
    #[inline(always)] fn greenyellow () -> Self { Self::from(Rgb24::from(0xadff2f  )) }
    #[inline(always)] fn grey    () -> Self { Self::from(Rgb24::from(0x808080  )) }
    #[inline(always)] fn honeydew    () -> Self { Self::from(Rgb24::from(0xf0fff0  )) }
    #[inline(always)] fn hotpink () -> Self { Self::from(Rgb24::from(0xff69b4  )) }
    #[inline(always)] fn indianred   () -> Self { Self::from(Rgb24::from(0xcd5c5c  )) }
    #[inline(always)] fn indigo  () -> Self { Self::from(Rgb24::from(0x4b0082  )) }
    #[inline(always)] fn ivory   () -> Self { Self::from(Rgb24::from(0xfffff0   )) }
    #[inline(always)] fn khaki   () -> Self { Self::from(Rgb24::from(0xf0e68c   )) }
    #[inline(always)] fn lavender () -> Self { Self::from(Rgb24::from(0xe6e6fa  )) }
    #[inline(always)] fn lavenderblush   () -> Self { Self::from(Rgb24::from(0xfff0f5  )) }
    #[inline(always)] fn lawngreen   () -> Self { Self::from(Rgb24::from(0x7cfc00  )) }
    #[inline(always)] fn lemonchiffon    () -> Self { Self::from(Rgb24::from(0xfffacd  )) }
    #[inline(always)] fn lightblue   () -> Self { Self::from(Rgb24::from(0xadd8e6  )) }
    #[inline(always)] fn lightcoral  () -> Self { Self::from(Rgb24::from(0xf08080  )) }
    #[inline(always)] fn lightcyan   () -> Self { Self::from(Rgb24::from(0xe0ffff  )) }
    #[inline(always)] fn lightgoldenrodyellow    () -> Self { Self::from(Rgb24::from(0xfafad2  )) }
    #[inline(always)] fn lightgray   () -> Self { Self::from(Rgb24::from(0xd3d3d3  )) }
    #[inline(always)] fn lightgreen  () -> Self { Self::from(Rgb24::from(0x90ee90  )) }
    #[inline(always)] fn lightgrey   () -> Self { Self::from(Rgb24::from(0xd3d3d3  )) }
    #[inline(always)] fn lightpink   () -> Self { Self::from(Rgb24::from(0xffb6c1  )) }
    #[inline(always)] fn lightsalmon () -> Self { Self::from(Rgb24::from(0xffa07a  )) }
    #[inline(always)] fn lightseagreen   () -> Self { Self::from(Rgb24::from(0x20b2aa  )) }
    #[inline(always)] fn lightskyblue    () -> Self { Self::from(Rgb24::from(0x87cefa  )) }
    #[inline(always)] fn lightslategray  () -> Self { Self::from(Rgb24::from(0x778899  )) }
    #[inline(always)] fn lightslategrey  () -> Self { Self::from(Rgb24::from(0x778899  )) }
    #[inline(always)] fn lightsteelblue  () -> Self { Self::from(Rgb24::from(0xb0c4de  )) }
    #[inline(always)] fn lightyellow () -> Self { Self::from(Rgb24::from(0xffffe0  )) }
    #[inline(always)] fn limegreen   () -> Self { Self::from(Rgb24::from(0x32cd32  )) }
    #[inline(always)] fn linen   () -> Self { Self::from(Rgb24::from(0xfaf0e6  )) }
    #[inline(always)] fn mediumaquamarine    () -> Self { Self::from(Rgb24::from(0x66cdaa  )) }
    #[inline(always)] fn mediumblue  () -> Self { Self::from(Rgb24::from(0x0000cd  )) }
    #[inline(always)] fn mediumorchid    () -> Self { Self::from(Rgb24::from(0xba55d3  )) }
    #[inline(always)] fn mediumpurple    () -> Self { Self::from(Rgb24::from(0x9370db  )) }
    #[inline(always)] fn mediumseagreen  () -> Self { Self::from(Rgb24::from(0x3cb371  )) }
    #[inline(always)] fn mediumslateblue () -> Self { Self::from(Rgb24::from(0x7b68ee  )) }
    #[inline(always)] fn mediumspringgreen   () -> Self { Self::from(Rgb24::from(0x00fa9a  )) }
    #[inline(always)] fn mediumturquoise () -> Self { Self::from(Rgb24::from(0x48d1cc  )) }
    #[inline(always)] fn mediumvioletred () -> Self { Self::from(Rgb24::from(0xc71585  )) }
    #[inline(always)] fn midnightblue    () -> Self { Self::from(Rgb24::from(0x191970  )) }
    #[inline(always)] fn mintcream   () -> Self { Self::from(Rgb24::from(0xf5fffa  )) }
    #[inline(always)] fn mistyrose   () -> Self { Self::from(Rgb24::from(0xffe4e1  )) }
    #[inline(always)] fn moccasin    () -> Self { Self::from(Rgb24::from(0xffe4b5  )) }
    #[inline(always)] fn navajowhite () -> Self { Self::from(Rgb24::from(0xffdead  )) }
    #[inline(always)] fn oldlace () -> Self { Self::from(Rgb24::from(0xfdf5e6  )) }
    #[inline(always)] fn olivedrab   () -> Self { Self::from(Rgb24::from(0x6b8e23  )) }
    #[inline(always)] fn orangered   () -> Self { Self::from(Rgb24::from(0xff4500  )) }
    #[inline(always)] fn orchid  () -> Self { Self::from(Rgb24::from(0xda70d6  )) }
    #[inline(always)] fn palegoldenrod   () -> Self { Self::from(Rgb24::from(0xeee8aa  )) }
    #[inline(always)] fn palegreen   () -> Self { Self::from(Rgb24::from(0x98fb98  )) }
    #[inline(always)] fn paleturquoise   () -> Self { Self::from(Rgb24::from(0xafeeee  )) }
    #[inline(always)] fn palevioletred   () -> Self { Self::from(Rgb24::from(0xdb7093  )) }
    #[inline(always)] fn papayawhip  () -> Self { Self::from(Rgb24::from(0xffefd5  )) }
    #[inline(always)] fn peachpuff   () -> Self { Self::from(Rgb24::from(0xffdab9  )) }
    #[inline(always)] fn peru    () -> Self { Self::from(Rgb24::from(0xcd853f  )) }
    #[inline(always)] fn pink    () -> Self { Self::from(Rgb24::from(0xffc0cb     )) }
    #[inline(always)] fn plum    () -> Self { Self::from(Rgb24::from(0xdda0dd     )) }
    #[inline(always)] fn powderblue () -> Self { Self::from(Rgb24::from(0xb0e0e6  )) }
    #[inline(always)] fn rosybrown  () -> Self { Self::from(Rgb24::from(0xbc8f8f  )) }
    #[inline(always)] fn royalblue   () -> Self { Self::from(Rgb24::from(0x4169e1  )) }
    #[inline(always)] fn saddlebrown () -> Self { Self::from(Rgb24::from(0x8b4513  )) }
    #[inline(always)] fn salmon  () -> Self { Self::from(Rgb24::from(0xfa8072  )) }
    #[inline(always)] fn sandybrown  () -> Self { Self::from(Rgb24::from(0xf4a460  )) }
    #[inline(always)] fn seagreen    () -> Self { Self::from(Rgb24::from(0x2e8b57  )) }
    #[inline(always)] fn seashell    () -> Self { Self::from(Rgb24::from(0xfff5ee  )) }
    #[inline(always)] fn sienna  () -> Self { Self::from(Rgb24::from(0xa0522d  )) }
    #[inline(always)] fn skyblue () -> Self { Self::from(Rgb24::from(0x87ceeb  )) }
    #[inline(always)] fn slateblue   () -> Self { Self::from(Rgb24::from(0x6a5acd  )) }
    #[inline(always)] fn slategray   () -> Self { Self::from(Rgb24::from(0x708090  )) }
    #[inline(always)] fn slategrey   () -> Self { Self::from(Rgb24::from(0x708090  )) }
    #[inline(always)] fn snow    () -> Self { Self::from(Rgb24::from(0xfffafa  )) }
    #[inline(always)] fn springgreen () -> Self { Self::from(Rgb24::from(0x00ff7f  )) }
    #[inline(always)] fn steelblue   () -> Self { Self::from(Rgb24::from(0x4682b4  )) }
    #[inline(always)] fn tan () -> Self { Self::from(Rgb24::from(0xd2b48c  )) }
    #[inline(always)] fn thistle () -> Self { Self::from(Rgb24::from(0xd8bfd8  )) }
    #[inline(always)] fn tomato  () -> Self { Self::from(Rgb24::from(0xff6347  )) }
    #[inline(always)] fn turquoise   () -> Self { Self::from(Rgb24::from(0x40e0d0  )) }
    #[inline(always)] fn violet  () -> Self { Self::from(Rgb24::from(0xee82ee  )) }
    #[inline(always)] fn wheat   () -> Self { Self::from(Rgb24::from(0xf5deb3  )) }
    #[inline(always)] fn whitesmoke  () -> Self { Self::from(Rgb24::from(0xf5f5f5    )) }
    #[inline(always)] fn yellowgreen   () -> Self { Self::from(Rgb24::from(0x9acd32  )) }
    #[inline(always)] fn rebeccapurple   () -> Self { Self::from(Rgb24::from(0x663399)) }
}
