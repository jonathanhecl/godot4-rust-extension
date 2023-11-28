use godot::prelude::*;

struct HideExampleExtension;

#[gdextension]
unsafe impl ExtensionLibrary for HideExampleExtension {}

use godot::engine::Node2D;

#[derive(GodotClass)]
#[class(base=Node2D)]

struct HideExample {

    #[base]
    node2d: Base<Node2D>
}

use godot::engine::INode2D;

#[godot_api]
impl INode2D for HideExample {
    fn init(node2d: Base<Node2D>) -> Self {
        godot_print!("HideExampleNode is initialized!");
        Self { node2d }
    }

    fn process(&mut self, delta: f64) {
        godot_print!("HideExampleNode is processing!");
    }
}
