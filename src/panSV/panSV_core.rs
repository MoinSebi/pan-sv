use crate::core::core::{Bubble, Posindex};
use hashbrown::HashMap;


#[derive(Debug, Clone)]
pub struct PanSVpos {
    pub start:  u32,
    pub end:  u32,
    pub core: u32,
}

/// For interval_open ->
pub struct TmpPos {
    pub acc:  String,
    pub start:  u32,
    pub core:  u32,
}