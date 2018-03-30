use Rgb24;
#[cfg(any(feature="tables", feature="basic_table"))]
use Entry;

#[cfg(any(feature="tables", feature="basic_table"))]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const BASIC_COLORS : &[Entry] = &[
    Entry { ident:"black"  , value:hex24!(0x000000) },
    Entry { ident:"white"  , value:hex24!(0xffffff) },
    Entry { ident:"red"    , value:hex24!(0xff0000) },
    Entry { ident:"green"  , value:hex24!(0x00ff00) },
    Entry { ident:"blue"   , value:hex24!(0x0000ff) },
    Entry { ident:"cyan"   , value:hex24!(0x00ffff) },
    Entry { ident:"magenta", value:hex24!(0xff00ff) },
    Entry { ident:"yellow" , value:hex24!(0xffff00) },
];


#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait BasicColors : From<Rgb24> {
    #[inline(always)] fn black  () -> Self { Self::from(Rgb24::new(0x00,0x00,0x00)) }
    #[inline(always)] fn white  () -> Self { Self::from(Rgb24::new(0xff,0xff,0xff)) }
    #[inline(always)] fn red    () -> Self { Self::from(Rgb24::new(0xff,0x00,0x00)) }
    #[inline(always)] fn green  () -> Self { Self::from(Rgb24::new(0x00,0xff,0x00)) }
    #[inline(always)] fn blue   () -> Self { Self::from(Rgb24::new(0x00,0x00,0xff)) }
    #[inline(always)] fn cyan   () -> Self { Self::from(Rgb24::new(0x00,0xff,0xff)) }
    #[inline(always)] fn magenta() -> Self { Self::from(Rgb24::new(0xff,0x00,0xff)) }
    #[inline(always)] fn yellow () -> Self { Self::from(Rgb24::new(0xff,0xff,0x00)) }
}


#[cfg(any(feature="tables", feature="basic_table"))]
#[cfg(test)]
mod test {
    use super::Rgb24;
    use super::BASIC_COLORS;

    #[test]
    fn approx_search() {
        let threshold = 0x80;
        let searched = Rgb24::new(0x00, 0x80, 0x00);
        let mut cols: Vec<_> = BASIC_COLORS
            .iter()
            .filter(|e| e.value.is_near(&searched, threshold))
            .collect();
        cols.sort_by_key(|e| e.value);
        println!("Closest to {:?} with threshold {} :", searched, threshold);
        for c in cols.iter() {
            println!("{:?}", c);
        }
    }
    #[test]
    fn approx_search_rgb() {
        let threshold = Rgb24::new(0xff, 0x80, 0x00);
        let searched = Rgb24::new(0x00, 0x80, 0x00);
        let mut cols: Vec<_> = BASIC_COLORS
            .iter()
            .filter(|e| e.value.is_near_rgb(&searched, &threshold))
            .collect();
        cols.sort_by_key(|e| e.value);
        println!("Closest to {:?} with threshold {:?} :", searched, threshold);
        for c in cols.iter() {
            println!("{:?}", c);
        }
    }

    extern crate edit_distance;
    use self::edit_distance::edit_distance;

    #[test]
    fn approx_search_ident() {
        let dist = 3;
        let searched = "blac";
        let mut cols: Vec<_> = BASIC_COLORS
            .iter()
            .filter(|e| edit_distance(e.ident, searched) <= dist)
            .collect();
        cols.sort_by_key(|e| e.ident);
        println!("Closest to {} within Levenshtein distance {} :", searched, dist);
        for c in cols.iter() {
            println!("{:?}", c);
        }
    }

    #[test]
    fn sort() {
        let mut cols: Vec<_> = BASIC_COLORS.iter().collect();
        println!("Sorted by ident :");
        cols.sort_by_key(|e| e.ident);
        for c in cols.iter().take(3) {
            println!("{:?}", c);
        }
        println!("Sorted by value :");
        cols.sort_by_key(|e| e.value);
        for c in cols.iter().take(3) {
            println!("{:?}", c);
        }
        println!("Sorted by ident, reversed :");
        cols.sort_by(|b, a| a.ident.cmp(b.ident));
        for c in cols.iter().take(3) {
            println!("{:?}", c);
        }
    }
}
