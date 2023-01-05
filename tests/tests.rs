use datadriven::{walk, walk_async, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        walk("tests/testdata", |f| {
            f.run(|s| -> Result<String> {
                let mut result = String::new();
                result.push_str(&s.input.trim());
                result.push_str("\n");
                Ok(result)
            })
        });
    }

    #[tokio::test]
    async fn run_async() {
        walk_async("tests/testdata_async", |mut f| async move {
            f.run_async(|s| async move {
                let mut result = String::new();
                result.push_str(&s.input.trim());
                result.push_str("\n");
                result
            })
            .await;
            f
        })
        .await;
    }
}
