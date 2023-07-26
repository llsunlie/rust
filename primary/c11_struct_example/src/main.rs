#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 40;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    /* tuple */
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((width1, height1))
    );

    /* struct */
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rec)
    );

    /* trait */
    let rec = Rectangle {width: 30, height: 50};
    println!("rec is {:?}", rec);
    println!("rec is {:#?}", rec);

    println!(
        "The area of the rectangle is {} square pixels.",
        rec.area()
    );

    /* another trait */
    let rec1 = Rectangle {width: 30, height: 50};
    let rec2 = Rectangle {width: 10, height: 40};
    let rec3 = Rectangle {width: 60, height: 45};

    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));

    /* associated funcations */
    let rec = Rectangle::build_square(3);
    println!("rec is {:#?}", rec);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}