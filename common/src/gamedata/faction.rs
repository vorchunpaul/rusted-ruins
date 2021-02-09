use crate::hashmap::HashMap;
use arrayvec::ArrayString;

/// Faction information.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Faction {
    /// Faction relation to player.
    relation_table: HashMap<FactionId, FactionRelation>,
}

impl Faction {
    pub fn new() -> Faction {
        Faction {
            relation_table: HashMap::default(),
        }
    }

    pub fn get(&self, faction: FactionId) -> FactionRelation {
        self.relation_table
            .get(&faction)
            .map(|f| *f)
            .unwrap_or(FactionRelation(0))
    }

    pub fn set(&mut self, faction: FactionId, relation: FactionRelation) {
        self.relation_table.insert(faction, relation);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FactionId(arrayvec::ArrayString<[u8; crate::basic::ARRAY_STR_ID_LEN]>);

impl Default for FactionId {
    fn default() -> FactionId {
        FactionId::unknown()
    }
}

impl FactionId {
    pub fn new(name: &str) -> Option<FactionId> {
        ArrayString::from(name).map(|s| FactionId(s)).ok()
    }

    pub fn unknown() -> FactionId {
        FactionId::new("!unknown").unwrap()
    }

    pub fn player() -> FactionId {
        FactionId::new("!player").unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FactionRelation(i16);
const FACTION_RELATION_MAX: i16 = 10000;
const FACTION_RELATION_MIN: i16 = -10000;

impl std::ops::Add<i16> for FactionRelation {
    type Output = Self;
    fn add(self, other: i16) -> Self {
        FactionRelation(std::cmp::min(
            self.0.wrapping_add(other),
            FACTION_RELATION_MAX,
        ))
    }
}

impl std::ops::Sub<i16> for FactionRelation {
    type Output = Self;
    fn sub(self, other: i16) -> Self {
        FactionRelation(std::cmp::max(
            self.0.wrapping_sub(other),
            FACTION_RELATION_MIN,
        ))
    }
}
