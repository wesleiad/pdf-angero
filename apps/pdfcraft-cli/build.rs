//! Windows only: embed version info (VERSIONINFO) into `pdf-angero-cli.exe`, so Explorer's
//! Details tab, `(Get-Item pdf-angero-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `pdfcraft.exe` (`apps/pdfcraft/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PDFCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "PDF - Angerô")
        .set("FileDescription", "PDF - Angerô")
        .set("LegalCopyright", "Grupo Angerô. MIT OR Apache-2.0 (ver NOTICE).")
        .set("OriginalFilename", "pdf-angero-cli.exe")
        .set("InternalName", "pdf-angero-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFCRAFT_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=pdf-angero-cli.exe built without version resources: {e}");
    }
}
