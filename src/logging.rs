use crate::monad::Monad;

#[derive(Debug)]
pub struct Logging<T> {
    value: T,
    logs: Vec<&'static str>,
}

impl<T> Monad<T> for Logging<T> {
    type Impl<U> = Logging<U>;

    // Constructor
    fn new(value: T) -> Self {
        Self {
            value: value,
            logs: Vec::new(),
        }
    }

    // Map / Flatmap / bind
    fn map<U, F>(self, transform: F) -> Logging<U>
    where
        F: FnOnce(T) -> Logging<U>,
    {
        let mut transformed = transform(self.value);
        let mut logs = self.logs;

        logs.append(&mut transformed.logs);

        Logging::<U> {
            value: transformed.value,
            logs: logs,
        }
    }
}

impl<T> Logging<T> {
    pub fn new_with_logs(value: T, log: &'static str) -> Self {
        Self {
            value: value,
            logs: vec![log],
        }
    }
}

impl Logging<i32> {
    pub fn double(self) -> Logging<i32> {
        self.map(|value| Logging::new_with_logs(value * 2, "doubled the value"))
    }
}
