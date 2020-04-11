use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

pub struct IDGenerator {
    counter: AtomicUsize
}

impl IDGenerator {
    pub fn new() -> IDGenerator {
        IDGenerator {
            counter: AtomicUsize::new(1),
        }
    }

    pub fn next_id(&self) -> usize {
        return self.counter.fetch_add(1, SeqCst);
    }
}

#[cfg(test)]
mod id_generator_test {
    use crate::id_generator::IDGenerator;

    #[test]
    fn next_id() {
        let gen = IDGenerator::new();
        let id = gen.next_id();
        assert_eq!(1, id);
        let id = gen.next_id();
        assert_eq!(2, id);
    }
}


