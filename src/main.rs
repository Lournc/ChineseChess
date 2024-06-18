//隐藏Windows上的控制台窗口
#![windows_subsystem = "windows"]

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // 创建视口选项，设置视口的内部大小为1280*720像素
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    // 运行egui应用程序
    eframe::run_native(
        "Chinese Chess", // 应用程序的标题
        options, // 视口选项
        Box::new(|cc| {
            // 为我们提供图像支持
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // 创建并返回一个实现了eframe::App trait的对象
            Box::new(MyApp::new(cc))
        }),
    )
}

enum ChessType {
    兵,
    炮,
    車,
    马,
    象,
    士,
    帅,
}

//MyApp记录两个玩家赢的次数，now表示当前行动的是哪个玩家（true红1，false黑2）
struct MyApp {
    p1: u32,
    p2: u32,
    now: bool,
}

//MyApp 结构体 new 函数
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 结构体赋初值
        Self {
            p1: 0,
            p2: 0,
            now: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 在中央面板上显示egui界面
        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示标题
            ui.heading("Chinese Chess");
            // 创建一个水平布局
            ui.horizontal(|ui| {
                let now_player_label = ui.label("Please Choose you next step: ");

                let player_now_lable = ui.label(if self.now { "Player 1" } else { "Player 2"});
            });

            ui.horizontal(|ui| {
                let p1_win_count = ui.label(String::from("Player 1 win: ") + &*self.p1.to_string());

                let p2_win_count = ui.label(String::from("Player 2 win: ") + &*self.p2.to_string());
            });
        });
    }
}