# Axum 애플리케이션 상태(State) 관리 모범 사례

이 문서는 Axum 웹 프레임워크에서 애플리케이션의 상태(`AppState`)를 효과적으로 관리하는 방법에 대한 논의를 정리한 것입니다.

## 질문 1: `.with_state()`는 한 번만 사용 가능한가요?

**A:** 네, 맞습니다. `axum::Router`의 `.with_state()` 메서드는 체인당 한 번만 호출하여 애플리케이션의 공유 상태를 주입합니다. 만약 여러 번 호출하면 마지막에 호출된 값으로 덮어쓰입니다.

따라서 여러 종류의 데이터를 상태로 관리하고 싶다면, **모든 데이터를 포함하는 단일 구조체(struct)**를 정의하고 그 인스턴스를 상태로 전달해야 합니다.

```rust
// 여러 데이터를 포함하는 단일 구조체
struct AppState {
    db_pool: DatabaseConnection,
    config: AppConfig,
}

// ...

let app = Router::new()
    .route("/", get(handler))
    .with_state(Arc::new(AppState { ... })); // Arc로 감싸서 공유
```

---

## 질문 2: AppState에 실행 중 변경 가능한 값을 두어도 괜찮을까요?

애플리케이션 상태를 설계할 때 두 가지 접근 방식을 고려할 수 있습니다.

1.  **설정(Configuration) 중심:** 애플리케이션 시작 시 로드되고 거의 변하지 않는 값들 (DB 접속 정보, 서버 포트 등)만 상태에 저장하는 방식.
2.  **가변 상태(Mutable State) 포함:** 위 설정 값들과 더불어, 런타임 중에 변경될 수 있는 값(예: 현재 접속자 수, 임시 데이터 등)을 함께 관리하는 방식.

여기서 "실행 중 상태를 변경해야 할 때, 인메모리(in-memory) `AppState`에 저장하는 것이 맞을까, 아니면 DB나 Redis 같은 외부 저장소를 사용하는 것이 맞을까?"라는 중요한 질문이 생깁니다.

**A:** **간단한 경우라면 `AppState`에서 직접 상태를 변경하는 것이 가능하며 매우 유용한 패턴입니다.** 단, Axum은 멀티스레드 환경에서 동작하므로, 여러 스레드에서 동시에 데이터를 안전하게 수정하기 위해 동시성 프리미티브(Concurrency Primitives)를 사용해야 합니다.

-   `Arc<Mutex<T>>`: 여러 스레드가 데이터(`T`)를 공유하며, 한 번에 하나의 스레드만 수정할 수 있도록 잠금(lock)을 거는 가장 일반적인 방법입니다.
-   `Arc<RwLock<T>>`: 읽기 작업이 쓰기 작업보다 훨씬 빈번할 때 사용합니다. 여러 스레드가 동시에 읽는 것을 허용하지만, 쓰기는 한 번에 하나만 허용하여 성능을 높일 수 있습니다.

```rust
// 가변 상태를 포함하는 AppState 예시
use std::sync::{Arc, Mutex};

struct AppState {
    auth_token: String, // 불변 설정값
    current_users: Arc<Mutex<u32>>, // 가변 상태
    data: Arc<Mutex<Vec<i32>>>,    // 가변 상태
}
```

---

## 해결책: 설정(Config)과 상태(State)의 분리 및 `FromRef` 활용

가장 깔끔하고 확장성 있는 구조는 **불변 설정**과 **가변 상태**를 명확히 분리하고, Axum의 `FromRef` 트레이트를 활용하는 것입니다.

### 추천 구조

1.  **`AppConfig` (불변 설정):** 애플리케이션 시작 시 한 번만 로드되는 설정 값들을 모아둡니다.
2.  **`RuntimeState` (가변 상태):** 실행 중 변경될 수 있는 값들을 `Arc<Mutex<T>>` 등으로 감싸서 관리합니다.
3.  **`AppState` (통합):** 위 두 구조체를 필드로 가지며, `#[derive(FromRef)]`를 추가합니다.

