use maolan_editor::app;
use maolan_widgets::iced::{
    Settings, Theme, application,
    executor::Executor,
    futures::{Future, io},
    window,
};
use maolan_widgets::iced_fonts::LUCIDE_FONT_BYTES;

struct EditExecutor(tokio::runtime::Runtime);

impl Executor for EditExecutor {
    fn new() -> Result<Self, io::Error> {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("edit-tokio")
            .enable_all()
            .build()
            .map(Self)
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let _handle = self.0.spawn(future);
    }

    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        let _guard = self.0.enter();
        f()
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        self.0.block_on(future)
    }
}

fn main() -> maolan_widgets::iced::Result {
    application(app::new, app::update, app::view)
        .executor::<EditExecutor>()
        .title(app::title)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .theme(|_: &app::EditApp| Theme::Dark)
        .font(LUCIDE_FONT_BYTES)
        .subscription(app::subscription)
        .window(window::Settings {
            exit_on_close_request: false,
            ..window::Settings::default()
        })
        .run()
}
