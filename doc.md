# Output format 
Explanation about the main output files from pan-sv. Some additional files are not listed here.


## Bubble stats (prefix.bubble.stats)

| Col | Type   | Description                                   |
|-----|--------|-----------------------------------------------|
| 1   | int    | Bubble id                                     |
| 2   | int    | Number of sub bubbles                         |
| 3   | int    | Minimal length of the bubble                  |
| 4   | int    | Maximum length of the bubble                  |
| 5   | float  | Mean length of the bubble                     |
| 6   | int    | Number of traversals                          |
| 7   | int    | Number of intervals                           |
| 8   | int    | Parent names (bubble id) (separated by comma) |
| 9   | int    | Anchor 1                                      |
| 10  | int    | Anchor 2                                      |
| 11  | float  | Ratio Min/Max                                 |
| 12  | Bool   | Small                                         |
| 13  | int    | Type                                          | 
| 14  | String | Tags                                          | 

Tags: 
- CL: Core level
- NL: Nestedness level (depth in a bubble)

## Type in bubble stats
**Info:**  
Ratio: Smallest traversal / Biggest traversal  
Small: Biggest traversal < 50 bp   
Big: Biggest traversal >= 50 bp  

| Number | Description                | Size   |
|--------|----------------------------|--------|
| 0      | SNP                        | small  |
| 1      | Indel (Ratio = 0)          | small  |
| 2      | MNP (Ratio != 0)           | small  |
| 3      | Indel (Ratio = 0)          | big    |
| 4      | Different size (Ratio<0.9) | big    |
| 5      | Same size (Ratio>0.9)      | big    |



## Bed output
| Col | Type   | Description     |
|-----|--------|-----------------|
| 1   | String | Genome name     |
| 2   | int    | Start position  |
| 3   | int    | End position    |
| 4   | int    | Bubble id       |
| 5   | int    | Traversal id    |