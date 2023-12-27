use std::fs;

use godot::prelude::*;

use godot::engine::INode2D;
use std::path::Path;

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

    fn process(&mut self, delta: f64) {
        //godot_print!("HideExampleNode is processing!");
        self.node2d.rotate((5f64 * delta) as f32);
    }
}

#[godot_api]
impl HideExample {

    // register the class to Godot
    #[func]
    fn hello_world(&mut self, name:GString) {
        godot_print!("Hello world! {name}");
        self.node2d.emit_signal("hello_world_signal".into(), &[]);
    }

    // emit signal
    #[signal]
    fn hello_world_signal();

    // open file like a text and return the content
    #[func]
    fn open_file(&mut self, path: GString) -> GString {
        godot_print!("Opening file {}", path);
        let path_str = path.to_string(); // Convert GString to String
        let path = Path::new(&path_str);
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let contents = GString::from(contents);
        return contents;        
    }

    // save file like a text
    #[func]
    fn save_file(&mut self, path: GString, content: GString) {
        godot_print!("Saving file {}", path);
        let path_str = path.to_string(); // Convert GString to String
        let path = Path::new(&path_str);
        let content_ref = content.to_string(); // Convert GString to String
        fs::write(path, content_ref).expect("Something went wrong writing the file");
    }

    // fibonacci function
    #[func]
    fn fibonacci(&mut self, n: i64) -> i64 {
        if n <= 1 {
            return n;
        }
        return self.fibonacci(n - 2) + self.fibonacci(n - 1);
    }

    // prime number function
    #[func]
    fn is_prime(&mut self, n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i = i + 6;
        }
        return true;
    }

}