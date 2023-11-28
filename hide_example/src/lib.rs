use godot::prelude::*;

struct HideExampleExtension;

#[gdextension]
unsafe impl ExtensionLibrary for HideExampleExtension {}

mod example;