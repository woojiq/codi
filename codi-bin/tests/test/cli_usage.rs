#![allow(unused_assignments)]

macro_rules! gen_cli_test {
    (OK $cmd:ident) => {
        $cmd = $cmd.success()
    };
    (ERR $cmd:ident) => {
        $cmd = $cmd.failure()
    };
    (eq $exp:expr) => {
        predicates::ord::eq($exp)
    };
    (regex $exp:expr) => {
        predicates::str::is_match($exp).unwrap()
    };
    (out $cmd:ident $pred:ident $exp:expr) => {
        $cmd = $cmd.stdout(gen_cli_test!($pred $exp))
    };
    (err $cmd:ident $pred:ident $exp:expr) => {
        $cmd = $cmd.stderr(gen_cli_test!($pred $exp))
    };
    ($name:ident $res:ident $args:expr; $($stream:ident $pred:ident $exp:expr),*) => {
        #[test]
        fn $name() {
            let mut cmd = assert_cmd::cargo_bin_cmd!("codi");
            let mut cmd = cmd.args($args).assert();
            $(gen_cli_test!($stream cmd $pred $exp);)*
            gen_cli_test!($res cmd);
        }
    };
    () => {}
}

const HELP_REGEX: &str = "(?s)Usage.*Args.*Options";

gen_cli_test! {help_msg OK ["--help"]; out regex HELP_REGEX}
gen_cli_test! {wrong_usage ERR ["--not-exist-option"]; err regex HELP_REGEX, err regex "invalid option '--not-exist-option'"}
gen_cli_test! {wrong_hex ERR ["12345"]; err regex "cannot parse argument \"12345\""}
gen_cli_test! {correct_hex OK ["#FF55FF"]; out eq
"+--------------------+------------+---------+----+
| Algorithm          | HTML color | Hex     |    |
+--------------------+------------+---------+----+
| > Original color   | unknown    | #FF55FF |    |
+--------------------+------------+---------+----+
| Euclidean          | violet     | #EE82EE |    |
+--------------------+------------+---------+----+
| Euclidean Improved | violet     | #EE82EE |    |
+--------------------+------------+---------+----+
| CIE94              | magenta    | #FF00FF |    |
+--------------------+------------+---------+----+
"
}
gen_cli_test! {hex_matches_html_color OK ["ff7f50"]; out regex "> Original color.*coral.* #FF7F50"}
