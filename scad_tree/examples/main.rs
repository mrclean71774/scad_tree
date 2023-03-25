use scad_tree::prelude::*;

fn main() {
    let result = metric_thread::hex_nut(12, 4.0, 36, true, 2.0, false, false);
    scad_file!("output/hex_nut.scad", result;);
}
