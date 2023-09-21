extern crate dxlib_rs;

/// --- use import
use dxlib_rs::dx_common::dxlib::*;
use dxlib_rs::dx_fps::*;
use dxlib_rs::dx_input::*;
use dxlib_rs::dx_window::*;
use rfd::FileDialog;
use std::fs::File;
use std::io::{self, Read};
use sys_info::mem_info;
use sys_info::*;
use text_io::read;
use winmsg::{message_box, MessageBoxIconType, MessageBoxReturnCode, MessageBoxType};
/// ---

/// --- Defines

/// --- Window Property
const WINDOW_TITLE: &str = "dxlib Rust動作確認";
const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

/// --- Input String Pos
const KEY_INPUT_STRING_X: i32 = 45;
const KEY_INPUT_STRING_Y: i32 = WINDOW_HEIGHT - 20;
/// --- Dialog file default path
const DEFAULT_FILE_DIALOG_PATH: &str = "C:\\Users\\";

/// ---

struct UserHelpMenu {
    x: i32,
    y: i32,
    text: String,
}
impl UserHelpMenu {
    fn new() -> UserHelpMenu {
        return UserHelpMenu {
            x: 0,
            y: 0,
            text: String::new(),
        };
    }
}

/// enum:SizeReration
/// Larger:大きい
/// Equal:等しい
/// Smaller:小さい
/// Unknown:不明
#[derive(PartialEq)]
enum SizeReration {
    Larger,  // グラフィックのサイズが指定のサイズより大きい
    Equal,   // グラフィックのサイズが指定のサイズと等しい
    Smaller, // グラフィックのサイズが指定のサイズより小さい
    Unknown, // グラフィックのサイズが不明
}
impl std::fmt::Debug for SizeReration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // match式で独自のフォーマット時の処理を書く
        match self {
            SizeReration::Larger => write!(f, "SizeReration:(Larger)"),
            SizeReration::Equal => write!(f, "SizeReration(Equal)"),
            SizeReration::Smaller => write!(f, "SizeReration(Smaller)"),
            SizeReration::Unknown => write!(f, "SizeReration(Unknown)"),
        }
    }
}

/// 指定のハンドルが指定のサイズ以上か以上か調べる
/// #Arguments
/// handle - 整数値
/// size_x - 整数値
/// size_y - 整数値
/// #Returns
/// Option型のSizeRerationを返す
/// 以上: Larger
/// 以下: Equal
/// 同等: Samller
/// 不明: Unknown
fn graph_to_size(handle: i32, size_x: i32, size_y: i32) -> Option<SizeReration> {
    let mut gh_x = 0;
    let mut gh_y = 0;
    dx_GetGraphSize(handle, &mut gh_x, &mut gh_y);
    if gh_x > size_x || gh_y > size_y {
        return Some(SizeReration::Larger);
    } else if gh_x == size_x || gh_y == size_y {
        return Some(SizeReration::Equal);
    } else if gh_x < size_x || gh_y < size_y {
        return Some(SizeReration::Smaller);
    } else {
        return Some(SizeReration::Unknown);
    }
}
// ファイルサイズをバイト単位で調べて表示する
/// #Arguments
/// path:ファイルのパス
fn file_size_print_to_bytes(path: &str) {
    let metadata = std::fs::metadata(path).unwrap();
    println!("file bytes:{}", metadata.len());
}

fn graph_data_print(handle: i32, size_x: i32, size_y: i32) {
    let mut gh_x = 0;
    let mut gh_y = 0;
    dx_GetGraphSize(handle, &mut gh_x, &mut gh_y);
    let size = graph_to_size(handle, size_x, size_y);
    println!("graph size_x:{:?} size_y:{:?}", size_x, size_y);
    println!("graph gh_x:{:?} gh_y:{:?}", gh_x, gh_y);
    println!("graph kind:{:?}", size.unwrap());
}
fn graph_handle_to_index_print(handle: &mut Vec<i32>) {
    for (index, val) in handle.iter().enumerate() {
        if *val != 0 && *val != -1 {
            println!("graph handle index:{} handle:{}", index, val);
        }
    }
}

