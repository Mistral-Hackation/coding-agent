fn main() {
    let path = std::path::Path::new(".output/output_20260228_145740");
    match build123d_cad::viewer::generate_and_open(path) {
        Ok(p) => println!("✅ Viewer: {}", p.display()),
        Err(e) => println!("❌ {}", e),
    }
}
