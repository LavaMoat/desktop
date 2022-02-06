use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItem},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

//use log::debug;

mod ipc;

pub fn window<S: AsRef<str>, T: AsRef<str>>(
    url: S,
    title: T,
) -> wry::Result<()> {
    let event_loop = EventLoop::<String>::with_user_event();
    let event_proxy = event_loop.create_proxy();

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

    let webview = WebViewBuilder::new(window)?
        .with_url(url.as_ref())?
        .with_ipc_handler(move |_, message| {
            //debug!("{}", message);
            let response =
                ipc::handle(&message).expect("failed to handle IPC message");
            if let Some(reply) = &response {
                let response = serde_json::to_string(reply)
                    .expect("failed to encode response as JSON");
                let script = format!("window.postMessage('{}')", response);
                event_proxy
                    .send_event(script)
                    .expect("failed to send script to event loop proxy");
            }
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                //println!("Window is ready!")
            }
            Event::UserEvent(script) => {
                //debug!("{}", script);
                webview
                    .evaluate_script(&script)
                    .expect("failed to evaluate script in webview");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
