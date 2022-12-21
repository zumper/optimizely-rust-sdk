#[macro_export]
macro_rules! user_attributes {
    { $( $key: expr => $value: expr),* $(,)?} => {
        {
            let mut attribute = optimizely::user_context::UserAttributes::new();

            $(
                attribute.insert($key.to_owned(), $value.to_owned());
            )*

            attribute
        }
    };
}

macro_rules! bool_field {
    ($value: ident, $name: expr) => {
        $value[$name]
            .take()
            .as_bool()
            .ok_or(crate::datafile::DatafileError::MissingField(String::from($name)))
    };
}

macro_rules! u32_field {
    ($value: ident, $name: expr) => {
        $value[$name]
            .take()
            .as_u32()
            .ok_or(crate::datafile::DatafileError::MissingField(String::from($name)))
    };
}

macro_rules! string_field {
    ($value: ident, $name: expr) => {
        $value[$name]
            .take_string()
            .ok_or(crate::datafile::DatafileError::MissingField(String::from($name)))
    };
}

macro_rules! list_field {
    ($value: ident, $name: expr, $closure: expr) => {
        $value[$name]
            .take()
            .members_mut()
            .map($closure)
            .collect::<anyhow::Result<Vec<_>>>()
    };
}

macro_rules! list_to_map {
    ($list: ident, $closure: expr) => {
        $list.into_iter().map($closure).collect()
    };
}
