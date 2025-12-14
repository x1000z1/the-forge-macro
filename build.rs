fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        res.set_icon("./assets/icon.ico");
        res.set("FileDescription", "TFM");
        res.set("ProductName", "The Forge Macro");
        res.set("ProductVersion", "0.0.1.1");
        res.set("FileVersion", "0.0.1.1");
        // res.set("CompanyName", "@x1000z1");
        // res.set("LegalCopyright", "Copyright (c) 2025 @x1000z1");

        res.compile().unwrap();
    }
}
