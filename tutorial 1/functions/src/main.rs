fn main() {
    println!("Functions");
    first_fn();
}

// simple function
fn first_fn(){
    println!("new Function");
    second_fn(34);
    third_fn(34,'H');
    ex();
    println!("{}",return_value());
}

// pass single params
fn second_fn(x:i32){
    println!("{}",x);
}

// pass multiple params
fn third_fn(x:i32, y:char){
    println!("{x}, {y}");
}

//expressions

fn ex(){
     let y = {
        let x = 9;
        x+1
     };
     println!("{y}")
}

// return value from function
fn return_value()-> i32{
    78+34
}