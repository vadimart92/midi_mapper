pub mod main {
    extern crate event_source;

    use event_source::main::EventEmitter;
    use std::sync::mpsc::Sender;

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum NoteEvent {
        NoteOn { pitch: u8, velocity: u8 },
        NoteOff {},
    }
    pub struct HidEventEmitter {
        sender: Option<Sender<NoteEvent>>,
    }

    impl HidEventEmitter {
        pub fn new() -> Self {
            HidEventEmitter { sender: None }
        }
        fn publish_event(&self, evt: NoteEvent) {
            self.sender.as_ref().unwrap().send(evt).unwrap()
        }
        pub fn start(&self) {}
    }
    impl EventEmitter<NoteEvent> for HidEventEmitter {
        fn connect(&mut self, sender: Sender<NoteEvent>) {
            self.sender = Some(sender);
        }
        fn disconnect(&mut self) {
            self.sender = None;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::main::{HidEventEmitter, NoteEvent};
        use event_source::main::EventEmitter;
        use std::sync::mpsc;

        #[test]
        fn connect_works() {
            let mut source = HidEventEmitter::new();
            let (send, recv) = mpsc::channel::<NoteEvent>();
            source.connect(send);
            let event = NoteEvent::NoteOn {
                velocity: 64,
                pitch: 12,
            };
            source.publish_event(event.clone());
            assert_eq!(Ok(event), recv.recv());
            source.disconnect();
        }
    }
}
