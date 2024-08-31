# Intro
* While going through  level 10, I found it extremely hard to understand macros and traits, and to implement the given question.
* What I did was go to ChatGPT, and to ask it give more basic level macros and traits problem so that I can implement them and get a better understanding of them. So, here they are

### Macros
- [ ] **Basic Macro Definition:** Write a macro that takes a single argument and prints it to the console.
- [ ] **Macro with Conditional Patterns:** Create a macro that takes one or two arguments. If one argument is provided, print it as-is. If two arguments are provided, print each argument on a new line.
- [ ] **Macro with Repetition:** Define a macro that takes a list of items and prints each item on a new line. The list can be provided with varying numbers of items.
- [ ] **Macro with Variable Arguments:** Write a macro that takes a variable number of arguments and creates a function that prints all arguments passed to it.
- [ ] **Macro with Syntax Checks:** Develop a macro that takes a struct name and generates a basic implementation for `Debug` for that struct. Ensure it handles cases where the struct might have different fields.
- [ ] **Simple Struct and Method Generation:** Create a macro that generates a simple struct with one field and provides a method to get that field’s value.
- [ ] **Struct Field Initialization:** Write a macro that generates a struct with specified fields and a method that initializes the struct with given values.
- [ ] **Implementing Traits via Macros:** Define a macro that implements the `Default` trait for a struct with default values for its fields.

### Traits
- [ ] **Basic Trait Definition**  
  Problem: Create a trait called `Describe` with a method `describe` that returns a `String` describing the object. Implement this trait for a simple struct.
- [ ] **Trait Implementation for Multiple Structs**  
  Problem: Define a trait `Area` with a method `area` that returns a `f64`. Implement this trait for two different structs: `Rectangle` and `Circle`.
- [ ] **Trait with Generic Types**  
  Problem: Create a trait `Sum` with a method `sum` that calculates the sum of values. Implement this trait for a struct `Container<T>` where `T` is a numeric type, and `Container` holds a collection of `T`.
- [ ] **Trait with Associated Types**  
  Problem: Design a trait `Shape` with an associated type `Dimensions`. Implement this trait for `Rectangle` and `Circle`, where `Dimensions` is a tuple representing the dimensions of the shape.
- [ ] **Trait Bound in Generic Function**  
  Problem: Define a trait `Volume` with a method `volume`. Create a generic function `print_volume<T: Volume>(shape: T)` that prints the volume of the shape. Implement `Volume` for a struct `Cube`.
- [ ] **Trait for Complex Geometry Calculations**  
  Problem: Create a trait `Shape` with methods `area` and `perimeter`. Implement this trait for `Triangle` and `Rectangle`. Ensure each implementation correctly calculates the area and perimeter based on the shape's properties.
- [ ] **Combining Traits and Generics**  
  Problem: Define two traits: `Describe` and `Area`. Implement a struct `ShapePrinter` that uses a generic type bound by both `Describe` and `Area`. Create a method that prints the shape's area and description.
- [ ] **Default Trait Implementation**  
  Problem: Design a trait `DefaultArea` with a default implementation of `area` that returns a constant value. Implement this trait for a struct `Square`, overriding the default `area` method.
- [ ] **Using Trait Objects**  
  Problem: Define a trait `Reportable` with methods for generating a textual report. Implement this trait for various data types, including a `User` struct and a `Product` struct. Write a function that accepts a trait object `&dyn Reportable` and generates a comprehensive report based on the data type.