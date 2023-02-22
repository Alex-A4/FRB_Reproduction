use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

fn main() {
    let raw_opts = RawOpts {
        rust_input: vec![
            "src/module1/some_struct.rs".to_string(),
            "src/module1/module1_1/usage_1.rs".to_string(),
            "src/module1/module1_2/usage_2.rs".to_string(),
        ],
        dart_output: vec![
            "../lib/src/some_struct.dart".to_string(),
            "../lib/src/usage_1.dart".to_string(),
            "../lib/src/usage_2.dart".to_string(),
        ],
        rust_output: Option::from(vec![
            "src/generated.rs".to_string(),
            "src/generated1.rs".to_string(),
            "src/generated2.rs".to_string(),
        ]),
        class_name: Option::from(vec![
            "SomeStruct".to_string(),
            "Usage1".to_string(),
            "Usage2".to_string(),
        ]),
        verbose: true,
        ..Default::default()
    };

    // Generate Rust & Dart ffi bridges
    let configs = config_parse(raw_opts);
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    // Format the generated Dart code
    _ = std::process::Command::new("flutter")
        .arg("format -l 100")
        .arg("..")
        .spawn();
}
