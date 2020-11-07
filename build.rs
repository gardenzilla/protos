fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("src/proto/").compile(
        &[
            "src/proto/prelude.proto",
            "src/proto/user.proto",
            "src/proto/customer.proto",
            "src/proto/email.proto",
            "src/proto/product.proto",
            "src/proto/product2.proto",
            "src/proto/source.proto",
            "src/proto/cash.proto",
            "src/proto/giftcard.proto",
            "src/proto/purchase.proto",
        ],
        &["src/proto"],
    )?;
    Ok(())
}
