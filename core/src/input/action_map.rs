use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::input::{InputAction, InputCombo};

pub type ActionMap = HashMap<Vec<InputCombo>, Rc<RefCell<InputAction>>>;
