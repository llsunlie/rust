use c42_oop_trait::{Draw, Button, SelectBox};

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

}

fn main() {
    let vec: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { width: 1, height: 2, label: String::from("button") }),
        Box::new(SelectBox { width: 3, height: 4, options: vec![String::from("value")] })
    ];

    let screen = Screen {
        components: vec,
    };

    screen.run();

    // let screen = Screen {
    //     components: vec![
    //         Box::new(String::from("Hi")),
    //     ],
    // };

    // screen.run();


}