fn main() {
    unsafe {
        // ウィンドウのスタイルを11にし、×ボタンを非表示にする。
        dx_SetWindowStyleMode(11);
        // --- 変数定義

        let mut gh_size_x = 0;
        let mut gh_size_y = 0;
        let mut game_end_flag = false;

        let mut ref_window = DxWindow::new();
        let window = ref_window.create_window(
            DxWindow::videomode(WINDOW_WIDTH, WINDOW_HEIGHT, 32).unwrap(),
            WINDOW_TITLE,
        );
        let mut conf_str: String = String::new();
        let mut s = 0;
        let mut file = File::open("./conf.txt").unwrap();

        file.read_to_string(&mut conf_str);

        let mut gh_load_num: usize = conf_str.trim().parse().expect("parse error!");
        let mut input = DxInput::new(Default::default());
        let mut fps = DxFps::new();
        let mut gh_draw_f = false;
        let mut help_draw_f = false;
        let mut string_x = 0;
        let mut string_y = 80;
        let mut string_color = dx_GetColor(255, 255, 255);
        let mut text: Vec<String> = Vec::new();
        let mut gh_handle = vec![0; gh_load_num];
        let mut gh_path: String = String::new();
        let mut gh_index: usize = 0;
        let mut gh_index_draw = 0;
        let mut snd_handle = 0;
        let mut snd_path: String = String::new();
        let mut div_gh_handle = vec![0; 100];
        let mut help_menu: [UserHelpMenu; 10];

        // ---

        // ---

        // ---

        dx_ChangeFont("メイリオ");
        // ウィンドウが開いている場合
        while window.is_open() {
            // 画面をクリア
            dx_ClearDrawScreen();
            // -- 描画・更新

            dx_DrawString(
                0,
                15,
                "サンプルを実行することができます",
                dx_GetColor(255, 255, 255),
            );

            dx_DrawString(
                0,
                30,
                "コロン(:)を押してコマンドを実行してください",
                dx_GetColor(255, 255, 255),
            );
            dx_DrawString(
                0,
                50,
                &format!("Graphics MaxNum:{}", gh_load_num),
                dx_GetColor(255, 255, 0),
            );
            dx_DrawString(
                0,
                60,
                &format!("DivGraphics MaxNum:{}", 11),
                dx_GetColor(255, 255, 0),
            );
            dx_DrawBox(
                0,
                75,
                WINDOW_WIDTH / 2 + 0,
                WINDOW_HEIGHT / 2 + 75,
                dx_GetColor(255, 255, 255),
                FALSE,
            );

            // コマンド 処理
            if dx_CheckHitKey(KEY_INPUT_COLON) == TRUE {
                dx_DrawBox(
                    0,
                    0,
                    WINDOW_WIDTH,
                    WINDOW_HEIGHT,
                    dx_GetColor(100, 100, 255),
                    FALSE,
                );
                dx_DrawString(0, 460, "cmd:", dx_GetColor(100, 100, 255));
                input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
            }
            // endflagが立っている場合、ループを抜ける
            if game_end_flag {
                println!("game_end_flag:{}", game_end_flag);
                println!("終了フラグが立ちました。ゲームを終了します。");
                break;
            }
            // コマンドの文字に対応する処理
            match input.get_input_str().as_str() {
                //  @sysに続くコマンドはすべてシステムコマンドとする
                "@sys" => {
                    input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);

                    if input.get_input_str().as_str() == "os" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);

                        if input.get_input_str().as_str() == "data" {
                            if let Ok(cpu_info) = sys_info::cpu_speed() {
                                println!("CPU Speed: {} MHz", cpu_info);
                            }

                            match mem_info() {
                                Ok(info) => {
                                    println!("Total memory: {} KB", info.total);
                                    println!("Free memory: {} KB", info.free);
                                    println!("Available memory: {} KB", info.avail);
                                    println!("Buffers: {} KB", info.buffers);
                                    println!("Cached: {} KB", info.cached);
                                }
                                Err(error) => {
                                    eprintln!("Error: {}", error);
                                }
                            }
                        }
                    }
                    if input.get_input_str().as_str() == "exit" {
                        let msg = message_box(
                            Some(""),
                            Some("終了しますか？"),
                            Some(MessageBoxType::YesNo),
                            Some(MessageBoxIconType::WARNING),
                            None,
                        );
                        match msg {
                            MessageBoxReturnCode::YES => {
                                game_end_flag = true;
                            }
                            MessageBoxReturnCode::NO => {
                                game_end_flag = false;
                            }
                            _ => {}
                        }
                    }

                    if input.get_input_str().as_str() == "title" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);
                        if input.get_input_str().as_str() == "set" {
                            input.wait_input_key_japanise(
                                KEY_INPUT_STRING_X,
                                KEY_INPUT_STRING_Y,
                                FALSE,
                            );
                            dx_SetMainWindowText(input.get_input_str().as_str());
                        }
                    }

                    if input.get_input_str().as_str() == "help" {}
                }
                "string" => {
                    input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);
                    if input.get_input_str().as_str() == "set" {
                        input.wait_input_key_japanise(
                            KEY_INPUT_STRING_X,
                            KEY_INPUT_STRING_Y,
                            FALSE,
                        );
                        text.push(input.get_input_str().to_string());
                    }

                    if input.get_input_str().as_str() == "color" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);

                        match input.get_input_str().as_str() {
                            "red" => {
                                string_color = dx_GetColor(255, 0, 0);
                            }
                            "green" => {
                                string_color = dx_GetColor(0, 255, 0);
                            }
                            "blue" => {
                                string_color = dx_GetColor(0, 0, 255);
                            }
                            _ => {}
                        }
                    }
                }
                "file" => {
                    input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
                    if input.get_input_str().as_str() == "size" {
                        let file = FileDialog::new()
                            .add_filter("Image Files", &["png", "jpg"])
                            .set_directory(DEFAULT_FILE_DIALOG_PATH)
                            .pick_file();
                        if let Some(file) = file {
                            file_size_print_to_bytes(&file.to_string_lossy().to_string());
                        } else {
                            println!("No file selected");
                        }
                    }
                }
                "divgraph" => {
                    input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);

                    if input.get_input_str().as_str() == "set" {}
                }
                "graph" => {
                    input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
                    if input.get_input_str().as_str() == "load" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
                        gh_index = input.get_input_str().as_str().parse::<usize>().unwrap();

                        let file = FileDialog::new()
                            .add_filter("Image Files", &["png", "jpg"])
                            .set_directory(DEFAULT_FILE_DIALOG_PATH)
                            .pick_file();
                        if let Some(file) = file {
                            gh_path = file.to_string_lossy().to_string();
                            println!("You selected: {}", file.display());
                            gh_handle.insert(gh_index, dx_LoadGraph(&gh_path));
                            if gh_handle[gh_index] == -1 {
                                let title = Some("ロード");
                                let main_msg = Some("画像の読み込みに失敗しました");
                                let icon_type = Some(MessageBoxIconType::ERROR);
                                let msg_type = None;
                                dx_GetGraphSize(
                                    gh_handle[gh_index],
                                    &mut gh_size_x,
                                    &mut gh_size_y,
                                );
                                if gh_handle[gh_index] == -1 {
                                    let msg =
                                        message_box(title, main_msg, msg_type, icon_type, None);
                                }
                            }
                        } else {
                            println!("No file selected");
                        }
                    }

                    if input.get_input_str().as_str() == "size" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
                        let mut gh_select_index: usize =
                            input.get_input_str().as_str().trim().parse().expect("");
                        graph_data_print(gh_handle[gh_select_index], WINDOW_WIDTH, WINDOW_HEIGHT);
                    }

                    if input.get_input_str().as_str() == "index" {
                        graph_handle_to_index_print(&mut gh_handle);
                    }
                    if input.get_input_str().as_str() == "draw" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
                        gh_index_draw = input.get_input_str().as_str().parse::<usize>().unwrap();
                        gh_draw_f = true;
                    }

                    if input.get_input_str().as_str() == "del" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, TRUE);
                        let gh_del_index = input.get_input_str().as_str().parse::<usize>().unwrap();

                        dx_DeleteGraph(gh_handle[gh_del_index]);
                        gh_handle[gh_del_index] = -1;
                        gh_draw_f = false;
                    }
                }
                "sound" => {
                    input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);
                    if input.get_input_str().as_str() == "load" {
                        let file = FileDialog::new()
                            .add_filter("Sound Files", &["ogg", "mp3", "wav"])
                            .set_directory(DEFAULT_FILE_DIALOG_PATH)
                            .pick_file();
                        if let Some(file) = file {
                            snd_path = file.to_string_lossy().to_string();
                            println!("You selected: {}", file.display());
                            snd_handle = dx_LoadSoundMem(&snd_path);
                            if snd_handle == -1 {
                                let title = Some("ロード");
                                let main_msg = Some("画像の読み込みに失敗しました");
                                let icon_type = Some(MessageBoxIconType::ERROR);
                                let msg_type = None;
                                if snd_handle == -1 {
                                    let msg =
                                        message_box(title, main_msg, msg_type, icon_type, None);
                                }
                            }
                        } else {
                            println!("No file selected");
                        }
                    }

                    if input.get_input_str().as_str() == "play" {
                        input.wait_input_key(KEY_INPUT_STRING_X, KEY_INPUT_STRING_Y, FALSE);

                        if input.get_input_str().as_str() == "back" {
                            dx_PlaySoundMem(snd_handle, DX_PLAYTYPE_BACK, TRUE);
                        }

                        if input.get_input_str().as_str() == "normal" {
                            dx_PlaySoundMem(snd_handle, DX_PLAYTYPE_NORMAL, TRUE);
                        }
                        if input.get_input_str().as_str() == "loop" {
                            dx_PlaySoundMem(snd_handle, DX_PLAYTYPE_LOOP, TRUE);
                        }
                    }

                    if input.get_input_str().as_str() == "del" {
                        dx_DeleteSoundMem(snd_handle);
                    }
                }
                _ => {}
            }
            // help_drawフラグが立っている場合、ヘルプリストを表示する
            if help_draw_f {}

            for val in text.clone() {
                dx_DrawString(
                    string_x,
                    string_y,
                    &format!("string: {} ", val),
                    string_color,
                );
            }
            // draw_flagが立っている場合、画像を表示する
            if gh_draw_f {
                let res =
                    graph_to_size(gh_handle[gh_index_draw], WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
                dx_GetGraphSize(gh_handle[gh_index_draw], &mut gh_size_x, &mut gh_size_y);

                if res == SizeReration::Larger || res != SizeReration::Smaller {
                    /*
                    let mut gh_size_x_helf = gh_size_x / 2;
                    let mut gh_size_y_helf = gh_size_y / 2;

                    // サイズがウィンドウの半分のサイズに収まるまで割る
                    while gh_size_x_helf > WINDOW_WIDTH/2 {
                        gh_size_x_helf /= 2;
                    }
                    while gh_size_y_helf > WINDOW_HEIGHT/2 {
                        gh_size_y_helf /= 2;
                    }
                    */
                    // 画像表示用枠より小さくして表示
                    dx_DrawExtendGraph(
                        5,
                        80,
                        5 + WINDOW_WIDTH / 3,
                        80 + WINDOW_HEIGHT / 3,
                        gh_handle[gh_index_draw],
                        TRUE,
                    );
                } else {
                    // 通常通り表示
                    dx_DrawGraph(5, 80, gh_handle[gh_index_draw], TRUE);
                }
            }
            // fpsの値の描画と待機
            fps.draw();
            fps.wait();
            // 画面を更新
            dx_ScreenFlip();

            // ---
        }
        for (val) in &gh_handle {
            // もしリストが空ではないか、0や-1ではない場合
            if !&gh_handle.is_empty() || *val != 0 || *val != -1 {
                // グラフィックハンドルを開放する
                dx_DeleteGraph(*val);
            }
        }
    }
}
