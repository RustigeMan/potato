#![allow(unused)]
extern crate rlua;

use std::fs;
use std::sync::mpsc::{channel, sync_channel};

use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, Style};

use rlua::{Lua, Result as LuaResult};

mod gfx;

mod threads;
use threads::gfx::GfxMsg;
use threads::inp::{InpMsg, InputState};
use threads::lua::LuaMsg;

mod lua_environment;

fn main() {
    let (lua_send, lua_recv) = sync_channel::<LuaMsg>(0);
    let (gfx_send, gfx_recv) = sync_channel::<GfxMsg>(16);
    let (inp_send, inp_recv) = channel::<InpMsg>();

    let input_state = InputState::new();

    let gfx_thread = threads::gfx::run_thread(gfx_recv, inp_send);
    let lua_thread = threads::lua::run_thread(lua_recv, gfx_send.clone(), input_state.clone());
    let __________ = threads::inp::run_thread(inp_recv, input_state.clone());

    match fs::read_to_string("main.lua") {
        Ok(file) => lua_send
            .send(LuaMsg::Run(file))
            .expect("Could not connect to Lua thread"),
        Err(err) => panic!("Could not open 'main.lua', got error:\n\n{}\n\n", err),
    };

    lua_send.send(LuaMsg::Exit).unwrap();
    lua_thread.join().unwrap();

    gfx_send.send(GfxMsg::Exit).unwrap();
    gfx_thread.join().unwrap();
}

fn init() -> LuaResult<RenderWindow> {
    let lua = Lua::new();

    let mut w: u32 = 0;
    let mut h: u32 = 0;
    let mut title = String::new();
    let mut vertical_sync = false;
    let mut full_screen = false;

    println!("Running 'init.lua'");
    lua.context(|lua_ctx| match fs::read_to_string("init.lua") {
        Ok(file) => {
            let chunk = lua_ctx.load(&file);
            chunk.exec().unwrap();

            let globals = lua_ctx.globals();
            w = match globals.get("screen_width") {
                Ok(screen_width) => screen_width,
                Err(err) => {
                    println!("WARNING: Could not get screen_width from init.lua. Using default value: 800");
                    800
                }
            };
            h = match globals.get("screen_height") {
                Ok(screen_height) => screen_height,
                Err(err) => {
                    println!("WARNING: Could not get screen_height from init.lua. Using default value: 600");
                    600
                }
            };
            vertical_sync = globals.get("vertical_sync").unwrap_or(false);
            full_screen = globals.get("full_screen").unwrap_or(false);
            title = globals.get("window_title").unwrap_or("POTATO".to_string());
        }
        Err(err) => panic!("Could not open 'init.lua', got error: \n\n{}\n\n", err),
    });

    println!("Initializing screen at {}x{}", w, h);
    let context_settings = ContextSettings {
        antialiasing_level: 0,
        ..Default::default()
    };
    let style = if full_screen {
        Style::FULLSCREEN
    } else {
        Style::CLOSE
    };
    let mut win = RenderWindow::new((w, h), &title, style, &context_settings);
    win.set_vertical_sync_enabled(vertical_sync);
    print!("Vertical sync ");
    if vertical_sync {
        println!("enabled");
    } else {
        println!("disabled");
    }
    win.set_key_repeat_enabled(false);

    Ok(win)
}
