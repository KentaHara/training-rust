#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn simple_panic(flg: bool) -> bool {
    if flg {
        panic!("Panic!!");
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // プロジェクト作成時に初期記載されているテスト
    //
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    //
    // 関数名を変えてテスト
    //
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    //
    // panicさせるパターン
    // - 正常に終了させるために、コメントアウトしています
    //
    #[test]
    fn another() {
        //     panic!("Make this test fail");
    }

    //
    // Rectangle->can_holdのテスト
    //
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //
    // `assert_eq!` のテスト
    //
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    //
    // `assert_ne!` のテスト
    //
    #[test]
    fn it_adds_two_negative() {
        assert_ne!(5, add_two(2));
    }

    //
    // テストメッセージを変更したテスト
    //
    #[test]
    fn change_error_message() {
        let result = String::from("Hello Rust!");
        assert!(
            result.contains("Hello New World!"),
            "result did not contain name, value was `{}`",
            result
        );
    }
    #[test]
    fn change_error_message_use_eq() {
        let result = String::from("Hello Rust!");
        assert_eq!(
            result,
            String::from("Hello New World!"),
            "result did not contain name, value was `{}`",
            result
        );
    }
    #[test]
    fn change_error_message_use_ne() {
        let result = String::from("Hello Rust!");
        assert_ne!(
            result,
            String::from("Hello Rust!"),
            "result did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_test_panic() {
        simple_panic(true);
    }

    #[test]
    #[should_panic]
    fn should_panic_test_nopanic() {
        simple_panic(false);
    }

    #[test]
    fn it_works_use_result_ok() -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn it_works_use_result_err() -> Result<(), String> {
        Err(String::from("ERROR MESSAGE"))
    }
}
