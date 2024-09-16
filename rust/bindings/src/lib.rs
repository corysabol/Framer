use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;
use mlua::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;

struct Framer;

#[gdextension]
unsafe impl ExtensionLibrary for Framer {}

#[derive(GodotClass)]
#[class(base=Node)]
struct FrameClient {
    base: Base<Node>,
    server_process: Option<std::process::Child>,
}

#[godot_api]
impl INode for FrameClient {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            server_process: None,
        }
    }
}

#[godot_api]
impl FrameClient {
    #[func]
    fn start_server(&mut self) {
        let server_process = Command::new("./bin/frame_server")
            .spawn()
            .expect("Failed to start Frame server");

        self.server_process = Some(server_process);
        godot_print!("Frame server started");
    }

    #[func]
    fn stop_server(&mut self) {
        if let Some(mut process) = self.server_process.take() {
            process.kill().expect("Failed to kill Frame server process");
            godot_print!("Frame server stopped");
        }
    }

    #[func]
    fn connect(&mut self) {
        let base = &self.base;
        std::thread::spawn(move || {
            let mut stream = TcpStream::connect("127.0.0.1:45789").unwrap();
            stream.write_all(b"connect").unwrap();

            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();

            // TODO: Emit signal that the connection was successful
        });
    }

    #[func]
    fn exec_lua(&mut self, expr: String) {
        // Pass lua code to the frame server to be executed on the frame over ble
        todo!("Not implemented")
    }

    #[func]
    fn notifications(&mut self) {}
}

/*
* See for important env information for building for android from windows
* https://github.com/mlua-rs/mlua/issues/358#issuecomment-1904089136
*/
#[derive(GodotClass)]
#[class(base=Node)]
struct LuaExec {}

#[godot_api]
impl INode for LuaExec {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Initializing Lua context");
        Self {}
    }
}

#[godot_api]
impl LuaExec {
    #[func]
    /// Takes a lua string, executes it, and returns the result as a string.
    /// TODO: workout return type
    fn execute(&mut self, expr: String) {
        let lua = Lua::new();
        lua.globals().set("__result__", "").unwrap();
        lua.load(format!("__result__ = {expr}")).exec().unwrap();
    }
}
