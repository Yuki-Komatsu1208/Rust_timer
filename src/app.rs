use std::time::{Duration, Instant};
use crate::timer::Timer;
use dioxus::prelude::*;
use dioxus::desktop::{
    trayicon::{
        init_tray_icon,
        menu::{Menu, MenuItem},
    },
    use_muda_event_handler,
    use_window,
    Config,
    WindowCloseBehaviour,
};
use crate::notification::show_timer_finished;

const MAIN_CSS: Asset = asset!("/assets/main.css");



#[component]
pub fn App() -> Element {


    let mut timer = use_signal(|| Timer::new(Duration::from_secs(25*60)));
    let mut elapsed = use_signal(||Duration::ZERO);
    let mut finished = use_signal(|| false);
    // 入力された分数
    let mut input_minutes = use_signal(|| 25.to_string());
    let mut input_error = use_signal(String::new);

    use_future(move||async move{
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let just_finished = timer.write().check_finished();
            let curren_elaosed = timer.read().elapsed();
            elapsed.set(curren_elaosed);
            if just_finished {
                finished.set(true);
                println!("タイマーが終了しました");
                show_timer_finished();
            }
        }
    });

    ///トレイ状態
    use_hook(||{
        let menu = Menu::new();

        let show_item = MenuItem::with_id("show","表示",true,None);
        let quit_item = MenuItem::with_id("quit","終了",true,None);

        menu.append_items(&[&show_item,&quit_item]).expect("トレイトメニューの追加に失敗しました");

        init_tray_icon(menu,None);

    });

    let window = use_window();
    ///トレイ上の操作イベント
    use_muda_event_handler(move |event|{
        match event.id.as_ref(){
            "show"=>{
                window.set_minimized(false);
                window.set_visible(true);
                window.set_focus();
            }
            "quit"=>{
                window.set_close_behavior(WindowCloseBehaviour::WindowCloses);
                window.close();
            }
            _=> {}
        }
    });

    ///UI
    rsx! {
        document::Link{rel:"stylesheet",href:MAIN_CSS}

        label{
            r#for:"minutes",
            "計測時間（分）"
        }
        input {
            id:"minutes",
            r#type:"number",
            min:"1",
            step:"1",
            value:input_minutes(),
            oninput: move |event| {
                input_minutes.set(event.value());
                input_error.set(String::new());
            }
        }

        button { 
            disabled: timer.read().is_running(),
            onclick: move |_|{
                let parsed_minutes = input_minutes().parse::<u32>();

                match parsed_minutes{
                    Ok(minutes) if(minutes > 0) =>{
                        let seconds = u64::from(minutes)*60;
                        let target = Duration::from_secs(seconds);
                        
                        timer.set(Timer::new(target));
                        elapsed.set(Duration::ZERO);
                        finished.set(false);    
                        input_error.set(String::new());
                    
                    }

                    _=>{
                        input_error.set("整数を入れてください".to_string());
                    }
                }
            },
        "設定"
        }

        if !input_error().is_empty() {
            p {"{input_error}"}
        }



        p{"経過時間:{elapsed().as_secs()}.{elapsed().subsec_millis():03}秒"}
        p {  
            if timer.read().is_running() {
                "動作中です"
            } else {
                "停止中です"
            }
        }

        if finished(){
            "タイマーが終了しました"
        }

        button {
            onclick: move |_| {
                finished.set(false);
                timer.write().start();
            },
            "開始"
        }

        button {
            onclick: move |_| {
                timer.write().stop();
                let current_elapsed = timer.read().elapsed();
                elapsed.set(current_elapsed);
            },
            "停止"
        }

        button {
            onclick: move |_| {
                timer.write().reset();
                elapsed.set(Duration::ZERO);
                finished.set(false);
            },
            "リセット"
        } 
        
    }
}
