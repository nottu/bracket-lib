use crate::{BracketCamera, BracketContext, TerminalScalingMode};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    mesh::Mesh2d,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use super::{BracketMesh, ScreenScaler};

pub(crate) fn update_consoles(
    mut ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    find_mesh: Query<(&BracketMesh, &Mesh2d)>,
    scaler: Res<ScreenScaler>,
) {
    let mut new_meshes: Vec<(Mesh2d, Mesh2d, bool)> = Vec::new();
    {
        let mut terms = ctx.terminals.lock();
        for (id, handle) in find_mesh.iter() {
            let terminal_id = id.0;
            let new_mesh = terms[terminal_id].new_mesh(&ctx, &mut meshes, &scaler);
            if let Some(new_mesh) = new_mesh {
                let old_mesh = handle.clone();
                new_meshes.push((old_mesh, Mesh2d(new_mesh), false));
            }
        }
    }

    new_meshes
        .drain(0..)
        .for_each(|m| ctx.mesh_replacement.push(m));
}

pub(crate) fn replace_meshes(
    mut ctx: ResMut<BracketContext>,
    mut ev_asset: MessageReader<AssetEvent<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut update_mesh: Query<&mut Mesh2d, With<BracketMesh>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::Added { id } = ev {
            for (old, new, done) in ctx.mesh_replacement.iter_mut() {
                if *id == new.id() {
                    for mut m in update_mesh.iter_mut() {
                        if old.id() == m.id() {
                            *m = new.clone();
                        }
                    }
                    *done = true;
                }
            }
        }
    }

    for (old, _, _) in ctx.mesh_replacement.iter().filter(|(_, _, done)| *done) {
        meshes.remove(old.id());
    }
    ctx.mesh_replacement.retain(|(_, _, done)| !done);
}

pub(crate) fn update_timing(mut ctx: ResMut<BracketContext>, diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.measurement() {
            ctx.fps = fps_avg.value.round();
        }
    }

    if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time.measurement() {
            ctx.frame_time_ms = (frame_time_avg.value * 1000.0).round();
        }
    }
}

pub(crate) fn window_resize(
    mut context: ResMut<BracketContext>,
    mut resize_event: MessageReader<WindowResized>,
    mut scaler: ResMut<ScreenScaler>,
) {
    for e in resize_event.read() {
        scaler.set_screen_size(e.width, e.height);
        if let TerminalScalingMode::ResizeTerminals = context.scaling_mode {
            context.resize_terminals(&scaler);
        }
        scaler.recalculate(context.get_pixel_size(), context.largest_font());
    }
}

pub(crate) fn apply_all_batches(mut context: ResMut<BracketContext>) {
    context.render_all_batches();
}

pub(crate) fn update_mouse_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<BracketCamera>>,
    mut context: ResMut<BracketContext>,
    scaler: Res<ScreenScaler>,
) {
    let Ok(wnd) = window_query.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            let result = (world_pos.x, world_pos.y);
            context.set_mouse_pixel_position(result, &scaler);
        }
    }
}
