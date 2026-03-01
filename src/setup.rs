//! Environment validation helpers.
//!
//! This module validates required runtime dependencies before launch.

/// Verifies that `python3` and the `build123d` package are available.
///
/// This check is designed as a fast pre-flight gate before running costly orchestration.
///
/// # Examples
///
/// ```no_run
/// use build123d_cad::setup::ensure_build123d;
///
/// if let Err(e) = ensure_build123d() {
///     eprintln!("{}", e);
/// }
/// ```
///
/// # Errors
///
/// Returns `Err(String)` if:
///
/// - `python3` is unavailable, or
/// - `python3` cannot import `build123d`.
///
/// # Panics
///
/// This function does not intentionally panic.
pub fn ensure_build123d() -> Result<(), String> {
    // Check if python3 is available
    let python_check = std::process::Command::new("python3")
        .arg("--version")
        .output();

    match python_check {
        Ok(output) if output.status.success() => {}
        _ => {
            return Err(
                "❌ python3 not found. Please install Python 3.9+ to use build123d.".to_string(),
            );
        }
    }

    // Check if build123d can be imported
    let import_check = std::process::Command::new("python3")
        .args(["-c", "import build123d; print(build123d.__version__)"])
        .output();

    match import_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✅ build123d {} detected", version.trim());
            Ok(())
        }
        _ => Err("❌ build123d not found. Please install it: pip install build123d".to_string()),
    }
}
