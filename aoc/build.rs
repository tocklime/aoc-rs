use walkdir::WalkDir;
pub fn main() {
    let files = WalkDir::new("solutions")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|x| x.file_type().is_file() && x.file_name().to_string_lossy().ends_with(".rs"));
    for f in files {
        println!("rerun-if-changed={}", f.path().display());
    }
}