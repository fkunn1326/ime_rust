
#[macro_export]
macro_rules! check_win32 {
    ($result:ident) => {
        if $result.is_ok() {
            return Ok(())
        } else {
            return Err(windows::core::Error::from($result.to_hresult()))
        }
    };
    ($result:ident, $value:ident) => {
        if $result.is_ok() {
            Ok($value)
        } else {
            Err(windows::core::Error::from($result.to_hresult()))
        }
    };
}

#[macro_export]
macro_rules! check_err {
    ($result:ident) => {
        if $result.is_err() {
            return $result.into()
        }
    };
}