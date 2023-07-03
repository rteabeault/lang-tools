use std::cmp::Ordering;
use std::str::SplitWhitespace;

use log::trace;
use rational::Rational;

/// Given two strings we want to iterate over them proportionally.
/// The source and target string are split on white space.
/// Each call of next will return a tuple where the first element is the next word from the
/// source string and the second element is a Vector of 0 or more words from the target word.
#[derive(Debug)]
pub struct ProportionalIter<'a> {
    source_iter: SplitWhitespace<'a>,
    target_iter: SplitWhitespace<'a>,
    floor: i128,
    floor_underage: Rational,
    ceiling: i128,
    ceiling_overage: Rational,
    overage: Rational,
}

impl<'a> ProportionalIter<'a> {
    pub fn new(
        source: &'a str,
        target: &'a str,
    ) -> Self {
        let source_iter = source.split_whitespace();
        let target_iter = target.split_whitespace();

        let source_len = source_iter.clone().count();
        let target_len = target_iter.clone().count();

        let diff_len: isize = target_len as isize - source_len as isize;

        let (floor, floor_underage) =
            Rational::new(diff_len as i128, source_len as i128).mixed_fraction();

        let (ceiling, ceiling_overage) = match floor_underage {
            // If the floor underage is 0 then we set the ceiling to be the same value as the floor
            // and the ceiling overage to 0. Otherwise we add 1 to the floor to get the ceiling
            // get the ceiling overage from 1 - floor_underage.
            r if r == Rational::zero() => (floor, Rational::zero()),
            _ => {
                if diff_len > 0 {
                    (floor + 1, Rational::one() - floor_underage)
                } else {
                    (floor - 1, -Rational::one() - floor_underage)
                }
            }
        };

        trace!("--------------------------------");
        trace!("Source length: {}", source_len);
        trace!("Target length: {}", target_len);
        trace!("Length diff: {}", diff_len);
        trace!("Floor: {}", floor);
        trace!("Floor underage: {}", floor_underage);
        trace!("Ceiling: {}", ceiling);
        trace!("Ceiling overage: {}", ceiling_overage);

        ProportionalIter {
            source_iter,
            target_iter,
            floor,
            floor_underage,
            ceiling,
            ceiling_overage,
            overage: Rational::zero(),
        }
    }

    fn get_next_target_words(&mut self) -> i128 {
        trace!("Getting num target words");
        trace!("Current overage is {:?}", self.overage);
        match self.overage.cmp(&Rational::zero()) {
            Ordering::Equal => {
                self.overage += self.floor_underage;
                1 + self.floor
            }
            Ordering::Greater => {
                if self.overage - self.ceiling_overage >= Rational::zero() {
                    trace!("Enough overage has accumulated to round up to ceiling");
                    self.overage -= self.ceiling_overage;
                    1 + self.ceiling
                } else {
                    trace!("Not enough overage has accumulated to round up to ceiling.");
                    self.overage += self.floor_underage;
                    1 + self.floor
                }
            }
            Ordering::Less => {
                if self.overage - self.ceiling_overage <= Rational::zero() {
                    trace!("Enough overage has accumulated to round up to ceiling");
                    self.overage -= self.ceiling_overage;
                    1 + self.ceiling
                } else {
                    trace!("Not enough overage has accumulated to round up to ceiling.");
                    self.overage += self.floor_underage;
                    1 + self.floor
                }
            }
        }
    }
}

