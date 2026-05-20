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

/// UIからオーディオスレッドへのメッセージ
pub enum UiToAudioMsg {
    /// 再生状態の変更
    SetPlaying(bool),
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
