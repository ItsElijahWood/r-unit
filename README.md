# R-unit a unit testing library

**R-unit** is a fast and easy-to-use library made in Rust, it's aim is to make testing simple and catch errors early.
If any of the functions fail it will panic with the build error.

## Installation

Add the library
```bash
   cargo add r_unit 
```

## Documentation

Check if two parameters which take an Any type are an equal value to each other
```
    use r_unit::is_equal;

    let a = 5;
    let b = 5;

    is_equal::is_equal(&a, &b);
```

Check if two parameters which take an Any type are not equal value to each other
```
    use r_unit::is_not_equal;

    let a = 5;
    let b = 6;
    
    is_not_equal::is_not_equal(&a, &b);
```

## String Library

Check if two parameters which take a &str type have the same length to each other 
```
    use r_unit::string;

    let a = "hi";
    let b = "hi";

    string::matches_length(&a, &b);
```