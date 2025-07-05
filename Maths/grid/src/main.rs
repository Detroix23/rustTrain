// GRID
// Main file for the Detoix's grid module.
// It allows multiple structured grid of various type.

#[derive(Debug)]
pub struct Tile {
    x: i32,
    y: i32,
    state: bool,
}

#[derive(Debug)]
pub struct Grid {
    div: String,
    tiles: Vec<Tile>,
}

// Constants
/// Fixed size means, if false, that going on a non-existant tile will generate a new; else, it will cancel the move.
/// Grid visualization will never generate new tiles.
pub const FIXED_SIZE: bool = true;
/// Grid size. All sides are equal
pub const GRID_SIZE: usize = 20;
/// String to be printed for ON tiles 
pub const TILE_STR_ON: &str = "#";
/// String to be printed for OFF tiles
pub const TILE_STR_OFF: &str = "0";
/// String to be printed when void (no tiles or grid-impossible)
pub const TILE_STR_VOID: &str = ".";

/// Create a grid
pub fn grid_init(div: String, size: usize, default_state: bool) -> Grid {
    let mut tiles: Vec<Tile> = Vec::new();
    for x in 0..size {
        for y in 0..size {
            tiles.push(Tile {
                x: x as i32,
                y: y as i32,
                state: default_state,
            });
        }
    }
    let grid: Grid = Grid {
        div,
        tiles,
    };

    grid
}

/// Use to push tiles into a grid. This function prevents to have multiple tiles with the same coord - the first will cancel any other.
pub fn add_tiles(tiles_in: Vec<Tile>) -> Vec<Tile> {
    let mut tiles_out: Vec<Tile> = Vec::new();
    for tile_in in tiles_in {
        let mut unique: bool = true;
        for tile_out in &tiles_out {
            unique = !(tile_out.x == tile_in.x && tile_out.y == tile_in.y);
        }
        if unique {
            tiles_out.push(tile_in);
        }
    }
    
    tiles_out
}

/// Debug grid visualization
fn grid_inline(grid: Grid) {
    let grid_size: usize = GRID_SIZE;
    // Iterate throught the grid
    for y in 0..grid_size {
        for x in 0..grid_size {
            //print!("({}; {})", x, y);
            // Check the tiles
            let mut found: bool = false;
            let mut i: usize = 0;
            while !found && i < grid.tiles.len() {
                let tile = &grid.tiles[i];
                if tile.x == x as i32 && tile.y == y as i32 {
                    if tile.state {
                        print!("{}", TILE_STR_ON);
                    } else {
                        print!("{}", TILE_STR_OFF);
                    }
                    found = true;
                }
                i += 1;
            }
            if !found {
                print!("{}", TILE_STR_VOID)
            }
        }
        println!();
    }
}

fn main() {
    let tiles_to_add: Vec<Tile> = vec![
            Tile{x: 0, y: 0, state: false},
            Tile{x: 0, y: 1, state: false},
            Tile{x: 0, y: 1, state: false},
            Tile{x: 0, y: -1, state: false},
    ];
    let grid_cartesian1: Grid = Grid {
        div: String::from("1sq"),
        tiles: add_tiles(tiles_to_add),
    };
    println!("The grid {:?}", grid_cartesian1);

    let mut grid_cartesian2: Grid = grid_init(String::from("1sq"), GRID_SIZE, false);
    grid_cartesian2.tiles.remove(34);
    grid_inline(grid_cartesian2);
}