use std::io;
use std::thread::sleep;

fn main() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();

        if input_string.trim() == "67"{
            println!("nigga, deine {} ist gay as fuck", input_string.trim());
        }else if input_string.trim() != "x" && input_string.trim() != "X" && input_string.trim() != "timer" {
            println!("You wrote {}", input_string);
        }else if input_string.trim() == "timer" {
            println!("Wie lange?");
            let mut timer_string = String::new();
            io::stdin().read_line(&mut timer_string).unwrap();
            let mut timer = 1;
            let input_int  = timer_string.trim().parse::<i32>().unwrap();
            println!("{} Sekunden, und los!", input_int);
            while timer != input_int+1 {
                println!("Der Timer sagt du bist ein bastard und {}", timer);
                timer = timer + 1;
                if timer == input_int+1{
                    println!("Done!");
                    return;
                }
                sleep(std::time::Duration::from_secs(1));

            }



        }
    }
    println!("See you later!");

}
