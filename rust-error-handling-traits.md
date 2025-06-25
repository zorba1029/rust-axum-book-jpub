# Rust 에러 처리와 변환 트레이트 완벽 가이드

## 목차
1. [IntoResponse 트레이트 (Axum)](#1-intoresponse-트레이트-axum)
2. [From과 Into 트레이트](#2-from과-into-트레이트)
3. [TryFrom과 TryInto 트레이트](#3-tryfrom과-tryinto-트레이트)
4. [에러 처리 크레이트](#4-에러-처리-크레이트)
5. [실전 예제](#5-실전-예제)

---

## 1. IntoResponse 트레이트 (Axum)

### IntoResponse란?

`IntoResponse`는 Axum 웹 프레임워크에서 핸들러가 반환하는 값을 HTTP 응답으로 변환하는 트레이트입니다.

```rust
// IntoResponse 트레이트 정의 (간략화)
pub trait IntoResponse {
    fn into_response(self) -> Response;
}
```

### 기본 예제

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// 사용자 정의 에러 타입
#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

// IntoResponse 구현
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // StatusCode와 JSON 메시지를 HTTP 응답으로 변환
        (self.code, Json(self.message)).into_response()
    }
}

// 핸들러에서 사용
async fn get_user(id: u32) -> Result<Json<User>, AppError> {
    if id == 0 {
        return Err(AppError {
            code: StatusCode::NOT_FOUND,
            message: "User not found".to_string(),
        });
    }
    // ... user 조회 로직
}
```

### IntoResponse의 장점

1. **자동 변환**: 핸들러가 반환하는 값이 자동으로 HTTP 응답으로 변환됨
2. **타입 안정성**: 컴파일 타임에 반환 타입 검증
3. **일관성**: 모든 에러가 동일한 형식으로 응답

---

## 2. From과 Into 트레이트

### From 트레이트

`From`은 타입 A를 타입 B로 변환하는 트레이트입니다. **실패하지 않는 변환**에 사용됩니다.

```rust
// From 트레이트 정의
pub trait From<T> {
    fn from(value: T) -> Self;
}
```

### Into 트레이트

`Into`는 `From`의 반대 방향 변환입니다. **`From`을 구현하면 `Into`는 자동으로 구현됩니다.**

```rust
// Into 트레이트 정의
pub trait Into<T> {
    fn into(self) -> T;
}
```

### From/Into 예제

```rust
use std::fmt;

// 커스텀 에러 타입들
#[derive(Debug)]
struct DatabaseError {
    message: String,
}

#[derive(Debug)]
struct ApiError {
    message: String,
    code: u16,
}

// DatabaseError → ApiError 변환
impl From<DatabaseError> for ApiError {
    fn from(db_err: DatabaseError) -> Self {
        ApiError {
            message: format!("Database error: {}", db_err.message),
            code: 500,
        }
    }
}

// 표준 라이브러리 에러 변환
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError {
            message: format!("IO error: {}", err),
            code: 500,
        }
    }
}

// 사용 예제
fn main() {
    // From 사용
    let db_err = DatabaseError { message: "Connection failed".to_string() };
    let api_err = ApiError::from(db_err);
    
    // Into 사용 (자동으로 사용 가능)
    let db_err2 = DatabaseError { message: "Query failed".to_string() };
    let api_err2: ApiError = db_err2.into();
    
    // ? 연산자와 함께 사용
    fn process_data() -> Result<String, ApiError> {
        let data = read_from_db()?;  // DatabaseError가 자동으로 ApiError로 변환됨
        Ok(data)
    }
}

fn read_from_db() -> Result<String, DatabaseError> {
    Err(DatabaseError { message: "DB connection lost".to_string() })
}
```

### From/Into의 주요 용도

1. **에러 타입 변환**: 다양한 에러를 통합 에러 타입으로 변환
2. **? 연산자 지원**: `From`을 구현하면 `?`로 자동 변환
3. **API 설계**: 유연한 입력 타입 지원 (`impl Into<String>` 등)

---

## 3. TryFrom과 TryInto 트레이트

### TryFrom 트레이트

`TryFrom`은 **실패할 수 있는 변환**을 위한 트레이트입니다.

```rust
// TryFrom 트레이트 정의
pub trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```

### TryInto 트레이트

`TryInto`는 `TryFrom`의 반대 방향입니다. `TryFrom`을 구현하면 자동으로 구현됩니다.

```rust
// TryInto 트레이트 정의
pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

### TryFrom/TryInto 예제

```rust
use std::convert::TryFrom;

// 나이를 표현하는 타입
#[derive(Debug)]
struct Age(u8);

// 나이 생성 시 유효성 검증
impl TryFrom<i32> for Age {
    type Error = String;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err("Age cannot be negative".to_string())
        } else if value > 150 {
            Err("Age cannot be greater than 150".to_string())
        } else {
            Ok(Age(value as u8))
        }
    }
}

// HTTP 상태 코드 변환
#[derive(Debug)]
enum HttpStatus {
    Ok,
    NotFound,
    ServerError,
}

impl TryFrom<u16> for HttpStatus {
    type Error = String;
    
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match code {
            200 => Ok(HttpStatus::Ok),
            404 => Ok(HttpStatus::NotFound),
            500 => Ok(HttpStatus::ServerError),
            _ => Err(format!("Unknown status code: {}", code)),
        }
    }
}

// 사용 예제
fn main() {
    // TryFrom 사용
    let age = Age::try_from(25);
    match age {
        Ok(a) => println!("Valid age: {:?}", a),
        Err(e) => println!("Invalid age: {}", e),
    }
    
    // TryInto 사용
    let status_code: u16 = 404;
    let status: Result<HttpStatus, _> = status_code.try_into();
    
    // ? 연산자와 함께 사용
    fn process_age(input: i32) -> Result<String, String> {
        let age = Age::try_from(input)?;
        Ok(format!("Age is {:?}", age))
    }
}
```

---

## 4. 에러 처리 크레이트

### anyhow 크레이트

`anyhow`는 **간단한 에러 처리**를 위한 크레이트입니다. 에러 타입을 신경쓰지 않아도 될 때 유용합니다.

```rust
use anyhow::{Result, Context, anyhow};

// anyhow::Result는 Result<T, anyhow::Error>의 별칭
fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;  // 에러에 컨텍스트 추가
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    if config.port == 0 {
        return Err(anyhow!("Invalid port number"));  // 커스텀 에러 생성
    }
    
    Ok(config)
}

// 다양한 에러 타입을 하나로 통합
fn process_data() -> Result<()> {
    let _file = std::fs::File::open("data.txt")?;  // io::Error
    let _num: i32 = "42".parse()?;  // ParseIntError
    let _config = read_config()?;  // anyhow::Error
    
    Ok(())
}

// main 함수에서 사용
fn main() -> Result<()> {
    process_data().context("Failed to process data")?;
    Ok(())
}
```

### thiserror 크레이트

`thiserror`는 **커스텀 에러 타입 정의**를 쉽게 만들어주는 크레이트입니다.

```rust
use thiserror::Error;

// 커스텀 에러 정의
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized")]
    Unauthorized,
}

// 사용 예제
async fn get_user(id: u32) -> Result<User, AppError> {
    if id == 0 {
        return Err(AppError::Validation {
            message: "Invalid user ID".to_string(),
        });
    }
    
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&pool)
        .await?;  // sqlx::Error가 자동으로 AppError::Database로 변환
    
    Ok(user)
}

// Axum과 함께 사용
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
            AppError::Validation { .. } => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };
        
        (status, message).into_response()
    }
}
```

### anyhow vs thiserror

| 특성 | anyhow | thiserror |
|------|--------|-----------|
| 사용 목적 | 애플리케이션 에러 | 라이브러리 에러 |
| 에러 타입 | 단일 타입 (`anyhow::Error`) | 커스텀 enum |
| 타입 정보 | 런타임에 확인 | 컴파일 타임에 확인 |
| 적합한 경우 | main 함수, 프로토타입 | API, 라이브러리 |

---

## 5. 실전 예제

### 완전한 Axum 애플리케이션 예제

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;

// 도메인 모델
#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// 요청 DTO
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// 커스텀 에러 (thiserror 사용)
#[derive(Error, Debug)]
enum ApiError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Email already exists")]
    EmailExists,
}

