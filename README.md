# godot4-rust-extension
Rust GDExtension example in Godot 4

https://godot-rust.github.io/book/intro/hello-world.html

> cargo new "{Project}" --lib

```
[lib]
crate-type = ["cdylib"] 

[dependencies]
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
```

> Crear {project}.gdextension

```
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]
macos.debug = "res://bin/libgdexample.macos.template_debug.framework"
macos.release = "res://bin/libgdexample.macos.template_release.framework"
windows.debug.x86_32 = "res://bin/libgdexample.windows.template_debug.x86_32.dll"
windows.release.x86_32 = "res://bin/libgdexample.windows.template_release.x86_32.dll"
windows.debug.x86_64 = "res://bin/libgdexample.windows.template_debug.x86_64.dll"
windows.release.x86_64 = "res://bin/libgdexample.windows.template_release.x86_64.dll"
linux.debug.x86_64 = "res://bin/libgdexample.linux.template_debug.x86_64.so"
linux.release.x86_64 = "res://bin/libgdexample.linux.template_release.x86_64.so"
linux.debug.arm64 = "res://bin/libgdexample.linux.template_debug.arm64.so"
linux.release.arm64 = "res://bin/libgdexample.linux.template_release.arm64.so"
linux.debug.rv64 = "res://bin/libgdexample.linux.template_debug.rv64.so"
linux.release.rv64 = "res://bin/libgdexample.linux.template_release.rv64.so"
android.debug.x86_64 = "res://bin/libgdexample.android.template_debug.x86_64.so"
android.release.x86_64 = "res://bin/libgdexample.android.template_release.x86_64.so"
android.debug.arm64 = "res://bin/libgdexample.android.template_debug.arm64.so"
android.release.arm64 = "res://bin/libgdexample.android.template_release.arm64.so"
```

> cargo build 
> cargo build --release

Para debug instalar extensión CodeLLDB en VSC. Luego ir a debug y crear LLDB.

```
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "preLaunchTask": "rust: cargo build",
            "name": "Debug",
            "args": [ "--path", "C:\\Users\\FinePointCGI\\Documents\\Rust Tutorial"],
            "cwd": "${workspaceFolder}",
            "windows": {
                "program" : "C:\\Users\\FinePointCGI\\Downloads\\Godot_v4.1.1-stable_mono_win64\\Godot_v4.1.1-stable_mono_win64\\Godot_v4.1.1-stable_mono_win64.exe"
            }
        }
    ]
}
```

tasks.json

```
{
	"version": "2.0.0",
	"tasks": [
		{
			"type": "cargo",
			"options": {
                "cwd": "${workspaceFolder}/hide_example"
            },
			"command": "build",
			"problemMatcher": [
				"$rustc"
			],
			"group": {
				"kind": "build",
				"isDefault": true
			},
			"label": "rust: cargo build"
		}
	],
}
```