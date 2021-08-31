use super::Game;
use common::gamedata::*;
use common::gobj;
use common::obj::*;
use common::objholder::*;
use geom::*;

pub fn start_build(game: &mut Game<'_>, pos: Vec2d, builder: CharaId, build_obj: BuildObj) {
    let wall_id = match build_obj {
        BuildObj::Wall(id) => id,
        _ => todo!(),
    };
    let wall_idx: WallIdx = gobj::id_to_idx(&wall_id);
    let wall_obj = gobj::get_obj(wall_idx);

    if !is_buildable(&game.gd, pos) {
        return;
    }

    let item_list = game
        .gd
        .get_item_list_mut(ItemListLocation::Chara { cid: builder });

    // Check player has needed materials
    for &(ref item_id, n) in &wall_obj.materials {
        let item_idx: ItemIdx = gobj::id_to_idx(item_id);
        let has = item_list.count(item_idx);
        if has < n {
            let needed = n - has;
            let item = crate::text::obj_txt(item_id);
            game_log_i!("building-shortage-material"; item=item, n=needed);
            return;
        }
    }

    // Consume needed materials
    for &(ref item_id, n) in &wall_obj.materials {
        let item_idx: ItemIdx = gobj::id_to_idx(item_id);
        item_list.consume(item_idx, n, |_, _| {}, false);
    }

    finish_build(game, pos, wall_idx);
}

pub fn finish_build(game: &mut Game<'_>, pos: Vec2d, wall_idx: WallIdx) {
    let map = game.gd.get_current_map_mut();
    map.set_wall(pos, wall_idx);
    audio::play_sound("finish-build");
}

fn is_buildable(gd: &GameData, pos: Vec2d) -> bool {
    let map = gd.get_current_map();

    if !map.is_inside(pos) {
        return false;
    }

    if map.tile[pos].wall.is_empty() {
        let tile = gobj::get_obj(map.tile[pos].main_tile());
        match tile.kind {
            TileKind::Ground => true,
            TileKind::Water => false,
        }
    } else {
        false
    }
}

/// Returns buildable object and its needed skill level list
pub fn build_obj_list() -> Vec<(BuildObj, u32)> {
    let mut list = Vec::new();

    for (i, tile) in gobj::get_objholder().tile.iter().enumerate() {
        if let Some(skill_level) = tile.build_skill {
            let id = gobj::idx_to_id(TileIdx::from_usize(i));
            list.push((BuildObj::Tile(id.into()), skill_level));
        }
    }

    for (i, wall) in gobj::get_objholder().wall.iter().enumerate() {
        if let Some(skill_level) = wall.build_skill {
            let id = gobj::idx_to_id(WallIdx::from_usize(i));
            list.push((BuildObj::Wall(id.into()), skill_level));
        }
    }

    list
}
