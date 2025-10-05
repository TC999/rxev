// Cargo.toml 依赖：
// x11rb = "0.12"

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::Event;
use std::env;

const INNER_WINDOW_WIDTH: u16 = 50;
const INNER_WINDOW_HEIGHT: u16 = 50;
const INNER_WINDOW_BORDER: u16 = 4;
const INNER_WINDOW_X: i16 = 10;
const INNER_WINDOW_Y: i16 = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let display = None; // 简化，实际根据命令行参数决定

    let (conn, screen_num) = RustConnection::connect(display)?;
    let setup = conn.setup();
    let screen = &setup.roots[screen_num];

    // 创建外部窗口
    let outer_window = conn.generate_id()?;
    let width = INNER_WINDOW_WIDTH + 2 * (INNER_WINDOW_BORDER + INNER_WINDOW_X as u16) + 100;
    let height = INNER_WINDOW_HEIGHT + 2 * (INNER_WINDOW_BORDER + INNER_WINDOW_Y as u16) + 100;

    conn.create_window(
        x11rb::COPY_FROM_PARENT, outer_window, screen.root,
        100, 100, width, height, 2,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .border_pixel(screen.black_pixel)
            .event_mask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE | EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY)
    )?;

    // 创建内部窗口
    let inner_window = conn.generate_id()?;
    conn.create_window(
        x11rb::COPY_FROM_PARENT, inner_window, outer_window,
        INNER_WINDOW_X, INNER_WINDOW_Y, INNER_WINDOW_WIDTH, INNER_WINDOW_HEIGHT, INNER_WINDOW_BORDER,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .border_pixel(screen.black_pixel)
            .event_mask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE | EventMask::EXPOSURE)
    )?;

    // 设置窗口标题
    conn.change_property8(
        PropMode::REPLACE, outer_window, AtomEnum::WM_NAME, AtomEnum::STRING,
        b"Event Tester"
    )?;

    // 显示窗口
    conn.map_window(inner_window)?;
    conn.map_window(outer_window)?;
    conn.flush()?;

    println!("Outer window is 0x{:x}, inner window is 0x{:x}", outer_window, inner_window);

    // 事件循环
    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::KeyPress(ev) => {
                println!("KeyPress event, detail: {:?}", ev);
            }
            Event::KeyRelease(ev) => {
                println!("KeyRelease event, detail: {:?}", ev);
            }
            Event::Expose(ev) => {
                println!("Expose event: x={}, y={}, width={}, height={}", ev.x, ev.y, ev.width, ev.height);
            }
            Event::ConfigureNotify(ev) => {
                println!("ConfigureNotify event: x={}, y={}, width={}, height={}", ev.x, ev.y, ev.width, ev.height);
            }
            Event::ClientMessage(ev) => {
                println!("ClientMessage event: {:?}", ev);
            }
            // 这里补充更多事件分支
            _ => {
                println!("Unknown event: {:?}", event);
            }
        }
    }
}