use std::collections::{HashMap, BTreeSet};
use crate::core::core::{Bubble, Posindex};


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


/// BubbleWrapper holds multiple index structures
///
/// - id2bubbles: Index to bubble structure (this might be changed to HashMap)
/// - id2interval: Index to PosIndex
///     - Posindex = (start, stop, String)
/// - anchor2bubble: (Start, end) -> bubble id
/// - anchor2interval: Posindex (reference) -> number of interval
/// - id2id: Welches posindex gehoert in welche bubble.
pub struct BubbleWrapper<'a>{
    pub id2bubble: HashMap<u32, Bubble>,
    pub id2interval: HashMap<u32, Posindex>,

    // change this
    pub anchor2bubble: HashMap<BTreeSet<&'a u32>, u32>,
    pub anchor2interval: HashMap<(&'a  u32, &'a  u32,&'a String), u32>, // this is the same as id2interval
    pub id2id: HashMap<(u32, u32, &'a  String), u32>,

}

impl BubbleWrapper<'_>{
    /// Initial constructor
    ///
    /// All values are empty
    pub fn new() -> Self {
        let id2bubble: HashMap<u32, Bubble> = HashMap::new();
        let id2interval: HashMap<u32, Posindex> = HashMap::new();
        let anchor2bubble: HashMap<BTreeSet<& u32>, u32> = HashMap::new();
        let anchor2interval: HashMap<(& u32, & u32, & String), u32> = HashMap::new();
        let id2id: HashMap<(u32, u32, & String), u32> = HashMap::new();

        Self{
            id2id: id2id,
            id2bubble: id2bubble,
            id2interval: id2interval,
            anchor2bubble: anchor2bubble,
            anchor2interval: anchor2interval,
        }
    }
}