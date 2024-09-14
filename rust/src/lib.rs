use godot::classes::INode;
use godot::prelude::*;
use mlua::prelude::*;

struct GodotFrame;

#[gdextension]
unsafe impl ExtensionLibrary for GodotFrame {}

#[derive(GodotClass)]
#[class(base=INode)]
struct FrameBLE {}

#[godot_api]
impl INode for FrameBLE {
    fn init(base: Base<INode>) -> Self {
        Self {}
    }
}

#[godot_api]
impl FrameBLE {
    fn exec_lua(expr: String) -> LuaResult<()> {
        let lua = Lua::new();
        lua.globals().set("__result__", None)?;
        lua.load(format!("__result__ = {expr}")).exec()?;
        Ok(())
    }

    #[func]
    fn pair(&mut self, device_name: String) -> Result<(), Error> {
        todo!("Not implemented")
    }
}

/*
* See for important env information for building for android from windows
* https://github.com/mlua-rs/mlua/issues/358#issuecomment-1904089136
*/
#[derive(GodotClass)]
#[class(base=INode)]
struct LuaExec {}

#[godot_api]
impl INode for LuaExec {
    fn init(base: Base<INode>) -> Self {
        godot_print!("Initializing Lua context");
        Self {}
    }
}

#[godot_api]
impl LuaExec {
    #[func]
    /// Takes a lua string, executes it, and returns the result as a string.
    fn execute(&mut self, expr: String) -> String {
        todo!("Not implemented")
    }
}
