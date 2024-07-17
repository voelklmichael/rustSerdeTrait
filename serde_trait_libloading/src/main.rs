pub fn main() {
    let lib_path = "/home/michael/rustSerdeTrait/serde_trait_libloading/libserde_trait_example.so";

    let (registered, result) = serde_trait_libloading::register_via_library(lib_path);
    result.unwrap();
    dbg!(registered);
}
