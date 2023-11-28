use godot::prelude::*;

use godot::engine::INode2D;

#[derive(GodotClass)]
#[class(base=Node2D)]

struct HideExample {

    #[base]
    node2d: Base<Node2D>
}


#[godot_api]
impl INode2D for HideExample {
    fn init(node2d: Base<Node2D>) -> Self {
        godot_print!("HideExampleNode is initialized!");
        Self { node2d }
    }

    //fn process(&mut self, delta: f64) {
        //godot_print!("HideExampleNode is processing!");
    //}
}

#[godot_api]
impl HideExample {
    #[func]
    fn hello_world(&mut self, name:GString) {
        godot_print!("Hello world! {name}");
        self.node2d.emit_signal("hello_world_signal".into(), &[]);
    }

    #[signal]
    fn hello_world_signal();
}