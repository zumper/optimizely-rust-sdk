#[macro_export]
macro_rules! user_attributes {
    { $( $key: expr => $value: expr),* $(,)?} => {
        {
            let mut attribute = optimizely::UserAttributes::new();

            $(
                attribute.insert($key.to_owned(), $value.to_owned());
            )*

            attribute
        }
    };
}

macro_rules! missing_field {
    ($name: expr) => {
        crate::datafile::DatafileError::MissingField(String::from($name))
    }
}

macro_rules! bool_field {
    ($value: ident, $name: expr) => {
        $value[$name]
            .take()
            .as_bool()
            .ok_or(missing_field!($name))
            .into_report()?
    };
}

macro_rules! u64_field {
    ($value: ident, $name: expr) => {
        $value[$name]
            .take()
            .as_u64()
            .ok_or(missing_field!($name))
            .into_report()?
    };
}

macro_rules! string_field {
    ($value: ident, $name: expr) => {
        {
            let owned_value = $value[$name].take();
            owned_value.as_str()
                .ok_or(missing_field!($name))
                .into_report()?
                .to_owned()
        }
    };
}

macro_rules! list_field {
    ($value: ident, $name: expr, $closure: expr) => {
        $value[$name]
            .take()
            .as_array_mut()
            .ok_or(missing_field!($name)).into_report()?
            .into_iter()
            .map($closure)
            .collect::<error_stack::Result<Vec<_>, crate::datafile::DatafileError>>()?
    };
}

macro_rules! list_to_map {
    ($list: ident, $closure: expr) => {
        $list.into_iter().map($closure).collect()
    };
}
