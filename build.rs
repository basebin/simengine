fn main() {
    #[cfg(feature = "cxx")]
    cxx_build::bridge("engine/cpp_bindings.rs").compile("simulation-engine-cpp");
}
