// Generic type?
struct Stack {
    storage: Vec<String>,
}

impl Stack {
    pub fn new() -> Stack {
        // The book recommends using an array and
        // a stack pointer that inc/dec on push/pop
        Stack {
            storage: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        // Append to the last of storage
    }

    pub fn pop(&mut self) {
        // Return the last element of storage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_stack() {
        let mut stack = Stack::new();
    }
}
