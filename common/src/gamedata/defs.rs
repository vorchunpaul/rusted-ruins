//! Miscellaneous type definitions

use ordered_float::NotNan;

use crate::gamedata::effect::Effect;
use crate::gamedata::skill::SkillKind;
use crate::objholder::ItemIdx;
use std::ops::{Index, IndexMut};

/// Elements of damage/attack
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Element {
    None = -1,
    Physical = 0,
    Fire = 1,
    Cold = 2,
    Shock = 3,
    Poison = 4,
    Spirit = 5,
}

pub const ELEMENTS: [Element; Element::Spirit as usize + 1] = [
    Element::Physical,
    Element::Fire,
    Element::Cold,
    Element::Shock,
    Element::Poison,
    Element::Spirit,
];

/// This array has the same size as element types.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ElementArray<T>(pub [T; Element::Spirit as usize + 1]);

impl<T> Index<Element> for ElementArray<T> {
    type Output = T;
    fn index(&self, e: Element) -> &T {
        assert_ne!(e, Element::None);
        &self.0[e as usize]
    }
}

impl<T> IndexMut<Element> for ElementArray<T> {
    fn index_mut(&mut self, e: Element) -> &mut T {
        assert_ne!(e, Element::None);
        &mut self.0[e as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ElementProtection(i8);

/// A recipe for creation
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub product: String,
    pub ingredients: Vec<(String, u32)>,
    pub facility: Option<String>,
    pub difficulty: u32,
    pub required_time: CreationRequiredTime,
    #[serde(default)]
    pub put_on_ground: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum CreationRequiredTime {
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ToolEffect {
    Build,
    Chop,
    Mine,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum UseEffect {
    Effect(Effect),
    Deed,
    Seed { plant: String },
    SelectBuilding,
}

/// Reward for quests or events
#[derive(Clone, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct Reward {
    pub money: i64,
    pub item: Vec<ItemIdx>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Harvest {
    pub kind: HarvestKind,
    /// item id and yield
    pub item: Vec<(String, u32, u32)>,
    pub difficulty: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum HarvestKind {
    Animal,
    Chop,
    Deconstruct,
    Plant,
    Mine,
}

impl HarvestKind {
    pub fn related_skill(self) -> SkillKind {
        match self {
            HarvestKind::Animal => SkillKind::Animals,
            HarvestKind::Chop => SkillKind::Plants,
            HarvestKind::Deconstruct => SkillKind::Construction,
            HarvestKind::Plant => SkillKind::Plants,
            HarvestKind::Mine => SkillKind::Mining,
        }
    }
}

/// Active skill id.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ActiveSkillId(pub String);

impl std::fmt::Display for ActiveSkillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveSkill {
    pub group: ActiveSkillGroup,
    pub icon: String,
    pub effect: Effect,
    #[serde(default)]
    pub power: f32,
    #[serde(default)]
    pub hit_power: f32,
    pub power_calc: PowerCalcMethod,
    #[serde(default)]
    pub cost_sp: u32,
    #[serde(default)]
    pub cost_mp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PowerCalcMethod {
    Num(f32),
    Magic,
    Custom(String),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct BasePower(pub NotNan<f32>, pub NotNan<f32>);

#[derive(Debug, Serialize, Deserialize)]
pub enum ActiveSkillGroup {
    Special,
    Magic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ActiveSkillOrigin {
    Inherent,
    Learned,
    Race,
    Class,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum PassiveEffect {
    AttrStr(i16),
    AttrVit(i16),
    AttrDex(i16),
    AttrInt(i16),
    AttrWil(i16),
    AttrCha(i16),
    AttrSpd(i16),
}

/// Unique id for used in game
pub type UniqueId = u64;

/// Unique id generator
pub trait UniqueIdGenerator {
    fn generate(&mut self) -> UniqueId;
}
