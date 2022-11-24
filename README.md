### shelf system in Rust ###

This program creates a configurable shelf and ouputs all the necessary boards' measurements and drill positions.

Currently usage is as follows:

1. set width, height, depth and extra_depth of shelf and thickness of the wood material.
2. set up levels with compartments with various widths and depths, extend to full width if neccessary.

planned features:
3. return a list of compartments - a list of boxes, each with width, height and depth.
4. return a list of all boards needed with drill and milling instructions.
5. options to put smaller compartments into larger ones i.e. "shelves inside of shelves"

support for movable parts e.g. a shelf that moves to the side making a shelve behind it accessable.

