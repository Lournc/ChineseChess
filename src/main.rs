#![windows_subsystem = "windows"]

use std::ops::Not;
use eframe::egui;
use eframe::egui::{FontId, RichText};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chinese Chess",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

#[derive(Default)]
struct MyApp {
    p1: u32,
    p2: u32,
    now: bool,
    pieces: Vec<(usize, usize, String, egui::Color32)>, // 包含颜色的棋子
    selected_piece: Option<usize>, // 移向的地方的坐标
    log: String,
    winner: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            p1: 0,
            p2: 0,
            now: true,
            pieces: get_initial_pieces(),
            selected_piece: None,
            log: String::default(),
            winner: String::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Please Choose your next step:");
                ui.label(if self.now { "Player 1" } else { "Player 2" });
            });

            ui.horizontal(|ui| {
                ui.label(format!("Player 1 win: {}", self.p1));
                ui.label(format!("Player 2 win: {}", self.p2));
            });

            let board_size = 600.0; // 棋盘大小
            let grid_size = board_size / 9.0;
            let stroke = egui::Stroke::new(2.0, egui::Color32::BLACK);

            let (response, painter) =
                ui.allocate_painter(egui::Vec2::new(board_size + 50.0, board_size + 50.0), egui::Sense::click());

            let margin = 25.0;
            let top_left = response.rect.min + egui::vec2(margin, margin);

            // 绘制横线
            for i in 0..10 {
                let y = top_left.y + i as f32 * grid_size;
                painter.line_segment(
                    [egui::pos2(top_left.x, y), egui::pos2(top_left.x + 8.0 * grid_size, y)],
                    stroke,
                );
            }

            // 绘制竖线
            for i in 0..9 {
                let x = top_left.x + i as f32 * grid_size;
                if i == 0 || i == 8 {
                    painter.line_segment(
                        [egui::pos2(x, top_left.y), egui::pos2(x, top_left.y + 9.0 * grid_size)],
                        stroke,
                    );
                } else {
                    painter.line_segment(
                        [egui::pos2(x, top_left.y), egui::pos2(x, top_left.y + 4.0 * grid_size)],
                        stroke,
                    );
                    painter.line_segment(
                        [egui::pos2(x, top_left.y + 5.0 * grid_size), egui::pos2(x, top_left.y + 9.0 * grid_size)],
                        stroke,
                    );
                }
            }

            // 绘制楚河汉界
            let river_y = top_left.y + 4.0 * grid_size;
            let river_y2 = top_left.y + 5.0 * grid_size;
            painter.line_segment(
                [egui::pos2(top_left.x, river_y), egui::pos2(top_left.x + 8.0 * grid_size, river_y)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(top_left.x, river_y2), egui::pos2(top_left.x + 8.0 * grid_size, river_y2)],
                stroke,
            );

            // 绘制底部
            draw_palace(&painter, &top_left, 3, 0, grid_size, stroke);

            // 绘制顶部
            draw_palace(&painter, &top_left, 3, 7, grid_size, stroke);

            // 检测鼠标点击位置
            if response.clicked() {
                self.log.push_str(&"Mouse Clicked\n");
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let grid_x = ((pointer_pos.x - top_left.x) / grid_size).round() as usize;
                    let grid_y = ((pointer_pos.y - top_left.y) / grid_size).round() as usize;

                    if let Some(selected_index) = self.selected_piece {
                        self.log.push_str(&format!("Chess position:{}, {}, Target Position:{}, {}\n",
                                                   self.pieces[selected_index].0, self.pieces[selected_index].1, grid_x, grid_y));

                        // 检查移动是否合法
                        let (valid_move, new_x, new_y) =
                            validate_move(&self.pieces[selected_index], (grid_x, grid_y), &self.pieces, self.now, &mut self.log);
                        self.log.push_str(&format!("Move valid: {}\n", valid_move));
                        if valid_move {
                            // 更新棋子位置
                            let old_x = self.pieces[selected_index].0;
                            let old_y = self.pieces[selected_index].1;
                            self.pieces[selected_index].0 = new_x;
                            self.pieces[selected_index].1 = new_y;
                            self.selected_piece = None;
                            self.now = !self.now;  // 切换玩家
                            self.log.push_str(&format!("Piece moved from ({}, {}) to ({}, {})\n", old_x, old_y, new_x, new_y));
                            // 判断新位置有没有棋子，有的话删除该棋子
                            if let Some(chess_idx) = self.pieces.iter()
                                .enumerate()
                                .find(|(_, piece)| piece.0 == new_x && piece.1 == new_y && is_current_player_piece(piece, self.now))
                                .map(|(index, _)| index) {
                                let (_, _, name, _) = self.pieces[chess_idx].clone();
                                self.pieces.remove(chess_idx);
                                if name == "将" || name == "帅" {
                                    if name == "将" { self.p1 += 1; } else { self.p2 += 1; }
                                    self.winner.push_str(if self.now { "Player 2 win!\n" } else { "Player 1 win!\n" });
                                    self.pieces = get_initial_pieces();
                                    self.now = true;
                                    self.selected_piece = None;
                                }
                            }
                        } else {
                            self.selected_piece = None;
                        }
                    } else {
                        // 选择棋子
                        self.selected_piece = self.pieces.iter()
                            .enumerate()
                            .find(|(_, piece)| piece.0 == grid_x && piece.1 == grid_y && is_current_player_piece(piece, self.now))
                            .map(|(index, _)| index);
                    }
                }
            }


            // 绘制棋子
            for (i, piece) in self.pieces.iter_mut().enumerate() {
                let piece_center = egui::pos2(
                    top_left.x + piece.0 as f32 * grid_size,
                    top_left.y + piece.1 as f32 * grid_size,
                );

                draw_piece(&painter, top_left, piece.0, piece.1, grid_size, &piece.2, piece.3);
            }
        });

        // 添加一个可拖动的窗口来显示日志信息
        egui::Window::new("Log")
            .resizable(true)
            .collapsible(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label(&self.log);
                    });
            });

        egui::Window::new("Winner!")
            .resizable(true)
            .collapsible(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label(RichText::new(&self.winner).font(FontId::proportional(35.0)));
                    });
            });
    }
}

