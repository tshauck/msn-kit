// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use crate::spectrum::Spectrum;

#[derive(Debug)]
pub struct Matcher {
    tolerance: f64,
    shift: f64,
}

impl Default for Matcher {
    fn default() -> Self {
        Matcher {
            tolerance: 0.05,
            shift: 0.0,
        }
    }
}

impl Matcher {
    pub fn new(tolerance: f64, shift: f64) -> Self {
        Matcher { tolerance, shift }
    }

    pub fn find_matches(self, spec_one: Spectrum, spec_two: Spectrum) -> Vec<(i32, i32)> {
        let mut lowest_idx = 0;
        let spec_two_n = spec_two.mz.len();

        let mut matches = Vec::<(i32, i32)>::new();

        for (i, mz1) in spec_one.mz.iter().enumerate() {
            let low_bound = mz1 - self.tolerance;
            let high_bound = mz1 + self.tolerance;

            for j in lowest_idx..spec_two_n {
                let mz2 = spec_two.mz[j] + self.shift;
                if mz2 > high_bound {
                    break;
                } else if mz2 < low_bound {
                    lowest_idx = j;
                } else {
                    matches.push((i as i32, j as i32));
                }
            }
        }

        return matches;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::spectrum::Spectrum;

    #[test]
    fn test_matcher() {
        let test_table = vec![
            (0.1, 0.0, vec![1.0, 2.0], vec![0.0, 1.05], vec![(0, 1)]),
            (
                0.1,
                -5.0,
                vec![100.0, 200.0, 300.0, 500.0],
                vec![105.0, 205.1, 300.0, 304.99, 500.1],
                vec![(0, 0), (1, 1), (2, 3)],
            ),
        ];

        for (tolerance, shift, mz1, mz2, expected) in test_table.iter() {
            let matcher = Matcher::new(*tolerance, *shift);

            let mut spec1 = Spectrum::empty();
            spec1.mz = mz1.clone();

            let mut spec2 = Spectrum::empty();
            spec2.mz = mz2.clone();

            let matches = matcher.find_matches(spec1, spec2);

            assert_eq!(matches, expected.clone());
        }
    }
}
