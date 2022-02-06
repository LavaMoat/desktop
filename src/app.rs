use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItem},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

pub fn window<S: AsRef<str>, T: AsRef<str>>(
    url: S,
    title: T,
) -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let mut menu_bar = MenuBar::new();
    let mut win_bar = MenuBar::new();

    win_bar.add_native_item(MenuItem::About("".to_string()));
    win_bar.add_native_item(MenuItem::EnterFullScreen);
    win_bar.add_native_item(MenuItem::Quit);
    menu_bar.add_submenu("Main", true, win_bar);

    let window = WindowBuilder::new()
        .with_title(title.as_ref())
        .with_menu(menu_bar)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url(url.as_ref())?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                //println!("Window is ready!")
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
