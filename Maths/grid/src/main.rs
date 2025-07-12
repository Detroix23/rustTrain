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
/// UI - Grid visualisation
pub struct UiTiles<'a> {
    on: &'a str,
    off: &'a str,
    void: &'a str,
}
/// UI - Grid to print chars
pub const UI_TILES: UiTiles = UiTiles {
    on: "#",
    off: "0",
    void: "."
};

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
    // Find maximums
    let mut max_x: i32 = 0;
    let mut min_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut min_y: i32 = 0;
    for tile in &grid.tiles {
        if tile.x > max_x {max_x = tile.x;}
        else if tile.x < min_x {min_x = tile.x;}
        if tile.y > max_y {max_y = tile.y;}
        else if tile.y < min_y {min_y = tile.y;}
    }
    let size_x: i32 = min_x.abs() + max_x.abs();
    let size_y: i32 = min_y.abs() + max_y.abs();
    println!("- Max: x={}, y={}; Min: x={}, y={};", max_x, max_y, min_x, min_y);
    println!("- Size: x={}, y={}; Center: x={}, y={}", size_x, size_y, size_x / 2, size_y / 2);
    // Iterate through the grid
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            //print!("({}; {})", x, y);
            // Check the tiles
            let mut found: bool = false;
            let mut i: usize = 0;
            while !found && i < grid.tiles.len() {
                let tile = &grid.tiles[i];
                if tile.x == x as i32 && tile.y == y as i32 {
                    if tile.state {
                        print!("{}", UI_TILES.on);
                    } else {
                        print!("{}", UI_TILES.off);
                    }
                    found = true;
                }
                i += 1;
            }
            if !found {
                print!("{}", UI_TILES.void);
            }
        }
        println!();
    }
}

fn main() {
    let tiles_to_add: Vec<Tile> = vec![
            Tile{x: 0, y: 0, state: true},
            Tile{x: 0, y: 1, state: true},
            Tile{x: 0, y: 1, state: true},
            Tile{x: 0, y: -1, state: true},
    ];
    let grid_cartesian1: Grid = Grid {
        div: String::from("1sq"),
        tiles: add_tiles(tiles_to_add),
    };
    println!("Grid 1: {:?}", grid_cartesian1);
    grid_inline(grid_cartesian1);

    println!("Grid 2");
    let mut grid_cartesian2: Grid = grid_init(String::from("1sq"), GRID_SIZE, false);
    grid_cartesian2.tiles.remove(34);
    grid_inline(grid_cartesian2);
}