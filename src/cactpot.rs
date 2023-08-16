use collect_slice::CollectSlice;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::borrow::Cow;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cactpot {
    board: [u8; 9],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Recommendation<'a> {
    Coordinates(Cow<'a, [usize]>),
    Rows(Cow<'a, [usize]>),
}

static OPENING_0: &[usize] = &[0, 2, 6, 8];

static OPENING_BOOK: &[[(f64, &[usize]); 9]; 9] = &[
    [
        (1677.7854166666664, &[2, 6]),
        (1665.8127976190476, &[2, 6]),
        (1662.504761904762, &[2, 6]),
        (1365.0047619047618, &[4]),
        (1359.5589285714286, &[4]),
        (1364.3044642857142, &[4]),
        (1454.5455357142855, &[4]),
        (1527.0875, &[2, 4, 6]),
        (1517.7214285714285, &[2, 4, 6]),
    ],
    [
        (1411.3541666666665, &[4]),
        (1414.9401785714288, &[4]),
        (1406.4190476190477, &[4]),
        (1443.3062499999999, &[6, 8]),
        (1444.3172619047618, &[4, 6, 8]),
        (1441.3663690476192, &[4]),
        (1485.6839285714286, &[4]),
        (1512.927976190476, &[0, 2]),
        (1518.466369047619, &[0, 2]),
    ],
    [
        (1677.7854166666664, &[0, 8]),
        (1665.8127976190476, &[0, 8]),
        (1662.504761904762, &[0, 8]),
        (1365.0047619047618, &[4]),
        (1359.5589285714286, &[4]),
        (1364.3044642857142, &[4]),
        (1454.5455357142855, &[4]),
        (1527.0875, &[0, 4, 8]),
        (1517.7214285714285, &[0, 4, 8]),
    ],
    [
        (1411.3541666666665, &[4]),
        (1414.9401785714288, &[4]),
        (1406.4190476190477, &[4]),
        (1443.3062499999999, &[2, 8]),
        (1444.3172619047618, &[2, 4, 8]),
        (1441.3663690476192, &[4]),
        (1485.6839285714286, &[4]),
        (1512.927976190476, &[0, 6]),
        (1518.466369047619, &[0, 6]),
    ],
    [
        (1860.4401785714285, &[0, 2, 6, 8]),
        (1832.5413690476191, &[0, 2, 6, 8]),
        (1834.179761904762, &[0, 2, 6, 8]),
        (1171.9669642857143, &[0, 2, 6, 8]),
        (1176.2047619047619, &[0, 2, 6, 8]),
        (1234.6142857142856, &[0, 2, 6, 8]),
        (1427.3583333333333, &[0, 2, 6, 8]),
        (1544.7607142857144, &[0, 2, 6, 8]),
        (1509.197619047619, &[0, 2, 6, 8]),
    ],
    [
        (1411.3541666666665, &[4]),
        (1414.9401785714288, &[4]),
        (1406.4190476190477, &[4]),
        (1443.3062499999999, &[0, 6]),
        (1444.3172619047618, &[0, 4, 6]),
        (1441.3663690476192, &[4]),
        (1485.6839285714286, &[4]),
        (1512.927976190476, &[2, 8]),
        (1518.466369047619, &[2, 8]),
    ],
    [
        (1677.7854166666664, &[0, 8]),
        (1665.8127976190476, &[0, 8]),
        (1662.504761904762, &[0, 8]),
        (1365.0047619047618, &[4]),
        (1359.5589285714286, &[4]),
        (1364.3044642857142, &[4]),
        (1454.5455357142855, &[4]),
        (1527.0875, &[0, 4, 8]),
        (1517.7214285714285, &[0, 4, 8]),
    ],
    [
        (1411.3541666666665, &[4]),
        (1414.9401785714288, &[4]),
        (1406.4190476190477, &[4]),
        (1443.3062499999999, &[0, 2]),
        (1444.3172619047618, &[0, 2, 4]),
        (1441.3663690476192, &[4]),
        (1485.6839285714286, &[4]),
        (1512.927976190476, &[6, 8]),
        (1518.466369047619, &[6, 8]),
    ],
    [
        (1677.7854166666664, &[2, 6]),
        (1665.8127976190476, &[2, 6]),
        (1662.504761904762, &[2, 6]),
        (1365.0047619047618, &[4]),
        (1359.5589285714286, &[4]),
        (1364.3044642857142, &[4]),
        (1454.5455357142855, &[4]),
        (1527.0875, &[2, 4, 6]),
        (1517.7214285714285, &[2, 4, 6]),
    ],
];

pub static ROWS: &[[usize; 3]; 8] = &[
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

const fn payout(sum: u8) -> usize {
    match sum {
        6 => 10000,
        7 => 36,
        8 => 720,
        9 => 360,
        10 => 80,
        11 => 252,
        12 => 108,
        13 => 72,
        14 => 54,
        15 => 180,
        16 => 72,
        17 => 180,
        18 => 119,
        19 => 36,
        20 => 306,
        21 => 1080,
        22 => 144,
        23 => 1800,
        24 => 3600,
        _ => 0,
    }
}

impl Cactpot {
    pub const fn new() -> Self {
        Cactpot { board: [0; 9] }
    }

    pub fn set(&mut self, coords: usize, value: u8) -> Result<()> {
        if coords >= self.board.len() {
            return Err(eyre!("invalid coords"));
        }
        if value > 9 {
            return Err(eyre!("values must be 0-9"));
        }
        if self
            .visible_values()
            .any(|visible_value| visible_value == value)
        {
            return Err(eyre!("values must be unique"));
        }

        self.board[coords] = value;
        Ok(())
    }

    pub fn solve(&self) -> (f64, Recommendation<'static>) {
        let mut hidden_indices = [Default::default(); 9];
        let hidden_indices_len = self.hidden_indices().collect_slice(&mut hidden_indices);
        let hidden_indices = &hidden_indices[0..hidden_indices_len];
        if hidden_indices_len >= 9 {
            (0.0, Recommendation::Coordinates(OPENING_0.into()))
        } else if hidden_indices_len == 8 {
            let visible_index = self.visible_indices().next().unwrap();
            let visible_value = self.visible_values().next().unwrap();
            let (value, spaces) = OPENING_BOOK[visible_index][visible_value as usize - 1];
            (value, Recommendation::Coordinates(spaces.into()))
        } else {
            let mut cactpot_copy = *self;

            if hidden_indices_len <= 5 {
                let mut num_permutations = 0;
                let mut totals = [0; 8];

                for permutation in self.hidden_values().permutations(hidden_indices.len()) {
                    num_permutations += 1;
                    for (idx, coords) in hidden_indices.iter().enumerate() {
                        cactpot_copy.board[*coords] = permutation[idx];
                    }

                    for (idx, coords_arr) in ROWS.iter().enumerate() {
                        totals[idx] += cactpot_copy.row_value(coords_arr);
                    }
                }

                let max = *totals.iter().max().unwrap();

                (
                    (max as f64) / num_permutations as f64,
                    Recommendation::Rows(
                        totals
                            .iter()
                            .enumerate()
                            .filter(|(_, value)| **value == max)
                            .map(|(idx, _)| idx)
                            .collect(),
                    ),
                )
            } else {
                let mut hidden_values = [Default::default(); 9];
                let hidden_values_len = self.hidden_values().collect_slice(&mut hidden_values);
                let hidden_values = &hidden_values[0..hidden_values_len];

                let mut totals = [0.0; 9];

                for coords in hidden_indices.into_iter() {
                    totals[*coords] = hidden_values
                        .iter()
                        .map(|value| {
                            cactpot_copy.board[*coords] = *value;
                            let result = cactpot_copy.solve().0;
                            cactpot_copy.board[*coords] = 0;
                            result
                        })
                        .sum();
                }

                let max = totals.iter().copied().fold(0.0, f64::max);

                (
                    max / hidden_indices_len as f64,
                    Recommendation::Coordinates(
                        totals
                            .iter()
                            .enumerate()
                            .filter(|(_, value)| (max - **value).abs() < 0.00001)
                            .map(|(idx, _)| idx)
                            .collect(),
                    ),
                )
            }
        }
    }

    pub fn show(&self, highlighted_spaces: &[usize]) -> Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);

        for y in 0..3 {
            for x in 0..3 {
                if highlighted_spaces
                    .iter()
                    .any(|coords| coords % 3 == x && coords / 3 == y)
                {
                    stdout
                        .set_color(&ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
                }

                write!(stdout, "\t{}", Self::square_to_ch(self.board[x + y * 3]))?;

                stdout.reset()?;
            }

            writeln!(stdout)?;
            for x in 0..3 {
                if highlighted_spaces
                    .iter()
                    .any(|coords| coords % 3 == x && coords / 3 == y)
                {
                    stdout
                        .set_color(&ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
                }

                write!(stdout, "\t {}, {}", x + 1, y + 1)?;

                stdout.reset()?;
            }

            writeln!(stdout)?;
            if y < 2 {
                writeln!(stdout)?;
            }
        }

        Ok(())
    }

    const fn square_to_ch(square: u8) -> char {
        match square {
            0 => '-',
            square => (b'0' + square) as char,
        }
    }

    /* const */

    fn row_value(&self, coords_arr: &[usize; 3]) -> usize {
        return payout(coords_arr.iter().map(|coords| self.board[*coords]).sum());
    }

    fn hidden_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(idx, value)| if *value == 0 { Some(idx) } else { None })
    }

    fn hidden_values(&self) -> impl Iterator<Item = u8> {
        let mut unseen = [true; 9];
        for value in self.board.iter() {
            if *value > 0 {
                unseen[*value as usize - 1] = false;
            }
        }
        (0u8..9).filter_map(move |value| {
            if unseen[value as usize] {
                Some(value + 1)
            } else {
                None
            }
        })
    }

    fn visible_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(idx, value)| if *value != 0 { Some(idx) } else { None })
    }

    fn visible_values(&self) -> impl Iterator<Item = u8> + '_ {
        self.board
            .iter()
            .filter_map(|value| if *value != 0 { Some(value) } else { None })
            .copied()
    }
}
