#![cfg_attr(not(test), no_std)]

pub struct Undroppable;

impl Drop for Undroppable {
    fn drop(&mut self) {
        extern "Rust" {
            #[link_name = "\nerror(undroppable): your program contains at least one instance of an Undroppable being dropped"]
            fn undefined() -> !;
        }

        unsafe { undefined() }
    }
}

impl Undroppable {
    pub fn set_down_gently(self) -> () {
        core::mem::forget(self)
    }
}

#[cfg(test)]
mod test {
    use crate::Undroppable;

    struct Contains {
        foo: String,
        bar: Undroppable,
    }

    #[test]
    fn test_defuse() {
        let x = Undroppable;
        x.set_down_gently();

        let y = Contains {
            foo: "hi".into(),
            bar: Undroppable,
        };

        let (a, b) = (y.foo, y.bar);
        b.set_down_gently();

        println!("{}", a);
    }

    // This doesn't compile
    // #[test]
    // fn test_doesnt_compile() {
    //     let x = Undroppable;
    // }
}
