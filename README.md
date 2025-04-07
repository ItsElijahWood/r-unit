# R-unit a unit testing library

**R-unit** is a fast and easy-to-use library made in Rust, it's aim is to make testing simple and catch errors early.
If any of the functions fail it will panic with the build error.

## Installation

Add the library
```bash
   cargo add r_unit 
```

## Documentation

### Math Library

Check if two parameters which take an Any type are an equal value to each other
```
    use r_unit::math;

    let a = 5;
    let b = 5;

    math::is_equal(&a, &b);
```

Check if two parameters which take an Any type are not equal value to each other
```
    use r_unit::math;

    let a = 5;
    let b = 6;
    
    math::is_not_equal(&a, &b);
```

Checks if parameter 1 is less than parameter 2
```
    use r_unit::math;

    let a = 5;
    let b = 6;
    
    math::is_less_than(&a, &b);
```
Checks if parameter 1 is greater than parameter 2
```
    use r_unit::math;

    let a = 7;
    let b = 6;
    
    math::is_greater_than(&a, &b);
```

Checks if parameter 1 is a negative number
```
    use r_unit::math;
   
    let a = -3;
    
    math::is_negative(&a);
```

Checks if parameter 1 is an odd number
```
    use r_unit::math;
  
    let a = 7;

    math::is_odd(&a);
```

Checks if parameter 1 is an even number
```
    use r_unit::math;

    let a = 6;

    math::is_even(&a);
```

### String Library

Check if two parameters which take a &str type have the same length to each other 
```
    use r_unit::string;

    let a = "hi";
    let b = "hi";

    string::matches_length(&a, &b);
```

Check if the provided parameter contains special characters, returns true or false
```
    use r_unit::string;

    let a = "hi %";

    if string::contains_special_characters(&a) {
        println!("Contains special characters");
    } else {
        println!("Does not contain special character");
    };
```

Check if parameter 1 contains numbers in a string slice or not, returns true or false
```
     use r_unit::string;

     let a = "hi 3";

     if string::contains_numbers(&a) {
         println!("Contains special characters.");
     } else {
         println!("Does not contain special characters.");
     };
```