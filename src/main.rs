

mod core;
#[allow(non_snake_case)]
mod panSV;

use hashbrown::HashMap;
use crate::core::counting::{CountNode};
use crate::panSV::algo::{check_bubble_size, nester_wrapper, create_bubbles_stupid, merge_traversals, connect_bubbles_multi, indel_detection, algo_panSV_multi2, new_bubble, check_parent, makesize, output1, save_stuff};
use crate::core::graph_helper::graph2pos;
use clap::{Arg, App, AppSettings};
use std::path::Path;
use std::process;
use crate::panSV::panSV_core::{PanSVpos};
use gfaR_wrapper::{NGfa, GraphWrapper};
use log::{ info, warn};
use crate::core::core::{Bubble, Posindex};
use crate::core::writer::{bubble_naming_new, writing_bed_solot};
use crate::core::logging::newbuilder;


fn main() {
    let matches = App::new("panSV")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Sebastian V, Christian K")
        .about("Bubble detection")

        .help_heading("Input options")
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Input GFA file")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("delimiter")
        .short('d')
        .long("delimiter")
        .about("Delimiter for between genome and chromosome")
        .takes_value(true))

        .help_heading("Output options")
        .arg(Arg::new("output")
            .display_order(1)
            .short('o')
            .long("output")
            .about("Output prefix")
            .takes_value(true)
            .default_value("panSV.output"))
        .arg(Arg::new("Nestedness")
            .long("nestedness")
            .about("Adds NL-tag (nestedness-level) to the stats output file [default: off]"))
        .help_heading("Threading")
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads")
            .default_value("1"))
        .help_heading("Processing information")
        .arg(Arg::new("quiet")
            .short('q')
            .about("No updating INFO messages"))
        .arg(Arg::new("verbose")
            .short('v')
            .about("-v = DEBUG | -vv = TRACE")
            .takes_value(true)
            .default_missing_value("v1"))
        .get_matches();

    // Checking verbose
    // Ugly, but needed - May end up in a small library later
    newbuilder(&matches);

    //-------------------------------------------------------------------------------------------------

    info!("Running pan-sv");
    let threads= matches.value_of("threads").unwrap().parse().unwrap();

    // Check if graph is running
    let mut graph_file = "not_relevant";
    if matches.is_present("gfa") {
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            graph_file = matches.value_of("gfa").unwrap();
        } else {
            warn!("No file with such name");
            process::exit(0x0100);
        }

    }

    // This is the prefix
    let outprefix= matches.value_of("output").unwrap();


    // Read the graph
    info!("Reading the graph");
    let mut graph: NGfa = NGfa::new();
    graph.from_file_direct2(graph_file);

    // Counting nodes
    let g2p = graph2pos(&graph);



    let mut counts: CountNode = CountNode::new();
    if matches.is_present("delimiter"){
        let mut gra_wrapper: GraphWrapper = GraphWrapper::new();
        gra_wrapper.from_ngfa(&graph, matches.value_of("delimiter").unwrap());
        info!("{} Genomes and {} Paths", gra_wrapper.genomes.len(), graph.paths.len());
        info!("Counting nodes");
        counts.counting_wrapper(&graph, &gra_wrapper);
    } else {
        info!("{} Genomes and {} Paths", graph.paths.len(), graph.paths.len());
        info!("Counting nodes");
        counts.counting_graph(&graph);
    }
    let paths = graph.paths.clone();
    let p2id = graph.path2id.clone();
    drop(graph);


    let bi_wrapper: HashMap<String, Vec<PanSVpos>>;
    let mut pre_intervals = algo_panSV_multi2(&paths, counts,  &threads, &p2id);
    let (mut intervals, mut bubbles) = new_bubble(&mut pre_intervals, &paths, &g2p);
    let (mut intervals2, mut rela) = check_parent(intervals);
    let sizes = makesize(&mut intervals2, &g2p, &paths);
    let dd = save_stuff(&intervals2, &paths);
    let trav = output1(intervals2, sizes,  &paths);

    info!("done")
    //
    //
    // let mut bub_intervals: Vec<Posindex> = Vec::new();
    // let mut bub_bubbles: Vec<Bubble> = Vec::new();
    // let mut anchor2bubble: HashMap<(u32, u32), u32> = HashMap::new();
    // let mut id2id: HashMap<(u32, u32, u32), u32> = HashMap::new();
    //
    // let mut parents: Vec<Vec<u32>> = Vec::new();
    // let mut children: Vec<Vec<u32>> = Vec::new();
    //
    //
    //
    // let tmp1 = create_bubbles_stupid(&bi_wrapper,  &mut id2id, &mut bub_intervals, &paths, &p2id, &threads);
    // id2id.shrink_to_fit();
    // bub_intervals.shrink_to_fit();
    //
    //
    // //info!("{:?}", bub_wrapper);
    // merge_traversals(tmp1, &paths, &mut bub_bubbles, &mut anchor2bubble, &threads);
    // bub_bubbles.shrink_to_fit();
    // let (mut id2id, parents, children) = connect_bubbles_multi(bi_wrapper, &mut bub_bubbles, id2id,&p2id, &threads);
    // let interval_numb = bub_intervals.len() as u32;
    //
    // indel_detection(&mut bub_bubbles, &anchor2bubble, &mut id2id, &mut bub_intervals, &paths, interval_numb);
    //
    //
    // info!("Write Traversal");
    // writing_bed_solot(& mut bub_bubbles, & bub_intervals, &g2p, &paths, outprefix);
    //
    // info!("Categorize bubbles");
    // check_bubble_size(&mut bub_bubbles);
    //
    // // if matches.is_present("Nestedness"){
    // //     info!("Nestedness");
    // //     nest_version2(& mut bub_wrapper);
    // // }
    //
    //
    // info!("Writing bubble stats");
    // bubble_naming_new(&bub_bubbles, children, parents, outprefix);




}

