/// Returns the simple name of the function this macro is called in.
macro_rules! fn_name_short {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let full_name = &name[..name.len() - 3];

        // Trim away wrapping closure name if this is enclosed in one.
        match full_name.rfind("::{{closure}}") {
            Some(pos) => {
                let full_name = &full_name[..pos];

                // Find and cut the rest of the path
                match full_name[..full_name.len()].rfind(':') {
                    Some(pos) => &full_name[pos + 1..],
                    None => full_name,
                }
            }
            None => {
                // Find and cut the rest of the path
                match full_name.rfind(':') {
                    Some(pos) => &full_name[pos + 1..full_name.len()],
                    None => full_name,
                }
            }
        }
    }};
}

pub(crate) use fn_name_short;
