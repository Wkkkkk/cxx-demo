fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/FiBA.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxx-demo");
}
