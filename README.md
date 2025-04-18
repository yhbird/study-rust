# study-rust
rust공부용

# 목차
1. Rust 입문 계기
2. Ubuntu Linux 20.04에서 Rust설치 (WSL on Windows)
3. 프로젝트 생성하기 (cargo 활용)

## Rust 입문 계기
### 기존 파이썬 ETL 개발 업무를 하면서 느낀점
- 파이썬이 메모리 부하가 심해 생각보다 많은 데이터를 처리하지 못하고 있는 것 같음
- 파이썬이 아닌 대체 프로그래밍 언어를 사용하면 ETL 작업 시간을 단축하고, 더 많은 데이터를 처리할 수 있다는 기대감

### Rust을 알게된 계기
- YouTube에서 Discord 앱의 Rust 활용 사례를 소개하는 영상을 접함
- 전세계 엄청난 메세지량을 처리하고 저장하는데 Rust를 사용한다는 점에서 매력을 느낌
- Rust를 연구하여 파이썬 대신에 빠르고 정확한 ETL 파이프라인 구축을 할 수 있다는 기대감

## Rust 설치 @ Ubuntu Linux 20.04 (WSL on Windows)  
### 왜 Windows에서 WSL를 사용하는가?
- 현재 집에서 사용하는 컴퓨터의 REM용량이 엄청커서 WSL을 통해 가상 리눅스를 설치하여 개발하기 편함
- Rust, Airflow가 설치되어 있지 않은 깡통 서버에 CI/CD 연습을 할 수 있음
- 실제 업무하면서, Windows에선 작동하던 Python 코드가 Linux에 넘어오면 라이브러리, 패키지 호환 이슈들이 발생 했었음

### WSL구축
PS(Powershell)에서 wsl 명령어 실행
```powershell
wsl --install -d Ubuntu-20.04
```

```bash
sudo apt update && sudo apt upgrade -y # 패키지 업데이트
sudo apt install build-essential # Rust 기본 컴파일러 설치
```

### Rust 설치 확인
```bash
curl https://sh.rustup.rs -sSf | sh # Rust 설치
source "$HOME/.cargo/env" # 설치 이후 첫실행
rustc --version
cargo --version
```

### Hello, World를 출력하여 Rust 실행확인
파일명 `main.rs`으로 스크립트 작성
```rust
// 파일명 main.rs 생성
fn main() {
    println!("Hello, world!");
}
```

파일명 `main.rs` 컴파일 후 실행
```bash
rustc main.rs
./main
```

## Cargo를 통해 프로젝트 생성 및 관리하기

cargo를 통해 프로젝트를 관리하고, 빌드까지 진행할 수 있다. Python에서 `pip` + `venv` + `setuptolls` 역할을 한 번에 해주는 도구


```bash
cargo new study_etl_01
cd study_etl_01
```

Rust를 공부하는 이유가 Airflow에서 실행하는 ETL 배치 작업에서 Python 스크립트가 아닌 다른 언어의 스크립트를 사용하면 작업 시간을 줄이고, 더 효율적인 작업을 할 수 있는지 비교해서 대체제 역할을 수행할 수 있는지 알아보기 위함이다.

따라서, 동일한 작업을 수행하는 Python 스크립트와 Rust 스크립트를 비교해서 기존 Python 스크립트를 대체할 수 있는지 알아본다.

Rust 바이너리 빌드 후 Bash 혹은 Python Operator로 실행하는 경우
```python
# BashOperator에서 실행
BashOperator(
    task_id='run_etl_rust',
    bash_command='/path/to/etl_binary --date {{ ds }}',
)
```

Airflow 백엔드를 Rust로 처리하도록 구성하고 Python 호출이 가능함
Cargo에 대한 설명과 자세한 프로젝트 생성 예시는 `study_etl_01` 경로 참고