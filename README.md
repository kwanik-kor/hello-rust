# hello-rust

> Repository to learn basic concept of Rust

## Keywords

### 1. 변수(Variables)
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

##### 1.5 함수(Function)
> Rust의 함수는 `snake_case`를 사용한다.

##### 1.5.1 함수 매개변수(Parameters)

- **전달인자(arguments)**
  - 함수로 전달되는 상수
- **매개변수(parameters)**
  - 함수는 고유한 부분인 특별한 변수 매개변수를 가짐
- 함수 선언부에는 반드시 각 매개변수의 타입을 정의해야 함

##### 1.5.2 구문과 표현식(Statements and Expressions)

- **구문(Statements)**
  - 어떤 명령들의 나열로 값을 반환하지 않는 어떤 동작을 수행
- **표현식(Expressions)**
  - 표현식은 결과값을 산출해냄
- 구문을 사용해서는 다른 변수에 값을 대입할 수 없음

```rust
fn main() {
  let x = 5;
  let y = {
    let x = 3;
    x + 1 // 표현식의 종결부에는 세미콜론을 사용하지 않는다.
  };
}
```

##### 1.5.3 반환 값을 갖는 함수
- `return`을 이용해서 반환할 수 잇으나, 암묵적으로 마지막 표현식을 반환함
- 정의된 유형의 값을 함수가 반환하지 않는 경우 `()` 비어있는 튜플을 반환함

---
### 2. 제어문(Control Flow)

##### 2.1 if 표현식
- `if` 식의 조건과 관련된 코드블럭을 갈래(arm)라고 함
- 다른 Language와 동일하지만 조건 문에 괄호를 필요로 하지 않는다
- 코드의 조건은 반드시 `bool` 값이어야 한다.

##### 2.1.1 let 구문에서 if 사용하기
> 코드 블럭은 마지막에 위치한 표현식을 산출하며 숫자는 그 자체로 표현식이다.
- `if`가 표현식이므로, `let` 구문의 우측에 사용할 수 있음
```rust
let condition = true;
let number = if condition {
    5
} else {
    6
};
```

##### 2.2 반복문과 반복(Repetition with Loops)
##### 2.2.1 `loop`

- `loop`은 그만두라고 명시하여 알려주기 전까지 코드블럭을 반복해서 수행한다
```rust
loop {
    println!("again!");
}
```

##### 2.2.2 `while`
```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number = number - 1;
}
```

##### 2.2.3 `for`
```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```

---
### 3. 구조체(Struct)
> 튜플(Tuple)과 유사한 형태이지만, Java Class의 필드와 마찬가지로, 각 구성요소들을 명명할 수 있으므로
> 값이 의미하는 바를 명확하게 할 수 있다.

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
```

##### 3.1 Rust 구조체 기본
- 구조체를 사용하기 위해서는 인스턴스(Instance)를 생성해야 함
  - 구조체에 정의한 필드 순서와 정의하는 순서가 일치하지 않아도 됨
  - `instance.field` 형태로 참조할 수 있음
  - 변경 가능한 `instance`를 생성하기 위해서는 `mut` 키워드 넣기
  - 특정 필드만 변경할 수 있도록 Rust는 허용하지 않음 
```rust
let user = User {
    username: String::from("rhksdlr134"),
    email: String::from("rhksdlr134@naver.com"),
    active: true,
    sign_in_count: 1
}
```

##### 3.2 필드 초기화 축약법(field init shortcut)
- 변수명과 구조체의 필드명이 같다면, 필드명을 번거롭게 다시 작성하지 않아도 됨

```rust
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```

##### 3.3 구조체 갱신법(Struct Update Syntax)
- 존재하는 인스턴스에서 기존 값의 대부분은 재사용하고, 몇몇 값만 바꿔 새 인스턴스를 생성하는 법

```rust
fn update_user(user: User, email: String, username: String) -> User {
  User {
    email,
    username,
    ..user
  }
}
```

##### 3.4 튜플 구조체(Tuple Struct)
- 구조체는 이름을 가지지만, 구조체의 필드는 튜플과 같이 이름을 갖지 않는 형태
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

##### 3.5 필드가 없는 유사 유닛 구조체(Unit-Like Structs without any fields)
- 유닛 타입인 `()`와 비슷하게 동작하여, `유사 유닛 구조체(unit-like sturct)`로 불림
- 특정한 타입의 트레잇(Trait)으로 구현해야 

##### 3.6 구조체의 소유권(Ownership)
- `&str` 이 아닌 `String`을 사용하는 이유는 구조체 전체가 유효한 동안 구조체가 그 데이터를 소유하게 하고자 함
- `라이프타임(lifetime)`을 사용해야만 구조체가 소유권이 없는 데이터의 참조를 저장할 수 있게됨

##### 3.7 메소드 문법(Method)
- 메소드는 함수와는 달리 구조체의 내용 안에 정의되며, 첫번재 파라미터가 항상 `self`이며, 이는 메소드가 호출되는 구조체의 인스턴스를 나타냄
  - struct 내부 혹은 열거형이나, 트레잇 객체 안에 정의됨
```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}
```

 - `&self`를 사용하는 것은, 소유권을 가져오는 것을 원하지 않으며, 구조체 내의 데이터를 읽기만 하기를 원하기 때문임
   - `&mut self`를 쓰는 경우는, 변형 이후에 호출하는 측에서 원본 인스턴스를 사용하는 것을 막고 싶을 때 사용됨
   - 해당 구조체가 사용되는 지점을 모두 찾기 보다, 하나의 `impl` 블록 내에서 해당 타입의 인스턴스로 할 수 있는 것을 모아두는 것
 - **연관 함수(Associated Functions)**
   - `self` 파라미터를 갖지 않는 함수를 정의하는 것으로, 해당 함수가 해당 구조체와 연관되어 있기 때문이다.
   - 함께 동작할 구조체의 인스턴스를 가지고 있지 않음
   - 구조체 이름과 함께 `::` 문법을 이용해 호출할 수 있음

---
### 4. 열거형과 패턴 매칭(Enums and Pattern Matching)
> Rust의 열거형은 F#, OCaml, Haskell과 같은 함수형 언어의 **대수 데이터 타입**과 가장 비슷하다.

```rust
// variants of enumeration
enum IpAddrKind {
  V4,
  V6,
}
```

##### 4.1 열거형 정의 방식
- Enum의 각 variant는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있음
- 어떤 종류의 데이터라도 넣을 수 있음

```rust
enum IpAddr {
  V4(String),
  V6(String),
}
```

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String)
}
```

##### 4.2 다양한 유형의 타입이 포함된 열거형

- 구조체와 마찬가지로 열거형에도 메소드를 정의할 수 있음

```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```

##### 4.3 Option을 통한 열거형 배우기

- Rust에는 null이 없음
- 이로 인해 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형인 `Option<T>`이 만들어짐
- 기본적으로 포함되어 있어, 명시적으로 가져오지 않아도 사용할 수 있음
  - `Option`의 variants인 `Some`과 `None`도 바로 사용할 수 있음
  - `None`을 사용할 경우 어떤 타입을 사용할 것인지 알려줘야 함

```rust
let some_number = Some(5);
let some_string = Some("string");

let absent_number: Option<i32> = None;
```