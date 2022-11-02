#[macro_export]
macro_rules! user_attributes {
    { $( $k: expr => $v: expr),* $(,)?} => {
        {
            let mut attribute = crate::UserAttributes::new();

            $(
                attribute.insert($k.to_string(), $v.to_string());
            )*

            attribute
        }
    };
}
