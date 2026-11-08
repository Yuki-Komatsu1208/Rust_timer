mod app;
mod timer;
mod notification;
use app::App;
use timer::Timer;
use dioxus::desktop::{Config, WindowCloseBehaviour};

fn main() {
    ///デスクトップアプリとして起動。
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_close_behaviour(WindowCloseBehaviour::WindowHides),
        )
        .launch(App);
}