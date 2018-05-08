extern crate assert_cli;

static INVALID_USAGE_OUTPUT_1: &'static str = "Usage: ";
static INVALID_USAGE_OUTPUT_2: &'static str = " <file.bf>";

#[cfg(test)]
mod integration {
    use assert_cli;
    use INVALID_USAGE_OUTPUT_1;
    use INVALID_USAGE_OUTPUT_2;

    #[test]
    fn without_args() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr()
            .contains(INVALID_USAGE_OUTPUT_1)
            .stderr()
            .contains(INVALID_USAGE_OUTPUT_2)
            .unwrap();
    }
}
