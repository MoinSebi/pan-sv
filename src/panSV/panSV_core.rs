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

#[derive(Debug, Clone)]
/// BubbleWrapper holds multiple index structures
///
/// - id2bubbles: Index to bubble structure (this might be changed to HashMap)
/// - id2interval: Index to PosIndex
///     - Posindex = (start, stop, String)
/// - anchor2bubble: (Start, end) -> bubble id
/// - anchor2interval: Posindex (reference) -> number of interval
/// - id2id: Welches posindex gehoert in welche bubble.
pub struct BubbleWrapper{
    pub bubbles: Vec<Bubble>,
    pub intervals: Vec<Posindex>,

    // change this
    pub anchor2bubble: HashMap<(u32, u32), u32>,
    //pub anchor2interval: HashMap<(&'a  u32, &'a  u32,&'a String), u32>, // this is the same as id2interval
    pub id2id: HashMap<(u32, u32, u32), u32>,

}

impl BubbleWrapper{
    /// Initial constructor
    ///
    /// All values are empty
    pub fn new() -> Self {
        let id2bubble: Vec<Bubble> = Vec::new();
        let id2interval: Vec<Posindex> = Vec::new();
        let anchor2bubble: HashMap<(u32, u32), u32> = HashMap::new();
        //let anchor2interval: HashMap<(& u32, & u32, & String), u32> = HashMap::new();
        let id2id: HashMap<(u32, u32, u32), u32> = HashMap::new();

        Self{
            id2id: id2id,
            bubbles: id2bubble,
            intervals: id2interval,
            anchor2bubble: anchor2bubble,
            //anchor2interval: anchor2interval,
        }
    }
}