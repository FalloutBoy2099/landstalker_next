//! Object shadows (landstalker.ini, [video] object_shadows): a dark oval on the ground under Nigel and the objects that are above it (a jump, a platform, a
//! flying enemy), so that their height can be read on the picture.

use crate::config::Config;
use crate::scripting::Scripting;
use crate::{depth, snap, Player, RoomRes, WorldRoot};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// The sprites that are the shadows (one per object above the ground this frame; the others are hidden), under the root of the room they belong to.
#[derive(Default)]
pub struct ShadowPool {
    root: Option<Entity>,
    sprites: Vec<Entity>,
    image: Option<Handle<Image>>,
}

/// What is above the ground by less than this has no shadow worth drawing.
const MIN_HEIGHT: f32 = 0.15;
/// The oval, in pixels: the width of a cell on the screen is 32 and its height 16.
const OVAL: (u32, u32) = (32, 16);

fn oval() -> Image {
    let (w, h) = OVAL;
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = ((x as f32 + 0.5) / w as f32 * 2.0 - 1.0, (y as f32 + 0.5) / h as f32 * 2.0 - 1.0);
            let inside = dx * dx + dy * dy <= 1.0;
            data.extend_from_slice(&[0, 0, 0, if inside { 170 } else { 0 }]);
        }
    }
    Image::new(Extent3d { width: w, height: h, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default())
}

#[allow(clippy::too_many_arguments)]
pub fn object_shadows(
    mut commands: Commands,
    config: Res<Config>,
    room: Res<RoomRes>,
    script: NonSend<Scripting>,
    players: Query<&Player>,
    roots: Query<Entity, With<WorldRoot>>,
    mut images: ResMut<Assets<Image>>,
    mut pool: Local<ShadowPool>,
    mut shadows: Query<(&mut Transform, &mut Visibility, &mut Sprite)>,
) {
    if !config.effects.object_shadows {
        for e in pool.sprites.drain(..) {
            commands.entity(e).despawn();
        }
        return;
    }
    let Ok(root) = roots.single() else { return };
    if pool.root != Some(root) {
        // a new room: the old shadows went with the old root
        pool.root = Some(root);
        pool.sprites.clear();
    }
    let image = pool.image.get_or_insert_with(|| images.add(oval())).clone();
    // what casts a shadow: Nigel and the entities above the ground (x, y, z, half the base)
    let snaps = script.entities();
    let mut casters: Vec<(f32, f32, f32, f32, Option<usize>)> = Vec::new();
    if let Ok(p) = players.single() {
        casters.push((p.x, p.y, p.z, crate::physics::PLAYER_RADIUS, None));
    }
    for e in &snaps {
        if e.visible && !e.carried && !e.dying && e.kind < 0xC0 && e.hbase > 0.0 {
            casters.push((e.x, e.y, e.z, e.hbase, Some(e.index)));
        }
    }
    let mut used = 0;
    if std::env::var("LS_LOG_SHADOWS").is_ok() {
        println!("[shadows] {} casters, pool {}", casters.len(), pool.sprites.len());
    }
    for (x, y, z, half, me) in casters {
        // the shadow falls on the floor, or on the platform (or the crate...) under the object: the highest top that is under its feet
        let mut ground = script.cell_floor(x, y, half);
        for o in &snaps {
            if Some(o.index) == me || !o.visible || !o.solid || o.carried || o.kind >= 0xC0 || o.hbase <= 0.0 {
                continue;
            }
            let top = o.z + o.hheight;
            if top <= z + 0.06 && top > ground && (x - o.x).abs() < o.hbase + half * 0.5 && (y - o.y).abs() < o.hbase + half * 0.5 {
                ground = top;
            }
        }
        if std::env::var("LS_LOG_SHADOWS").is_ok() && me.is_none() {
            println!("[shadows] Nigel z {:.2} ground {:.2}", z, ground);
        }
        if z - ground < MIN_HEIGHT {
            continue;
        }
        let at = snap(room.to_screen(x, y, ground));
        let position = Vec3::new(at.x, -at.y, depth(x, y, ground) - 0.0004);
        let size = Vec2::new(OVAL.0 as f32 * half * 2.0 * 0.9, OVAL.1 as f32 * half * 2.0 * 0.9);
        if let Some(&e) = pool.sprites.get(used) {
            if let Ok((mut tf, mut vis, mut sprite)) = shadows.get_mut(e) {
                tf.translation = position;
                sprite.custom_size = Some(size);
                *vis = Visibility::Inherited;
            }
        } else {
            let mut sprite = Sprite::from_image(image.clone());
            sprite.custom_size = Some(size);
            let mut id = None;
            commands.entity(root).with_children(|c| {
                id = Some(c.spawn((sprite, Transform::from_translation(position), Visibility::Inherited)).id());
            });
            pool.sprites.extend(id);
        }
        used += 1;
    }
    for e in pool.sprites.iter().skip(used) {
        if let Ok((_, mut vis, _)) = shadows.get_mut(*e) {
            *vis = Visibility::Hidden;
        }
    }
}
