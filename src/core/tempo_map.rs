/// テンポと拍子記号の変更を管理するモジュール
/// 
/// タイムライン上の特定の時間（Tick）におけるBPMや拍子のイベントリストを保持し、
/// 指定した時間におけるテンポ情報を取得する機能を提供します。
///
/// 特定のTickにおけるBPMの変更を表すイベント
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TempoEvent {
    pub tick: u64,
    pub bpm: f64,
}

/// 特定のTickにおける拍子記号の変更を表すイベント
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSignatureEvent {
    pub tick: u64,
    pub numerator: u8,
    pub denominator: u8,
}

/// テンポと拍子記号のマップ
#[derive(Debug, Clone)]
pub struct TempoMap {
    tempo_events: Vec<TempoEvent>,
    time_signature_events: Vec<TimeSignatureEvent>,
}

impl TempoMap {
    /// 新しいTempoMapを作成します
    pub fn new() -> Self {
        // デフォルトのテンポ（120 BPM）と拍子（4/4）をTick 0の初期イベントとして設定
        Self {
            tempo_events: vec![TempoEvent { tick: 0, bpm: 120.0 }],
            time_signature_events: vec![TimeSignatureEvent { tick: 0, numerator: 4, denominator: 4 }],
        }
    }

    /// テンポ変更イベントを追加します
    pub fn add_tempo_event(&mut self, tick: u64, bpm: f64) {
        self.tempo_events.push(TempoEvent { tick, bpm });
        // イベントをTickの昇順に保つ
        self.tempo_events.sort_by_key(|e| e.tick);
    }

    /// 拍子変更イベントを追加します
    pub fn add_time_signature_event(&mut self, tick: u64, numerator: u8, denominator: u8) {
        self.time_signature_events.push(TimeSignatureEvent { tick, numerator, denominator });
        // イベントをTickの昇順に保つ
        self.time_signature_events.sort_by_key(|e| e.tick);
    }

    /// 指定したTickにおけるテンポ（BPM）を取得します
    pub fn get_tempo_at(&self, tick: u64) -> f64 {
        // 指定されたTick以下の最大のTickを持つイベントのBPMを返す
        let mut current_bpm = 120.0;
        for event in &self.tempo_events {
            if event.tick <= tick {
                current_bpm = event.bpm;
            } else {
                break;
            }
        }
        current_bpm
    }

    /// 指定したTickにおける拍子記号（分子, 分母）を取得します
    pub fn get_time_signature_at(&self, tick: u64) -> (u8, u8) {
        // 指定されたTick以下の最大のTickを持つイベントの拍子を返す
        let mut current_sig = (4, 4);
        for event in &self.time_signature_events {
            if event.tick <= tick {
                current_sig = (event.numerator, event.denominator);
            } else {
                break;
            }
        }
        current_sig
    }
}
