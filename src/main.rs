use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers,
    map::{TilemapId, TilemapSize, TilemapSpacing, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::{get_tilemap_center_transform, ArrayTextureLoader},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use jiff::civil::Weekday;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TilemapPlugin,
            WorldInspectorPlugin::new(),
            // This plugin maps inputs to an input-type agnostic action-state
            // We need to provide it with an enum which stores the possible actions a player could take
            // The InputMap and ActionState components will be added to any entity with the Car component
            InputManagerPlugin::<Action>::default(),
        ))
        .add_systems(Startup, setup)
        // Read the ActionState in your systems using queries!
        // .add_systems(Update, moving)
        .run();
}

#[derive(Component)]
struct Player;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.3, 0.3, 1.0),
            ..default()
        },
        ..default()
    });

    // commands.spawn((
    //     // Create a TextBundle that has a Text with a single section.
    //     TextBundle::from_section(
    //         // Accepts a `String` or any type that converts into a `String`, such as `&str`
    //         "Mo Tu We Th Fr Sa Su",
    //         TextStyle {
    //             // This font is loaded and will be used instead of the default font.
    //             font: asset_server.load("fonts/snuggle_upper.ttf"),
    //             font_size: 100.0,
    //             ..default()
    //         },
    //     ) // Set the justification of the Text
    //     .with_text_justify(JustifyText::Center)
    //     // Set the style of the TextBundle itself.
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(5.0),
    //         left: Val::Px(5.0),
    //         ..default()
    //     }),
    // ));

    // <--------------------- 49 ------------------------>
    //                                                   |
    //  Mo Tu We Th Fr Sa Su                             |
    //     01 02 03 04 05 06   ⚀⚀ ⚀⚀ ⚀⚀ ⚀⚀ ⚀⚀ ⚀⚀ ⚀⚀      ^
    //  07 08 09 10 11 12 13   vv vv vv vv vv vv vv      |
    //  14 15 16 17 18 19 20                             |
    //  21 22 23 24 25 26 27                          ⌂  12
    //  28 29 30 31                                      |
    //                                                   |
    //                                                   |
    //                                                   |
    //                                                   |
    //                                                   V

    let texture_handle: Handle<Image> = asset_server.load("calendar.png");
    let map_size = TilemapSize {
        x: 7 + 1 + 7 + 2,
        y: 1 + 6 + 6,
    };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    // for x in 0..map_size.x {
    //     for y in 0..map_size.y {
    //         let tile_pos = TilePos { x, y };
    //         let tile_entity = commands
    //             .spawn(TileBundle {
    //                 position: tile_pos,
    //                 tilemap_id: TilemapId(tilemap_entity),
    //                 ..Default::default()
    //             })
    //             .id();
    //         tile_storage.set(&tile_pos, tile_entity);
    //     }
    // }

    helpers::filling::fill_tilemap_rect_color(
        TileTextureIndex(7),
        TilePos::new(0, 0),
        map_size,
        Color::BLACK,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    // helpers::filling::fill_tilemap_rect(
    //     TileTextureIndex(0),
    //     TilePos { x: 0, y: 11 },
    //     TilemapSize { x: 7, y: 1 },
    //     TilemapId(tilemap_entity),
    //     &mut commands,
    //     &mut tile_storage,
    // );

    commands.entity(tilemap_entity).with_children(|parent| {
        for weekday in Weekday::Monday.cycle_forward().take(7) {
            let id = weekday as u32 - 1;
            let tile_pos = TilePos { x: id, y: 11 };
            let tile_entity = parent
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(id),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    });

    helpers::filling::fill_tilemap_rect(
        TileTextureIndex(7),
        TilePos { x: 0, y: 6 },
        TilemapSize { x: 7, y: 5 },
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        spacing: TilemapSpacing::new(0.0, 0.0),
        ..default()
    });
}

// Query for the `ActionState` component in your game logic systems!
fn moving(_time: Res<Time>, mut query: Query<(&ActionState<Action>, &Transform), With<Player>>) {
    let (action_state, _transform) = query.single_mut();
    // Each action has a button-like state of its own that you can check
    if action_state.pressed(&Action::MoveUp) {
    } else if action_state.pressed(&Action::MoveDown) {
    } else if action_state.pressed(&Action::MoveLeft) {
    } else if action_state.pressed(&Action::MoveRight) {
    }
}
