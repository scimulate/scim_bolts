//use opencascade_sys;


#[cxx::bridge]
mod ffi {
    /*
    // Shared structs with fields visible to both languages.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }
    */

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        //include!("/usr/include/opencascade/");
        //include!("/usr/include/gsl/");
        include!("scim_bolts/include/squared.h");
        
        fn squared();

        //include!("demo/include/blobstore.h");

        //type BlobstoreClient;

        //fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        //fn put(&self, parts: &mut MultiBuf) -> u64;
        //fn tag(&self, blobid: u64, tag: &str);
        //fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}

fn main() {
    //opencascade_sys::ffi::BRepPrimAPI_MakeCylinder(f64->1.0, 1.0);
    println!("Hello, world!");

    ffi::squared();

    //cxx::bridge!();

    //ffi::squared();
    //squared(&mut 3.1);
    //println!("{}", ffi::squared(&mut 5.2));
}
