use std::io::{stdout, Write};
use crossterm::{execute, terminal, event::{Event, KeyCode, poll, read}, style::{Print, ResetColor, SetForegroundColor, Color}, cursor::MoveTo};

fn main() {
    let mut buffer = String::new(); // 缓冲区，用于存储用户输入的文本
    let mut cursor_x = 0; // 光标的 X 位置
    let mut cursor_y = 0; // 光标的 Y 位置

    // 初始化终端
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    // 绘制界面
    draw_interface(&mut stdout, &buffer, cursor_x, cursor_y);

    loop {
        // 检查是否有事件
        if poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    KeyCode::Char(c) => {
                        buffer.insert(cursor_y * 80 + cursor_x, c);
                        cursor_x += 1;
                    }
                    KeyCode::Backspace => {
                        if cursor_x > 0 {
                            cursor_x -= 1;
                            buffer.remove(cursor_y * 80 + cursor_x);
                        }
                    }
                    KeyCode::Enter => {
                        cursor_x = 0;
                        cursor_y += 1;
                    }
                    KeyCode::Esc => {
                        break; // 退出编辑器
                    }
                    _ => {}
                }
                // 重绘界面
                draw_interface(&mut stdout, &buffer, cursor_x, cursor_y);
            }
        }
    }

    // 恢复终端并退出
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}

fn draw_interface(stdout: &mut std::io::Stdout, buffer: &String, cursor_x: usize, cursor_y: usize) {
    // 清空终端
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    // 设置前景色
    execute!(stdout, SetForegroundColor(Color::White)).unwrap();

    // 打印缓冲区内容
    for (i, c) in buffer.chars().enumerate() {
        execute!(stdout, Print(c)).unwrap();
        if (i + 1) % 80 == 0 {
            execute!(stdout, Print('\n')).unwrap();
        }
    }

    // 设置光标位置
    execute!(stdout, MoveTo(cursor_x as u16, cursor_y as u16)).unwrap();

    // 重置颜色
    execute!(stdout, ResetColor).unwrap();

    // 刷新屏幕
    stdout.flush().unwrap();
}
