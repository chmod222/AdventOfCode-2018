pub mod input {
    use std::io::{self, BufRead};

    pub fn read_stdin_lines() -> Result<Vec<String>, io::Error> {
        let stdin = io::stdin();

        stdin.lock().lines().collect()
    }
}

#[cfg(test)]
mod tests {
}
