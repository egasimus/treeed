use std::io::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use thatsit::{*, crossterm::{style::Color, event::{Event, KeyEvent, KeyCode}}};
use thatsit_widgets::*;

/// Exit flag. Setting this to true terminates the main loop.
static EXITED: AtomicBool = AtomicBool::new(false);

pub(crate) fn main () -> Result<()> {
    run(&EXITED, &mut std::io::stdout(), App::new())
}

/// The main app object, containing a menu of supported devices.
#[derive(Debug)]
struct App {
    /// A reference to the exit flag to end the main loop.
    exited:  &'static AtomicBool,
}

impl App {
    fn new () -> Self {
        Self { exited: &EXITED }
    }
    /// Set the exit flag, terminating the main loop before the next render.
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn load (&mut self, path: &impl AsRef<std::path::Path>) {
    }
    fn save (&self, path: &impl AsRef<std::path::Path>) {
    }
}

impl Widget for App {

    impl_render!(self, out, area => {
        Aligned(Align::Center, Border(InsetTall, Stacked::y(|add|{
            //add(&self.devices);
        }))).render(out, area)
    });

    impl_handle!(self, event => {
        Ok(if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event {
            self.exit();
            true
        } else {
            //self.devices.handle(event)?
            false
        })
    });

}
