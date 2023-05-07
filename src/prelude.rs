/*
    Result Types
*/

pub type BGCResult<T, const T2: u8> = std::result::Result<T, crate::errors::BGCError<T2>>;
pub type GENResult<T> = BGCResult<T, { crate::errors::BGCErrorType::GEN as u8 }>;
pub type WEBResult = BGCResult<actix_web::HttpResponse, { crate::errors::BGCErrorType::WEB as u8 }>;
pub type APIResult = BGCResult<actix_web::HttpResponse, { crate::errors::BGCErrorType::API as u8 }>;
