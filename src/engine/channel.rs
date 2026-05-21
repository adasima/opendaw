use crate::engine::synth::{AdsrParams, Waveform};
use ringbuf::HeapRb;
use ringbuf::traits::Split;
use ringbuf::wrap::caching::CachingCons;
use ringbuf::wrap::caching::CachingProd;

/// オーディオスレッドからUIスレッドへのプロデューサー
pub type AudioToUiProducer<T> = CachingProd<std::sync::Arc<HeapRb<T>>>;
/// オーディオスレッドからUIスレッドへのコンシューマー
pub type AudioToUiConsumer<T> = CachingCons<std::sync::Arc<HeapRb<T>>>;

/// UIスレッドからオーディオスレッドへのプロデューサー
pub type UiToAudioProducer<T> = CachingProd<std::sync::Arc<HeapRb<T>>>;
/// UIスレッドからオーディオスレッドへのコンシューマー
pub type UiToAudioConsumer<T> = CachingCons<std::sync::Arc<HeapRb<T>>>;

/// 最大同時発音数
pub const MAX_ACTIVE_NOTES: usize = 16;

/// UIからオーディオスレッドへのメッセージ
pub enum UiToAudioMsg {
    /// 再生状態の変更
    SetPlaying(bool),
    /// トラックIDとアクティブなノートの周波数配列、有効なノート数
    ActiveNotes(usize, [f32; MAX_ACTIVE_NOTES], usize),
    /// シンセサイザーパラメータの更新
    UpdateSynthParams(usize, Waveform, AdsrParams),
}

/// オーディオスレッドからUIへのメッセージ
pub enum AudioToUiMsg {
    /// 現在の再生位置
    PlaybackPosition(f32),
}

/// UI用チャンネルペア
pub type UiChannels = (
    UiToAudioProducer<UiToAudioMsg>,
    AudioToUiConsumer<AudioToUiMsg>,
);
/// オーディオ用チャンネルペア
pub type AudioChannels = (
    UiToAudioConsumer<UiToAudioMsg>,
    AudioToUiProducer<AudioToUiMsg>,
);

/// UIとオーディオスレッド間の通信チャンネルを作成する
pub fn create_channels(capacity: usize) -> (UiChannels, AudioChannels) {
    let to_audio = HeapRb::<UiToAudioMsg>::new(capacity);
    let to_ui = HeapRb::<AudioToUiMsg>::new(capacity);

    let (ui_prod, audio_cons) = to_audio.split();
    let (audio_prod, ui_cons) = to_ui.split();

    ((ui_prod, ui_cons), (audio_cons, audio_prod))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ringbuf::traits::{Consumer, Producer};

    #[test]
    fn test_channels() {
        let ((mut ui_prod, mut ui_cons), (mut audio_cons, mut audio_prod)) = create_channels(10);

        // UI -> Audio
        assert!(ui_prod.try_push(UiToAudioMsg::SetPlaying(true)).is_ok());
        if let Some(UiToAudioMsg::SetPlaying(playing)) = audio_cons.try_pop() {
            assert!(playing);
        } else {
            panic!("Message not received");
        }

        // UI -> Audio (ActiveNotes)
        let mut notes = [0.0; MAX_ACTIVE_NOTES];
        notes[0] = 440.0;
        assert!(ui_prod.try_push(UiToAudioMsg::ActiveNotes(1, notes, 1)).is_ok());
        if let Some(UiToAudioMsg::ActiveNotes(id, recv_notes, count)) = audio_cons.try_pop() {
            assert_eq!(id, 1);
            assert_eq!(count, 1);
            assert_eq!(recv_notes[0], 440.0);
        } else {
            panic!("ActiveNotes message not received");
        }

        // Audio -> UI
        assert!(
            audio_prod
                .try_push(AudioToUiMsg::PlaybackPosition(1.5))
                .is_ok()
        );
        if let Some(AudioToUiMsg::PlaybackPosition(pos)) = ui_cons.try_pop() {
            assert_eq!(pos, 1.5);
        } else {
            panic!("Message not received");
        }
    }
}
