pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(screen_system);
    }
}

fn screen_system(
    game_data: Res<GameData>,
    mut commands: Commands,
) {

}