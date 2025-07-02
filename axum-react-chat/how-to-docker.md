# 로컬 환경에서 Docker를 사용하여 개발 환경 설정하기

이 문서는 `axum-react-chat` 프로젝트를 로컬 컴퓨터의 Docker 환경에서 설정하고 실행하는 방법을 안내합니다.

## 1. 필요한 파일

프로젝트를 Docker로 실행하려면, 프로젝트 루트 디렉터리에 `Dockerfile`과 `docker-compose.yml` 두 개의 파일이 필요합니다.

### `Dockerfile`

이 파일은 우리 애플리케이션의 Docker 이미지를 어떻게 만들지에 대한 조립 설명서입니다. 프론트엔드 빌드, 백엔드 빌드, 그리고 최종 실행 이미지를 만드는 3단계로 구성됩니다.

```dockerfile
# 1. 프론트엔드 빌드 스테이지
FROM node:20-alpine AS frontend
WORKDIR /app
COPY frontend .
RUN yarn install
RUN yarn run vite build --outDir dist

# 2. 러스트 빌드 스테이지
FROM rust:latest AS backend
WORKDIR /app
COPY backend .
RUN cargo build --release --bin docker

# 3. 최종 프로덕션 스테이지
FROM debian:bookworm-slim
WORKDIR /app

# SSL 라이브러리 설치 (DB 접속에 필요)
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# 빌드된 프론트엔드/백엔드 파일 복사
COPY --from=frontend /app/dist static
COPY --from=backend /app/target/release/docker app

# 포트 노출
EXPOSE 3000

# 애플리케이션 실행
CMD ["./app"]
```

### `docker-compose.yml`

이 파일은 우리 앱 컨테이너와 데이터베이스(PostgreSQL) 컨테이너를 함께 묶어 실행하고 관리하기 위한 설계도입니다.

```yaml
version: '3.8'

services:
  db:
    image: postgres:15-alpine
    container_name: chat_db
    restart: always
    environment:
      - POSTGRES_USER=axum
      - POSTGRES_PASSWORD=1234
      - POSTGRES_DB=axum_react_chat
    ports:
      # 로컬 PC의 DB 도구에서 접속할 수 있도록 포트를 외부로 노출합니다.
      - "5432:5432"
    volumes:
      # 컨테이너가 삭제되어도 데이터가 유지되도록 볼륨을 설정합니다.
      - postgres_data:/var/lib/postgresql/data

  app:
    build: .
    container_name: axum_chat_app
    restart: always
    depends_on:
      - db
    ports:
      - "3000:3000"
    environment:
      # Docker Compose 네트워크 내에서 'db'라는 서비스 이름으로 DB에 접속합니다.
      - DATABASE_URL=postgres://axum:1234@db:5432/axum_react_chat

volumes:
  postgres_data:
```

## 2. 실행 및 관리

터미널에서 프로젝트 루트 디렉터리로 이동한 후, 다음 명령어들을 사용합니다.

#### 모든 서비스 시작 (빌드 + 실행 + 마이그레이션)

아래 명령어 하나만 실행하면 이미지를 빌드하고, DB와 앱 컨테이너를 실행하며, 앱이 시작될 때 자동으로 데이터베이스 마이그레이션(테이블 생성)까지 수행합니다.

```bash
docker-compose up --build
```

백그라운드에서 실행하려면 `-d` 플래그를 추가합니다.

```bash
docker-compose up --build -d
```

#### 모든 서비스 중지 및 제거

```bash
docker-compose down
```

## 3. 데이터베이스 접속

로컬 PC에 설치된 DBeaver, DataGrip 같은 DB 도구에서 실행 중인 데이터베이스에 접속할 수 있습니다.

*   **Host (호스트):** `localhost`
*   **Port (포트):** `5432`
*   **Database (데이터베이스):** `axum_react_chat`
*   **User / Role (사용자):** `axum`
*   **Password (비밀번호):** `1234`
*   **JDBC URL:** `jdbc:postgresql://localhost:5432/axum_react_chat`

**주의:** DB 도구로 접속하려면 `docker-compose up` 명령으로 컨테이너가 실행 중인 상태여야 합니다.

## 4. 문제 해결

#### DB 접속이 안 되거나 테이블이 없는 문제가 발생할 경우

이전의 잘못된 설정으로 생성된 데이터가 Docker 볼륨에 남아있는 것이 원인일 수 있습니다. 다음 순서대로 모든 것을 초기화하고 깨끗한 상태에서 다시 시작하세요.

```bash
# 1. 현재 실행 중인 모든 컨테이너를 중지하고 제거합니다.
docker-compose down

# 2. 데이터가 저장된 볼륨을 완전히 삭제합니다.
docker volume rm axum-react-chat_postgres_data

# 3. 모든 것을 새로 빌드하고 시작합니다.
docker-compose up --build
``` 