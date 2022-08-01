# pan-sv

Bubble detection using pan-level approach in variation graphs.

##Installation 

**From source** 
```asm
git clone https://github.com/MoinSebi/pan-sv.git
cd pan-sv 
cargo build --release
```

## Running 


**Help message**
```text 
panSV 0.1.0

Sebastian V

Bubble detection

USAGE:
    pan-sv [FLAGS] [OPTIONS] --gfa <gfa>

FLAGS:
    -h, --help          Print help information
    -n, --naming        Change the naming
        --nestedness    Add nestedness to the stats output
    -q                  No updating INFO messages
        --traversal     Additional traversal file as output
    -V, --version       Print version information

OPTIONS:
    -d, --delimiter <delimiter>    Delimiter for between genome and chromosome
    -g, --gfa <gfa>                Input GFA file
    -o, --output <output>          Output prefix [default: panSV.output]
    -t, --threads <threads>        Number of threads [default: 1]
    -u, --unique <unique>          Return additional files with unique traversals above THIS value
                                   [default: 50]
    -v <verbose>                   -v = DEBUG | -vv = TRACE
```

**Example:** 
```bash
./pan-sv -g data/testGraph.gfa -o panSV.out
```



### Output
[Documentation](doc.md)
