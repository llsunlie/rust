fn main() {
    /* get max value */
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number
        }
    }

    println!("The largest number is {}", largest);

    /* generics */
    let number_list = vec![34, 50, 25, 100, 65];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    /* generics in struct */
    struct Point<T> {
        x: T,
        y: T,
    }

    let a = Point {x: 1, y: 2 };
    let b = Point {x: 1.0, y: 2.0 };
    // let c = Point {x: 1, y: 4.0};

    struct Point_2<T, U> {
        x: T,
        y: U,
    }

    let a = Point_2 {x: 1, y: 2.2};

    /* generics in enum */
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    /* generics in method */
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let a = Point { x: 1, y: 2 };
    let t1 = a.x();
    let t2 = a.y();
    let t3 = a.x();
    let t4 = a.y();
    println!("t1 = {}, t2 = {}, t3 = {}, t4 = {}", t1, t2, t3, t4); 

    let b = Point { x: 1.0, y: 1.1 };
    let c: Point<f32> = Point { x: 1.0, y: 3.2 };
    let t = c.distance_from_origin();
    println!("t = {}", t);

    impl<T, U> Point_2<T, U> {
        fn mixup<V, W>(self, other_point_2: Point_2<V, W>) -> Point_2<T, W> {
            Point_2 { x: self.x, y: other_point_2.y }
        }
    }

    let p1 = Point_2 { x: 5, y: 10.4 };
    let p2 = Point_2 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    /* monomorphization */
    enum Option_i32 {
        Some(i32),
        None,
    }
    
    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);    

    println!("===");

    let list = vec![34, 50, 25, 100, 65];

    let t1 = get_largest_new(&list);
    let t2 = get_largest_new_1(&list);
    let t3 = get_largest_new_2(&list);

    println!("t1 = {}, t2 = {}, t3 = {}", t1, t2, t3);
}

fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn get_largest_new<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn get_largest_new_1<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

fn get_largest_new_2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest

}