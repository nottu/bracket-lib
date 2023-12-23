use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(
            BTermBuilder::simple_80x50().with_scaling_mode(TerminalScalingMode::ResizeTerminals),
        )
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>) {
    ctx.set_active_console(0);
    ctx.cls();
    let (width, height) = ctx.get_char_size();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
    ctx.print(
        1,
        3,
        format!("Console is currently {width}x{height} characters."),
    );
    ctx.draw_hollow_box_double(0, 0, width - 1, height - 1, WHITE, NAVY);
}
