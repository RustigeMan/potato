use rlua::{UserData, UserDataMethods};
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{ContextSettings, Event as SFMLEvent, Style};

pub struct Window(RenderWindow);

impl Window {
    pub fn new() -> Self {
        let context_settings = ContextSettings {
            antialiasing_level: 0,
            ..Default::default()
        };
        let render_win = RenderWindow::new((800, 600), "POTATO", Style::CLOSE, &context_settings);
        Window(render_win)
    }

    pub fn clear(&mut self) {
        self.0.clear(&Color::BLACK)
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        if let Some(evt) = self.0.poll_event() {
            Some(Event(evt))
        } else {
            None
        }
    }
}

impl UserData for Window {
    fn add_methods<'lua, M>(methods: &mut M)
    where
        M: UserDataMethods<'lua, Self>,
    {
        methods.add_method("clear", |_, win, ()| Ok(()));
    }
}

pub struct Event(pub SFMLEvent);

impl UserData for Event {}
