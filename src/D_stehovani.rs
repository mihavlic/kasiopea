#![allow(non_snake_case)]

fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

const FULL: bool = false;
const EMPTY: bool = true;

fn main() {
    let T = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..T {
        let line = stdin_line();
        let mut numbers = line.split_whitespace();

        let R = numbers.next().unwrap().parse::<usize>().unwrap();
        let S = numbers.next().unwrap().parse::<usize>().unwrap();
        let K = numbers.next().unwrap().parse::<usize>().unwrap();

        // true means empty, false is blocked
        let mut grid: Box<[bool]> = vec![EMPTY; R * S].into_boxed_slice();

        let mut door = None;
        for y in 0..R {
            let line = stdin_line();
            let mut letters = line.chars();

            for x in 0..S {
                let letter = letters.next().unwrap();

                let cell = match letter {
                    '.' => EMPTY,
                    'X' => FULL,
                    'D' => {
                        door = Some((x, y));
                        // the door is the first cell on the stack, as such it is not marked as blocked automatically
                        FULL
                    },
                    _ => unreachable!("Invalid character")
                };

                grid[y*S + x] = cell;
            }
        }
        let door = door.expect("There must be a door");

        let mut stack: Vec<(i32, i32, u32)> = Vec::new();
        stack.push((door.0 as i32, door.1 as i32, 0));

        let mut levels = 1;
        let mut last_index = 0;
        while stack.len() < K {
            let prev_last_index = last_index;
            last_index = stack.len();

            for i in prev_last_index..last_index {
                let (x, y, _) = stack[i];
                let x = x as i32;
                let y = y as i32;
                // _ 1 _
                // 4 + 2
                // _ 3 _
                let mut grid_ref = (S as i32, R as i32, &mut *grid);
                check_cell(&mut stack, &mut grid_ref, levels, x,   y+1);
                check_cell(&mut stack, &mut grid_ref, levels, x+1, y);
                check_cell(&mut stack, &mut grid_ref, levels, x  , y-1);
                check_cell(&mut stack, &mut grid_ref, levels, x-1, y);
            }

            // no new cells are reachable ie. there is not enough space to fill with boxes
            if last_index == stack.len() {
                break;
            }

            levels += 1;
        }

        // not possible to store all boxes
        if stack.len() < K {
            println!("-1");
        } else {
            let mut total_steps = 0;
            for &(_x, _y, steps) in &stack[..K] {
                // must make steps there and back again
                total_steps += steps * 2;
            }
            println!("{}", total_steps);

            // debug print the populated grid with step values
            // this breaks down if you have u32::max levels in the grid, I think by that time you would start running out of memory
            // also formatting becomes ugly when the step count is more than 99
            // let mut grid = grid.iter().map(|&cell| if cell == FULL {u32::MAX} else {u32::MAX - 1}).collect::<Vec<_>>();
            // for &(x, y, steps) in &stack[..K] {
            //     grid[y as usize*S + x as usize] = steps;
            // }
            // for y in 0..R {
            //     for x in 0..S {
            //         let cell = grid[y*S + x];
    
            //         if cell == u32::MAX {
            //             print!(" X ")
            //         } else if cell == u32::MAX - 1 {
            //             print!(" . ");
            //         } else {
            //             print!("{:2} ", cell);
            //         }
            //     }
            //     println!();
            // }
        }

    }
}

fn check_cell(stack: &mut Vec<(i32, i32, u32)>, grid: &mut (i32, i32, &mut [bool]), steps: u32, x: i32, y: i32) {

    let S = grid.0;
    let R = grid.1;
    let grid = &mut grid.2;

    // out of bounds
    if x < 0 || x >= S || y < 0 || y >= R {
        return;
    }

    let cell = &mut grid[(y*S + x) as usize];

    // the cell already has a box on it
    if *cell == FULL {
        return;
    }
    // put a box on it
    *cell = FULL;

    // all is well, push the cell to the stack
    stack.push((x, y, steps));
}