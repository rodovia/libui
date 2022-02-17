fn main() 
{
    let build = cmake::build("..");
    println!("cargo:rustc-link-search=native={}", build.display());
    println!("cargo:rustc-link-search=native={}/bin", build.display());
    println!("cargo:rustc-link-lib=dylib=libui");
}