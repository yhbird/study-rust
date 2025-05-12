# Study_etl_02

API 호출하는 함수를 간단하게 작성하고 테스트하는 프로젝트

## 프로젝트 목표
- API호출하는 함수들을 간단하게 작성하면서 Rust 코드의 문법과 양식을 이해한다.
- API를 호출하여 결과값을 얻어내는 간단한 테스트를 진행한다.
- 에러가 발생 하였을때 예외처리를 알아본다.

## API 호출 예시
https://openapi.nexon.com 에서 제공하는 OPEN API를 통해 간단한 데이터 조회

/maplestory/v1/id 사용하여 메이플스토리 게임 캐릭터의 식별코드(OCID)를 조회한다.

## 스크립트 및 배운내용 요약
### 1. 함수정의
```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
```
- `fn main()` : 프로그램의 진입점
- `Result<(), Box<dyn std::error::Error>>`  
    에러가 발생할 수 있는 함수임을 나타냄  
    `Result<T, E>` : 성공시 T, 실패시 E 반환  
    `()` : 반환값 없음 (`void`처럼 동작)
- `Box<dyn std::error::Error>`  
    다양한 종류의 에러를 다루는 포인터 타입

### 2. `Ok(())`의 의미
python에서 함수의 결과를 `return None` 하듯이, rust에서는 `Ok(())`으로 함수의 결과를 return한다. 반드시 `Ok`라고 적어야 하며, `ok`, `OK` 등의 오타는 문법 오류가 발생한다.
```
error[E0425]: cannot find function `ok` in this scope
   --> src/main.rs:38:5
    |
38  |     ok(())
    |     ^^ help: a tuple variant with a similar name exists (notice the capitalization): `Ok`
```
### 3. HeaderMap의 역할
```rust
let mut headers = HeaderMap::new();
headers.insert("x-nxopen-api-key", test_token.parse()?);
```

- `HeaderMap`은 HTTP 요청의 **헤더 정보(key-value)** 를 저장하는 자료구조

실제로 reqwest의 .headers() 메서드에 넘겨주면 요청에 포함됨

### 4. 메서드형 함수 뒤에 붙은 ? 연산자
```rust
let token = test_token.parse()?;   // 실패 시 즉시 함수 종료
let response = client.send()?;     // 실패 시 즉시 Err 리턴
```
Rust의 에러 전파(Propagate Error) 연산자
- ?는 Result 혹은 Option 타입에만 붙음
- ?내부에는 아래와 같이 동작함
```rust
match some_result {
    Ok(value) => value,
    Err(e) => return Err(e.into()),
}
```
`?` 를 붙이면 에러가 발생할때 즉시 현재 함수를 탈출하고 에러를 반환한다.