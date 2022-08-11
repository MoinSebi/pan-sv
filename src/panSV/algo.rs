use std::cmp::{max, min};
use std::collections::{BTreeSet};
use crate::core::counting::{CountNode};
use crate::panSV::panSV_core::{PanSVpos, TmpPos, BubbleWrapper};
use crate::core::core::{Posindex, Bubble, Traversal};
use related_intervals::{make_nested, Network};
use gfaR_wrapper::NPath;
use std::io::{self, Write};
use std::iter::FromIterator;
use std::ops::Add;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use bifurcation::helper::chunk_inplace;
use hashbrown::HashMap;
use log::{debug, info};


pub fn algo_panSV_multi(paths: &Vec<NPath>, counts: &CountNode, threads: &usize) -> HashMap<String, Vec<PanSVpos>>{

    let mut result_panSV2: HashMap<String, Vec<PanSVpos>> = HashMap::new();
    let mut max_index: HashMap<String, usize> = HashMap::new();
    let mut index = 0;
    // We create String -> (start, stop, core)
    for x in paths.iter(){
        let ki: Vec<_> = Vec::new();
        result_panSV2.insert(x.name.to_owned().clone(), ki);
    }
    let mut ko = HashMap::new();
    let chunks = chunk_inplace(paths.clone(), threads.clone());
    let rr = Arc::new(Mutex::new(ko));
    let op1 = Arc::new(counts.clone());
    let ll = paths.len();
    let total_len = Arc::new(ll);
    let ii = Arc::new(Mutex::new(index));
    let mut handles = Vec::new();
    for chunk in chunks{
        let j = rr.clone();
        let op = op1.clone();
        let i2 = ii.clone();
        let lo = total_len.clone();
        let handle = thread::spawn(move || {

            let mut lastcore: u32;
            let p = 10;

            let mut result_panSV: HashMap<String, Vec<PanSVpos>> = HashMap::new();
            for x in chunk.iter(){
                let ki: Vec<_> = Vec::new();
                result_panSV.insert(x.name.to_owned().clone(), ki);
            }
            for x in chunk{
                //max_index.insert(x.name.clone(), x.nodes.len()-1);
                lastcore = 1;

                // All "open" intervals
                let mut interval_open:  Vec<TmpPos> = Vec::new();

                io::stderr().flush().unwrap();
                // Iterate over all nodes
                for (index, node) in x.nodes.iter().enumerate() {

                    // if core is smaller than before -> open new bubble
                    if op.ncount[node] < lastcore {
                        interval_open.push(TmpPos { acc: x.name.clone(), start: (index - 1) as u32, core: lastcore});

                    }
                    // If bigger -> close bubble
                    else if (op.ncount[node] > lastcore) & (interval_open.len() > 0) {
                        lastcore = op.ncount[node];

                        // There is no bubble opened with this core level
                        let mut trig = false;

                        // List which open trans are removed later
                        let mut remove_list: Vec<usize> = Vec::new();


                        // We iterate over all open bubbles
                        for (index_open, o_trans) in interval_open.iter().enumerate() {
                            // Check if we find the same core level
                            if (o_trans.core == op.ncount[node]) | (interval_open[interval_open.len() - 1].core < op.ncount[node]){
                                trig = true;
                            }


                            // If one open_interval has smaller (or same) core level -> close
                            if o_trans.core <= op.ncount[node] {
                                // why this?
                                if index != 0 {
                                    result_panSV.get_mut(&o_trans.acc).unwrap().push(PanSVpos {start: o_trans.start, end: index as u32, core: o_trans.core});

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
                            result_panSV.get_mut(&x.name).unwrap().push(PanSVpos {start: interval_open[interval_open.len() - 1].start, end: index as u32, core: lastcore});
                        }

                    }
                    lastcore = op.ncount[node];

                }
                let mut imut = i2.lock().unwrap();
                *imut = *imut + 1;
                info!("({}/{}) {}", imut, lo, x.name );
            }


            let mut u = j.lock().unwrap();
            for (key, value) in result_panSV.iter() {
                u.insert(key.clone(), value.clone());
            };

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()

    }
    let mut result = HashMap::new();
    for x in rr.lock().unwrap().iter(){
        result.insert(x.0.clone(), x.1.clone());
    }


    let result_result = sort_trav(result);
    result_result
}

/// Sorting vector in hashmaps
///
/// smallest a into biggest b
pub fn sort_trav(result:  HashMap<String, Vec<PanSVpos>>) -> HashMap<String, Vec<PanSVpos>>{

    let mut new_result: HashMap<String, Vec<PanSVpos>> = HashMap::new();


    for (key, panSV_vec) in result.iter(){
        let mut panSV_new = Vec::new();
        for entry in panSV_vec.iter(){
            panSV_new.push(entry.clone());
        }
        panSV_new.sort_by(|a, b| (a.start.cmp(&b.start).then(b.end.cmp(&a.end))));
        new_result.insert(key.to_owned().clone(), panSV_new) ;
        //v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    }
    new_result.shrink_to_fit();
    new_result
}


pub fn create_bubbles_stupid(input: & HashMap<String, Vec<PanSVpos>>, paths: &   Vec<NPath>, path2index: &HashMap<String, usize>, threads: &usize) -> (HashMap<(u32, u32, u32), Vec<(Posindex, u32)>>, BubbleWrapper) {
    info!("Create bubbles");
    let chunks = chunk_inplace(paths.clone(), threads.clone());

    let arc_input = Arc::new(input.clone());
    let arc_index = Arc::new(Mutex::new(0));
    let arc_total_len = Arc::new(input.len());

    let mut output:HashMap<(u32, u32, u32), Vec<(Posindex)>> = HashMap::new();
    let arc_output = Arc::new(Mutex::new(output));
    let arc_path2index = Arc::new(path2index.clone());


    let mut handles = Vec::new();


    for chunk in chunks {
        let carc_index = arc_index.clone();
        let carc_total_len = arc_total_len.clone();

        let carc_input = arc_input.clone();
        let p2i2 = arc_path2index.clone();
        let arc_yo2 = arc_output.clone();

        let handle = thread::spawn(move || {

            for path in chunk {
                let mut result = Vec::new();
                for pos in carc_input[&path.name].iter() {
                    let m1 = path.nodes[pos.start as usize];
                    let m2 = path.nodes[pos.end as usize];
                    let bub_ids: (u32, u32) = (min(m1, m2), max(m1, m2));
                    let pindex = Posindex { from: pos.start.clone(), to: pos.end.clone(), acc: *p2i2.get(&path.name).unwrap() as u32 };
                    result.push((bub_ids, pindex, pos.core));
                }
                result.shrink_to_fit();
                let mut h = arc_yo2.lock().unwrap();
                add_new_bubbles(result, &mut h);
                let mut imut = carc_index.lock().unwrap();
                *imut = *imut + 1;
                info!("({}/{}) {}", imut, carc_total_len, path.name );
            }

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }


    let u = Arc::try_unwrap(arc_output).unwrap();
    let mut u = u.into_inner().unwrap();
    let (u,p) = bw_index(u);
    (u, p)
}


#[inline]
pub fn add_new_bubbles(input: Vec<((u32, u32), Posindex, u32)>, f: &mut MutexGuard<HashMap<(u32, u32, u32), Vec<Posindex>>>){
    debug!("Add new bubbles: Index");
    for (i,x) in input.iter().enumerate(){
        f.entry((x.0.0, x.0.1, x.2)).and_modify(| e| {e.push((x.1.clone()))}).or_insert(vec![(x.1.clone())]);
    }
}




pub fn bw_index(input: HashMap<(u32, u32, u32), Vec<Posindex>>) ->  (HashMap<(u32, u32, u32), Vec<(Posindex, u32)>>, BubbleWrapper){
    debug!("BW INDEX");
    let mut bw = BubbleWrapper::new();
    let mut res1 = HashMap::new();

    let mut count = 0;
    for (i,x) in input.iter().enumerate(){
        let mut o = Vec::new();
        for y in x.1.iter(){
            bw.intervals.push(y.clone());
            bw.id2id.insert((y.from.clone(), y.to.clone(), y.acc.clone()), i as u32);
            o.push((y.clone(),count));
            count += 1;

        }
        res1.insert(x.0.clone(), o);
    }
    bw.intervals.shrink_to_fit();
    bw.id2id.shrink_to_fit();
    (res1, bw)
}

pub fn merge_traversals(input: HashMap<(u32, u32, u32), Vec<(Posindex, u32)>>, paths: &   Vec<NPath>, path2index: &HashMap<String, usize>, bw: &mut BubbleWrapper){
    info!("Merge traversals");


    let f: Vec<((u32, u32, u32), Vec<(Posindex, u32)>)> = input.into_iter().map(|(x,y)| (x,y)).collect();
    let chunks = chunk_inplace(f, 2);
    let mut res = Vec::new();
    let arc_res = Arc::new(Mutex::new(res));
    let arc_p2i = Arc::new(paths.clone());
    let arc_bw = Arc::new(bw.clone());
    let mut handles = Vec::new();
    for x in chunks{

        let arc_res2 = arc_res.clone();
        let arc_p2i2 = arc_p2i.clone();
        let arc_bw2 = arc_bw.clone();

        let handle = thread::spawn(move || {
            let mut gg = Vec::new();
            for y in x{
                let mut ss:  Vec<Vec<(u32, bool)>> = Vec::new();
                let mut go: Vec<Vec<u32>> = Vec::new();
                for (pos, id) in y.1 {
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

                gg.push((y.0, go));
            }
                let mut ff = arc_res2.lock().unwrap();
                ff.extend(gg);

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }


    let u = Arc::try_unwrap(arc_res).unwrap();
    let mut u = u.into_inner().unwrap();
    make_bubbles(bw, u);


}


pub fn make_bubbles(bw: &mut BubbleWrapper,  u: Vec<((u32, u32, u32), Vec<Vec<u32>>)>){
    info!("Make real bubbles");
    let mut tcount = 0;
    let mut i = 0;
    for (bub, t) in u {
        bw.anchor2bubble.insert((bub.0, bub.1), i as u32);
        let ll = t.len();
        bw.bubbles.push(Bubble::new2(bub.2, bub.0, bub.1, i, t, tcount));
        tcount += ll as u32;
        i += 1;
    }
    bw.bubbles.shrink_to_fit();
    bw.anchor2bubble.shrink_to_fit();
}




// /// We make bubbles *_* lets go!
// ///
// ///
// pub fn create_bubbles(inp: & HashMap<String, Vec<PanSVpos>>, p: &   Vec<NPath>, ghm: & HashMap<String, Vec<usize>>, path2index: &HashMap<String, usize>, threads : &usize) -> BubbleWrapper{
//
//     info!("Create bubbles");
//     let mut result: BubbleWrapper = BubbleWrapper::new();
//
//     let mut tcount = 0;
//     let mut bcount = 0;
//     let mut trcount = 0;
//
//     for (i,x) in p.iter().enumerate(){
//
//         info!("({}/{}) {}\r", i+1, p.len(), x.name);
//         io::stdout().flush().unwrap();
//
//
//         for pos in inp[&x.name].iter(){
//
//             // Make start and end position
//             let newbub: (u32, u32) = (min(x.nodes[pos.start as usize], x.nodes[pos.end as usize]), max(x.nodes[pos.start as usize], x.nodes[pos.end as usize]));
//
//             // Len of the traversal
//             let len_trav: usize  = ghm.get(&x.name).unwrap()[pos.end as usize-1] -  ghm.get(&x.name).unwrap()[pos.start as usize];
//
//
//             let tt = Traversal{length: len_trav as u32, pos: vec![tcount], id: 0};
//             let k: Vec<u32> = x.nodes[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();
//             let k2: Vec<bool> = x.dir[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();
//
//             let mut k10: Vec<(u32, bool)> = Vec::new();
//             for x in 0..k.len(){
//                 k10.push((k[x], k2[x]));
//             }
//
//             /*
//             If we have the bubble
//             -> check if traversal in bubble
//                 yes -> add pos to traversal
//                 no -> create new traversal, add pos
//             add all to the bubble
//             */
//             if result.anchor2bubble.contains_key(&newbub){
//
//                 // make traversal
//                 // Vec -> meta
//                 //println!("{:?}", k2);
//
//
//                 // This bubble we are looking at
//                 let temp_bcount = result.anchor2bubble.get(&newbub).unwrap();
//                 let bub = result.bubbles.get_mut(*temp_bcount as usize).unwrap();
//                 //result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
//                 let bub_id = bub.id.clone();
//
//                 // Check if traversal already there
//                 if bub.traversals.contains_key(&k10){
//                     result.bubbles.get_mut(*temp_bcount as usize).unwrap().traversals.get_mut(&k10).unwrap().add_pos(tcount);
//
//                     //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k).unwrap().addPos(tcount);
//                 }
//                 else {
//
//                     result.bubbles.get_mut(*temp_bcount as usize).unwrap().traversals.insert(k10.clone(), tt);
//                     result.bubbles.get_mut(*temp_bcount as usize).unwrap().traversals.get_mut(&k10).unwrap().id = trcount;
//                     trcount += 1;
//                     //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k,tt);
//
//                 }
//
//                 result.id2id.insert((pos.start.clone(), pos.end.clone(), i as u32), bub_id);
//                 result.anchor2bubble.insert(newbub, bub_id);
//
//
//
//
//
//                 //pV.id2bubble.get_mut(& pV.Anchor2bubble[&newbub]).unwrap().addPos(tcount);
//
//             } else {
//                 /*
//                 Create new bubble
//                 Create new traversal
//                 Create pos
//
//                  please save how to make vector -> Btree
//                  */
//                 // Make traversal
//
//
//                 result.anchor2bubble.insert(newbub, bcount);
//                 result.bubbles.push(Bubble::new(pos.core.clone(), x.nodes[pos.start as usize].clone(), x.nodes[pos.end as usize].clone(),
//                                                 tcount, bcount, tt, k10.clone()));
//                 result.bubbles.get_mut(bcount as usize).unwrap().traversals.get_mut(&k10).unwrap().id = trcount;
//
//                 //result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
//                 result.id2id.insert((pos.start.clone(), pos.end.clone(), i as u32), bcount);
//                 trcount += 1;
//
//
//
//
//                 bcount += 1;
//             }
//             result.intervals.push(Posindex {from: pos.start.clone(), to: pos.end.clone(), acc: i as u32});
//
//             tcount += 1;
//
//
//         }
//
//     }
//     result.bubbles.shrink_to_fit();
//     result.anchor2bubble.shrink_to_fit();
//     result.intervals.shrink_to_fit();
//     result.id2id.shrink_to_fit();
//     // Connect bubbles
//     connect_bubbles_multi(inp, result, path2index, threads)
//
//
// }

// /// Indel detection
// ///
// /// Iterate over nodes in path
// /// If two nodes after each othera are borders of bubbles
// /// Add traversal to bubble
// pub fn indel_detection(r: & mut BubbleWrapper, paths: &Vec<NPath>, last_id: u32){
//     let mut ll = last_id.clone();
//
//     for (i, path) in paths.iter().enumerate(){
//         for x in 0..path.nodes.len()-1{
//             let mut ind: (u32, u32) = (min(path.nodes[x],path.nodes[x+1] ), max(path.nodes[x],path.nodes[x+1]));
//             if r.anchor2bubble.contains_key(&ind){
//
//                 let bub =  r.bubbles.get_mut(*r.anchor2bubble.get(&ind).unwrap() as usize).unwrap();
//                 //if ! bub.acc.contains(& path.name) {
//
//                 let k: Vec<(u32, bool)> = vec![];
//                 let jo: Traversal = Traversal::new(ll, 0);
//                 r.intervals.push(Posindex { from: (x as u32), to: ((x + 1) as u32), acc: i as u32});
//                 r.id2id.insert(((x as u32), ((x + 1) as u32), i as u32), bub.id.clone());
//                 bub.traversals.push(ll.clone());
//                 ll += 1;
//
//                 //}
//
//             }
//         }
//     }
// }

/// Wrapper for connecting bubbles multithreaded
///
///
pub fn connect_bubbles_multi(hm: &HashMap<String, Vec<PanSVpos>>, result:  BubbleWrapper, p2i: &HashMap<String, usize>, threads: &usize) -> BubbleWrapper{
    info!("Connect bubbles");

    let mut g = Vec::new();
    for (k,v) in hm.iter(){
        g.push((k.clone(),v.clone()));
    }

    let chunks = chunk_inplace(g, threads.clone());
    //let rr = Arc::new(Mutex::new(Vec::new()));
    let mut go = Arc::new(Mutex::new(0));

    let test = Arc::new(Mutex::new(result));
    let te = Arc::new(p2i.clone());

    let mut handles = Vec::new();
    let total_len = Arc::new(hm.len());


    for chunk in chunks{
        //let j = rr.clone();
        let i2 = go.clone();
        let lo = total_len.clone();

        let test2 = test.clone();
        let te2 = te.clone();

        let handle = thread::spawn(move || {
            for (i ,(k,v)) in chunk.iter().enumerate(){
                let mut jo: Vec<(u32, u32)> = Vec::new();
                for x in v.iter() {
                    jo.push((x.start.clone(), x.end.clone()));
                }
                let mut network = related_intervals::create_network_hashmap(&jo);
                make_nested(&jo, & mut network);

                let mut test3 = test2.lock().unwrap();
                let ote = &(te2.get(k).unwrap().clone() as u32);
                connect_bubbles(&network, test3, ote);

                // let mut rrr = j.lock().unwrap();
                // rrr.push((k.clone(), network));

                let mut imut = i2.lock().unwrap();
                *imut = *imut + 1;
                info!("({}/{}) {}", imut, lo, k);

            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()

    }
    // info!("Connecting");
    // for (k,v) in rr.lock().unwrap().iter(){
    //     connect_bubbles(&v, result, &(p2i.get(k).unwrap().clone() as u32))
    // }

    let u = Arc::try_unwrap(test).unwrap();
    let mut u = u.into_inner().unwrap();
    u.bubbles.shrink_to_fit();
    u.anchor2bubble.shrink_to_fit();
    u.intervals.shrink_to_fit();
    u.id2id.shrink_to_fit();
    u

}

/// Conntect bubbles and add children and parents
pub fn connect_bubbles(hm: &std::collections::HashMap<(u32, u32), Network>, mut result: MutexGuard<BubbleWrapper>, s: &u32){
    for (k,v) in hm.iter(){
        let index = &result.id2id.get(&(k.0, k.1, s.clone())).unwrap().clone();
        let mut ii: Vec<u32> = Vec::new();
        for x in v.parent.iter(){
            let ko = result.id2id.get(&(x.0, x.1, s.clone())).unwrap().clone();
            ii.push(ko);
        }
        for x in ii.iter(){
            result.bubbles.get_mut(*x as usize).unwrap().children.insert(index.clone());
            result.bubbles.get_mut(*index as usize).unwrap().parents.insert(x.clone().clone());
        }

    }

}

/// Checking bubble size
/// TODO
/// - Categories (additional function)
/// -
pub fn check_bubble_size(h: & mut BubbleWrapper){

    for x in 0..h.bubbles.len(){
        let mut min = u32::MAX;

        let mut max = u32::MIN;
        let bubble = h.bubbles.get_mut(x).unwrap();
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

// #[allow(dead_code)]
// /// Wrapper for nestedness function
// pub fn nest_wrapper(h: & mut BubbleWrapper){
//     for x in 0..h.id2bubble.len(){
//         let k = h.id2bubble.get_mut(&(x as u32)).unwrap();
//         if k.parents.len() == 0{
//             k.nestedness = 1;
//             let kk = k.children.clone();
//             let mut seen = HashSet::new();
//             seen.insert(k.id);
//             for ok in kk.iter(){
//                 nest_function(h, &ok, 2, &mut seen);
//             }
//
//         }
//     }
// }
//
// #[allow(dead_code)]
// /// Get nestedness functions
// pub fn nest_function(h: & mut BubbleWrapper, id: &u32, core: u16, seen: & mut HashSet<u32>){
//     let k = h.id2bubble.get_mut(id).unwrap();
//
//     if k.nestedness != 0{
//         k.nestedness = min(k.nestedness, core)
//     } else {
//         k.nestedness = core
//     }
//     if k.children.len() > 0{
//         let kk = k.children.clone();
//         for x in kk.iter(){
//             let mut seen2 = seen.clone();
//             seen2.insert(k.id);
//             if seen2.contains(x){
//                 nest_function(h, &x, core +1, & mut seen2)
//             }
//         }
//     }
// }



#[allow(dead_code)]
/// Get the real nestedness
pub fn nest_version2(h: & mut BubbleWrapper){
    for x in 0..h.bubbles.len(){
        let level = go1(x as u32, h) as u16;
        let g = h.bubbles.get_mut(x).unwrap();
        g.nestedness = level;
    }
}

#[allow(dead_code)]
/// Function for nest_version2
pub fn go1(id: u32, h: & mut BubbleWrapper) -> usize{
    let mut bubble = h.bubbles.get(id as usize).unwrap();
    let mut  count: usize = 0;
    loop {
        count += 1;
        if bubble.parents.len() == 0{
            break
        }
        eprintln!("parent {}", bubble.parents.len());
        let parent = bubble.parents.iter().next().unwrap().clone();
        eprintln!("parent2 {}", bubble.parents.len());

        bubble = h.bubbles.get(parent as usize).unwrap();
    }
     return count
}