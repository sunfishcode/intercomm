use io_extras::grip::{IntoGrip, OwnedGrip};
use ordered_float::NotNan;
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum InterType {
    S8,
    U8,
    S16,
    U16,
    S32,
    U32,
    S64,
    U64,
    F32,
    F64,
    Char,
    String,
    Bool,
    Handle,
    Variant(Vec<(String, InterType)>),
    Record(Vec<(String, InterType)>),
    List(Box<InterType>),
    Tuple(Vec<InterType>),
    Flags(Vec<String>),
    Enum(Vec<String>),
    Option(Box<InterType>),
    Union(Vec<InterType>),
    Result(Box<InterType>, Box<InterType>),
}

#[derive(Debug, Clone)]
pub enum InterVal {
    S8(i8),
    U8(u8),
    S16(i16),
    U16(u16),
    S32(i32),
    U32(u32),
    S64(i64),
    U64(u64),
    F32(Option<ordered_float::NotNan<f32>>),
    F64(Option<ordered_float::NotNan<f64>>),
    Char(char),
    String(Cow<'static, str>),
    Bool(bool),
    Handle(Arc<OwnedGrip>),
    Variant(Vec<(String, InterType)>, String, Box<InterVal>),
    Record(Vec<(String, InterVal)>),
    List(Box<InterType>, Vec<InterVal>),
    Tuple(Vec<InterVal>),
    Flags(Vec<(String, bool)>),
    Enum(Vec<String>, String, Box<InterVal>),
    Option(Box<InterType>, Option<Box<InterVal>>),
    Union(Vec<InterType>, usize, Box<InterVal>),
    Result(
        Box<InterType>,
        Box<InterType>,
        Result<Box<InterVal>, Box<InterVal>>,
    ),
}

impl InterVal {
    pub fn ty(&self) -> InterType {
        match self {
            Self::S8(_) => InterType::S8,
            Self::U8(_) => InterType::U8,
            Self::S16(_) => InterType::S16,
            Self::U16(_) => InterType::U16,
            Self::S32(_) => InterType::S32,
            Self::U32(_) => InterType::U32,
            Self::S64(_) => InterType::S64,
            Self::U64(_) => InterType::U64,
            Self::F32(_) => InterType::F32,
            Self::F64(_) => InterType::F64,
            Self::Char(_) => InterType::Char,
            Self::String(_) => InterType::String,
            Self::Bool(_) => InterType::Bool,
            Self::Handle(_) => InterType::Handle,
            Self::Variant(arms, _, _) => InterType::Variant(arms.to_vec()),
            Self::Record(fields) => InterType::Record(
                fields
                    .iter()
                    .map(|(name, value)| (name.clone(), value.ty()))
                    .collect(),
            ),
            Self::List(ty, _) => InterType::List(ty.clone()),
            Self::Tuple(fields) => {
                InterType::Tuple(fields.iter().map(|value| value.ty()).collect())
            }
            Self::Flags(vals) => {
                InterType::Flags(vals.iter().map(|(name, _)| name.clone()).collect())
            }
            Self::Enum(arms, _, _) => InterType::Enum(arms.to_vec()),
            Self::Option(ty, _) => InterType::Option(ty.clone()),
            Self::Union(ty, _, _) => InterType::Union(ty.clone()),
            Self::Result(ok, err, _) => InterType::Result(ok.clone(), err.clone()),
        }
    }

