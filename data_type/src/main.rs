fn main() {

    // Declare a variable
    /**
     *  company name is: TutorialsPoint
        company rating on 5 is:4.5
        company is growing: true
        company icon is: ♥
     */
    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let is_growing_boolean = true;          // boolean type
    let icon_char = '♥';                    //unicode character type

    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}",icon_char);

    let float_with_separator = 11_000.555_001;
    println!("float value {}",float_with_separator);

    let int_with_separator = 50_000;
    println!("int value {}",int_with_separator);

    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';

    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}",emoji);



}
