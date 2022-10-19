use crate::core::core::{Bubble, Posindex};
use hashbrown::HashMap;


#[derive(Debug, Clone)]
/// Same as PosIndex but with core instead of accession
pub struct PanSVpos {
    pub start:  u32,
    pub end:  u32,
    pub core: u32,
}

#[derive(Debug, Clone)]
/// To construct the PanSVpos
pub struct TmpPos {
    pub acc:  String,
    pub start:  u32,
    pub core:  u32,
    pub node1: u32,
}