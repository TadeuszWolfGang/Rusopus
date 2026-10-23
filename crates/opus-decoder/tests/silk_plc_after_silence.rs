use opus_decoder::OpusDecoder;

/// A SILK wideband 20 ms packet that decodes to near silence.
const QUIET: [u8; 10] = [0x48, 0x09, 0x9e, 0xee, 0x99, 0xb6, 0xe5, 0x73, 0xb3, 0xad];

fn rms(samples: &[f32]) -> f32 {
    (samples.iter().map(|s| s * s).sum::<f32>() / samples.len().max(1) as f32).sqrt()
}

#[test]
fn concealing_after_silence_stays_silent() {
    let mut decoder = OpusDecoder::new(48_000, 2).expect("decoder");
    let mut out = vec![0.0f32; decoder.max_frame_size_per_channel() * 2];
    let mut loudest = 0.0f32;
    for _ in 0..3 {
        let n = decoder
            .decode_float(&QUIET, &mut out, false)
            .expect("decodes");
        loudest = loudest.max(rms(&out[..n * 2]));
    }
    assert!(loudest < 0.01, "near silence: {loudest}");
    for lost in 0..3 {
        let n = decoder
            .decode_float(&[], &mut out, false)
            .expect("conceals");
        let concealed = rms(&out[..n * 2]);
        assert!(
            concealed <= loudest + 0.001,
            "lost frame {lost}: {concealed}"
        );
    }
}
