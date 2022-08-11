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
    pub children: HashSet<u32>,
    pub parents: HashSet<u32>,
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
    /// Bubble constructor
    pub fn new(core: u32, start: u32, end: u32, first:u32, i: u32, trav: Traversal, s : Vec<(u32, bool)>) -> Self{
        let mut u: Vec<u32> = Vec::new();
        u.push(first);
        let u2: HashSet<u32> = HashSet::new();
        let u3: HashSet<u32> = HashSet::new();
        let mut u4: HashMap<Vec<(u32, bool) >, Traversal> = HashMap::new();
        u4.insert(s, trav);

        Self {
            start: start,
            end: end,
            children: u2,
            parents: u3,
            id: i,
            traversals: Vec::new(),
            core: core,
            small: true,
            ratio: 0.0,
            category: 0,
            nestedness: 0

        }
    }

    pub fn new2(core: u32, start: u32, end: u32, i: u32, groups: Vec<Vec<u32>>, last: u32) -> Self{

        let u2: HashSet<u32> = HashSet::new();
        let u3: HashSet<u32> = HashSet::new();
        let mut rr = Vec::with_capacity(groups.len());
        let mut h = last;
        for x in groups{
            rr.push(Traversal{length: 0, pos: x, id: h});
            h += 1;
        }

        Self {
            start: start,
            end: end,
            children: u2,
            parents: u3,
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

impl  Traversal{
    pub fn add_pos(&mut self, pos: u32){
        self.pos.push(pos);
    }



    pub fn new(posid: u32, lens: u32) -> Self{
        let mut k: Vec<u32> = Vec::new();
        k.push(posid);
        Self{
            length: lens,
            pos: k,
            id: 0
        }
    }

    #[allow(dead_code)]
    pub fn get_pos(& self) -> usize{
        self.pos.len()
    }
}



