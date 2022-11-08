#[macro_export]
macro_rules! user_attributes {
    { $( $key: expr => $value: expr),* $(,)?} => {
        {
            let mut attribute = UserAttributes::new();

            $(
                attribute.insert($key.to_owned(), $value.to_owned());
            )*

            attribute
        }
    };
}

macro_rules! string_field {
    ($value: ident, $name: expr) => {
        $value[$name]
            .take_string()
            .ok_or(DatafileError::MissingField(String::from($name)))
    };
}

macro_rules! list_field {
    ($value: ident, $name: expr, $closure: expr) => {
        $value[$name]
            .take()
            .members_mut()
            .map($closure)
            .collect::<Result<Vec<_>, _>>()
    };
}
