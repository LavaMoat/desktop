use std::collections::HashMap;
use url::Url;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItem},
        window::WindowBuilder,
    },
    http::ResponseBuilder,
    webview::WebViewBuilder,
};

use std::sync::mpsc::channel;

//use log::debug;

mod ipc;

pub async fn window<S: AsRef<str>, T: AsRef<str>>(
    url: S,
    title: T,
) -> wry::Result<()> {
    let event_loop = EventLoop::<String>::with_user_event();
    let event_proxy = event_loop.create_proxy();

    let (tx, rx) = channel::<String>();

    // Spawn a worker thread to execute IPC messages
    // and pass them back to the webview via the event proxy
    // when the JSON-RPC evaluation determines a reply is required.
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new()
            .expect("could not create tokio runtime");
        runtime
            .block_on(async {
                while let Ok(message) = rx.recv() {
                    let response = ipc::handle(&message).await?;
                    if let Some(reply) = &response {
                        let response = serde_json::to_string(reply)?;
                        let script =
                            format!("window.postMessage({})", response);
                        event_proxy.send_event(script)?;
                    }
                }
                Ok::<(), anyhow::Error>(())
            })
            .expect("failed to execute IPC bridge runtime");
    });

    let mut menu_bar = MenuBar::new();
    let mut win_bar = MenuBar::new();

    win_bar.add_native_item(MenuItem::About("".to_string()));
    win_bar.add_native_item(MenuItem::EnterFullScreen);
    win_bar.add_native_item(MenuItem::CloseWindow);
    win_bar.add_native_item(MenuItem::Quit);
    menu_bar.add_submenu("Main", true, win_bar);

    let window = WindowBuilder::new()
        .with_title(title.as_ref())
        .with_menu(menu_bar)
        .build(&event_loop)?;

    let dev_tool = if cfg!(debug) { true } else { false };

    let webview = WebViewBuilder::new(window)?
        .with_dev_tool(dev_tool)
        .with_custom_protocol("qrcode".into(), move |request| {
            let path = request.uri().replace("qrcode://", "");
            let uri = Url::parse("http://example.com")
                .unwrap()
                .join(&path)
                .unwrap();

            let query: HashMap<String, String> = uri
                .query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect::<_>();

            if let Some(value) = query.get("text") {
                let data: Vec<u8> = vec![];
                ResponseBuilder::new().mimetype("image/png").body(data)
            } else {
                ResponseBuilder::new()
                    .status(400)
                    .mimetype("text/plain")
                    .body("Bad request".as_bytes().to_vec())
            }
        })
        .with_url(url.as_ref())?
        .with_ipc_handler(move |_, message| {
            tx.send(message)
                .expect("failed to send IPC message to async thread (bridge)");
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                webview.devtool();
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
