// LABYRINTH

use std::mem::discriminant;

// Imports
use grid::{self, grid_init, grid_inline, grid_state_tile, grid_update_tile, Grid, Tile, TileState, UiTiles};
use rand::{rng, seq::{self, IndexedRandom}, Rng};

// Vars
/// Private struct to locate the generator.
#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum OrdinalDirections {
    North,
    East,
    South,
    West,
}

/// UI - Visualisation.
pub const LABYRINTH_UI_TILES: UiTiles = UiTiles {
    on: "🟥",
    off: "⬜",
    void: "⬛"
};

/// # Labyrinth generator; memory based, no recursion.
/// Take random directions and saved the path in a vector. It "hits a wall" if the tile after wich its facing is a path. When stuck, go back one step reading its memory.
pub fn generator_random_memory_based(grid_size: usize) -> Grid {
    // Init
    let grid_default_state: bool = false;
    let grid_kind: String = String::from("1sq");
    let mut grid_labyrinth: Grid = grid_init(grid_kind, grid_size, grid_default_state);  

    // Start at the middle of the grid
    let mut generator_eye: Position = Position { x: grid_labyrinth.size.x as i32 / 2, y: grid_labyrinth.size.y as i32 / 2 };
    let mut generator_position: Position = Position { x: grid_labyrinth.size.x as i32 / 2, y: grid_labyrinth.size.y as i32 / 2 };
    let mut generator_path: Vec<Position> = vec![generator_position];
    // Generator, end when the generator has backed up totaly.
    while generator_path.len() > 0 {
        let mut offset_x: i32 = 0;
        let mut offset_y: i32 = 0;
        let mut good_path: bool = false;
        let mut available_directions: Vec<OrdinalDirections> = vec![OrdinalDirections::North, OrdinalDirections::East, OrdinalDirections::South, OrdinalDirections::West];

        // Expect to find a good path; if not, if all direction are blocked
        while !good_path && available_directions.len() > 0 {
            let direction: &OrdinalDirections  = available_directions.choose(&mut rand::rng()).expect("(!) - Something went wrong with the random choice");
            match direction {
                OrdinalDirections::North => offset_y = 1,
                OrdinalDirections::East => offset_x = 1,
                OrdinalDirections::South => offset_y = -1,
                OrdinalDirections::West => offset_x = -1,
            };
            generator_position = Position { x: generator_position.x + offset_x, y: generator_position.y + offset_y};
            generator_eye = Position {x: generator_eye.x + 2 * offset_x, y: generator_eye.y + 2 * offset_y};

            let eyed2_position: TileState = grid_state_tile(&grid_labyrinth, generator_eye.x, generator_eye.y);
            let eyed1_position: TileState = grid_state_tile(&grid_labyrinth, generator_position.x, generator_position.y);
            if eyed2_position == TileState::Off && eyed1_position == TileState::Off {
                grid_labyrinth = grid_update_tile(grid_labyrinth, generator_position.x, generator_position.y, !grid_default_state);
                good_path = true;
            } else {
                available_directions.remove(available_directions.iter().position(|d| d == direction).expect("(!) - Can't find direction."));
            }
        }
        if good_path {
            generator_path.push(generator_position);
        } else {
            generator_path.pop();
            if generator_path.len() > 0 {
                generator_position = *generator_path.last().expect("(!) - Path's empty.");
            } else {
                println!("- Reached end.");
                generator_position = Position {x: 0, y: 0};
            }
            
        }
    }

    grid_labyrinth
}

fn main() {
    println!("# Labyrinth.");

    let labyrinth_size: usize = 20;
    let labyrinth: Grid = generator_random_memory_based(labyrinth_size);
    println!("## Results - Labyrinth: ");
    grid_inline(labyrinth, LABYRINTH_UI_TILES);
}