`FromRef`를 사용하면 핸들러에서 전체 `AppState`가 아닌, 자신이 필요로 하는 부분 상태(sub-state)만 `State<T>`로 주입받을 수 있어 코드의 의존성이 명확해집니다.

### 구현 예시

**`Cargo.toml`**
```toml
# axum의 macros 피처를 활성화해야 FromRef를 쓸 수 있습니다.
axum = { version = "0.7.4", features = ["macros"] }
```

**`main.rs`**
```rust
use axum::{
    extract::{FromRef, State},
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};

// 1. 읽기 전용 설정을 위한 구조체
#[derive(Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub admin_api_key: String,
}

// 2. 변경 가능한 런타임 상태를 위한 구조체
#[derive(Clone)]
pub struct RuntimeState {
    pub current_users: Arc<Mutex<u32>>,
}

// 3. 위 둘을 모두 포함하고, FromRef를 derive하는 전체 AppState
#[derive(Clone, FromRef)]
pub struct AppState {
    // 필드의 타입은 고유해야 합니다.
    pub config: AppConfig,
    pub runtime: RuntimeState,
}

// 이 핸들러는 AppConfig만 필요로 합니다.
async fn show_config(State(config): State<AppConfig>) {
    // AppState 전체가 아닌 AppConfig만 받아서 사용
    println!("Admin API Key: {}", config.admin_api_key);
}

// 이 핸들러는 RuntimeState만 필요로 하고, 값을 변경합니다.
async fn increment_users(State(runtime): State<RuntimeState>) {
    // AppState 전체가 아닌 RuntimeState만 받아서 사용
    let mut users = runtime.current_users.lock().unwrap();
    *users += 1;
    println!("Current users: {}", *users);
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        config: AppConfig {
            server_host: "127.0.0.1".to_string(),
            admin_api_key: "super-secret".to_string(),
        },
        runtime: RuntimeState {
            current_users: Arc::new(Mutex::new(0)),
        },
    };

    let app = Router::new()
        .route("/config", get(show_config))
        .route("/increment", get(increment_users))
        .with_state(app_state);

    // ... 서버 실행
}
```

### 이 구조의 핵심 요약

1.  `config` 필드는 초기에 한 번 설정하면 **불변(immutable)**으로 유지합니다.
2.  `runtime` 필드는 실행 중 **변경 가능한(mutable)** 값들을 관리합니다.
3.  `FromRef`는 `AppState` 레벨에서 각 필드 타입의 중복을 허용하지 않지만, 하위 구조체인 `AppConfig`나 `RuntimeState` 내부에서는 타입이 같은 필드를 여러 개 가질 수 있습니다. 이는 `FromRef`가 하위 구조체에는 적용되지 않기 때문입니다.

---

## 결론: 언제 인메모리 상태를 쓰고, 언제 DB/Redis를 쓸까?

| 구분 | 인메모리 `AppState`가 적합한 경우 | DB/Redis 사용이 더 나은 경우 |
| :--- | :--- | :--- |
| **데이터 종류** | 휘발성 데이터 (서버 재시작 시 사라져도 무방) | 영속성이 필요한 모든 비즈니스 데이터 |
| **사용 예시** | 현재 접속자 수, 간단한 카운터, 임시 캐시 | 사용자 정보, 게시글, 주문 내역 등 |
| **환경** | 단일 서버 인스턴스 환경 | 여러 서버로 확장(Scale-out)하는 분산 환경 |
| **요구사항** | 네트워크 지연 없는 매우 빠른 응답 속도 | 복잡한 쿼리, 데이터 트랜잭션, 대용량 데이터 처리 |

결론적으로, 간단한 휘발성 데이터를 다루기 위해 `AppState` 내에서 `Arc<Mutex<T>>`를 사용하는 것은 매우 실용적이고 효율적인 Axum 패턴입니다. 데이터의 영속성이 필요하거나 여러 서버 인스턴스가 상태를 공유해야 할 경우에는 DB나 Redis를 사용하는 것이 올바른 선택입니다. 