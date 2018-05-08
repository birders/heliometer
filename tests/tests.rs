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
        let mut fake_input = std::io::Cursor::new(vec![0]);
        let mut fake_output = std::io::Cursor::new(vec![0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref()[0], &b'#');
    }

    #[test]
    fn overflow() {
        let mut file = std::fs::File::open("examples/256.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![0]);
        let mut fake_output = std::io::Cursor::new(vec![0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref()[0], &0);
    }

    #[test]
    fn bubble_sort() {
        let mut file = std::fs::File::open("examples/bsort.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![50, 20, 19, 255, 19, 4, 103, 57, 0, 53]);
        let mut fake_output = std::io::Cursor::new(vec![0; 10]);
        let expected_output = std::io::Cursor::new(vec![4, 19, 19, 20, 50, 57, 103, 255, 0, 0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref(), &expected_output.get_ref());
    }

    #[test]
    fn insertion_sort() {
        let mut file = std::fs::File::open("examples/isort.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![50, 20, 19, 255, 19, 4, 103, 57, 0, 53]);
        let mut fake_output = std::io::Cursor::new(vec![0; 10]);
        let expected_output = std::io::Cursor::new(vec![4, 19, 19, 20, 50, 57, 103, 255, 0, 0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref(), &expected_output.get_ref());
    }

    #[test]
    fn quick_sort() {
        let mut file = std::fs::File::open("examples/qsort.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![50, 20, 19, 255, 19, 4, 103, 57, 0, 53]);
        let mut fake_output = std::io::Cursor::new(vec![0; 10]);
        let expected_output = std::io::Cursor::new(vec![4, 19, 19, 20, 50, 57, 103, 255, 0, 0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref(), &expected_output.get_ref());
    }

    #[test]
    fn rot13() {
        let mut file = std::fs::File::open("examples/rot13.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![37, 85, 200, 0]);
        let mut fake_output = std::io::Cursor::new(vec![0; 4]);
        let expected_output = std::io::Cursor::new(vec![37, 72, 200, 0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref(), &expected_output.get_ref());
    }

    #[should_panic]
    #[test]
    fn unmatched_right_bracket() {
        let mut file = std::fs::File::open("examples/unmatched_right.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![0]);
        let mut fake_output = std::io::Cursor::new(vec![0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
    }

    /*
     * #[should_panic]
     * #[test]
     * fn unmatched_left_bracket() {
     *     let mut file = std::fs::File::open("examples/unmatched_left.bf").unwrap();
     *     let mut input = String::new();
     *     file.read_to_string(&mut input).unwrap();
     *     let mut fake_input = std::io::Cursor::new(vec![0]);
     *     let mut fake_output = std::io::Cursor::new(vec![0]);
     *     heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
     * }
     */

    #[test]
    fn other_program_check() {
        let mut file = std::fs::File::open("examples/other.bf").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let mut fake_input = std::io::Cursor::new(vec![0]);
        let mut fake_output = std::io::Cursor::new(vec![0]);
        heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        assert_eq!(&fake_output.get_ref()[0], &b'H');
    }

}
