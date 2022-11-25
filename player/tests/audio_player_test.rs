#[cfg(test)]
mod audio_player_test {
    use audioplayer::audio_player::AudioPlayer;

    #[test]
    fn test_play_file() {
        let player = AudioPlayer::new();
        player.play(r"G:\Audio\sg-timeispend.mp3");
    }
}
