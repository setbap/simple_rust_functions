trait Deadline {
    fn is_passed(&self) -> bool;
}

#[derive(Debug)]
struct Event {
    name: String,
    date: String,
}
