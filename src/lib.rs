/// Makes the string argument a String, by using `String::from()` (see [String] for details).
///
/// # Example
/// ```
/// # #[macro_use] extern crate gen_macros; fn main() {
/// let text: String = s!("Hello");
/// # }
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! s {
    ($arg:expr) => {{ String::from($arg) }}
}

/// Generates the code to initialize a [HashMap](std::collections::HashMap).
///
/// # Example
/// ```
/// # #[macro_use] extern crate gen_macros; fn main() {
/// let days = hashmap!(0 => "Monday", 1 => "Tuesday", 2 => "Wednesday");
/// # }
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections;
    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }


    #[test]
    fn test_s() {
        let test_string = s!("string test");
        assert_eq!(type_of(&test_string), "&alloc::string::String");
        assert_eq!(test_string, "string test");
    }

    #[test]
    fn test_hashmap() {
        let test_hashmap = hashmap!{ 10 => "ten".to_string(), 20 => "twenty".to_string() };
        assert_eq!(type_of(&test_hashmap), "&std::collections::hash::map::HashMap<u32, alloc::string::String>");
        let mut d_expected: collections::HashMap<u32, String> = collections::HashMap::new();
        d_expected.insert(10, s!("ten"));
        d_expected.insert(20, s!("twenty"));
        assert_eq!(test_hashmap, d_expected);
    }
}