impl<'a> Iterator for ProportionalIter<'a> {
    type Item = (&'a str, Vec<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.source_iter.next() {
            Some(source) => {
                let num_target_words = self.get_next_target_words();
                let target: Vec<&str> = self.target_iter
                    .by_ref()
                    .take(num_target_words as usize)
                    .collect();

                assert_eq!(target.len() as i128, num_target_words,
                           "Not enough target words found! {:?}", self);

                Some((source, target))
            }
            None => {
                let next_target = self.target_iter.next();

                assert_eq!(next_target, None,
                           "Source words exhausted yet target words remain {}. {:?}",
                           next_target.unwrap(), self);

                assert_eq!(self.overage, Rational::zero(),
                           "Bookkeeping while iterating is broken. {:?}", self);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test two sentences of equal size
    //
    // Source length: 4
    // Target length: 4
    // Length diff: 0
    // Floor: 0
    // Floor underage: 0/1
    // Ceiling: 0
    // Ceiling overage: 0/1
    //
    // Expected target word counts:
    // source word   target words  current overage
    // 1             1             0
    // 2             1             0
    // 3             1             0
    // 4             1             0
    #[test]
    fn zipped_source_size_equals_target_size() {
        let mut iter = ProportionalIter::new(
            "one two three four",
            "one' two' three' four'",
        );

        assert_eq!(iter.next(), Some(("one", vec!["one'"])));
        assert_eq!(iter.next(), Some(("two", vec!["two'"])));
        assert_eq!(iter.next(), Some(("three", vec!["three'"])));
        assert_eq!(iter.next(), Some(("four", vec!["four'"])));
        assert_eq!(iter.next(), None);
    }

    // Source is length 1 and target is length 2.
    //
    // Source length: 1
    // Target length: 2
    // Length diff: 1
    // Floor: 1
    // Floor underage: 0/1
    // Ceiling: 1
    // Ceiling overage: 0/1
    //
    // Expected target word counts:
    // source word   target words  current overage
    // 1             2             0
    #[test]
    fn zipped_source_one_target_two() {
        let mut iter = ProportionalIter::new(
            "one",
            "one' two'",
        );

        assert_eq!(iter.next(), Some(("one", vec!["one'", "two'"])));
        assert_eq!(iter.next(), None);
    }

    // Source is length 2 and target is length 4.
    //
    // Source length: 2
    // Target length: 4
    // Length diff: 2
    // Floor: 1
    // Floor underage: 0/1
    // Ceiling: 1
    // Ceiling overage: 0/1
    //
    // source word   target words  current overage
    // 1             2             0
    // 2             2             0
    #[test]
    fn zipped_source_half_target() {
        let mut iter = ProportionalIter::new(
            "one two",
            "one' two' three' four'"
        );

        assert_eq!(iter.next(), Some(("one", vec!["one'", "two'"])));
        assert_eq!(iter.next(), Some(("two", vec!["three'", "four'"])));
        assert_eq!(iter.next(), None);
    }

    // Source is length 5 and target is length 7.
    //
    // Source length: 5
    // Target length: 7
    // Length diff: 2
    // Floor: 0
    // Floor underage: 2/5
    // Ceiling: 1
    // Ceiling overage: 3/5
    //
    // source word   target words  current overage
    // 1             1             2/5
    // 2             1             4/5
    // 3             2             1/5
    // 4             1             3/5
    // 5             2             0
    #[test]
    fn zipped_source_five_target_two() {
        let mut iter = ProportionalIter::new(
            "one two three four five",
            "one' two' three' four' five' six' seven'",
        );

        assert_eq!(iter.next(), Some(("one", vec!["one'"])));
        assert_eq!(iter.next(), Some(("two", vec!["two'"])));
        assert_eq!(iter.next(), Some(("three", vec!["three'", "four'"])));
        assert_eq!(iter.next(), Some(("four", vec!["five'"])));
        assert_eq!(iter.next(), Some(("five", vec!["six'", "seven'"])));
        assert_eq!(iter.next(), None);
    }

    // Source is length 4 and target is length 7.
    //
    // Source length: 4
    // Target length: 7
    // Length diff: 3
    // Floor: 0
    // Floor underage: 3/4
    // Ceiling: 1
    // Ceiling overage: 1/4
    //
    // source word   target words  current overage
    // 1             1             3/4
    // 2             2             2/4
    // 3             2             1/4
    // 4             2             0
    #[test]
    fn zipped_source_four_target_seven() {
        let mut iter = ProportionalIter::new(
            "one two three four",
            "one' two' three' four' five' six' seven'",
        );

        assert_eq!(iter.next(), Some(("one", vec!["one'"])));
        assert_eq!(iter.next(), Some(("two", vec!["two'", "three'"])));
        assert_eq!(iter.next(), Some(("three", vec!["four'", "five'"])));
        assert_eq!(iter.next(), Some(("four", vec!["six'", "seven'"])));
        assert_eq!(iter.next(), None);
    }

    // source: 10
    // target: 7
    // diff: -3
    //
    // diff / source = 3 / 10
    //
    // ceiling: -1
    // ceiling_overrage = -7 / 10
    // floor  : 0
    // floor_underage   = -3 / 10
    //
    // 1 + floor to start
    //
    // 1 -> 1 => -3/10
    // 2 -> 1 => -6/10
    // 3 -> 1 => -9/10
    // 4 -> 0 => -2/10
    // 5 -> 1 => -5/10
    // 6 -> 1 => -8/10
    // 7 -> 0 => -1/10
    // 8 -> 1 => -4/10
    // 9 -> 1 => -7/10
    // 10 -> 0 => 0
    #[test]
    fn zipped_source_ten_target_seven() {
        let mut iter = ProportionalIter::new(
            "one two three four five six seven eight nine ten",
            "one' two' three' four' five' six' seven'",
        );

        assert_eq!(iter.next(), Some(("one", vec!["one'"])));
        assert_eq!(iter.next(), Some(("two", vec!["two'"])));
        assert_eq!(iter.next(), Some(("three", vec!["three'"])));
        assert_eq!(iter.next(), Some(("four", vec![])));
        assert_eq!(iter.next(), Some(("five", vec!["four'"])));
        assert_eq!(iter.next(), Some(("six", vec!["five'"])));
        assert_eq!(iter.next(), Some(("seven", vec![])));
        assert_eq!(iter.next(), Some(("eight", vec!["six'"])));
        assert_eq!(iter.next(), Some(("nine", vec!["seven'"])));
        assert_eq!(iter.next(), Some(("ten", vec![])));
        assert_eq!(iter.next(), None);
    }
}
