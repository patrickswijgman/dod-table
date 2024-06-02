# Table

A generic struct for Rust to create a contiguous memory block.
Inspired by the [Data Oriented Design book by Richard Fabian](https://www.dataorienteddesign.com/dodbook.pdf).

## Installation

```shell
cargo add dod-table
```

## Usage

```rust
// Make sure that the type used in the table implements the Default trait.
#[derive(Default)]
struct Entity {
    x: f32,
    y: f32,
}

// The global data for our game.
struct Data {
    // Memory reserved for (in this specific example) 512 entities.
    entities: Table<Entity, 512>,
    // We choose a specific element in the table for the player entity.
    player_id: usize,
}

// We could also derive the Default trait here like the Entity struct, but lets
// instead implement a 'new' method for our Data struct.
impl Data {
    fn new() -> Self {
        Self {
            // Table implements the Default trait, hence that its type also needs to implement it.
            entities: Table::default(),
            // Let's reserve the first index as the player entity.
            player_id: 0,
        }
    }
}

// Your game's entrypoint.
fn main() {
    let data = Data::new();

    // Initialize the player data.
    let player = data.entities.get_mut(player_id);
    player.x = 100;
    player.y = 100;

    loop {
        // The game logic...
    }
}
```
