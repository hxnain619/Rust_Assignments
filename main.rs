fn main() {


    // Task 1
    let some_word = "Pakistan Zindabad";

    println!("The Word is {} \n", some_word);
    println!("The Length of word is {} \n \n", some_word.len());   

    // Task 2

    let integer_one:i64 = 85;
    let integer_two:i32 = -550;

    println!("{} \n {}",integer_one, integer_two);

    // Task 3

    let float_number:f32 = 56.6;
    println!("{}", float_number);

    // Task 4

    let (number1,  number2) = (76, 23);
    let sum = number1 + number2;
    let substract = number1 - number2;
    let multiply = number1 * number2;
    let divide = number1 / number2;
    let modulus = number1 % number2;

    println!("{} + {} = {} \n \n", number1, number2, sum);
    println!("{} - {} = {} \n \n", number1, number2, substract);
    println!("{} x {} = {}", number1, number2, multiply);
    println!("{} / {} = {}", number1, number2, divide);
    println!("{} % {} = {} \n \n \n", number1, number2, modulus);

    // Task 5

    let numbers = [100 ,150, 200, 250 ,300];

    println!("{:?} \n", numbers);
    println!("{} \n",numbers[1]);
    println!("{} \n",numbers[3]);

    // Task 6
    let tuple = ("IoT", "AI", "Cloud", 500.65, 8645, 65.4);

    println!("{:?} \n {} \n {} \n {}", tuple, tuple.2, tuple.4, tuple.5);

    // Task 7
    
    ADDITION(12, 12, 312);

    // Task 8
    
    println!("{:?}",MULTIPLICATION(12.12, 123.12, 1.90));

    // Task 9
   
    println!("\n \n Marks Scored 35");
    check_result(35);

    println!("\n \n Marks Scored 70");
    check_result(70);


    // Task 10
    let year = 2019;

    if (year%4) != 0 {
        println!("{} isnt a leap year", year);
    }else {
        println!("{} isnt a leap year", year);
    }

    // Task 11
    
    println!("even numbers... \n");
    for x in 2..20 {
        if x%2 == 0 {
            println!("{}", x);
        }
    }

    // Task 12
    
    println!("odd numbers... \n");
    for x in 0..20 {
        if x%3 == 0 {
            println!("{}", x);
        }
    }

    // Task 13
   
    println!("Table... \n \n");
    print_table(5);

}

fn print_table(num:i32) {
    
    for x in 1..10 {
        println!("{} x {} = {} \n",num,x,x*num);
    }
}


fn ADDITION(a:i32, b:i32, c:i32) {

    println!("sum of all params is {} \n \n \n ", a+b+c)

}


fn MULTIPLICATION(a:f64, b:f64, c:f64) -> f64 {

    return a*b*c;

}

fn check_result(marks:i64) {

    if marks > 80{
        println!("Your Grade is  A+");
    }else if marks > 70 && marks <= 80 {
        println!("Your Grade is A");
    }else if marks > 60 && marks <= 70 {
        println!("Your Grade is B");
    }else if marks > 50 && marks <= 60 {
        println!("Your Grade is C");
    }else if marks > 40 && marks <= 50 {
        println!("Your Grade is D");
    }else if marks < 40{
        println!("Your Grade is F");
    }

}