// IntoResponse 구현
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            ApiError::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string().as_str()),
            ApiError::EmailExists => (StatusCode::CONFLICT, "Email already exists"),
        };
        
        let body = Json(serde_json::json!({
            "error": error_message,
        }));
        
        (status, body).into_response()
    }
}

// 유효성 검증 (TryFrom 사용)
impl TryFrom<CreateUser> for User {
    type Error = ApiError;
    
    fn try_from(input: CreateUser) -> Result<Self, Self::Error> {
        // 이메일 유효성 검증
        if !input.email.contains('@') {
            return Err(ApiError::InvalidInput("Invalid email format".to_string()));
        }
        
        // 이름 길이 검증
        if input.name.len() < 2 {
            return Err(ApiError::InvalidInput("Name too short".to_string()));
        }
        
        Ok(User {
            id: 0,  // DB에서 생성됨
            name: input.name,
            email: input.email,
        })
    }
}

// 핸들러 함수들
async fn create_user(
    State(pool): State<PgPool>,
    Json(input): Json<CreateUser>,
) -> Result<Json<User>, ApiError> {
    // TryFrom으로 유효성 검증
    let mut user = User::try_from(input)?;
    
    // 이메일 중복 확인
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        user.email
    )
    .fetch_one(&pool)
    .await?
    .unwrap_or(false);
    
    if exists {
        return Err(ApiError::EmailExists);
    }
    
    // 사용자 생성
    let row = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        user.name,
        user.email
    )
    .fetch_one(&pool)
    .await?;
    
    user.id = row.id;
    Ok(Json(user))
}

