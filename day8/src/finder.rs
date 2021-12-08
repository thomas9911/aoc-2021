use std::collections::{BTreeMap, BTreeSet};

use derive_more::{Deref, DerefMut};
use lazy_static::lazy_static;
use maplit::{btreemap, btreeset};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Segments {
    Top,
    TopRight,
    Middle,
    BottomRight,
    Bottom,
    BottomLeft,
    TopLeft,
}

lazy_static! {
    static ref DIGITS: BTreeMap<usize, BTreeSet<Segments>> = {
        use Segments::*;
        btreemap! {
            0 => btreeset!{
                Top,
                TopRight,
                BottomRight,
                Bottom,
                BottomLeft,
                TopLeft,
            },
            1 => btreeset!{
                TopRight,
                BottomRight
            },
            2 => btreeset!{
                Top,
                TopRight,
                Middle,
                Bottom,
                BottomLeft,
            },
            3 => btreeset!{
                Top,
                TopRight,
                Middle,
                BottomRight,
                Bottom,
            },
            4 => btreeset!{
                TopRight,
                Middle,
                BottomRight,
                TopLeft,
            },
            5 => btreeset!{
                Top,
                Middle,
                BottomRight,
                Bottom,
                TopLeft,
            },
            6 => btreeset!{
                Top,
                Middle,
                BottomRight,
                Bottom,
                BottomLeft,
                TopLeft,
            },
            7 => btreeset!{
                Top,
                TopRight,
                BottomRight,
            },
            8 => btreeset!{
                Top,
                TopRight,
                Middle,
                BottomRight,
                Bottom,
                BottomLeft,
                TopLeft,
            },
            9 => btreeset!{
                Top,
                TopRight,
                Middle,
                BottomRight,
                Bottom,
                TopLeft,
            },
        }
    };
}

#[derive(Debug, Deref, DerefMut)]
pub struct Finder {
    digits: BTreeMap<usize, BTreeSet<Segments>>,
}

impl Finder {
    pub fn new() -> Finder {
        Finder {
            digits: DIGITS.clone(),
        }
    }

    pub fn digits_that_contain_segment(&self, segment: &Segments) -> BTreeSet<usize> {
        self.digits_that_contain_any_segments(&[*segment])
    }

    pub fn digits_that_contain_any_segments(
        &self,
        contain_segments: &[Segments],
    ) -> BTreeSet<usize> {
        let mut set = BTreeSet::new();
        for (digit, segments) in self.iter() {
            if contain_segments.iter().any(|seg| segments.contains(seg)) {
                set.insert(*digit);
            }
        }
        set
    }

    pub fn digits_that_contain_all_segments(
        &self,
        contain_segments: &[Segments],
    ) -> BTreeSet<usize> {
        let mut set = BTreeSet::new();
        for (digit, segments) in self.iter() {
            if contain_segments.iter().all(|seg| segments.contains(seg)) {
                set.insert(*digit);
            }
        }
        set
    }

    pub fn find_match(&self, segments: &BTreeSet<Segments>) -> Option<usize> {
        for (digit, set) in self.digits.iter() {
            if set == segments {
                return Some(*digit);
            }
        }

        None
    }

    pub fn digits_that_have_amount_of_segments<'a>(
        &'a self,
        amount: usize,
    ) -> Box<dyn Iterator<Item = (&usize, &BTreeSet<Segments>)> + 'a> {
        Box::new(self.iter().filter(move |x| x.1.len() == amount))
    }
}

#[cfg(test)]
mod finder_tests {
    use super::{btreeset, Finder, Segments};

    #[test]
    fn digits_that_contain_any_segments() {
        let finder = Finder::new();
        let a =
            finder.digits_that_contain_any_segments(&[Segments::TopRight, Segments::BottomLeft]);

        let expected = btreeset! {0, 1, 2, 3, 4, 6, 7, 8, 9};
        assert_eq!(expected, a);
    }

    #[test]
    fn digits_that_contain_all_segments() {
        let finder = Finder::new();
        let a =
            finder.digits_that_contain_all_segments(&[Segments::TopRight, Segments::BottomLeft]);

        let expected = btreeset! {0,2,8};
        assert_eq!(expected, a);
    }

    #[test]
    fn digits_that_have_amount_of_segments_3() {
        let finder = Finder::new();
        let b: Vec<_> = finder
            .digits_that_have_amount_of_segments(3)
            .map(|x| x.0)
            .collect();

        assert_eq!(vec![&7], b)
    }

    #[test]
    fn digits_that_have_amount_of_segments_5() {
        let finder = Finder::new();
        let b: Vec<_> = finder
            .digits_that_have_amount_of_segments(5)
            .map(|x| x.0)
            .collect();

        assert_eq!(vec![&2, &3, &5], b)
    }

    #[test]
    fn find_match_found() {
        let finder = Finder::new();
        let set = btreeset! {Segments::TopRight, Segments::BottomRight, Segments::Top, Segments::Middle, Segments::Bottom};
        let c = finder.find_match(&set);

        assert_eq!(c, Some(3))
    }

    #[test]
    fn find_match_not_found() {
        let finder = Finder::new();
        let set = btreeset! {Segments::TopRight, Segments::BottomRight, Segments::Top, Segments::Middle, Segments::Bottom, Segments::BottomLeft};
        let c = finder.find_match(&set);

        assert_eq!(c, None)
    }
}

#[test]
fn check_segments_of_digits() {
    for (digit, segments) in DIGITS.iter() {
        let valid = match digit {
            0 => segments.len() == 6,
            1 => segments.len() == 2,
            2 => segments.len() == 5,
            3 => segments.len() == 5,
            4 => segments.len() == 4,
            5 => segments.len() == 5,
            6 => segments.len() == 6,
            7 => segments.len() == 3,
            8 => segments.len() == 7,
            9 => segments.len() == 6,
            _ => panic!("invalid digit"),
        };

        if !valid {
            panic!("segments of {} are invalid", digit)
        }
    }
}
