use std::collections::{HashMap, BTreeSet};
use crate::core::counting::{CountNode};
use crate::panSV::panSV_core::{PanSVpos, TmpPos, BubbleWrapper};
use crate::core::core::{Posindex, Bubble, Traversal};
use related_intervals::{make_nested, Network};
use gfaR_wrapper::NPath;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use bifurcation::helper::chunk_inplace;
use log::info;


#[allow(non_snake_case)]
/// PanSV algorithm
///
/// Input: paths, counts
/// Output: HM(String -> Vec<pos>)
/// panSVpos: index, index, core
///
pub fn algo_panSV(paths: & Vec<NPath>, counts: &CountNode) -> (HashMap<String, Vec<PanSVpos>>, HashMap<String, usize>) {
    info!("PanSV running");

    let mut lastcore: u32;
    #[allow(non_snake_case)]
        let mut result_panSV: HashMap<String, Vec<PanSVpos>> = HashMap::new();

    let mut max_index: HashMap<String, usize> = HashMap::new();

    // We create String -> (start, stop, core)
    for x in paths{
        let ki: Vec<_> = Vec::new();
        result_panSV.insert(x.name.to_owned().clone(), ki);
    }


    // Iterate over each path
    for (i, x) in paths.iter().enumerate() {
        // Need max index later
        max_index.insert(x.name.clone(), x.nodes.len()-1);
        lastcore = 1;

        // All "open" intervals
        let mut interval_open:  Vec<TmpPos> = Vec::new();

        info!("({}/{}) {}\r", i+1, paths.len(), x.name);
        io::stderr().flush().unwrap();
        // Iterate over all nodes
        for (index, node) in x.nodes.iter().enumerate() {

            // if core is smaller than before -> open new bubble
            if counts.ncount[node] < lastcore {
                interval_open.push(TmpPos { acc: x.name.clone(), start: (index - 1) as u32, core: lastcore});

            }
            // If bigger -> close bubble
            else if (counts.ncount[node] > lastcore) & (interval_open.len() > 0) {
                lastcore = counts.ncount[node];

                // There is no bubble opened with this core level
                let mut trig = false;

                // List which open trans are removed later
                let mut remove_list: Vec<usize> = Vec::new();


                // We iterate over all open bubbles
                for (index_open, o_trans) in interval_open.iter().enumerate() {
                    // Check if we find the same core level
                    if (o_trans.core == counts.ncount[node]) | (interval_open[interval_open.len() - 1].core < counts.ncount[node]){
                        trig = true;
                    }


                    // If one open_interval has smaller (or same) core level -> close
                    if o_trans.core <= counts.ncount[node] {
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
            lastcore = counts.ncount[node];

        }
        // This the other end - its one longer than the rest (identifier)

    }
    let result_result = sort_trav(result_panSV);
    //println!("{:?}", result_result);

    (result_result, max_index)
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
    new_result
}







/// We make bubbles *_* lets go!
///
///
pub fn create_bubbles<'a, 'b>(inp: &'a HashMap<String, Vec<PanSVpos>>, p: &'a   Vec<NPath>, ghm: &'b HashMap<String, Vec<usize>>, threads : &usize) -> BubbleWrapper<'a>{

    info!("Create bubbles");
    let mut result: BubbleWrapper = BubbleWrapper::new();

    let mut tcount = 0;
    let mut bcount = 0;
    let mut trcount = 0;

    for (i,x) in p.iter().enumerate(){

        info!("({}/{}) {}\r", i+1, p.len(), x.name);
        io::stdout().flush().unwrap();


        for pos in inp[&x.name].iter(){

            // Make start and end position
            let mut newbub = BTreeSet::new();
            newbub.insert(& x.nodes[pos.start as usize]);
            newbub.insert(& x.nodes[pos.end as usize]);

            // Len of the traversal
            let len_trav: usize  = ghm.get(&x.name).unwrap()[pos.end as usize-1] -  ghm.get(&x.name).unwrap()[pos.start as usize];


            let tt = Traversal{length: len_trav as u32, pos: vec![tcount], id: 0};
            let k: Vec<u32> = x.nodes[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();
            let k2: Vec<bool> = x.dir[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();

            let mut k10: Vec<(u32, bool)> = Vec::new();
            for x in 0..k.len(){
                k10.push((k[x], k2[x]));
            }

            /*
            If we have the bubble
            -> check if traversal in bubble
                yes -> add pos to traversal
                no -> create new traversal, add pos
            add all to the bubble
            */
            if result.anchor2bubble.contains_key(&newbub){

                // make traversal
                // Vec -> meta
                //println!("{:?}", k2);


                // This bubble we are looking at
                let temp_bcount = result.anchor2bubble.get(&newbub).unwrap();
                let bub = result.id2bubble.get_mut(temp_bcount).unwrap();
                result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
                let bub_id = bub.id.clone();

                // Check if traversal already there
                if bub.traversals.contains_key(&k10){
                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k10).unwrap().add_pos(tcount);

                    //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k).unwrap().addPos(tcount);
                }
                else {

                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k10.clone(),tt);
                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k10).unwrap().id = trcount;
                    trcount += 1;
                    //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k,tt);

                }

                result.id2id.insert((pos.start.clone(), pos.end.clone(), &x.name), bub_id);
                result.anchor2bubble.insert(newbub, bub_id);





                //pV.id2bubble.get_mut(& pV.Anchor2bubble[&newbub]).unwrap().addPos(tcount);

            } else {
                /*
                Create new bubble
                Create new traversal
                Create pos

                 please save how to make vector -> Btree
                 */
                // Make traversal


                result.anchor2bubble.insert(newbub, bcount);
                result.id2bubble.insert(bcount, Bubble::new(pos.core.clone(), x.nodes[pos.start as usize].clone(), x.nodes[pos.end as usize].clone(),
                                                            tcount, bcount, tt, k10.clone()));
                result.id2bubble.get_mut(&bcount).unwrap().traversals.get_mut(&k10).unwrap().id = trcount;

                result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
                result.id2id.insert((pos.start.clone(), pos.end.clone(), &x.name), bcount);
                trcount += 1;




                bcount += 1;
            }
            result.id2interval.insert(tcount, Posindex {from: pos.start.clone(), to: pos.end.clone(), acc: x.name.clone()});

            tcount += 1;


        }

    }
    // Connect bubbles
    connect_bubbles_multi(inp, & mut result, threads);

    result

}

/// Indel detection
///
/// Iterate over nodes in path
/// If two nodes after each othera are borders of bubbles
/// Add traversal to bubble
pub fn indel_detection<'a>(r: & mut BubbleWrapper<'a>, paths: &'a Vec<NPath>, last_id: u32){
    let mut ll = last_id.clone() + 1;

    for path in paths.iter(){
        for x in 0..path.nodes.len()-1{
            let mut ind = BTreeSet::new();
            ind.insert(&path.nodes[x]);
            ind.insert(&path.nodes[x+1]);
            if r.anchor2bubble.contains_key(&ind){

                let bub =  r.id2bubble.get_mut(r.anchor2bubble.get(&ind).unwrap()).unwrap();
                //if ! bub.acc.contains(& path.name) {

                let k: Vec<(u32, bool)> = vec![];
                let jo: Traversal = Traversal::new(ll, 0);
                r.id2interval.insert(ll, Posindex { from: (x as u32), to: ((x + 1) as u32), acc: path.name.clone()});
                r.id2id.insert(((x as u32), ((x + 1) as u32), &path.name), bub.id.clone());
                if bub.traversals.contains_key(&k) {
                    bub.traversals.get_mut(&k).unwrap().pos.push(ll);
                } else {
                    bub.traversals.insert(k.clone(), jo);
                }
                ll += 1;

                //}

            }
        }
    }
}

pub fn connect_bubbles_multi(hm: &HashMap<String, Vec<PanSVpos>>, result: &  mut BubbleWrapper, threads: &usize){


    let mut g = Vec::new();
    for (k,v) in hm.iter(){
        g.push((k.clone(),v.clone()));
    }

    let chunks = chunk_inplace(g, threads.clone());
    let rr = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for chunk in chunks{
        let j = rr.clone();
        let handle = thread::spawn(move || {
            for (i ,(k,v)) in chunk.iter().enumerate(){

                info!("({}/{}) {}\r", i+1, 10, k);
                io::stdout().flush().unwrap();

                let mut jo: Vec<(u32, u32)> = Vec::new();
                for x in v.iter() {
                    jo.push((x.start.clone(), x.end.clone()));
                }
                let mut network = related_intervals::create_network_hashmap(&jo);
                make_nested(&jo, & mut network);
                let mut rrr = j.lock().unwrap();
                rrr.push((k.clone(), network));

            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()

    }
    for (k,v) in rr.lock().unwrap().iter(){
        connect_bubbles(&v, result, k)
    }

}

/// Conntect bubbles and add children and parents
pub fn connect_bubbles(hm: &HashMap<(u32, u32), Network>, result: & mut BubbleWrapper, s: &String){
    for (k,v) in hm.iter(){
        let index = result.id2id.get(&(k.0, k.1, s)).unwrap();
        let mut ii: Vec<&u32> = Vec::new();
        for x in v.parent.iter(){
            ii.push(result.id2id.get(&(x.0, x.1, s)).unwrap());
        }
        for x in ii.iter(){
            result.id2bubble.get_mut(x).unwrap().children.insert(index.clone());
            result.id2bubble.get_mut(index).unwrap().parents.insert(x.clone().clone());
        }

    }

}

/// Checking bubble size
/// TODO
/// - Categories (additional function)
/// -
pub fn check_bubble_size(h: & mut BubbleWrapper){
    let k: Vec<u32> = h.id2bubble.keys().copied().collect();
    for x in k.iter(){
        let mut min = u32::MAX;

        let mut max = u32::MIN;
        let bubble = h.id2bubble.get_mut(x).unwrap();
        for x in bubble.traversals.iter() {
            if x.1.length > max {
                max = x.1.length.clone();
            }
            if x.1.length < min {
                min = x.1.length.clone();
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
    for x in 0..h.id2bubble.len(){
        let level = go1(x as u32, h) as u16;
        let g = h.id2bubble.get_mut(&(x as u32)).unwrap();
        g.nestedness = level;
    }
}

#[allow(dead_code)]
/// Function for nest_version2
pub fn go1(id: u32, h: & mut BubbleWrapper) -> usize{
    let mut bubble = h.id2bubble.get(&id).unwrap();
    let mut  count: usize = 0;
    loop {
        count += 1;
        if bubble.parents.len() == 0{
            break
        }
        eprintln!("parent {}", bubble.parents.len());
        let parent = bubble.parents.iter().next().unwrap().clone();
        eprintln!("parent2 {}", bubble.parents.len());

        bubble = h.id2bubble.get(&parent).unwrap();
    }
     return count
}