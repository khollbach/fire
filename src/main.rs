use std::{thread, time::Duration};

use rand::Rng;

const ROWS: usize = 50;
const COLS: usize = 50;

const FIRE: char = '🔥';
const WOOD: char = '🪵';
const BURNED_OUT: char = ' ';
const _FIREWORK: char = '🎆';

const DELAY_MS: u64 = 300;

fn main() {
    let mut forest = Forest::all_wood();

    // The circle of life... ☯️
    loop {
        forest.burn();
        forest.grow();
    }
}

// (VERY DRY)
#[derive(Clone)]
struct Forest {
    pixels: Vec<Vec<char>>,
}

impl Forest {
    fn all_wood() -> Forest {
        let pixels = vec![vec![WOOD; COLS]; ROWS];
        Forest { pixels }
    }

    /// Return the number of steps it took to burn down the whole forest.
    ///
    /// Note that there may be a few trees that survived.
    fn burn(&mut self) -> usize {
        self.spark();
        self.display();

        for i in 0.. {
            if self.pixels_currently_on_fire().next().is_none() {
                return i;
            }

            self.burn_one_step();
            self.display();
        }

        unreachable!();
    }

    fn grow(&mut self, time_steps: usize) {
        todo!()
    }

    fn spark(&mut self) {
        // randomly choose a place to "light"
        let mut rng = rand::thread_rng();
        let r: usize = rng.gen_range(0..ROWS);
        let c: usize = rng.gen_range(0..COLS);
        self.pixels[r][c] = FIRE;
    }

    fn display(&self) {
        clear();

        for line in &self.pixels {
            let s: String = line
                .iter()
                .flat_map(|&c| if c == ' ' { vec![' ', ' '] } else { vec![c] })
                .collect();
            println!("{s}");
        }

        thread::sleep(Duration::from_millis(DELAY_MS));
    }

    fn burn_one_step(&mut self) {
        let old_forest = self.clone();

        for (r, c) in old_forest.pixels_currently_on_fire() {
            for (r2, c2) in nbrs(r as isize, c as isize) {
                let r2 = r2 as usize;
                let c2 = c2 as usize;

                if self.should_light(r2, c2) {
                    self.pixels[r2][c2] = FIRE;
                }
            }

            // maybe we burn out
            let p: f64 = rand::random(); // 0 <= p <= 1
            if p < 0.3 {
                self.pixels[r][c] = BURNED_OUT;
            }
        }
    }

    fn pixels_currently_on_fire(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        // for r in 0..ROWS {
        //     for c in 0..COLS {
        //         if self.pixels[r][c] == FIRE {
        //             // "return" it
        //             yield (r, c);
        //         } else {
        //             // don't return it
        //         }
        //     }
        // }

        (0..ROWS).flat_map(move |r| {
            (0..COLS).filter_map(move |c| {
                if self.pixels[r][c] == FIRE {
                    // "return" it
                    Some((r, c))
                } else {
                    // don't return it
                    None
                }
            })
        })
    }

    fn should_light(&self, r: usize, c: usize) -> bool {
        self.pixels[r][c] == WOOD && rand::random()
    }
}

fn nbrs(r: isize, c: isize) -> Vec<(isize, isize)> {
    let mut out = Vec::with_capacity(4);

    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let r2 = r + dr;
        let c2 = c + dc;

        if 0 <= r2 && r2 < ROWS as isize && 0 <= c2 && c2 < COLS as isize {
            out.push((r2, c2));
        }
    }

    out
}

fn clear() {
    // println!();
    // println!();
    // println!();
    let esc = 27 as char;
    print!("{esc}[2J{esc}[1;1H");
    print!("{esc}[2J");
}
