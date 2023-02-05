#[macro_export]
macro_rules! unwrap_result_or_return_err {
    ($subject: ident) => {
        match $subject {
            Ok(item) => item,
            Err(error) => {
                return Err(error);
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_result_or {
    ($subject: ident, $action: block) => {
        match $subject {
            Ok(item) => item,
            Err(_) => $action,
        }
    };
}

#[macro_export]
macro_rules! unwrap_option_or {
    ($subject: ident, $action: block) => {
        match $subject {
            Some(item) => item,
            None => $action,
        }
    };
}
