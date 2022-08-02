use crate::core::core::Bubble;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
use gfaR_wrapper::NPath;
use bifurcation::helper::chunk_inplace;
use crate::panSV::panSV_core::{BubbleWrapper};
use crate::core::helper::{bool2string_dir, hashset2string};

/// Write bubbles with unique id
/// Read doc/bubble.stats
/// - no complex naming (no recursion)
/// - additional file for "parent ids"
pub fn bubble_naming_new(hm1: & HashMap<u32, Bubble>, out: &str){
    let f = File::create([out, "bubble", "stats"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
           "bubbleID",
           "#Nestedness",
           "#subBubbles",
           "minLen",
           "maxLen",
           "meanLen",
           "#traversal",
           "#intervals",
           "Parents",
            "Anchor1",
            "Anchor2",
            "Ratio",
            "Small",
            "Type", ).expect("Can not write stats file");
    for (_k,v) in hm1.iter(){
        let (max, min ,mean) = v.traversal_stats();
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tCL:{}\n", v.id, v.nestedness, v.children.len(), min, max, mean, v.traversals.len(), v.number_interval(), hashset2string(&v.parents, ","), v.start, v.end, v.ratio, v.small, v.category, v.core).expect("Not able to write bubble stats");
    }
}


/// Naming bubble parent-child relationship
///
/// Additional file nedded for new bubble naming
pub fn bubble_parent_structure(hm1: & HashMap<u32, Bubble>, out: &str){
    let f = File::create([out, "bubble", "txt"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "bubble_id\tchildren_id\tparents_id\n").expect("Not able to write bubble nestedness file");
    for (_k,v) in hm1.iter(){
        write!(f, "{}\t{:?}\t{:?}\n", v.id, v.children, v.parents).expect("Not able to write bubble nestedness file");
    }
}






/// Writing bed file
/// Accession - FROM - TO - BUBBLE ID - BUBBLE CORE - TRAVERSAL
/// Iterate over id2interval bubble_wrapper
pub fn writing_bed(r: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, paths: &Vec<NPath>, out: &str){

    let f = File::create([out, "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for (_k,v) in r.id2interval.iter() {
        let from_id: usize = index2.get(&paths[v.acc as usize].name).unwrap()[v.from as usize];
        let mut to_id:usize = index2.get(&paths[v.acc as usize].name).unwrap()[v.to as usize-1];

        if v.to == v.from+1{
            to_id = from_id.clone();
        }
        let bub = r.id2bubble.get(r.id2id.get(&(v.from, v.to, v.acc)).unwrap()).unwrap();
        let (max, min ,_mean) = bub.traversal_stats();

        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
               paths[v.clone().acc as usize].name,
               from_id,
               to_id,
               bub.id,
               bub.core,
                bub.category,
                bub.small ,
                max,
                min,).expect("Not able to write to file");
    }
}


/// Writing bed file
/// Accession - FROM - TO - BUBBLE ID - BUBBLE CORE - TRAVERSAL
/// Iterate over id2interval bubble_wrapper
pub fn writing_bed2(r: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, paths: &Vec<NPath>, out: &str) {
    let f = File::create([out, "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let p = &r.id2interval;
    let p2 = &r.id2bubble;

    for (k, bub) in p2.iter() {
        let (max, min, _mean) = bub.traversal_stats();
        for x in bub.traversals.iter() {
            for x1 in x.1.pos.iter() {
                let pos = p.get(&x1).unwrap();
                let from_id: usize = index2.get(&paths[pos.acc as usize].name).unwrap()[pos.from as usize];
                let mut to_id: usize = index2.get(&paths[pos.acc as usize].name).unwrap()[pos.to as usize - 1];
                if pos.to == pos.from + 1 {
                    to_id = from_id.clone();
                }



                write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                       paths[pos.acc as usize].name,
                       from_id,
                       to_id,
                       bub.id,
                       bub.core,
                       bub.category,
                       bub.small,
                       max,
                       min).expect("Not able to write to file");
            }
        }
    }
}



/// Write bed file
/// Documentation found here doc.md
pub fn writing_bed_traversals(h: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, paths: &Vec<NPath>, out: &str){
    let f = File::create([out, "traversal", "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){
        for y in x.1.traversals.iter(){
            for x1 in y.1.pos.iter(){

                let k = h.id2interval.get(&x1).unwrap();
                let from_id: usize = index2.get(&paths[k.acc as usize].name).unwrap()[k.from as usize];
                let mut to_id:usize = index2.get(&paths[k.acc as usize].name).unwrap()[k.to as usize-1];
                if k.to == k.from+1{
                    to_id = from_id.clone();
                }
                let bub = h.id2bubble.get(h.id2id.get(&(k.from, k.to, k.acc)).unwrap()).unwrap();
                write!(f, "{}\t{}\t{}\t{}\t{}\t{}\n", k.acc, from_id, to_id, bub.id, bub.core, y.1.id).expect("Can't write traversal file");
            }


        }
    }
}



/// Writing traversal file
/// Printing:
/// traversal Len bubble
pub fn writing_traversals(h: &BubbleWrapper, out: &str){
    let f = File::create([out, "traversal", "txt"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){

        for y in x.1.traversals.iter(){
            let mut o: Vec<String> = Vec::new();
            for x1 in y.0.iter(){
                let j: String =  x.0.to_string() + &bool2string_dir(x1.1);
                o.push(j);

            }

            //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");
            write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, y.1.id).expect("Can't write traversal file");

        }
    }
}


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