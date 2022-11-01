#[macro_export]
macro_rules! attributes {
    { $( $k: expr => $v: expr),* $(,)?} => {
        {
            let mut attribute = crate::Attributes::new();

            $(
                attribute.insert($k.to_string(), $v.to_string());
            )*

            attribute
        }
    };
}
