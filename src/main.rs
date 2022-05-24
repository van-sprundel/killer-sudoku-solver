use bevy::app::App;

use bevy::prelude::*;
use killer_sudoku::data::cage_table::CageTable;
use killer_sudoku::plugins::board_plugin::BoardPlugin;

// fn main() {
//     let mut app = App::new();
//     app.insert_resource(WindowDescriptor {
//         title: "sudoku".to_string(),
//         width: 300.,
//         height: 300.,
//         ..Default::default()
//     });
//     app.add_plugins(DefaultPlugins);
//
//     app.add_plugin(BoardPlugin);
//     app.add_startup_system(setup_camera);
//     app.run();
// }
//
// fn setup_camera(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
// }

fn main() {
    let cage_table = CageTable::from_file("./assets/cage_table.txt");

    for x in cage_table.0 {
        println!("{:?}", x);
    }
}