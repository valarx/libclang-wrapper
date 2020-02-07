use clang_sys::*;
use libclang_wrapper;
use libclang_wrapper::index;

//#[cfg(feature = "runtime")]
//#[test]
//fn test() {
//    load().unwrap();
//    parse();
//    unload().unwrap();
//}

#[cfg(not(feature = "runtime"))]
#[test]
fn parse_source() {
    let source = index::Source::new(
        "tests/header.h".to_owned(),
        index::DeclarationFromPHCMode::Exclude,
        index::DiagnosticsMode::Enabled,
        index::TUOptionsBuilder::new(),
    );
    match source {
        Ok(source) => {
            assert_eq!(source.cursor_data[0], "add");
            assert_eq!(source.cursor_data[1], "a");
            assert_eq!(source.cursor_data[2], "b");
        }
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
fn test_support() {
    let clang = support::Clang::find(None, &[]).unwrap();
    println!("{:?}", clang);
}