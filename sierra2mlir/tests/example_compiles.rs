macro_rules! impl_tests {
    ( $( $name:ident ),* $(,)? ) => {
        $(
            #[test]
            fn $name() {
                use std::fs::read_to_string;

                let program_path = concat!("../examples/", stringify!($name), ".sierra");
                let sierra_source =
                    read_to_string(program_path).expect("Could not read Sierra source code");

                sierra2mlir::compile(&sierra_source, false, false, false, 1).expect("Error compiling sierra program");
            }
        )*
    };
}

impl_tests!(
    boolean,
    casts,
    destructure,
    felt_is_zero,
    fib,
    fib_simple,
    print_test,
    program,
    simple,
    simple_enum,
    types,
    uint,
);