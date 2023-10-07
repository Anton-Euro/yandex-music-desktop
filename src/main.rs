#![windows_subsystem = "windows"]

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder, dpi
    },
    webview::WebViewBuilder,
};

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, MenuId},
    TrayIconBuilder,
};

use base64::{Engine as _, engine::general_purpose};
use image;

fn main() -> wry::Result<()> {
    let base64_icon: &str = "iVBORw0KGgoAAAANSUhEUgAAAVQAAAFUCAYAAAB7ksS1AAAACXBIWXMAAAsTAAALEwEAmpwYAAAE7mlUWHRYTUw6Y29tLmFkb2JlLnhtcAAAAAAAPD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4gPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iQWRvYmUgWE1QIENvcmUgOS4wLWMwMDAgNzkuMTcxYzI3ZiwgMjAyMi8wOC8xNi0xODowMjo0MyAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczpkYz0iaHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnRzLzEuMS8iIHhtbG5zOnBob3Rvc2hvcD0iaHR0cDovL25zLmFkb2JlLmNvbS9waG90b3Nob3AvMS4wLyIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0RXZ0PSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VFdmVudCMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIDI0LjEgKFdpbmRvd3MpIiB4bXA6Q3JlYXRlRGF0ZT0iMjAyMy0wOS0yNlQyMTowNzozOSswMzowMCIgeG1wOk1vZGlmeURhdGU9IjIwMjMtMDktMjZUMjE6Mjk6NDYrMDM6MDAiIHhtcDpNZXRhZGF0YURhdGU9IjIwMjMtMDktMjZUMjE6Mjk6NDYrMDM6MDAiIGRjOmZvcm1hdD0iaW1hZ2UvcG5nIiBwaG90b3Nob3A6Q29sb3JNb2RlPSIzIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjIwZWFhODBjLTYyNTMtOGU0Yi05ZWRlLWQwZmYxMDVkZDNiOCIgeG1wTU06RG9jdW1lbnRJRD0i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";

    let icon_window = load_icon_window(base64_icon);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Yandex Music")
        .with_window_icon(Some(icon_window))
        .with_resizable(false)
        .with_inner_size(dpi::Size::Physical(dpi::PhysicalSize{width: 794, height: 700}))
        .build(&event_loop)?;
    let webview = WebViewBuilder::new(window)?
        .with_url("https://music.yandex.by/home")?
        .build()?;

    let icon = load_icon(base64_icon);

    let tray_menu = Menu::new();

    let _ = tray_menu.append_items(&[
        &MenuItem::new("Развернуть", true, None),
        &MenuItem::new("Выход", true, None)
    ]);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("yandex music")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::NewEvents(StartCause::Init) => (),
            Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
            } => {
                webview.window().set_visible(false);
            },
            _ => (),
        }

        if let Ok(event) = menu_channel.try_recv() {

            if event.id == MenuId("1001".to_string()) {
                webview.window().set_visible(true); 
            }

            if event.id == MenuId("1002".to_string()) {
                tray_icon.take();
                *control_flow = ControlFlow::Exit
            }
        }
    });
}

fn load_icon(base64_icon: &str) -> tray_icon::Icon {
    let icon_data = general_purpose::STANDARD.decode(base64_icon).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(&icon_data)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn load_icon_window(base64_icon: &str) -> wry::application::window::Icon {
    let icon_data = general_purpose::STANDARD.decode(base64_icon).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(&icon_data)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    wry::application::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}