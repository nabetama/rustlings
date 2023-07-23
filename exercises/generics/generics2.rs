// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// 任意の型Tを キー：`value` で保持する構造体
struct Wrapper<T> {
    value: T,
}

// 任意の型Tに対して, Wrapper<T>型のインスタンスを生成する関数
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
#[derive(PartialEq, Debug)]
struct Dummy {
    value: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo".to_string()).value, "Foo");
    }

    #[test]
    fn store_char_in_wrapper() {
        assert_eq!(Wrapper::new('M').value, 'M');
    }

    #[test]
    fn store_bool_in_wrapper() {
        assert_eq!(Wrapper::new(true).value, true);
    }

    #[test]
    fn store_tuple_in_wrapper() {
        assert_eq!(Wrapper::new((1, 2)).value, (1, 2));
    }

    #[test]
    fn store_vec_in_wrapper() {
        assert_eq!(Wrapper::new(vec![1, 2, 3]).value, vec![1, 2, 3]);
    }

    #[test]
    fn store_dummy() {
        assert_eq!(Wrapper::new(Dummy { value: 42 }).value, Dummy { value: 42 });
    }
}
