/// Tests proc macro `Command`

mod tests {
    use rustphie_macros::*;
    use rustphie_helpers::*;

    #[test]
    fn test_basic() {
        #[Command]
        #[command(command = "test", regex = ".*")]
        struct Command {
            arg: String,
        }
        let res = Command::parse("/test arg", "");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().arg, "arg");
    }

    #[test]
    fn test_option_none_arg() {
        #[Command]
        #[command(command = "test", regex = "(meow|)")]
        struct Command {
            arg: OptionArg<String>
        }
        let res: Result<Command, ParseError> = Command::parse("/test", "");
        assert!(res.is_ok());
        assert!(res.unwrap().arg.is_none());
    }

    #[test]
    fn test_option_some_arg() {
        #[Command]
        #[command(command = "test", regex = "(meow|)")]
        struct Command {
            arg: OptionArg<String>
        }
        let res: Result<Command, ParseError> = Command::parse("/test meow", "");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().arg, "meow");
    }

    // TODO: test failure cases
    // TODO: test with bot username
    // TODO: test with empty args
}
