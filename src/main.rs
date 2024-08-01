use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use rand::Rng; // Para generar números aleatorios
mod framebuffer;
use framebuffer::FrameBuffer;

// Define los patrones
fn add_block(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let block = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)];
    for &(bx, by) in &block {
        if by < board.len() && bx < board[0].len() {
            board[by][bx] = 1;
        }
    }
}

fn add_blinker(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let blinker = [(x, y), (x + 1, y), (x + 2, y)];
    for &(bx, by) in &blinker {
        if by < board.len() && bx < board[0].len() {
            board[by][bx] = 1;
        }
    }
}

fn add_toad(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let toad = [(x, y), (x + 1, y), (x + 2, y), (x + 1, y + 1), (x + 2, y + 1), (x + 3, y + 1)];
    for &(tx, ty) in &toad {
        if ty < board.len() && tx < board[0].len() {
            board[ty][tx] = 1;
        }
    }
}

fn add_glider(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let glider = [(x, y), (x + 1, y + 1), (x + 2, y - 1), (x + 2, y), (x + 2, y + 1)];
    for &(gx, gy) in &glider {
        if gy < board.len() && gx < board[0].len() {
            board[gy][gx] = 1;
        }
    }
}

fn add_lightweight_spaceship(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let lwss = [(x, y), (x, y + 2), (x + 1, y + 3), (x + 2, y + 3), (x + 3, y + 3), (x + 4, y + 2), (x + 4, y)];
    for &(sx, sy) in &lwss {
        if sy < board.len() && sx < board[0].len() {
            board[sy][sx] = 1;
        }
    }
}

fn add_pulsar(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let pulsar = [
        (x + 2, y), (x + 3, y), (x + 4, y), (x + 8, y), (x + 9, y), (x + 10, y),
        (x, y + 2), (x + 5, y + 2), (x + 7, y + 2), (x + 12, y + 2),
        (x, y + 3), (x + 5, y + 3), (x + 7, y + 3), (x + 12, y + 3),
        (x, y + 4), (x + 5, y + 4), (x + 7, y + 4), (x + 12, y + 4),
        (x + 2, y + 5), (x + 3, y + 5), (x + 4, y + 5), (x + 8, y + 5), (x + 9, y + 5), (x + 10, y + 5),
        (x + 2, y + 7), (x + 3, y + 7), (x + 4, y + 7), (x + 8, y + 7), (x + 9, y + 7), (x + 10, y + 7),
        (x, y + 8), (x + 5, y + 8), (x + 7, y + 8), (x + 12, y + 8),
        (x, y + 9), (x + 5, y + 9), (x + 7, y + 9), (x + 12, y + 9),
        (x, y + 10), (x + 5, y + 10), (x + 7, y + 10), (x + 12, y + 10),
        (x + 2, y + 12), (x + 3, y + 12), (x + 4, y + 12), (x + 8, y + 12), (x + 9, y + 12), (x + 10, y + 12),
    ];
    for &(px, py) in &pulsar {
        if py < board.len() && px < board[0].len() {
            board[py][px] = 1;
        }
    }
}

// Funciones auxiliares
fn count_neighbors(board: &Vec<Vec<u8>>, x: usize, y: usize, width: usize, height: usize) -> u8 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx + width as isize) as usize % width;
            let ny = (y as isize + dy + height as isize) as usize % height;
            count += board[ny][nx];
        }
    }
    count
}

fn next_state(board: &Vec<Vec<u8>>, width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut new_board = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            let neighbors = count_neighbors(board, x, y, width, height);
            if board[y][x] == 1 {
                if neighbors < 2 || neighbors > 3 {
                    new_board[y][x] = 0;
                } else {
                    new_board[y][x] = 1;
                }
            } else {
                if neighbors == 3 {
                    new_board[y][x] = 1;
                }
            }
        }
    }
    new_board
}

fn render(framebuffer: &mut FrameBuffer, board: &Vec<Vec<u8>>, width: usize, height: usize) {
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFDDDD);
    for y in 0..height {
        for x in 0..width {
            if board[y][x] == 1 {
                framebuffer.point(x, y);
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut board = vec![vec![0; framebuffer_width]; framebuffer_height];

    // Añadir varios patrones en diferentes posiciones y estados
    let mut rng = rand::thread_rng();
    for _ in 0..30 {
        let x = rng.gen_range(0..framebuffer_width);
        let y = rng.gen_range(0..framebuffer_height);
        match rng.gen_range(0..6) {
            0 => add_block(&mut board, x, y),
            1 => add_blinker(&mut board, x, y),
            2 => add_toad(&mut board, x, y),
            3 => add_glider(&mut board, x, y),
            4 => add_lightweight_spaceship(&mut board, x, y),
            5 => add_pulsar(&mut board, x, y),
            _ => (),
        }
    }

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        board = next_state(&board, framebuffer_width, framebuffer_height);
        render(&mut framebuffer, &board, framebuffer_width, framebuffer_height);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
