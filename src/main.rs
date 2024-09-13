fn main() {
    let greeting:String = String::from("hello boss");
    println!("{}",greeting);
    println!("{}",greeting.chars().nth(1).unwrap());

    let is_even: bool = true;
    if is_even {
        println!("the number is even");
    }
    for i in 0..11{
        print!("{} ",i);
    }
    println!();
    let first_word = get_first_word(greeting);
    print!("{}",first_word);
}


fn get_first_word(sentence:String)->String{
    let mut ans:String = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char ==' '{
            break;
        }
    }
    return ans;
}
