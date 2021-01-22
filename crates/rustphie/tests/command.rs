/// Tests proc macro `Command`

#[cfg(test)]
mod command_parser_tests {
    use rustphie_helpers::*;
    use rustphie_macros::*;

    #[test]
    fn test_basic() {
        #[derive(Command)]
        #[command(command = "test", parser = "re", regex = "(.*)")]
        struct Command {
            arg: String,
        }
        let res = Command::parse("/test arg", "");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().arg, "arg");
    }

    #[test]
    fn test_option_none_arg() {
        #[derive(Command)]
        #[command(command = "test", parser = "re", regex = "(meow|)")]
        struct Command {
            arg: OptionArg<String>
        }
        let res: Result<Command, ParseError> = Command::parse("/test", "");
        assert!(res.is_ok());
        assert!(res.unwrap().arg.is_none());
    }

    #[test]
    fn test_option_some_arg() {
        #[derive(Command)]
        #[command(command = "test", parser = "re", regex = "(meow|)")]
        struct Command {
            arg: OptionArg<String>
        }
        let res: Result<Command, ParseError> = Command::parse("/test meow", "");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().arg.as_ref().unwrap(), &"meow".to_string());
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Command)]
        #[command(command = "test", parser = "re", regex = "(meow)")]
        struct Test(String);

        let res = Test::parse("/test meow", "");
        assert_eq!(res.unwrap().0, "meow");
    }

    #[test]
    fn test_empty_args() {
        #[derive(Command)]
        #[command(command = "test")]
        struct Test;

        let res = Test::parse("/test", "");
        assert!(res.is_ok());
    }

    #[test]
    fn test_bot_username_fail() {
        #[derive(Command)]
        #[command(command = "test")]
        struct Test;

        let res = Test::parse("/test@notbot", "bot");
        assert!(res.is_err());
    }

    #[test]
    fn test_bot_username_pass() {
        #[derive(Command)]
        #[command(command = "test")]
        struct Test;

        let res = Test::parse("/test@bot", "bot");
        assert!(res.is_ok());
    }

    // TODO: test failure cases
}
