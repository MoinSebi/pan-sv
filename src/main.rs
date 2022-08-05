

mod core;
#[allow(non_snake_case)]
mod panSV;

use hashbrown::HashMap;
use std::env::args;
use crate::core::counting::{CountNode};
use crate::panSV::algo::{create_bubbles, indel_detection, check_bubble_size, nest_version2, algo_panSV_multi};
use crate::core::graph_helper::graph2pos;
use clap::{Arg, App, AppSettings};
use std::path::Path;
use std::process;
use crate::panSV::panSV_core::{BubbleWrapper, PanSVpos};
use gfaR_wrapper::{NGfa, GraphWrapper};
use log::{info, LevelFilter, warn};
use crate::core::writer::{writing_traversals, writing_bed, bubble_naming_new, bubble_parent_structure, writing_bed_traversals, writing_bed2};
use std::io::Write;
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
        .arg(Arg::new("traversal")
            .long("traversal")
            .about("Report additional traversals in additional file"))
        .arg(Arg::new("unique")
            .display_order(2)
            .short('u')
            .long("unique")
            .about("Report unique traversals with a size above this level [default: off]")
            .default_value("50")
            .takes_value(true))
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

    info!("Welcome to pan-sv");
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
    let mut graph: NGfa = NGfa::new();
    graph.from_file_direct2(graph_file);

    // Counting nodes
    let mut bub_wrapper: BubbleWrapper;
    let bi_wrapper: HashMap<String, Vec<PanSVpos>>;
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
    bi_wrapper = algo_panSV_multi(&graph.paths, &counts, &threads);
    bub_wrapper = create_bubbles(&bi_wrapper, &graph.paths, &g2p, &graph.path2id, &threads);
    info!("Indel detection");
    let interval_numb = bub_wrapper.intervals.len() as u32;
    indel_detection(& mut bub_wrapper, &graph.paths, interval_numb);




    info!("Categorize bubbles");
    check_bubble_size(&mut bub_wrapper);

    if matches.is_present("Nestedness"){
        info!("Nestedness");
        nest_version2(& mut bub_wrapper);
    }


    info!("Writing bubble stats");
    bubble_naming_new(&bub_wrapper.bubbles, outprefix);
    //bubble_parent_structure(&bub_wrapper.bubbles, outprefix);




    info!("Writing bed");
    writing_bed2(&bub_wrapper, &g2p, &graph.paths, outprefix);


    //writing_bed_traversals(&bub_wrapper, &g2p, &graph.paths, outprefix);


    // if matches.is_present("traversal"){
    //     info!("Writing traversal");
    //     writing_traversals(&bub_wrapper, outprefix);
    // }
    //
    // if matches.is_present("unique"){
    //     info!("Writing traversal");
    //     let size: usize = matches.value_of("unique").unwrap().parse().unwrap();
    //     writing_uniques_bed(&bub_wrapper, &g2p, outprefix, size);
    //     writing_uniques_bed_stats(&bub_wrapper, &g2p, outprefix, size);
    // }



}

