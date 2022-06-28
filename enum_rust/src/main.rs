
//Enum
#[derive(Debug)]
enum IpAddressKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]
struct _IpAddress {
    kind: IpAddressKind,
    address: String
}

impl _IpAddress {
    fn some_function () {
        println!("Blockchain development")
    }
}

//Option enum

fn main() {
    let localhost = IpAddressKind::V4(127,0,0,1);

    println!("Localhost = {:#?}", localhost);

    //Option enum
    let x = 5;
    let y = None;

    let sum = x + y.unwrap_or(7);
    println!("Sum = {}", sum);

    //Match Option
    decimals(Coin::Bitcoin(Balance::Fish));

    let five = Some(5);

    let six = plus_one(five);
    println!("six = {:#?}", six);

    let none = plus_one(None);
    println!("None = {:#?}", none);

    let _value = Some(4);
    match _value {
        Some(5) => println!("Bang 5"),
        _ => println!("Khac 5"),
    }
}

#[derive(Debug)]
enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark,
}

enum Coin {
    Solana,
    Ethereum,
    Near,
    Bitcoin(Balance)
}

fn decimals (coin: Coin) -> u8 {
    match coin {
        Coin::Solana => {
            println!("Solana Match");
            1
        },
        Coin::Ethereum => 10,
        Coin::Near => 20,
        Coin::Bitcoin(b) => {
            println!("I am a {:#?}", b);
            30
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => {Some(x + 1)},
        _ => None
    }
}