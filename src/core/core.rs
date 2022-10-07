use std::collections::{HashMap, HashSet};
use std::hash::Hash;


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
/// Positional information for each interval
/// From: starting index
/// To: end index
/// Acc: Accession
/// Comment: Might use a reference here
pub struct Posindex {
    pub from:  u32,
    pub to:   u32,
    pub acc:  u32,
}

#[derive(Debug, Clone)]
/// Bubbles have a start and stop node (stored by node id)
/// Additional information
/// - ID
/// - Children (list of bubble ids)
/// - Parents (list of bubble ids)
/// - traversal ("Unique" order of nodes between start and end)
/// - Core (Core level)
pub struct Bubble {
    pub start: u32,
    pub end: u32,
    pub id: u32,
    pub traversals: Vec<Traversal>,
    // this is kinda panSV specific
    pub core: u32,

    // Classification
    pub small: bool,
    pub ratio: f32,
    pub category: u8,
    pub nestedness: u16,

    // 0 = SNP, 1 = INDEL, 2 = MNP || 3 = INDEL, 4 = DifferentSize, 5 = SameSize

}


impl Bubble {


    pub fn new(core: u32, start: u32, end: u32, i: u32, groups: Vec<Vec<u32>>, last: u32) -> Self{

        let u2: HashSet<u32> = HashSet::new();
        let u3: HashSet<u32> = HashSet::new();
        let mut rr = Vec::with_capacity(groups.len());
        let mut h = last;
        for x in groups{
            rr.push(Traversal{length: 0, pos: x, id: h});
            h += 1;

        }
        rr.shrink_to_fit();

        Self {
            start: start,
            end: end,
            id: i,
            traversals: rr,
            core: core,
            small: true,
            ratio: 0.0,
            category: 0,
            nestedness: 0

        }
    }

    /// Mean, max and min length of all traversals
    pub fn traversal_stats(&self) -> (u32, u32, f32){
        let mut all_length = Vec::new();
        for v in self.traversals.iter(){
            all_length.push(v.length);
        }

        let m1 = all_length.iter().max().unwrap();
        let m2 = all_length.iter().min().unwrap();
        let sum: u32 = all_length.iter().sum();

        let mean  = sum as f32 / all_length.len() as f32;
        (m1.clone(), m2.clone(),mean)
    }



    /// Total number of intervals
    pub fn number_interval(&self) -> usize{
        let mut number = 0;
        for v in self.traversals.iter(){
            number += v.pos.len();
        }
        number
    }

    #[allow(dead_code)]
    /// Number of different accessions
    pub fn number_acc(&self, hm: &HashMap<u32, Posindex>) -> usize{
        let mut accession_numb= HashSet::new();
        for v in self.traversals.iter(){
            for x in v.pos.iter(){
                accession_numb.insert(hm.get(x).unwrap().acc.clone());
            }
        }
        accession_numb.len()
    }

}







#[derive(Hash, Eq, PartialEq, Debug, Clone)]
/// Traversal a unique connections between two specific bubbles
/// Contains:
/// - sequence length
/// - list of traversal ids
pub struct Traversal {
    pub length: u32, // Sequence length
    pub pos: Vec<u32>,
    pub id: u32,
}



