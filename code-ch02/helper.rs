//! Small Chapter 2 helper utilities.

/// Runs one test callback, mirroring the source chapter's `helper.run(test)` helper.
pub fn run(test: impl FnOnce()) {
    test();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn runs_the_supplied_callback() {
        let mut was_run = false;
        run(|| was_run = true);
        assert!(was_run);
    }
}
