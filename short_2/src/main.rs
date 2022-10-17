use std::io;

fn main() {
    let ballon_volume = 0.07;
    println!("Each ballon has a volume of {}m^3", ballon_volume);
    balloon_number(ballon_volume);
}

fn balloon_number(balloon_volume: f32){
    let mut room_volume = room_volume();
    room_volume = room_volume / 1000000 as f32;
    println!("The volume of the room is... {}m^3", room_volume);
    // {1} will use the first (not 0th) value given after the "" so in this example room_volume
    println!("The total number of balloons is {1} / {0} which = {2}", balloon_volume, room_volume, room_volume / balloon_volume);

}

fn room_volume() -> f32{
    let length = length();
    let height = height();
    let width = width();

    let volume: f32 = (length * width * height) as f32;
    volume
}

fn length() -> i32 {
    println!("Enter the length of the room");

    let mut length_input = String::new();

    // Below reads the input from the user
    io::stdin()
        .read_line(&mut length_input)
        .expect("Nothing was read");

    let length_inputted_int = length_input.trim().parse::<i32>().unwrap(); // This converts from a  String to i32

    length_inputted_int // This returns the value in length_inputted_int
}

fn width() -> i32 {
    println!("Enter the width of the room");

    let mut width_input = String::new();

    // Below reads the input from the user
    io::stdin()
        .read_line(&mut width_input)
        .expect("Nothing was read");

    let width_inputted_int = width_input.trim().parse::<i32>().unwrap(); // This converts from a  String to i32

    width_inputted_int // This returns the value in length_inputted_int
}

fn height() -> i32 {
    println!("Enter the height of the room");

    let mut height_input = String::new();

    // Below reads the input from the user
    io::stdin()
        .read_line(&mut height_input)
        .expect("Nothing was read");

    let height_inputted_int = height_input.trim().parse::<i32>().unwrap(); // This converts from a  String to i32

    height_inputted_int // This returns the value in length_inputted_int
}