fn is_current_player_piece(piece: &(usize, usize, String, egui::Color32), now: bool) -> bool {
    let (_, _, _, color) = piece;
    (now && *color == egui::Color32::RED) || (!now && *color == egui::Color32::BLACK)
}

fn draw_palace(painter: &egui::Painter, top_left: &egui::Pos2, x: usize, y: usize, grid_size: f32, stroke: egui::Stroke) {
    let palace_top_left = egui::pos2(top_left.x + x as f32 * grid_size, top_left.y + y as f32 * grid_size);
    painter.line_segment(
        [palace_top_left, egui::pos2(palace_top_left.x + 2.0 * grid_size, palace_top_left.y + 2.0 * grid_size)],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(palace_top_left.x + 2.0 * grid_size, palace_top_left.y), palace_top_left],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(palace_top_left.x, palace_top_left.y + 2.0 * grid_size), egui::pos2(palace_top_left.x + 2.0 * grid_size, palace_top_left.y)],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(palace_top_left.x + 2.0 * grid_size, palace_top_left.y + 2.0 * grid_size), egui::pos2(palace_top_left.x, palace_top_left.y + 2.0 * grid_size)],
        stroke,
    );
}

fn draw_piece(painter: &egui::Painter, top_left: egui::Pos2, x: usize, y: usize, grid_size: f32, text: &str, color: egui::Color32) {
    let center = egui::pos2(
        top_left.x + x as f32 * grid_size,
        top_left.y + y as f32 * grid_size,
    );

    painter.circle_filled(center, grid_size * 0.4, egui::Color32::from_rgb(255, 204, 153));
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(grid_size * 0.5),
        color,
    );
}

