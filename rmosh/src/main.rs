pub mod scheme {
    #[derive(Debug, PartialEq)]
    pub enum ObjectType {
        Symbol,
    }

    // We use least significant bits as object tag.
    #[repr(C, align(8))]
    pub struct Object {
        obj_type: ObjectType,
        ptr: *const u8,
    }

    impl Object {
        const NUM_TAG_BITS: isize = 3;
        const TAG_MASK: isize = 7;
        const TAG_HEAP_OBJ: isize = 0;
        const TAG_FIXNUM: isize = 1;
        const TAG_PAIR: isize = 1 << 1;
        const TAG_CLEAR: isize = !Self::TAG_MASK;

        // Note: Object reference has static lifetime
        // because it lives for the entire lifetime of the running program.
        pub fn new_fixnum(n: isize) -> &'static Object {
            let obj = n << Self::NUM_TAG_BITS;
            let obj = obj | Self::TAG_FIXNUM;
            // Cast as raw pointer.
            let ptr = obj as *const Object;
            // Pointer to reference.
            unsafe { &*ptr }
        }

        pub fn new_pair(first: &Object, second: &Object) -> &'static Object {
            let obj = Box::new(Pair {
                first: first,
                second: second,
            });
            let ptr = Box::into_raw(obj) as isize;
            let ptr = ptr | Self::TAG_PAIR;
            let ptr = ptr as *const Object;
            unsafe { &*ptr }
        }

        pub fn new_symbol(value: &str) -> &'static Object {
            let symbol = Box::new(Symbol {
                value: value.to_string(),
            });
            let ptr = Box::into_raw(symbol) as *const u8;
            let obj = Box::new(Object {
                obj_type: ObjectType::Symbol,
                ptr: ptr,
            });
            let ptr = Box::into_raw(obj) as *const Object;
            unsafe { &*ptr }
        }

        pub fn is_pair(&self) -> bool {
            self.tag() == Self::TAG_PAIR
        }

        pub fn is_symbol(&self) -> bool {
            if self.tag() != Self::TAG_HEAP_OBJ {
                return false;
            }
            self.obj_type == ObjectType::Symbol
        }

        pub fn is_fixnum(&self) -> bool {
            self.tag() == Self::TAG_FIXNUM
        }

        pub fn to_symbol(&self) -> &Symbol {
            let ptr = self.ptr as *const Symbol;
            unsafe { &*ptr }
        }

        pub fn to_pair(&self) -> &Pair {
            let ptr = self as *const Object;
            let ptr = ptr as isize;
            let ptr = ptr & Self::TAG_CLEAR;
            let ptr = ptr as *const Pair;
            unsafe { &*ptr }
        }

        pub fn to_fixnum(&self) -> isize {
            let ptr = self as *const Object;
            ptr as isize >> Self::NUM_TAG_BITS
        }

        fn tag(&self) -> isize {
            let ptr = self as *const Object as isize;
            ptr & Self::TAG_MASK
        }
    }

    pub struct Pair<'a> {
        pub first: &'a Object,
        pub second: &'a Object,
    }

    #[repr(C, align(8))]
    pub struct Symbol {
        pub value: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixnum() {
        let obj = scheme::Object::new_fixnum(123456);
        assert!(obj.is_fixnum());
        assert_eq!(obj.to_fixnum(), 123456);
    }

    #[test]
    fn test_not_fixnum() {
        let obj = scheme::Object::new_symbol("foo");
        assert!(!obj.is_fixnum());
    }

    #[test]
    fn test_pair() {
        let first = scheme::Object::new_fixnum(1234);
        let second = scheme::Object::new_fixnum(5678);
        let obj = scheme::Object::new_pair(first, second);
        assert!(obj.is_pair());
        let pair = obj.to_pair();
        assert!(pair.first.is_fixnum());
        assert!(pair.second.is_fixnum());
        assert_eq!(pair.first.to_fixnum(), 1234);
        assert_eq!(pair.second.to_fixnum(), 5678);
    }

    #[test]
    fn test_symbol() {
        let obj = scheme::Object::new_symbol("foo");
        assert!(obj.is_symbol());
        let symbol = obj.to_symbol();
        assert_eq!(symbol.value, String::from("foo"));
    }
}

fn main() {
    let address = 0xdeadbeefusize;
    let _raw_pointer = address as *const i32;
    /*
     * Of course this causes Segmentation fault.
    unsafe {
        println!("pointer={}", *_raw_pointer);
    }
    */
}
