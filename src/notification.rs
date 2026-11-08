use notify_rust::Notification;

pub fn show_timer_finished(){
    let result = Notification::new()
        .summary("タイマー")
        .body("タイマーが終了しました")
        .show();

    match result{
        Ok(_) => {},
        Err(e) => eprintln!("通知の表示に失敗しました: {}", e),
    }
}