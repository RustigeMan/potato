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
    let (gfx_send, gfx_recv) = sync_channel::<GfxMsg>(128);
    let (inp_send, inp_recv) = channel::<InpMsg>();

    let input_state = InputState::new();

    let gfx_thread = threads::gfx::run_thread(gfx_recv, inp_send.clone());
    let lua_thread = threads::lua::run_thread(lua_recv, gfx_send.clone(), input_state.clone());
    let inp_thread = threads::inp::run_thread(inp_recv, input_state.clone());

    let file = fs::read_to_string("main.lua").expect("Could not open 'main.lua'");
    lua_send
        .send(LuaMsg::Run(file))
        .expect("Could not connect to Lua thread");

    // No more work after main.lua...
    println!("Exiting when main Lua thread has finished...");
    lua_send
        .send(LuaMsg::Exit)
        .expect("Could not send exit message to Lua thread");
    lua_thread.join().expect("Failed to join Lua thread");

    println!("Exiting...");
    gfx_send
        .send(GfxMsg::Exit)
        .expect("Could not send exit message to GFX thread");
    gfx_thread.join().expect("Failed to join GFX thread");

    inp_send
        .send(InpMsg::Exit)
        .expect("Could not send exit message to input thread");
    inp_thread.join().expect("Failed to join input thread");


    println!("Done.");
}

fn init() -> LuaResult<RenderWindow> {
    let lua = Lua::new();

    let mut w: u32 = 0;
    let mut h: u32 = 0;
    let mut title = String::new();
    let mut vertical_sync = false;
    let mut full_screen = false;

    println!("Running 'init.lua'");
    lua.context(|lua_ctx| {
        let file = fs::read_to_string("init.lua").expect("Could not open 'init.lua'");
        let chunk = lua_ctx.load(&file);
        chunk.exec().expect("Could not run Lua chunk");

        let globals = lua_ctx.globals();
        w = match globals.get("screen_width") {
            Ok(screen_width) => screen_width,
            Err(_) => {
                println!(
                    "WARNING: Could not get screen_width from 'init.lua'. Using default value: 800"
                );
                800
            }
        };
        h = match globals.get("screen_height") {
            Ok(screen_height) => screen_height,
            Err(_) => {
                println!(
                    "WARNING: Could not get screen_height from 'init.lua'. Using default value: 600"
                );
                600
            }
        };
        vertical_sync = globals.get("vertical_sync").unwrap_or(false);
        full_screen = globals.get("full_screen").unwrap_or(false);
        title = globals.get("window_title").unwrap_or("POTATO".to_string());
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
