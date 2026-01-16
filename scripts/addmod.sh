#!/bin/bash
# add another module for discovery
# in src/discoverer/xxxx
# must be in root dir
new_mod=$1
upper=${new_mod^^}
new_file=src/discoverer/$new_mod.rs
cp src/discoverer/png.rs $new_file
sed -i "s/PNG/$upper/g" $new_file
sed -i "/#tag/a\pub mod $new_mod;" src/discoverer/mod.rs
