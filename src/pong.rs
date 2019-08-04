extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

// arena size constants
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

// paddle size constants
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    // constructor for Paddle structs
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position paddles
    let y = ARENA_HEIGHT / 2.0; // middle of screen
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // SpriteRender is a compenent that tells which sprite should be drawn for an entity
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // because paddle is in first index position on sprite sheet
    };

    // create left paddle entity
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // create right paddle entity
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render.clone())
        .build();
}

// setup camera
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();

    // creates an xyz coordinate position of the middle of the arena
    transform.set_translation_xyz(
        ARENA_WIDTH * 0.5,
        ARENA_HEIGHT * 0.5,
        1.0); // 1 unit back on z axis to be able to see whole arena

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

// import spritesheet assets
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        // texture_storage is a cloneable reference to the texture
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

// define our game state
pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load sprite sheet necessary to render graphics
        let sprite_sheet_handle = load_sprite_sheet(world);
        
        initialise_camera(world);

        world.register::<Paddle>();
        initialise_paddles(world, sprite_sheet_handle);
    }
}
