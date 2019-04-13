use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::lua_environment::Image;

use sfml::window::Event;

use crate::gfx::Renderer;
use crate::threads::inp::InpMsg; // inp: input

#[derive(Debug)]
pub enum GfxMsg {
    //InitScreen(u32, u32, String),
    Clear,
    Display,
    LoadImg(String, Image),
    DrawImg(Image, f32, f32),
    //DrawRect(f32, f32, f32, f32, Color),
    Exit,
}

pub fn run_thread(gfx_recv: Receiver<GfxMsg>, inp_send: Sender<InpMsg>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let win = crate::init().expect("Initialization failed!");
        let mut renderer = Renderer::new(win);

        loop {
            while let Ok(message) = gfx_recv.try_recv() {
                //println!("{:?}", message);
                if let GfxMsg::Exit = message {
                    return;
                } else {
                    renderer.process_message(message);
                }
            }

            /* I don't like that I have to process input events in the render
             * thread, because the RenderWindow class cannot be shared across
             * threads. So let's use a channel that doesn't block and use a
             * blocking syncing mechanism in the input thread:
             */
            while let Some(event) = renderer.win().poll_event() {
                match event {
                    Event::KeyPressed { code, .. } => inp_send
                        .send(InpMsg::KeyDown(code))
                        .expect("Could not send KeyDown message!"),
                    Event::KeyReleased { code, .. } => inp_send
                        .send(InpMsg::KeyUp(code))
                        .expect("Could not send KeyUp message!"),
                    _ => {}
                }
            }
        }
    })
}
