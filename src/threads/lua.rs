use super::gfx::GfxMsg;
use crate::lua_environment::LuaApi;

use crate::threads::inp::InputState;
//use crate::lua_thread::Msg as LuaMsg;
use rlua::{Context, Lua, Result as LuaResult, ToLua};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;

use std::thread;

pub enum LuaMsg {
    Run(String),
    Call(String, Vec<LuaVal>),
    Exit,
}

pub enum LuaVal {
    Num(f64),
    Str(String),
    Tbl(HashMap<LuaVal, LuaVal>),
}

impl<'lua> ToLua<'lua> for LuaVal {
    fn to_lua(self, lua_ctx: Context<'lua>) -> LuaResult<rlua::Value> {
        match self {
            LuaVal::Num(num) => num.to_lua(lua_ctx),
            LuaVal::Str(str) => str.to_lua(lua_ctx),
            LuaVal::Tbl(_map) => {
                panic!("Do not know how to convert hashmap to lua value... :-(");
                /*
                let table = lua_ctx.create_table().unwrap();
                for (key, val) in map.iter() {
                    table.set(key, val.to_lua(lua_ctx).unwrap().to_lua().unwrap());
                }

                Ok(rlua::Value::Table(table))
                */
            }
        }
    }
}

pub fn run_thread(
    lua_recv: Receiver<LuaMsg>,
    gfx_send: SyncSender<GfxMsg>,
    inp_state: Arc<InputState>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        let lua = Lua::new();
        lua.context(|lua_ctx| {
            let mut lua_api = LuaApi::new(lua_ctx, &gfx_send, inp_state);

            while let Ok(msg) = lua_recv.recv() {
                match msg {
                    LuaMsg::Run(lua_code) => {
                        if let Err(err) = lua_api.run(&lua_code) {
                            panic!("Lua failed with error:\n\n{}\n\n", err)
                        }
                    }
                    LuaMsg::Call(_, _) => panic!("Can't call functions yet."),
                    LuaMsg::Exit => return,
                }
            }
            panic!("Error receiving message for Lua")
        });
    })
}