async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(ApiError::UserNotFound)?;
    
    Ok(Json(user))
}

// anyhow를 사용한 설정 로딩
async fn load_config() -> anyhow::Result<Config> {
    use anyhow::Context;
    
    let config_path = std::env::var("CONFIG_PATH")
        .context("CONFIG_PATH not set")?;
    
    let content = tokio::fs::read_to_string(&config_path)
        .await
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}

// 메인 함수
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 설정 로드 (anyhow 사용)
    let config = load_config().await?;
    
    // DB 연결
    let pool = PgPool::connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;
    
    // 라우터 구성
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(pool);
    
    // 서버 시작
    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .context("Failed to bind address")?;
    
    println!("Server running on {}", config.server_addr);
    
    axum::serve(listener, app)
        .await
        .context("Server error")?;
    
    Ok(())
}
```

---

## 정리

### 변환 트레이트 사용 지침

1. **From/Into**: 실패하지 않는 변환
   - 에러 타입 변환
   - API 인터페이스 유연성

2. **TryFrom/TryInto**: 실패할 수 있는 변환
   - 유효성 검증
   - 파싱/변환 작업

3. **IntoResponse**: Axum 전용
   - HTTP 응답 변환
   - 에러 응답 포맷팅

### 에러 처리 크레이트 선택

1. **anyhow 사용**: 
   - 애플리케이션 메인 로직
   - 빠른 프로토타이핑
   - 에러 타입이 중요하지 않을 때

2. **thiserror 사용**:
   - 라이브러리 개발
   - 구조화된 에러 필요
   - API 에러 응답

### 모범 사례

1. **라이브러리**: `thiserror`로 구조화된 에러 정의
2. **애플리케이션**: `anyhow`로 간단한 에러 처리
3. **웹 API**: `thiserror` + `IntoResponse`로 일관된 에러 응답
4. **변환**: 적절한 트레이트 선택 (From vs TryFrom)

이러한 도구들을 적절히 조합하면 견고하고 유지보수하기 쉬운 Rust 애플리케이션을 만들 수 있습니다. 