fn get_initial_pieces() -> Vec<(usize, usize, String, egui::Color32)> {
    vec![
        (0, 0, "车".to_string(), egui::Color32::BLACK), (1, 0, "马".to_string(), egui::Color32::BLACK),
        (2, 0, "相".to_string(), egui::Color32::BLACK), (3, 0, "士".to_string(), egui::Color32::BLACK),
        (4, 0, "将".to_string(), egui::Color32::BLACK), (5, 0, "士".to_string(), egui::Color32::BLACK),
        (6, 0, "相".to_string(), egui::Color32::BLACK), (7, 0, "马".to_string(), egui::Color32::BLACK),
        (8, 0, "车".to_string(), egui::Color32::BLACK), (1, 2, "炮".to_string(), egui::Color32::BLACK),
        (7, 2, "炮".to_string(), egui::Color32::BLACK), (0, 3, "兵".to_string(), egui::Color32::BLACK),
        (2, 3, "兵".to_string(), egui::Color32::BLACK), (4, 3, "兵".to_string(), egui::Color32::BLACK),
        (6, 3, "兵".to_string(), egui::Color32::BLACK), (8, 3, "兵".to_string(), egui::Color32::BLACK),
        (0, 9, "车".to_string(), egui::Color32::RED), (1, 9, "马".to_string(), egui::Color32::RED),
        (2, 9, "象".to_string(), egui::Color32::RED), (3, 9, "仕".to_string(), egui::Color32::RED),
        (4, 9, "帅".to_string(), egui::Color32::RED), (5, 9, "仕".to_string(), egui::Color32::RED),
        (6, 9, "象".to_string(), egui::Color32::RED), (7, 9, "马".to_string(), egui::Color32::RED),
        (8, 9, "车".to_string(), egui::Color32::RED), (1, 7, "炮".to_string(), egui::Color32::RED),
        (7, 7, "炮".to_string(), egui::Color32::RED), (0, 6, "卒".to_string(), egui::Color32::RED),
        (2, 6, "卒".to_string(), egui::Color32::RED), (4, 6, "卒".to_string(), egui::Color32::RED),
        (6, 6, "卒".to_string(), egui::Color32::RED), (8, 6, "卒".to_string(), egui::Color32::RED),
    ]
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("font/chinese_font.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

fn validate_move(piece: &(usize, usize, String, egui::Color32), target: (usize, usize), pieces: &[(usize, usize, String, egui::Color32)], now: bool, log: &mut String) -> (bool, usize, usize) {
    let (x, y, kind, color) = piece;
    let (tx, ty) = target;

    // 检查是否当前玩家的棋子
    if ((now && *color == egui::Color32::RED) || (!now && *color == egui::Color32::BLACK)).not() {
        log.push_str(&format!("now:{}, color{}\n", now, if *color == egui::Color32::RED { &"RED" } else { &"BLACK" }));
        return (false, *x, *y); // 如果不是当前玩家的棋子，移动无效
    }

    let is_valid = match kind.as_str() {
        "车" => validate_chariots_move((*x, *y), target, pieces, log),
        "马" => validate_horse_move((*x, *y), target, pieces, log),
        "象" | "相" => validate_elephant_move((*x, *y), target, *color, pieces, log),
        "士" | "仕" => validate_guard_move((*x, *y), target, *color, log),
        "将" | "帅" => validate_king_move((*x, *y), target, *color, log),
        "炮" => validate_cannon_move((*x, *y), target, pieces, log),
        "卒" | "兵" => validate_pawn_move((*x, *y), target, *color, log),
        _ => false,
    };

    (is_valid, tx, ty)
}


fn validate_chariots_move(start: (usize, usize), target: (usize, usize), pieces: &[(usize, usize, String, egui::Color32)], log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    if sx == tx {
        for y in usize::min(sy, ty) + 1..usize::max(sy, ty) {
            if pieces.iter().any(|p| p.0 == sx && p.1 == y) {
                return false;
            }
        }
        true
    } else if sy == ty {
        for x in usize::min(sx, tx) + 1..usize::max(sx, tx) {
            if pieces.iter().any(|p| p.0 == x && p.1 == sy) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn validate_horse_move(start: (usize, usize), target: (usize, usize), pieces: &[(usize, usize, String, egui::Color32)], log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    let dx = sx.abs_diff(tx);
    let dy = sy.abs_diff(ty);

    if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
        if dx == 2 {
            let middle_x = (sx + tx) / 2;
            if pieces.iter().any(|p| p.0 == middle_x && p.1 == sy) {
                return false;
            }
        } else {
            let middle_y = (sy + ty) / 2;
            if pieces.iter().any(|p| p.0 == sx && p.1 == middle_y) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn validate_elephant_move(start: (usize, usize), target: (usize, usize), color: egui::Color32, pieces: &[(usize, usize, String, egui::Color32)], log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    let dx = sx.abs_diff(tx);
    let dy = sy.abs_diff(ty);

    if dx == 2 && dy == 2 {
        let middle_x = (sx + tx) / 2;
        let middle_y = (sy + ty) / 2;

        if sy < 5 && ty >= 5 {
            return false;
        }

        if sy >= 5 && ty < 5 {
            return false;
        }

        !pieces.iter().any(|p| p.0 == middle_x && p.1 == middle_y)
    } else {
        false
    }
}

fn validate_guard_move(start: (usize, usize), target: (usize, usize), color: egui::Color32, log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    let dx = sx.abs_diff(tx);
    let dy = sy.abs_diff(ty);

    if dx == 1 && dy == 1 {
        if color == egui::Color32::BLACK {
            tx >= 3 && tx <= 5 && ty <= 2
        } else {
            tx >= 3 && tx <= 5 && ty >= 7 && ty <= 9
        }
    } else {
        false
    }
}

fn validate_king_move(start: (usize, usize), target: (usize, usize), color: egui::Color32, log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    let dx = sx.abs_diff(tx);
    let dy = sy.abs_diff(ty);

    if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) {
        if color == egui::Color32::BLACK {
            tx >= 3 && tx <= 5 && ty <= 2
        } else {
            tx >= 3 && tx <= 5 && ty >= 7 && ty <= 9
        }
    } else {
        false
    }
}

fn validate_cannon_move(start: (usize, usize), target: (usize, usize), pieces: &[(usize, usize, String, egui::Color32)], log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    if sx == tx {
        let mut obstacle_count = 0;
        for y in usize::min(sy, ty) + 1..usize::max(sy, ty) {
            if pieces.iter().any(|p| p.0 == sx && p.1 == y) {
                obstacle_count += 1;
            }
        }
        if obstacle_count == 0 {
            return !pieces.iter().any(|p| p.0 == tx && p.1 == ty); // 不跳子时不能吃子
        } else if obstacle_count == 1 {
            return pieces.iter().any(|p| p.0 == tx && p.1 == ty); // 跳子时必须吃子
        } else {
            return false;
        }
    } else if sy == ty {
        let mut obstacle_count = 0;
        for x in usize::min(sx, tx) + 1..usize::max(sx, tx) {
            if pieces.iter().any(|p| p.0 == x && p.1 == sy) {
                obstacle_count += 1;
            }
        }
        if obstacle_count == 0 {
            return !pieces.iter().any(|p| p.0 == tx && p.1 == ty); // 不跳子时不能吃子
        } else if obstacle_count == 1 {
            return pieces.iter().any(|p| p.0 == tx && p.1 == ty); // 跳子时必须吃子
        } else {
            return false;
        }
    } else {
        false
    }
}

fn validate_pawn_move(start: (usize, usize), target: (usize, usize), color: egui::Color32, log: &mut String) -> bool {
    let (sx, sy) = start;
    let (tx, ty) = target;

    if color == egui::Color32::BLACK {
        if sy < 5 {
            sx == tx && ty == sy + 1
        } else {
            sx == tx && ty == sy + 1 || sy == ty && tx.abs_diff(sx) == 1
        }
    } else {
        if sy >= 5 {
            sx == tx && ty == sy - 1
        } else {
            sx == tx && ty == sy - 1 || sy == ty && tx.abs_diff(sx) == 1
        }
    }
}

