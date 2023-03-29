# scad_tree

A Rust solid modeling library that generates OpenSCAD code.

To use this library in your project add this to your Cargo.toml.

```toml
[dependencies]
scad_tree = { git="https://github.com/mrclean71774/scad_tree" }
```

To run the examples clone this repository and from the root folder run

```text
cargo run --example example_name
```

---

## Project Structure

* scad_tree - The main crate with examples of usage.
* scad_tree_math - Linear algebra and other math mostly re-exported from scad_tree.
* images - Images of things made with this library and images of some examples.
* output - Output from some of the examples.

---

A [Blender](blender.org) render of metric thread parts.
![metric_thread.png](https://github.com/mrclean71774/scad_tree/blob/main/images/metric_thread.png)

A photo of my bottle example 3D printed.
![bottle_make.jpg](https://github.com/mrclean71774/scad_tree/blob/main/images/bottle_make.jpg)
