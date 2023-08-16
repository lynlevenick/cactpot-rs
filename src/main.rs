mod cactpot;

use cactpot::{Cactpot, Recommendation};
use eyre::{eyre, Result};
use rustyline::Editor;

fn parse_coords(input: &str) -> Result<usize> {
    let mut coords = [Default::default(); 2];
    let mut seen_splits = 0;
    for split in input.split(',').map(|s| s.trim()) {
        coords[seen_splits] = split.parse::<usize>()?;
        seen_splits += 1;
        if seen_splits > 2 {
            return Err(eyre!("provide two coordinates"));
        }
    }
    if seen_splits < 2 {
        return Err(eyre!("provide two coordinates"));
    }

    let coords_idx = coords[0] + coords[1] * 3 - 4;
    if coords_idx > 8 {
        return Err(eyre!("coords must be between 1,1 and 3,3"));
    }

    Ok(coords_idx)
}
fn parse_value(input: &str) -> Result<u8> {
    let value: u8 = input.parse()?;
    if value < 1 || value > 9 {
        return Err(eyre!("value must be 1-9"));
    }

    Ok(value)
}

fn read_repeatedly<H: rustyline::Helper, R, F: Fn(&str) -> Result<R>>(
    rl: &mut Editor<H>,
    prompt: &str,
    f: F,
) -> Result<R> {
    loop {
        let resp = rl.readline(prompt)?;
        match f(&resp) {
            Ok(result) => return Ok(result),
            Err(e) => println!("invalid response: {}", e),
        }
    }
}

fn main() -> Result<()> {
    let mut cactpot = Cactpot::new();
    let mut rl = Editor::<()>::new();

    cactpot.show(&[])?;
    let coords = read_repeatedly(&mut rl, "where is the pre-revealed space? ", parse_coords)?;
    let value = read_repeatedly(&mut rl, "what is its value? ", parse_value)?;
    cactpot.set(coords, value)?;

    for _ in 0..3 {
        let (_, coords_arr) = cactpot.solve();
        if let Recommendation::Coordinates(coords_arr) = coords_arr {
            cactpot.show(&coords_arr)?;
            let coords = if coords_arr.len() == 1 {
                coords_arr[0]
            } else {
                read_repeatedly(&mut rl, "where did you move? ", parse_coords)?
            };
            let value = read_repeatedly(&mut rl, "what was its value? ", parse_value)?;
            cactpot.set(coords, value)?;
        } else {
            return Err(eyre!("programmer error"));
        }
    }

    if let (value, Recommendation::Rows(rows_arr)) = cactpot.solve() {
        cactpot.show(
            &rows_arr
                .iter()
                .flat_map(|row| cactpot::ROWS[*row].iter().copied())
                .collect::<Vec<_>>(),
        )?;
        println!("Expected {} MGP", value);
    } else {
        return Err(eyre!("programmer error"));
    }

    Ok(())
}
