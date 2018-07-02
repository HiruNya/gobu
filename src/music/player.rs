use rodio::{
    Sink,
    Source,
    Decoder,
    default_output_device,
};
use std::{
    io::Cursor,
    thread::spawn,
    sync::{
        Arc,
        mpsc::{
            Sender,
            Receiver,
            channel,
            SendError,
        },
    },
};

/// The Music Player used to play music
pub struct Player {
    sender: Sender<PlayerMsg>,
    receiver: Receiver<PlayerMsg>,
}
impl Player {
    /// Create a new [`Player`] struct.
    pub fn new() -> Self {
        let (to_thread, from_player) = channel();
        let (to_player, from_thread) = channel();
        spawn_thread(to_player, from_player);
        Player {
            sender: to_thread,
            receiver: from_thread,
        }
    }
    /// Play the player.
    pub fn play(&mut self) -> Result<(), SendError<PlayerMsg>> {
        self.sender.send(PlayerMsg::Play)
    }
    /// Stop the player.
    pub fn stop(&mut self) -> Result<(), SendError<PlayerMsg>> {
        self.sender.send(PlayerMsg::Stop)
    }
    /// Pause the player.
    pub fn pause(&mut self) -> Result<(), SendError<PlayerMsg>> {
        self.sender.send(PlayerMsg::Pause)
    }
    /// Set the player using the buffered data of a music file.
    pub fn set(&mut self, buf: Arc<[u8]>) -> Result<(), SendError<PlayerMsg>> {
        self.sender.send(PlayerMsg::AddMusic(buf))?;
        self.sender.send(PlayerMsg::Play)
    }
    /// All the new music that would be set would go infinitely if set to true.
    pub fn set_loop(&mut self, loop_: bool) -> Result<(), SendError<PlayerMsg>> {
        self.sender.send(PlayerMsg::Loop(loop_))
    }
}

fn spawn_thread(sender: Sender<PlayerMsg>, receiver: Receiver<PlayerMsg>) {
    spawn(move ||{
        use self::PlayerMsg::*;
        if let Some(device) = default_output_device() {
            let sink = Sink::new(&device);
            let mut loop_ = true;
            while let Ok(msg) = receiver.recv() {
                match msg {
                    Play => {
                        sink.play();
                    },
                    Stop => {
                        sink.stop();
                    },
                    Pause => {
                        sink.pause();
                    },
                    AddMusic(buf) => {
                        let curs = Cursor::new(buf);
                        match Decoder::new(curs) {
                            Ok(d) => {
                                if loop_ {
                                    sink.append(d.repeat_infinite());
                                } else {
                                    sink.append(d);
                                }
                            },
                            Err(_) => {
                                sender.send(PlayerClosed)
                                    .unwrap();
                                break
                            },
                        }
                    },
                    SetVolume() => {},
                    PlayerClosed => { break },
                    Loop(l) => {
                        loop_ = l;
                    }
                }
            }
        } else { sender.send(PlayerClosed).unwrap(); }
    });
}

pub enum PlayerMsg {
    Play,
    Stop,
    Pause,
    AddMusic(Arc<[u8]>),
    SetVolume(),
    PlayerClosed,
    Loop(bool),
}