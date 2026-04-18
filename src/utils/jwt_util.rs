use crate::common::error::AppError;
use crate::common::error::AppError::JwtTokenError;
use jsonwebtoken::{decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use nameof::name_of;

pub const JWT_EXPIRATION_SECONDS: u64 = 86400; // 24小时过期时间，单位为秒
pub const JWT_SECRET: &str = "123";
pub const JWT_ISSUER: &str = "koobe";
pub const JWT_AUDIENCE: &str = "rust_admin";
pub const JWT_SUBJECT: &str = "rust_admin";
pub const JWT_JTI: &str = "ignore";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtToken {
    pub id: i64,
    pub username: String,
    aud: String,
    // (audience)：受众
    exp: usize,
    iat: usize,
    // (Issued At)：签发时间
    iss: String,
    // (issuer)：签发人
    nbf: usize,
    // (Not Before)：生效时间
    sub: String,
    // (subject)：主题
    jti: String, // (JWT ID)：编号
}

impl JwtToken {
    pub fn new(id: i64, username: &str) -> JwtToken {
        let now = SystemTime::now();
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");
        let exp = Duration::from_secs(JWT_EXPIRATION_SECONDS);//过期时间

        JwtToken {
            id,
            username: String::from(username),
            aud: String::from(JWT_AUDIENCE), // (audience)：受众
            exp: (now + exp).as_secs() as usize,
            iat: now.as_secs() as usize,     // (Issued At)：签发时间
            nbf: now.as_secs() as usize,     // (Not Before)：生效时间
            iss: String::from(JWT_ISSUER),   // (issuer)：签发人
            sub: String::from(JWT_SUBJECT),  // (subject)：主题
            jti: String::from(JWT_JTI),      // (JWT ID)：编号
        }
    }

    pub fn get_exp(&self) -> usize {
        self.exp
    }

    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, AppError> {
        match encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_ref())) {
            Ok(t) => Ok(t),
            Err(_) => Err(JwtTokenError("create token error".to_string())),
        }
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JwtToken, AppError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 60;
        validation.sub = Some(JWT_SUBJECT.to_string());
        validation.set_audience(&[JWT_AUDIENCE]);
        validation.set_required_spec_claims(&[name_of!(exp in JwtToken),name_of!(sub in JwtToken),name_of!(aud in JwtToken)]);
        match decode::<JwtToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(JwtTokenError("InvalidToken".to_string())),
                ErrorKind::InvalidIssuer => Err(JwtTokenError("InvalidIssuer".to_string())),
                ErrorKind::ExpiredSignature => Err(JwtTokenError("TokenExpired".to_string())),
                _ => Err(JwtTokenError("InvalidToken Errors".to_string())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::jwt_util;
    use crate::utils::jwt_util::JwtToken;

    #[test]
    fn test_jwt() {
        let jwt = JwtToken::new(1, "koobe");
        let res = jwt.create_token(jwt_util::JWT_SECRET).unwrap_or_default();
        println!("{:?}", res);
        let token = JwtToken::verify(jwt_util::JWT_SECRET, &res);
        println!("{:?}", token)
    }
}
