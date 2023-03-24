use scad_tree::prelude::*;

fn main() {
    scad_file!("output/threaded_rod.scad",
        metric_thread::threaded_rod(12, 30.0, 36, true, 270.0, true, 270.0, true, true);
    );
}
