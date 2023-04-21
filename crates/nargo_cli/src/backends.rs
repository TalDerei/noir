cfg_if::cfg_if! {
    if #[cfg(any(feature = "plonk_bn254", feature = "plonk_bn254_wasm"))] {
        pub(crate) use aztec_backend::Barretenberg as ConcreteBackend;
    } else {
        compile_error!("please specify a backend to compile with");
    }
}

// Backend usage is mutually exclusive so we only want a single feature to be activated.
#[cfg(all(feature = "plonk_bn254", feature = "plonk_bn254_wasm"))]
compile_error!(
    "feature \"plonk_bn254\"  and feature \"plonk_bn254_wasm\" cannot be enabled at the same time"
);
