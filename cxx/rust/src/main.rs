#[cxx::bridge]
mod ffi {
    struct Cat {
        name_: String
    }

    extern "Rust" {
        #[cxx_name="make"]
        fn new_cat(name: &str) -> Cat;
        fn meow(self: &Cat);
        fn name(self: &Cat) -> &str;
    }

    unsafe extern "C++" {
        include!("cats/include/cats.hpp");
        fn test();
    }
}

fn new_cat(name: &str) -> ffi::Cat {
    ffi::Cat{name_: name.to_owned()}
}

    impl ffi::Cat {
        fn new(name: &str) -> ffi::Cat {
            new_cat(name)
        }

        fn meow(&self) {
            println!("meow\n");
        }

        fn name(&self) -> &str {
            &self.name_
        }
    }


fn main() {
    ffi::test();
}
