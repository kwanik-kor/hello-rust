# hello-rust

> Repository to learn basic concept of Rust

### Keywords

#### 1. 변수(Variables)
##### 1.1 불변성(Mutability)
- 기본 변수는 **불변성**을 갖는다.
  - 불변성으로 선언한 값을 변경하고자할 경우 Compile Error를 발생시킨다.
  - Compiler가 변경되지 않은 값에 대한 보증을 해주는 것을 원칙으로 하는데, 코드 작성이나 분석 시에 변수의 값이 어떻게 변경되는지 추적할 필요가 없다는 것은 코드를 합리적으로 만들기 때문이다.
```rust
let x = 5;
x = 6; // fist assignment to 'x'
```
- `mut`: 가변성 변수 선언
  - 변수의 값을 변경하는 것을 허용함
  - 해당 변수의 값을 변경할 것이라는 의도를 표현함
  - **대규모 데이터 구조체**의 경우, 가변 인스턴스를 사용하는 것이 새 인스턴스를 할당하고 반환하는 것보다 효율적일 수 있음
```rust
let mut x = 5;
x = 6; // It works
```

##### 1.2 상수(Constant)
- 상수에 대해서는 가변성을 허락할 수 없으며, 모든 단어를 대문자로 사용할 것을 원칙으로 한다.

##### 1.3 Shadowing
- 선언한 변수와 같은 이름의 새 변수를 선언할 수 있는 것
- `shadowing` 시에 `let`를 사용해야 하며, 마지막 선언 이후로는 불변성을 갖음
```rust
let spaces = "    ";
let spaces = spaces.len(); // We don't need 'spaces_str' or 'spaces_len' 
```

##### 1.4 Data Type
> Rust는 타입이 고정된 언어이며, 모든 변수의 타입은 컴파일 시에 반드시 정해져 있어야 한다.

##### 1.4.1 스칼라 타입(Scalar Types)
- **정수형 타입(Integer Types)**
  - `isize`와 `usize`의 경우, 프로그램이 동작하는 컴퓨터 환경에 따라 결정
  - 정수형 리터럴 사용 가능

| length | signed | unsigned |
|:------:|:------:|:--------:|
|  8bit  |   i8   |    u8    |
| 16bit  |  i16   |   u16    |
| 32bit  |  i32   |   u32    |
| 64bit  |  i64   |   u64    |
|  arch  | isize  |  usize   |

- **부동 소수점 타입(Floating-Point Types)**
  - `f32`, `f64`
  - 기본 타입은 `f64`
    - 최신 CPU 상에서는 `f64`가 `f32`와 비슷한 속도를 내지만 더 정밀한 표현이 가능하기 때문
- **Boolean 타입**
  - `bool`
- **문자 타입(Character Type)**
  - `char`

##### 1.4.2 복합 타입(Compound Types)

- **튜플(Tuple)**
  - 서로 다른 타입 혹은 값들을 하나의 타입으로 묶을 수 있음
    - 튜플의 인덱스를 이용해 직접접근할 수 있음
  - **구조해체(Destructing)** : 하나의 튜플을 부분으로 나누는 것
```rust
    let tuple = (500, 6.3, 1); // tuple
    let (x, y, z) = tuple; // destructing
```
    
- **배열(Array)**
  - 배열은 모든 요소가 같은 타입이어야 함.
  - 고정된 길이를 가짐
  - 데이터를 `heap` 보다 `stack`에 할당하는 것을 원할 때 혹은, 고정된 요소를 갖는다고 확신하는 경우에 사용할 것