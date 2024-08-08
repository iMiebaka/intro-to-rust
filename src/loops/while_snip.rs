
pub fn main(){
    let mut  number: u32 = 10;
    let mut break_loop = true;
    while number > 5 {
        // code to execute
        number -= 1;
        println!("number is currently at {}", number);
        
    }
    
    while break_loop{
        number -= 1;
        println!("number is currently at {}", number);
        if number == 0{
            break_loop = false
        }
    }
}