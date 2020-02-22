use async_trait::async_trait;
use std::time::Duration;
use tokio::time::delay_for;

#[async_trait]
trait Person {
    async fn get_name(&self) -> String;
}

struct Man;

#[async_trait]
impl Person for Man {
    async fn get_name(&self) -> String {
        delay_for(Duration::from_secs(1)).await;
        "Marcel".to_string()
    }
}

impl Default for Man {
    fn default() -> Self {
        Man
    }
}

struct Printer {
    man: Box<dyn Person>,
}

impl Default for Printer {
    fn default() -> Self {
        let man = Man::default();

        Printer {
            man: Box::from(man)
        }
    }
}

impl Printer {
    fn new(man: Box<dyn Person>) -> Self {
        Printer {
            man
        }
    }
}

#[tokio::main]
async fn main() {
    let p = Printer::default();
    let name = p.man.get_name().await;
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use mockall::predicate::*;

    mock! {
        pub Man {
            fn get_name(&self) -> String {}
        }
    }

    #[async_trait]
    impl Person for MockMan {
        async fn get_name(&self) -> String {
            delay_for(Duration::from_secs(1)).await;
            self.get_name()
        }
    }

    #[tokio::test]
    async fn test_person() {
        let p = Printer::default();
        assert_eq!("Marcel".to_string(), p.man.get_name().await);
    }

    #[tokio::test]
    async fn test_mocked_person() {
        let mut mock = MockMan::default();
        mock.expect_get_name().times(1).returning(|| "Horaci".to_string());

        let p = Printer::new(Box::from(mock));
        assert_eq!("Horaci".to_string(), p.man.get_name().await);
    }
}
