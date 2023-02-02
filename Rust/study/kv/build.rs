fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]); // 生产 Bytes类型编译,而非缺省的 Vec<u8>
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
