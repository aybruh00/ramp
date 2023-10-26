pub mod player {
    use rodio::{Sink, OutputStreamHandle};
    use rodio::{Source, Sample};
    use rodio::cpal::FromSample;

    use anyhow::Result;

    pub struct Player {
        rodio_sink: Sink,
        // now_playing: bool,
        // remaining_in_queue: u32
    }

    impl Player{
        pub fn default(handle: &OutputStreamHandle) -> Result<Player> {
            Ok(Player{
                rodio_sink: Sink::try_new(handle)?,
            })
        }

        pub fn add_to_queue<S>(&self, source: S) 
        where 
            S: Source + Send + 'static,
            S::Item: Sample + Send,
            f32: FromSample<S::Item>,
        {
            self.rodio_sink.append(source);
            self.play();
        }
        
        pub fn toggle_play(&self) -> bool {
            if self.rodio_sink.is_paused() {
                self.play();
                true
            } else {
                self.pause();
                false
            }
        }

        fn play(&self) {
            self.rodio_sink.play();
        }

        fn pause(&self) {
            self.rodio_sink.pause();
        }

        pub fn stop_playing(&self) {
            self.rodio_sink.stop();
        }

        pub fn wait(&self) {
            self.rodio_sink.sleep_until_end();
        }
    }
}
