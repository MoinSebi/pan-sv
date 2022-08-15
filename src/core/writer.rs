use crate::core::core::Bubble;
use std::fs::File;
use std::io::{Write, BufWriter};
use gfaR_wrapper::NPath;
use crate::panSV::panSV_core::{BubbleWrapper};
use crate::core::helper::{hashset2string};

/// Write bubbles with unique id
/// Read doc/bubble.stats
/// - no complex naming (no recursion)
/// - additional file for "parent ids"
pub fn bubble_naming_new(hm1: & Vec<Bubble>, out: &str){
    let f = File::create([out, "bubble", "stats"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
           "bubbleID",
           "#subBubbles",
           "minLen",
           "maxLen",
           "meanLen",
           "#traversals",
           "#intervals",
           "Parents",
            "Anchor1",
            "Anchor2",
            "Ratio",
            "Small",
            "Type", ).expect("Can not write stats file");
    for v in hm1.iter(){
        let (max, min ,mean) = v.traversal_stats();
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tCL:{}\tNL:{}\n", v.id, v.children.len(), min, max, mean, v.traversals.len(), v.number_interval(), hashset2string(&v.parents, ","), v.start, v.end, v.ratio, v.small, v.category, v.core, v.nestedness).expect("Not able to write bubble stats");
    }
}


#[allow(dead_code)]
/// Naming bubble parent-child relationship
///
/// Additional file nedded for new bubble naming
pub fn bubble_parent_structure(hm1: & Vec<Bubble>, out: &str){
    let f = File::create([out, "bubble", "txt"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "bubble_id\tchildren_id\tparents_id\n").expect("Not able to write bubble nestedness file");
    for v in hm1.iter(){
        write!(f, "{}\t{:?}\t{:?}\n", v.id, v.children, v.parents).expect("Not able to write bubble nestedness file");
    }
}


/// Writing bed file
/// Accession - FROM - TO - BUBBLE ID - BUBBLE CORE - TRAVERSAL
/// Iterate over id2interval bubble_wrapper
pub fn writing_bed_solot(r: &mut BubbleWrapper, index2: & hashbrown::HashMap<String, Vec<usize>>, paths: &Vec<NPath>, out: &str) {
    let f = File::create([out, "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let p = &r.intervals;
    let p2 = & mut r.bubbles;

    for bub in p2.iter_mut() {
        for x in bub.traversals.iter_mut(){
            for x1 in x.pos.iter_mut() {
                let pos = p.get(*x1 as usize).unwrap();
                let from_id: usize = index2.get(&paths[pos.acc as usize].name).unwrap()[pos.from as usize];
                let mut to_id: usize = index2.get(&paths[pos.acc as usize].name).unwrap()[pos.to as usize - 1];
                if pos.to == pos.from + 1 {
                    to_id = from_id.clone();
                }
                x.length = (to_id - from_id) as u32;

                write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t\n",
                       paths[pos.acc as usize].name,
                       from_id,
                       to_id,
                       bub.id,
                       bub.core,
                x.id).expect("Not able to write to file");
            }
        }
    }
}


// /// Writing traversal file
// /// Printing:
// /// traversal Len bubble
// pub fn writing_traversals(h: &BubbleWrapper, out: &str){
//     let f = File::create([out, "traversal", "txt"].join(".")).expect("Unable to create file");
//     let mut f = BufWriter::new(f);
//     for x in h.bubbles.iter(){
//
//         for y in x.traversals.iter(){
//             let mut o: Vec<String> = Vec::new();
//             for x1 in y.iter(){
//                 let j: String =  x1.0.to_string() + &bool2string_dir(x1.1);
//                 o.push(j);
//
//             }
//
//             //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");
//             write!(f, "{}\t{}\t{}\n", o.join(","), y.length, y.id).expect("Can't write traversal file");
//
//         }
//     }
// }


// /// Writing traversal file
// /// Printing:
// /// traversal Len bubble
// pub fn writing_uniques_bed(h: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, out: &str, size: usize){
//     let f = File::create([out, "traversal", "unique", "bed"].join(".")).expect("Unable to create file");
//     let mut f = BufWriter::new(f);
//     for x in h.id2bubble.iter(){
//
//         for y in x.1.traversals.iter(){
//             let k = h.id2interval.get(&y.1.pos[0]).unwrap();
//             let from_id: usize = index2.get(&k.acc).unwrap()[k.from as usize];
//             let mut to_id:usize = index2.get(&k.acc).unwrap()[k.to as usize-1];
//             if k.to == k.from+1{
//                 to_id = from_id.clone();
//             }
//
//             if to_id - from_id > size{
//                 write!(f, "{}\t{}\t{}\n", k.acc, from_id, to_id).expect("Can't write traversal file");
//             }
//
//             //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");
//
//         }
//     }
// }



// /// Writing traversal file
// /// Printing:
// /// bubble_id traversalid nodes, nodes
// pub fn writing_uniques_bed_stats(h: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, out: &str, size: usize){
//     let f = File::create([out, "traversal", "unique", "bubble", "bed"].join(".")).expect("Unable to create file");
//     let mut f = BufWriter::new(f);
//     for x in h.id2bubble.iter(){
//
//         for y in x.1.traversals.iter(){
//             let k = h.id2interval.get(&y.1.pos[0]).unwrap();
//             let from_id: usize = index2.get(&k.acc).unwrap()[k.from as usize];
//             let mut to_id:usize = index2.get(&k.acc).unwrap()[k.to as usize-1];
//             if k.to == k.from+1{
//                 to_id = from_id.clone();
//             }
//             let mut o: Vec<String> = Vec::new();
//             for x1 in y.0.iter(){
//                 let j: String =  x1.0.to_string() + &bool2string_dir(x1.1);
//                 o.push(j);
//
//             }
//
//             if to_id - from_id > size{
//                 write!(f, "{}\t{}\t{}\n", x.1.id, y.1.id, o.join(",")).expect("Can't write traversal file");
//             }
//
//             //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");
//
//         }
//     }
// }