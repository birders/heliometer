extern crate assert_cli;
extern crate heliometer;

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

mod tests {
    use heliometer;
    use std;
    use std::io::Read;

    #[test]
    fn far_right() {

        let mut file = std::fs::File::open("examples/far_right.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        let mut buff = std::io::Cursor::new(vec![0; 1]);

        heliometer::execute(&input, &mut std::io::stdin(), &mut buff).unwrap();

        assert_eq!(&buff.get_ref()[0], &b'#');
    }
}
