fn main() {
    for i in 1..11 {
        println!("i is: {}", i);
    }

    let numbers = 30..51;

    for number in numbers {
        println!("number is: {}", number);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for animal in animals.iter() {
        println!("animal is: {}", animal);
    }

    for (index, animal) in animals.iter().enumerate() {
        println!("animal in index {} is: {}", index, animal);
    }
}
