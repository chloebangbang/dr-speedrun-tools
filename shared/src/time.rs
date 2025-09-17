const FPS: usize = 30;

pub fn frames_to_dhms(frames: usize) -> (usize, u8, u8, u8, u8) {
    let mut frame_remainder = frames;
    let d = frame_remainder / (24 * 60 * 60 * FPS);
    frame_remainder = frame_remainder % (24 * 60 * 60 * FPS);
    let h = (frame_remainder / (60 * 60 * FPS)) as u8;
    frame_remainder = frame_remainder % (60 * 60 * FPS);
    let m = (frame_remainder / (60 * FPS)) as u8;
    frame_remainder = frame_remainder % (60 * FPS);
    let (s, cs) = frames_to_secs(frame_remainder);

    return (d, h, m, s as u8, cs);
}

pub fn frames_to_secs(frames: usize) -> (usize, u8) {
    let mut frame_remainder = frames;

    let s = frame_remainder / FPS;
    frame_remainder = frame_remainder % FPS;
    let cs = (frame_remainder * 100 / FPS) as u8;

    return (s, cs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timing_tests() {
        assert_eq!(frames_to_secs(15), (0, 50));
        assert_eq!(frames_to_dhms(15), (0, 0, 0, 0, 50));

        assert_eq!(frames_to_secs(30), (1, 0));
        assert_eq!(frames_to_dhms(30), (0, 0, 0, 1, 0));

        assert_eq!(frames_to_dhms(3600 * 30), (0, 1, 0, 0, 0));
        assert_eq!(frames_to_secs(3600 * 30), (3600, 0));
    }
}