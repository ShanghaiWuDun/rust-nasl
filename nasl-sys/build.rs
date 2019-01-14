fn main(){
    println!("cargo:rustc-link-lib=dylib=nasl");
    println!("cargo:rustc-link-search=native=./c/");
}
