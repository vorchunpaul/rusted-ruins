use super::effect::do_effect;
use super::extrait::*;
use super::{Game, InfoGetter};
use crate::text::ToText;
use common::gamedata::*;
use ordered_float::NotNan;
use rules::RULES;

/// Return true if success.
pub fn use_active_skill(
    game: &mut Game<'_>,
    active_skill_id: &ActiveSkillId,
    cid: CharaId,
    target: CharaId,
) -> bool {
    if !game.gd.target_visible(cid, target) {
        return false;
    }

    let active_skill = if let Some(active_skill) = RULES.active_skills.get(active_skill_id) {
        active_skill
    } else {
        warn!("unknown active_skill \"{}\"", active_skill_id);
        return false;
    };

    let chara = game.gd.chara.get(cid);
    if !chara.active_skill_available(active_skill) {
        return false;
    }

    let power =
        crate::game::active_skill::calc_power(&game.gd, active_skill, cid) * active_skill.power;
    let hit_power = active_skill.hit_power;

    let chara = game.gd.chara.get(cid);
    trace!(
        "{} uses active skill \"{}\", power = {}, hit_power = {}",
        chara.to_text(),
        active_skill_id,
        power,
        hit_power,
    );

    match active_skill.group {
        ActiveSkillGroup::Magic => {
            game_log!("use-active-skill-magic"; chara=chara, active_skill=active_skill_id);
        }
        ActiveSkillGroup::Special => {
            game_log!("use-active-skill-special"; chara=chara, active_skill=active_skill_id);
        }
    }

    do_effect(
        game,
        &active_skill.effect,
        Some(cid),
        target,
        power,
        hit_power,
    );
    true
}

pub fn calc_power(gd: &GameData, active_skill: &'static ActiveSkill, cid: CharaId) -> f32 {
    match active_skill.power_calc {
        PowerCalcMethod::Num(n) => n,
        PowerCalcMethod::Magic => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::MagicDevice) as f32;
            let int = chara.attr.int as f32;
            skill_level * int
        }
        _ => todo!(),
    }
}

#[extend::ext(pub)]
impl BasePower {
    fn calc(&self, factor: f32) -> f32 {
        let factor = NotNan::new(factor).unwrap();
        let base_power = self.0 * factor;
        let power_var = self.1 * factor;
        let power_min = std::cmp::max(base_power - power_var, NotNan::new(0.0).unwrap());
        let power_max = base_power + power_var;
        let power = if power_max > power_min {
            rng::gen_range(power_min..power_max)
        } else {
            power_min
        };
        power.into_inner()
    }

    fn calc_without_var(&self, factor: f32) -> f32 {
        self.0.into_inner() * factor
    }
}
