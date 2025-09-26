#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/topGem.ico"); // multi-res .ico (16/32/48/256) recommended
    res.compile().unwrap();
}
#[cfg(not(windows))]
fn main() {}
