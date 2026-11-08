use std::time::{Duration, Instant};
#[derive(Debug)]

pub struct Timer{
    ///開始時刻
    start_at: Option<Instant>,
    ///経過時間
    accumulated: Duration,
    ///測定目標時間
    target:Duration,
}

impl Timer{
    ///新しいタイマーを作成。
    /// # 引数
    /// `target`：測定目標時間（秒）
    /// # パニック
    /// `target`：が0の場合にパニック。
    pub fn new(target:Duration) -> Self{
        assert!(!target.is_zero(),"0秒より大きくしてください。");
        Self{
            start_at:None,
            accumulated: Duration::ZERO,
            target: target,
        }
    }

    pub fn check_finished(&mut self) -> bool{
        if !self.is_running(){
            return false;
        }
        
        if self.elapsed() >= self.target{
            self.reset();
            return true;
        }
        false
    }

    pub fn start(&mut self){
        if self.start_at.is_none(){
            //Option<T>なのでSomeで包む
            self.start_at = Some(Instant::now());
        }
    }

    pub fn stop(&mut self){      
        //start_atに値が入っていればそれをstartに取り出す
        if let Some(start) = self.start_at.take(){
            self.accumulated += start.elapsed();
        }
    }

    pub fn reset(&mut self){
        self.start_at = None;
        self.accumulated = Duration::ZERO;
    }

    pub fn is_running(&self) -> bool {
        self.start_at.is_some()
    }

    pub fn accumulated(&self) -> Duration {
        self.accumulated
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_at{
            Some(start) => self.accumulated + start.elapsed(),
            None => self.accumulated,
        }
    }

}