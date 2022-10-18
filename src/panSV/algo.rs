use std::cmp::{max, min};
use std::hash::Hash;
use std::ops::Deref;
use crate::core::counting::{CountNode};
use crate::panSV::panSV_core::{PanSVpos, TmpPos};
use crate::core::core::{Posindex, Bubble, Traversal};
use related_intervals::{make_nested, Network};
use gfaR_wrapper::NPath;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use bifurcation::helper::chunk_inplace;
use crossbeam_channel::unbounded;
use hashbrown::{HashMap, HashSet};
use log::{debug, info};



/// Detect start and end position of bubbles
///
/// Idea:
/// 1. Iterate over each path
/// 2. Save start, end position (index) + core level of bubbles
/// 3. Save the structure in a HashMap
///
pub fn algo_panSV_multi2(paths: &Vec<NPath>, counts: CountNode, threads: &usize, path2index: &HashMap<String, usize>) -> Vec<(usize, u32, u32, u32, u32, u32)>{
    info!("Running pan-sv algorithm");




    let chunks = chunk_inplace(paths.clone(), threads.clone());
    let arc_results = Arc::new(Mutex::new(Vec::new()));
    let arc_counts = Arc::new(counts);
    let arc_cc = Arc::new(path2index.clone());


    // Indexing
    let total_len = Arc::new(paths.len());
    let genome_count = Arc::new(Mutex::new(0));

    // Handles
    let mut handles = Vec::new();


    // Iterate over packs of paths
    for chunk in chunks{
        let carc_results = arc_results.clone();
        let carc_counts = arc_counts.clone();
        let carc_cc = arc_cc.clone();

        let carc_genome_count = genome_count.clone();
        let carc_total_len = total_len.clone();

        let handle = thread::spawn(move || {

            let mut lastcore: u32;
            let mut lastnode: u32;
            let mut result_panSV = Vec::new();
            let mut index1;

            for x in chunk{
                //max_index.insert(x.name.clone(), x.nodes.len()-1);
                lastcore = 1;
                lastnode = 1;
                index1 = carc_cc.get(&x.name).unwrap().clone();


                // All "open" intervals
                let mut interval_open:  Vec<TmpPos> = Vec::new();

                // Iterate over all nodes
                for (index, node) in x.nodes.iter().enumerate() {

                    // if core is smaller than before -> open new bubble
                    if carc_counts.ncount[node] < lastcore {
                        interval_open.push(TmpPos { acc: x.name.clone(), start: (index - 1) as u32, core: lastcore, node1: lastnode});

                    }
                    // If bigger -> close bubble
                    else if (carc_counts.ncount[node] > lastcore) & (interval_open.len() > 0) {
                        lastcore = carc_counts.ncount[node];


                        // There is no bubble opened with this core level
                        let mut trig = false;

                        // List which open trans are removed later
                        let mut remove_list: Vec<usize> = Vec::new();


                        // We iterate over all open bubbles
                        for (index_open, o_trans) in interval_open.iter().enumerate() {
                            // Check if we find the same core level
                            if (o_trans.core == carc_counts.ncount[node]) | (interval_open[interval_open.len() - 1].core < carc_counts.ncount[node]){
                                trig = true;
                            }


                            // If one open_interval has smaller (or same) core level -> close
                            if o_trans.core <= carc_counts.ncount[node] {
                                // why this?
                                if index != 0 {
                                    result_panSV.push((index1, interval_open[interval_open.len() - 1].start, index as u32, lastcore, interval_open[interval_open.len() - 1].node1, node.clone()));

                                }
                                remove_list.push(index_open);
                            }
                        }
                        // Remove stuff from the interval_open list
                        for (index_r, index_remove) in remove_list.iter().enumerate() {
                            interval_open.remove(*index_remove - index_r);
                        }

                        // If there is not a open interval which has the same core level -> this still exists
                        if !trig {
                            //println!("BIG HIT");
                            result_panSV.push((index1, interval_open[interval_open.len() - 1].start, index as u32, lastcore, interval_open[interval_open.len() - 1].node1, node.clone()));
                        }

                    }
                    lastcore = carc_counts.ncount[node];
                    lastnode = node.clone();

                }
                let mut imut = carc_genome_count.lock().unwrap();
                *imut = *imut + 1;
                debug!("({}/{}) {}", imut, carc_total_len, x.name );
                result_panSV.shrink_to_fit();
            }

            let mut u = carc_results.lock().unwrap();
            for value in result_panSV {
                u.push(value);
            };

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()

    }


    let result_result = Arc::try_unwrap(arc_results).unwrap().into_inner().unwrap();
    result_result
}

pub fn new_bubble(d1:&mut Vec<(usize, u32, u32, u32, u32, u32)>){
    d1.sort_by_key(|a| (a.4, a.5));
    println!("{:?}", d1);

    let mut bubbles = Vec::new();
    let mut intervals = Vec::new();
    let mut f = 0;
    let mut f2 = 0;
    let mut count = 0;
    let mut gg = HashMap::new();

    for x in d1.into_iter(){
        if (x.4 != f) | (x.5 != f2){
            bubbles.push((x.4, x.5, x.3));
            count += 1;
            gg.insert((x.4, x.5), count);
            f = x.4;
            f2 = x.5;
        }
        intervals.push((x.0, x.1, x.2, count));

    }
    println!("{:?}", bubbles);
    println!("{:?}", intervals);

}




/// Detect start and end position of bubbles
///
/// Idea:
/// 1. Iterate over each path
/// 2. Save start, end position (index) + core level of bubbles
/// 3. Save the structure in a HashMap
///
// pub fn algo_panSV_multi(paths: &Vec<NPath>, counts: CountNode, threads: &usize) -> HashMap<String, Vec<PanSVpos>>{
//     info!("Running pan-sv algorithm");
//
//
//
//
//     let chunks = chunk_inplace(paths.clone(), threads.clone());
//     let arc_results = Arc::new(Mutex::new(HashMap::new()));
//     let arc_counts = Arc::new(counts);
//
//
//     // Indexing
//     let total_len = Arc::new(paths.len());
//     let genome_count = Arc::new(Mutex::new(0));
//
//     // Handles
//     let mut handles = Vec::new();
//
//
//     // Iterate over packs of paths
//     for chunk in chunks{
//         let carc_results = arc_results.clone();
//         let carc_counts = arc_counts.clone();
//
//         let carc_genome_count = genome_count.clone();
//         let carc_total_len = total_len.clone();
//
//         let handle = thread::spawn(move || {
//
//             let mut lastcore: u32;
//             let mut result_panSV: HashMap<String, Vec<PanSVpos>> = HashMap::new();
//             for x in chunk.iter(){
//                 let ki: Vec<_> = Vec::new();
//                 result_panSV.insert(x.name.to_owned().clone(), ki);
//             }
//             for x in chunk{
//                 //max_index.insert(x.name.clone(), x.nodes.len()-1);
//                 lastcore = 1;
//
//                 // All "open" intervals
//                 let mut interval_open:  Vec<TmpPos> = Vec::new();
//
//                 // Iterate over all nodes
//                 for (index, node) in x.nodes.iter().enumerate() {
//
//                     // if core is smaller than before -> open new bubble
//                     if carc_counts.ncount[node] < lastcore {
//                         interval_open.push(TmpPos { acc: x.name.clone(), start: (index - 1) as u32, core: lastcore});
//
//                     }
//                     // If bigger -> close bubble
//                     else if (carc_counts.ncount[node] > lastcore) & (interval_open.len() > 0) {
//                         lastcore = carc_counts.ncount[node];
//
//                         // There is no bubble opened with this core level
//                         let mut trig = false;
//
//                         // List which open trans are removed later
//                         let mut remove_list: Vec<usize> = Vec::new();
//
//
//                         // We iterate over all open bubbles
//                         for (index_open, o_trans) in interval_open.iter().enumerate() {
//                             // Check if we find the same core level
//                             if (o_trans.core == carc_counts.ncount[node]) | (interval_open[interval_open.len() - 1].core < carc_counts.ncount[node]){
//                                 trig = true;
//                             }
//
//
//                             // If one open_interval has smaller (or same) core level -> close
//                             if o_trans.core <= carc_counts.ncount[node] {
//                                 // why this?
//                                 if index != 0 {
//                                     result_panSV.get_mut(&o_trans.acc).unwrap().push(PanSVpos {start: o_trans.start, end: index as u32, core: o_trans.core});
//
//                                 }
//                                 remove_list.push(index_open);
//                             }
//                         }
//                         // Remove stuff from the interval_open list
//                         for (index_r, index_remove) in remove_list.iter().enumerate() {
//                             interval_open.remove(*index_remove - index_r);
//                         }
//
//                         // If there is not a open interval which has the same core level -> this still exists
//                         if !trig {
//                             //println!("BIG HIT");
//                             result_panSV.get_mut(&x.name).unwrap().push(PanSVpos {start: interval_open[interval_open.len() - 1].start, end: index as u32, core: lastcore});
//                         }
//
//                     }
//                     lastcore = carc_counts.ncount[node];
//
//                 }
//                 let mut imut = carc_genome_count.lock().unwrap();
//                 *imut = *imut + 1;
//                 debug!("({}/{}) {}", imut, carc_total_len, x.name );
//                 result_panSV.get_mut(&x.name).unwrap().shrink_to_fit();
//             }
//
//             let mut u = carc_results.lock().unwrap();
//             for (key, value) in result_panSV {
//                 u.insert(key, value);
//             };
//
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap()
//
//     }
//
//
//     let result_result = sort_trav(Arc::try_unwrap(arc_results).unwrap().into_inner().unwrap(), threads);
//     result_result
// }

/// Sort the pansv vector
///
/// smallest a into biggest b
pub fn sort_trav(result:  HashMap<String, Vec<PanSVpos>>, threads: &usize) -> HashMap<String, Vec<PanSVpos>>{
    info!("Sorting detected bubbles");

    let mut new_result: HashMap<String, Vec<PanSVpos>> = HashMap::new();
    let mut g: Vec<(String, Vec<PanSVpos>)> = result.into_iter().map(|s| s).collect();
    let leng = g.len();
    let chunks = chunk_inplace(g, threads.clone());
    let (send, rev) = unbounded();

    for chunk in chunks.into_iter() {
        let send = send.clone();
        let handle = thread::spawn(move || {
            for (key, panSV_vec) in chunk.into_iter(){
                let mut panSV_new = panSV_vec.clone();
                panSV_new.sort_by(|a, b| (a.start.cmp(&b.start).then(b.end.cmp(&a.end))));
                send.send((key, panSV_new)).unwrap();


            }
        });
    }
    for x in 0..leng{
        let (key, y) = rev.recv().unwrap();
        new_result.insert(key, y);
    }


    new_result.shrink_to_fit();
    new_result
}

/// Creating bubbles and more
///
/// 1. Iterate over each "path"
///     1. Get the node at each start and end position
///     2. Create a new data: (start_node, end_node, (start_index, end_index, path_id), core number)
/// 2. Create a bubble with:
///     1. id2id = (node1, node2, acc) -> posindex
///     2. intervals = [posindex]
///
/// (start, stop, acc), Vec<(Posindex(start, stop, acc),
pub fn create_bubbles_stupid(input: & HashMap<String, Vec<PanSVpos>>, id2id: &mut HashMap<(u32, u32, u32), u32>, intervals: &mut Vec<Posindex>, paths: &   Vec<NPath>, path2index: &HashMap<String, usize>, threads: &usize) -> Vec<((u32, u32, u32), Vec<(Posindex, u32)>)> {
    info!("Create bubbles");
    let chunks = chunk_inplace(paths.clone(), threads.clone());
    let chunk_n = input.len();


    let arc_input = Arc::new(input.clone());
    let arc_index = Arc::new(Mutex::new(0));
    let arc_total_len = Arc::new(input.len());

    let arc_path2index = Arc::new(path2index.clone());


    let mut handles = Vec::new();
    let (send, rev) = unbounded();


    for chunk in chunks {
        let send = send.clone();
        let carc_index = arc_index.clone();
        let carc_total_len = arc_total_len.clone();
        let carc_input = arc_input.clone();
        let p2i2 = arc_path2index.clone();

        let handle = thread::spawn(move || {

            for path in chunk {
                let mut result = Vec::new();
                let path_id = *p2i2.get(&path.name).unwrap() as u32;
                for pos in carc_input[&path.name].iter() {
                    let m1 = path.nodes[pos.start as usize];
                    let m2 = path.nodes[pos.end as usize];

                    let bub_ids: (u32, u32) = (min(m1, m2), max(m1, m2));
                    let pindex = Posindex { from: pos.start.clone(), to: pos.end.clone(), acc: path_id};
                    result.push((bub_ids, pindex, pos.core));
                }
                result.shrink_to_fit();
                // This is printing
                // let mut imut = carc_index.lock().unwrap();
                // *imut = *imut + 1;
                // debug!("({}/{}) {}", imut, carc_total_len, path.name );
                send.send(result).unwrap();
            }


        });
        handles.push(handle);
    }

    let mut result = HashMap::new();
    for x in 0..chunk_n{
        let v = rev.recv().unwrap();
        add_new_bubbles(v, &mut result);
    }


    let u= bw_index( result, id2id, intervals);
    u
}


#[inline]
/// Convert the data
///
/// Hashmap (node1, node2, accession), [(index1, index2, core)]
///
pub fn add_new_bubbles(input: Vec<((u32, u32), Posindex, u32)>, f: &mut HashMap<(u32, u32, u32), Vec<Posindex>>){
    debug!("Add new bubbles: Index");
    for x in input.into_iter(){
        f.entry((x.0.0, x.0.1, x.2)).and_modify(| e| {e.push(x.1.clone())}).or_insert(vec![(x.1.clone())]);
    }
}



/// Creates bubble wrapper index
/// 1. id2id = (from_index, to_index, acc_id) -> pos_index
/// 2. intervals = [from_index, to_index, acc_id]
/// 3. output: (from_index, to_index, acc_index), Vec<PosIndex>)
pub fn bw_index(input: HashMap<(u32, u32, u32), Vec<Posindex>>, id2id: &mut HashMap<(u32, u32, u32), u32>, intervals: &mut Vec<Posindex>) ->  Vec<((u32, u32, u32), Vec<(Posindex, u32)>)>{
    info!("BW INDEX");
    let mut res1 = Vec::new();


    let mut count = 0;
    // Iterate over all "personal" bubbles and check all intervals
    for (index1, x) in input.into_iter().enumerate(){
        let mut o = Vec::new();
        for y in x.1.into_iter(){
            id2id.insert((y.from, y.to, y.acc), index1 as u32);
            o.push((y.clone(),count));
            intervals.push(y);
            count += 1;

        }
        o.shrink_to_fit();
        res1.push((x.0, o));
    }


    res1
}



/// You have a list of all start and end positions and try to merge those, who are same
///
pub fn merge_traversals(input: Vec<((u32, u32, u32), Vec<(Posindex, u32)>)>, paths: & Vec<NPath>, bubbles: &mut Vec<Bubble>, anchor2bubble: &mut HashMap<(u32, u32), u32>, threads: &usize){
    info!("Merge intervals");
    let chunks = chunk_inplace(input, threads.clone());


    let arc_res = Arc::new(Mutex::new(Vec::new()));
    let arc_p2i = Arc::new(paths.clone());

    let mut handles = Vec::new();


    for chunk in chunks{

        let arc_res2 = arc_res.clone();
        let arc_p2i2 = arc_p2i.clone();

        let handle = thread::spawn(move || {

            let mut gg = Vec::new();
            for bub2trav in chunk.into_iter() {
                let mut ss:  Vec<Vec<(u32, bool)>> = Vec::new();
                let mut go: Vec<Vec<u32>> = Vec::new();
                for (pos, id) in bub2trav.1 {
                    // Man brauch nicht die bub_id sonden die posindex id
                    let idid = id;

                    let p = &arc_p2i2[pos.acc as usize];
                    let k: Vec<u32> = p.nodes[(pos.from + 1) as usize..pos.to as usize].iter().cloned().collect();
                    let k2: Vec<bool> = p.dir[(pos.from + 1) as usize..pos.to as usize].iter().cloned().collect();
                    let mut k10 = Vec::new();
                    for x in 0..k.len() {
                        k10.push((k[x], k2[x]));
                    }


                    if ss.contains(&k10) {
                        let index = ss.iter().position(|r| *r == k10).unwrap();
                        go[index].push(idid.clone())
                    } else {
                        go.push(vec![idid.clone()]);
                        ss.push(k10);
                    }
                }
                go.shrink_to_fit();
                gg.push((bub2trav.0, go));
            }
            gg.shrink_to_fit();
            let mut ff = arc_res2.lock().unwrap();
            ff.push(gg);
            ff.shrink_to_fit();

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }


    let u = Arc::try_unwrap(arc_res).unwrap();
    let mut u = u.into_inner().unwrap();
    u.shrink_to_fit();
    make_bubbles(bubbles, anchor2bubble, u);


}

/// You actually create bubbles
///
pub fn make_bubbles(bubbles: &mut Vec<Bubble>, anchor2bubble: &mut HashMap<(u32, u32), u32>,  u: Vec<Vec<((u32, u32, u32), Vec<Vec<u32>>)>>) {
    info!("Make real bubbles");
    let mut tcount = 0;
    let mut i = 0;
    for x in u{
        for (bub, t) in x.into_iter() {
            anchor2bubble.insert((bub.0, bub.1), i as u32);
            let ll = t.len();
            bubbles.push(Bubble::new(bub.2, bub.0, bub.1, i, t, tcount));
            tcount += ll as u32;
            i += 1;
        }
    }
    bubbles.shrink_to_fit();
    anchor2bubble.shrink_to_fit();
}


/// Indel detection
///
/// Iterate over nodes in path
/// If two nodes after each othera are borders of bubbles
/// Add traversal to bubble
pub fn indel_detection(bubbles: &mut Vec<Bubble>, anchor2bubble: & HashMap<(u32, u32), u32>, id2id: &mut HashMap<(u32, u32, u32), u32>, intervals: &mut Vec<Posindex>, paths: &Vec<NPath>, last_id: u32){
    info!("InDel detection");
    let mut ll = last_id.clone();
    let mut last_tra = last_id.clone();

    for (i, path) in paths.iter().enumerate(){
        for x in 0..path.nodes.len()-1{
            let m1 = path.nodes[x];
            let m2 = path.nodes[x+1];
            let ind: (u32, u32) = (min(m1, m2 ), max(m1, m2));
            if anchor2bubble.contains_key(&ind){

                let bub =  bubbles.get_mut(*anchor2bubble.get(&ind).unwrap() as usize).unwrap();
                //if ! bub.acc.contains(& path.name) {

                let trav: Traversal = Traversal {pos: vec![ll], id: last_tra, length: 0};
                intervals.push(Posindex { from: (x as u32), to: ((x + 1) as u32), acc: i as u32});
                id2id.insert(((x as u32), ((x + 1) as u32), i as u32), bub.id.clone());
                bub.traversals.push(trav);
                ll += 1;
                last_tra += 1;

            }
        }
    }
}

/// Wrapper for connecting bubbles multithreaded
///
///
pub fn connect_bubbles_multi(hm: HashMap<String, Vec<PanSVpos>>, bubbles: &mut Vec<Bubble>, id2id: HashMap<(u32, u32, u32), u32>, p2i: &HashMap<String, usize>, threads: &usize) -> (HashMap<(u32, u32, u32), u32>, Vec<Vec<u32>>, Vec<Vec<u32>>){
    info!("Connect bubbles");

    let ff = hm.len().clone();
    let mut g: Vec<(String, Vec<PanSVpos>)> = hm.into_iter().map(|s| s).collect();
    g.shrink_to_fit();

    // For Counting
    let total_len = Arc::new(g.len() as f64);
    let genome_count = Arc::new(Mutex::new(0));

    let chunks = chunk_inplace(g, threads.clone());
    //let arc_result = Arc::new(Mutex::new(Vec::new()));

    let arc_p2i = Arc::new(p2i.clone());

    //let mut handles = Vec::new();


    let arc_id2id = Arc::new(id2id);


    let (send, rev) = unbounded();


    for chunk in chunks{
        let send = send.clone();
        //let j = rr.clone();
        let carc_genome_count = genome_count.clone();
        let carc_total_len = total_len.clone();
        //let card_result = arc_result.clone();

        let carc_p2i = arc_p2i.clone();
        let card_id2id = arc_id2id.clone();

        thread::spawn(move || {
            //let mut gg = vec![];
            let mut c = 0;
            for (k,v) in chunk.into_iter(){
                let mut vv =  vec![];

                let start_end = v.into_iter().map(|s| (s.start, s.end)).collect();
                let mut network = related_intervals::create_network_hashmap(&start_end);

                make_nested(&start_end, & mut network);
                let path2index_var = &(carc_p2i.get(&k).unwrap().clone() as u32);


                // Writing
                // let mut imut = carc_genome_count.lock().unwrap();
                // *imut = *imut + 1;
                // debug!("({}/{}) {}", imut, carc_total_len, k);
                let mut rr = HashMap::new();
                merge_bubbles(network, & mut rr, &card_id2id, &path2index_var);
                vv.push(rr);
                send.send(vv).unwrap();
            }
        });
    }
    let mut gg: Vec<Vec<u32>> = vec![Vec::new(); bubbles.len()];
    let mut gg2: Vec<Vec<u32>> = vec![Vec::new(); bubbles.len()];


    info!("Merge in bubble space");
    for x in 0..ff{
        for y in rev.recv().unwrap(){
            in_bubbles2(y, &mut gg, &mut gg2);
        }
    }
    let f1 = clean_up(gg);
    let f2 = clean_up(gg2);

    // get it back
    let id2id2 = Arc::try_unwrap(arc_id2id).unwrap();

    (id2id2, f1, f2)

}

/// Take the "network" and merge it into the graph structure
///
pub fn merge_bubbles(hm: HashMap<(u32, u32), Network>, result: &mut HashMap<u32,   HashSet<u32>>, bw: &Arc<HashMap<(u32, u32, u32), u32>>, s: &u32){
    let s2 = s.clone();
    for (k,v) in hm.into_iter() {
        let bub_id = bw.get(&(k.0, k.1, s2)).unwrap();
        for x in v.parent.into_iter() {
            let id = bw.get(&(x.0, x.1, s2)).unwrap().clone();
            result.entry(*bub_id).and_modify(|e| { e.insert(id); }).or_insert(HashSet::from([id]));
        }
    }
}

/// Add children and parents to bubble data structure
pub fn in_bubbles2(result: HashMap<u32, HashSet<u32>>, bw: &mut Vec<Vec<u32>>, bw2: &mut Vec<Vec<u32>>){
    for (bub_id, hs) in result.into_iter(){
        for x in hs.into_iter() {
            bw[x as usize].push(bub_id);
            bw2[bub_id as usize].push(x);

        }
    }
}

pub fn clean_up(input: Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    let mut data: Vec<Vec<u32>> = Vec::new();
    for x in input.into_iter(){
        let f: HashSet<u32> = x.into_iter().collect();
        let f2: Vec<u32> = f.into_iter().collect();
        data.push(f2);
    }
    return data
}

// /// Add children and parents to bubble data structure
// pub fn in_bubbles(result: HashMap<u32, HashSet<u32>>, bw: &mut Vec<Bubble>){
//     for (bub_id, hs) in result.into_iter(){
//         for x in hs.into_iter() {
//             bw.get_mut(x as usize).unwrap().children.insert(bub_id);
//             bw.get_mut(bub_id as usize).unwrap().parents.insert(x);
//         }
//     }
// }

/// Checking bubble size
/// TODO
/// - Categories (additional function)
/// -
pub fn check_bubble_size(bubbles: &mut Vec<Bubble>){

    for y in 0..bubbles.len(){
        let mut min = u32::MAX;

        let mut max = u32::MIN;
        let bubble = &mut bubbles[y];
        for x in bubble.traversals.iter() {
            if x.length > max {
                max = x.length.clone();
            }
            if x.length < min {
                min = x.length.clone();
            }
        }


        bubble.ratio = min as f32/max as f32;
        if max > 50{
            bubble.small = false;
            if min == 0{
                bubble.category = 3;
            } else if bubble.ratio > 0.8 {
                bubble.category = 4;
            } else {
                bubble.category = 5;
            }

        } else {
            if (min == 1) & (max == 1){
                bubble.category = 0;
            }
            else if min == 0{
                bubble.category = 1;
            }
            else {
                bubble.category = 2;
            }
        }

    }

}


#[allow(dead_code)]
/// Get the real nestedness
pub fn nester_wrapper(bubbles: & mut Vec<Bubble>, par1: & Vec<Vec<u32>>){
    for x in 0..bubbles.len(){
        let level = nester_rec(x as u32, bubbles, par1) as u16;
        let g = bubbles.get_mut(x).unwrap();
        g.nestedness = level;
    }
}

#[allow(dead_code)]
/// Function for nest_version2
pub fn nester_rec(id: u32, bubble: & mut Vec<Bubble>, par1: & Vec<Vec<u32>>) -> usize{
    let mut bubble_new = bubble.get(id as usize).unwrap();
    let mut  count: usize = 0;
    loop {
        count += 1;
        if par1.get(bubble_new.id as usize).unwrap().len() == 0{
            break
        }

        let parent = par1.get(bubble_new.id as usize).unwrap().iter().next().unwrap().clone();

        bubble_new = bubble.get(parent as usize).unwrap();
    }
     return count
}