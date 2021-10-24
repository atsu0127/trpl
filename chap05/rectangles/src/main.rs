fn main() {
    let rec1 = Rectangle {
        width: 100,
        height: 200,
    };

    println!("{}", rec1.area());
    println!("{:#?}", rec1);

    let rec2 = rec1.resize(10);
    println!("{}", rec2.area());
    println!("{:#?}", rec2);

    let rec_big = Rectangle {
        width: 1000,
        height: 1000
    };
    let rec_small = Rectangle {
        width: 10,
        height: 10,
    };
    let rec_mid = Rectangle {
        width: 100,
        height: 100,
    };

    println!("Can rec_big contains rec_mid? -> {}", rec_big.contain(&rec_mid));
    println!("Can rec_small contains rec_mid? -> {}", rec_small.contain(&rec_mid));

    println!("10 x 10 square: {:?}", Rectangle::square(10));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // これはクラスメソッド
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn contain(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn resize(self, rate: u32) -> Rectangle {
        Rectangle {
            width: self.width * rate,
            height: self.height * rate,
        }
    }

    // これはstatic関数(第一引数にselfがない)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}