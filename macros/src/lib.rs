#[macro_export]
macro_rules! include_str_as_fn {
    ($name:ident, $file:expr) => {
        pub fn $name() -> &'static str {
            include_str!($file)
        }
    };
}

#[macro_export]
macro_rules! response_new {
    ($type:ty, $subtype:ty) => {
        impl $type {
            pub fn new(error: Option<AppError>, data: Option<$subtype>) -> Self {
                Self { error, data }
            }
        }
    };
}

#[macro_export]
macro_rules! respone_to_result {
    ($type:ty, $subtype:ty) => {
        impl ToResult for $type {
            type Msg = $subtype;

            fn to_result(&self) -> Result<&Self::Msg, MyError> {
                if let Some(v) = self.data.as_ref() {
                    Ok(v)
                } else {
                    Err(self.error.as_ref().unwrap().into())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! result_to_vec {
    ($type:ty, $subtype:ty) => {
        impl ToVec for Result<$subtype, AppError> {
            fn to_vec(&self) -> Vec<u8> {
                match self {
                    Ok(v) => <$type>::new(None, Some(v.to_owned())).into(),
                    Err(e) => <$type>::new(Some(e.to_owned()), None).into(),
                }
            }
        }
    };
}
