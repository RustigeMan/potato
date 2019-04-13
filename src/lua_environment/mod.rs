mod key_map;
use key_map::gen_key_map;

use std::clone::Clone;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use crate::threads::gfx::GfxMsg;
//use crate::threads::gfx::Msg::*;
use crate::threads::inp::InputState;

use rlua::{Context, Result as LuaResult, UserData};
use rlua::{FromLuaMulti, ToLuaMulti};


pub struct LuaApi<'ctx> {
    context: Context<'ctx>,
    input_state: Arc<InputState>,
}

static SET_FAIL: &str = "Failed to set Lua value";
static TABLE_FAIL: &str = "Failed to create table";
static GFX_MSG_FAIL: &str = "Failed to send GFX message";
impl<'ctx> LuaApi<'ctx> {
    pub fn new<'lua>(
        context: Context<'lua>,
        gfx_send: &SyncSender<GfxMsg>,
        input_state: Arc<InputState>,
    ) -> LuaApi<'lua> {
        let mut lua_api = LuaApi {
            context: context,
            input_state: input_state,
        };
        lua_api.add_api(gfx_send);
        lua_api
    }

    fn add_api(&mut self, gfx_send: &SyncSender<GfxMsg>) {
        println!("Initializing Lua API");
        let globals = self.context.globals();
        let gfx = self.context.create_table().expect(TABLE_FAIL);

        let sender = gfx_send.clone();
        let clear_screen = self.create_function(move |_, ()| {
            sender.send(GfxMsg::Clear).expect(GFX_MSG_FAIL);
            Ok(())
        });

        let sender = gfx_send.clone();
        let display = self.create_function(move |_, ()| {
            sender.send(GfxMsg::Display).expect(GFX_MSG_FAIL);
            Ok(())
        });

        let sender = gfx_send.clone();
        let load_img_with_id = self.create_function(move |_, (path, id): (String, u32)| {
            let img = Image(id);
            sender
                .send(GfxMsg::LoadImg(path, img.clone()))
                .expect(GFX_MSG_FAIL);
            Ok(img)
        });

        let sender = gfx_send.clone();
        let draw_img = self.create_function(move |_, (img, x, y): (Image, f32, f32)| {
            sender.send(GfxMsg::DrawImg(img, x, y)).expect(GFX_MSG_FAIL);
            Ok(())
        });

        gfx.set("clear_screen", clear_screen).expect(SET_FAIL);
        gfx.set("display", display).expect(SET_FAIL);
        gfx.set("load_img_with_id", load_img_with_id)
            .expect(SET_FAIL);
        gfx.set("draw_img", draw_img).expect(SET_FAIL);
        globals.set("gfx", gfx).expect(SET_FAIL);
        self.run(
            "
            gfx.load_img = 
                (function ()
                    local img_id_counter = 0
                    return function(path)
                        img_id_counter = img_id_counter + 1
                        print('Generated Image ID: ' .. img_id_counter)
                        return gfx.load_img_with_id(path, img_id_counter)
                    end
                end)()
            ",
        )
        .expect("Failed to run lua code fragment to create function load_img()");

        let inp = self.context.create_table().expect(TABLE_FAIL);

        let key_map = gen_key_map();
        let input_state = self.input_state.clone();
        let key_down = self.create_function(move |_, key_name: String| {
            let key = key_map
                .get(&key_name)
                .expect(&format!("Unknown key name: '{}'", key_name));
            Ok(input_state.key_down(key.clone()))
        });

        inp.set("key_down", key_down).expect(SET_FAIL);

        globals.set("inp", inp).expect(SET_FAIL);

        let sleep = self.create_function(|_, milliseconds: f64| {
            let milliseconds = if milliseconds < 0.0 {
                println!("WARNING: Trying to sleep for less than 0 milliseconds");
                0
            } else {
                milliseconds as u64
            };
            std::thread::sleep(std::time::Duration::from_millis(milliseconds));
            Ok(())
        });

        let start = std::time::Instant::now();
        let ticks = self.create_function(move |_, ()| {
            let now = std::time::Instant::now();
            Ok((now - start).as_millis())
        });

        globals.set("sleep", sleep).expect(SET_FAIL);
        globals.set("ticks", ticks).expect(SET_FAIL);
    }

    fn create_function<A, R, F>(&mut self, fun: F) -> rlua::Function<'ctx>
    where
        A: FromLuaMulti<'ctx>,
        R: ToLuaMulti<'ctx>,
        F: 'static + Send + Fn(Context<'ctx>, A) -> LuaResult<R>,
    {
        self.context
            .create_function(fun)
            .expect("Failed to create Lua function")
    }

    pub fn run(&mut self, code: &str) -> LuaResult<()> {
        let chunk = self.context.load(code);
        chunk.exec()?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Image(u32);

impl Image {
    pub fn id(&self) -> u32 {
        self.0
    }
}
impl UserData for Image {}
