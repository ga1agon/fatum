use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Hash, Default, Serialize, Deserialize)]
pub enum MouseScroll {
	#[default]
	None = -1,
	Up,
	Down,
	Left,
	Right
}
