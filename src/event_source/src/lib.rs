pub mod main {
    use std::sync::mpsc::Sender;

    pub trait EventEmitter<T> {
        fn connect(&mut self, sender: Sender<T>);
        fn disconnect(&mut self);
    }

    pub struct TestEventEmitter<T>
    where
        T: Copy,
    {
        pub test_events: Vec<T>,
        sender: Option<Sender<T>>,
    }

    impl<T> TestEventEmitter<T> where T: Copy {}

    impl<T> TestEventEmitter<T>
    where
        T: Copy,
    {
        pub fn new(events: Vec<T>) -> Self {
            TestEventEmitter {
                test_events: events,
                sender: None,
            }
        }
    }
    impl<T> EventEmitter<T> for TestEventEmitter<T>
    where
        T: Copy,
    {
        fn connect(&mut self, sender: Sender<T>) {
            self.sender = Some(sender.clone());
            let sender = self.sender.as_ref().unwrap();
            for test_event in self.test_events.iter() {
                sender.send(*test_event).unwrap();
            }
        }
        fn disconnect(&mut self) {
            self.sender = None;
        }
    }
}
#[cfg(test)]
mod test_event_emitter_tests {
    use crate::main::{EventEmitter, TestEventEmitter};
    use std::sync::mpsc::{channel, RecvTimeoutError};
    use std::time::Duration;

    #[test]
    fn connect_should_publish_test_events() {
        let (send, recv) = channel();
        let mut source = TestEventEmitter::new(vec![1, 2, 3]);
        {
            source.connect(send);
            assert_eq!(Ok(1), recv.recv_timeout(Duration::from_millis(10)));
            assert_eq!(Ok(2), recv.recv());
            assert_eq!(Ok(3), recv.recv());
            source.disconnect();
        }
        assert_eq!(
            Err(RecvTimeoutError::Disconnected),
            recv.recv_timeout(Duration::from_millis(10))
        );
    }
}
