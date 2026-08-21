use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::Map;
use bevy::prelude::{Component, Timer};
use crate::entity::status_buf::StatusBuff::Imprison;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum StatusBuff {
	///禁锢 - 无法移动 - 携带禁锢持续时间的计时器。
	Imprison,
}

pub struct StatusBuffArg{
	timer: Timer
}

#[derive(Component, Default)]
pub struct EntityStatus {
	pub list: HashMap<StatusBuff, StatusBuffArg>,
}