    pub fn str(s: &'static str) -> Self {
        Self::String(Cow::Borrowed(s))
    }
}

pub trait InterTypeable: Into<InterVal> {
    fn inter_ty() -> InterType;
}

impl InterTypeable for i8 {
    fn inter_ty() -> InterType {
        InterType::S8
    }
}
impl From<i8> for InterVal {
    fn from(val: i8) -> InterVal {
        InterVal::S8(val)
    }
}
impl InterTypeable for u8 {
    fn inter_ty() -> InterType {
        InterType::U8
    }
}
impl From<u8> for InterVal {
    fn from(val: u8) -> InterVal {
        InterVal::U8(val)
    }
}
impl InterTypeable for i16 {
    fn inter_ty() -> InterType {
        InterType::S16
    }
}
impl From<i16> for InterVal {
    fn from(val: i16) -> InterVal {
        InterVal::S16(val)
    }
}
impl InterTypeable for u16 {
    fn inter_ty() -> InterType {
        InterType::U16
    }
}
impl From<u16> for InterVal {
    fn from(val: u16) -> InterVal {
        InterVal::U16(val)
    }
}
impl InterTypeable for i32 {
    fn inter_ty() -> InterType {
        InterType::S32
    }
}
impl From<i32> for InterVal {
    fn from(val: i32) -> InterVal {
        InterVal::S32(val)
    }
}
impl InterTypeable for u32 {
    fn inter_ty() -> InterType {
        InterType::U32
    }
}
impl From<u32> for InterVal {
    fn from(val: u32) -> InterVal {
        InterVal::U32(val)
    }
}
impl InterTypeable for i64 {
    fn inter_ty() -> InterType {
        InterType::S64
    }
}
impl From<i64> for InterVal {
    fn from(val: i64) -> InterVal {
        InterVal::S64(val)
    }
}
impl InterTypeable for u64 {
    fn inter_ty() -> InterType {
        InterType::U64
    }
}
impl From<u64> for InterVal {
    fn from(val: u64) -> InterVal {
        InterVal::U64(val)
    }
}
impl InterTypeable for NotNan<f32> {
    fn inter_ty() -> InterType {
        InterType::F32
    }
}
impl From<NotNan<f32>> for InterVal {
    fn from(val: NotNan<f32>) -> InterVal {
        InterVal::F32(Some(val))
    }
}
impl InterTypeable for NotNan<f64> {
    fn inter_ty() -> InterType {
        InterType::F64
    }
}
impl From<NotNan<f64>> for InterVal {
    fn from(val: NotNan<f64>) -> InterVal {
        InterVal::F64(Some(val))
    }
}
impl InterTypeable for char {
    fn inter_ty() -> InterType {
        InterType::Char
    }
}
impl From<char> for InterVal {
    fn from(val: char) -> InterVal {
        InterVal::Char(val)
    }
}
impl InterTypeable for String {
    fn inter_ty() -> InterType {
        InterType::String
    }
}
impl From<String> for InterVal {
    fn from(val: String) -> InterVal {
        InterVal::String(Cow::Owned(val))
    }
}
impl InterTypeable for &'static str {
    fn inter_ty() -> InterType {
        InterType::String
    }
}
impl From<&'static str> for InterVal {
    fn from(val: &'static str) -> InterVal {
        InterVal::String(Cow::Borrowed(val))
    }
}
impl InterTypeable for bool {
    fn inter_ty() -> InterType {
        InterType::Bool
    }
}
impl From<bool> for InterVal {
    fn from(val: bool) -> InterVal {
        InterVal::Bool(val)
    }
}
impl InterTypeable for std::fs::File {
    fn inter_ty() -> InterType {
        InterType::Handle
    }
}
impl From<std::fs::File> for InterVal {
    fn from(val: std::fs::File) -> InterVal {
        InterVal::Handle(Arc::new(val.into_grip()))
    }
}
impl InterTypeable for std::net::TcpStream {
    fn inter_ty() -> InterType {
        InterType::Handle
    }
}
impl From<std::net::TcpStream> for InterVal {
    fn from(val: std::net::TcpStream) -> InterVal {
        InterVal::Handle(Arc::new(val.into_grip()))
    }
}
impl InterTypeable for Arc<OwnedGrip> {
    fn inter_ty() -> InterType {
        InterType::Handle
    }
}
impl From<Arc<OwnedGrip>> for InterVal {
    fn from(val: Arc<OwnedGrip>) -> InterVal {
        InterVal::Handle(val)
    }
}
impl<T: InterTypeable> InterTypeable for Vec<T> {
    fn inter_ty() -> InterType {
        InterType::List(Box::new(T::inter_ty()))
    }
}
impl<T: InterTypeable> From<Vec<T>> for InterVal {
    fn from(val: Vec<T>) -> InterVal {
        InterVal::List(
            Box::new(T::inter_ty()),
            val.into_iter().map(Into::into).collect(),
        )
    }
}
impl<T: InterTypeable> InterTypeable for Option<T> {
    fn inter_ty() -> InterType {
        InterType::Option(Box::new(T::inter_ty()))
    }
}
impl<T: InterTypeable> From<Option<T>> for InterVal {
    fn from(val: Option<T>) -> InterVal {
        InterVal::Option(Box::new(T::inter_ty()), val.map(|t| Box::new(t.into())))
    }
}
impl<T: InterTypeable, E: InterTypeable> InterTypeable for Result<T, E> {
    fn inter_ty() -> InterType {
        InterType::Result(Box::new(T::inter_ty()), Box::new(E::inter_ty()))
    }
}
impl<T: InterTypeable, E: InterTypeable> From<Result<T, E>> for InterVal {
    fn from(val: Result<T, E>) -> InterVal {
        InterVal::Result(
            Box::new(T::inter_ty()),
            Box::new(E::inter_ty()),
            val.map(|t| Box::new(t.into()))
                .map_err(|t| Box::new(t.into())),
        )
    }
}
