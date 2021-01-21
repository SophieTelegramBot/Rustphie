
#[cfg(test)]
mod callback_data_parser {
    use rustphie_macros::*;
    use rustphie_helpers::*;

    #[test]
    fn basic_test() {
        #[derive(CallbackQuery)]
        #[callback_query(prefix = "test")]
        struct Test {}
        let res = Test::new();
        assert_eq!(res, "test");
    }

    #[test]
    fn field_eq_test() {
        #[derive(CallbackQuery)]
        #[callback_query(prefix = "test")]
        struct Test {
            arg1: String,
            arg2: bool,
            arg3: i32,
        }

        let payload = Test::new("awoo".into(), true, 12);
        let res = Test::parse(payload).unwrap();

        assert_eq!(res.arg1, "awoo");
        assert_eq!(res.arg2, true);
        assert_eq!(res.arg3, 12);
    }
}
