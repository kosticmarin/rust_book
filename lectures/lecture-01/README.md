# Assigment 1
Implement the command line program 'Factorial'. The program has
a command line interface and takes no arguments upon startup.

Upon start up users are shown a message to enter a single number
from a range 3 to 20 inclusive. In the case if the user enters a
number that is not from the range or not a number at all the program
will print out an appropriate message and promt the user to try again.
Once a correct number is enterd the program will print out the result
of factorial calculation (see [Factorial Wiki](https://en.wikipedia.org/wiki/Factorial)).
Make sure to use the correct primitive type in the calculation result
to cover the whole range of numbers.

The program terminates when the user enters `exit`(case insensitive).

The factorial calculation should be implemented in function.
If the calculation cannot be executed the function must return
an `Result` type.

Further think about the code what part of it has what responsiblility
in order to better structure and organize it.

## Example progam usage

For the following example Your program should produce the identical results.

```console
foo@bar:~$ ./factorial
Enter a number > number
'number' is not a valid number.
Enter a number > 3
3! = 6
Enter a number > 3.14
'3.14' is not a valid number.
Enter a number > 1
'1' is not in the valid range. Try a number from range [2, 20].
Enter a number > exit
Goodbye!
```

# Assigment 2

Implement the command line program called 'Rectangle'. The program
uses two command line arguments the width and height of the rectangle.
Then the program prints the perimiter and the area of the defined
rectangle. Further if the program is started without command line
arguments it will ask the users to enter the width and the height to
define the rectangle.

The constrains of the program are that the numbers entered must
be positive real numbers. Further the program accepts only zero
or two command line arguments (width and height), otherwise it
should print out a help message and exit.

Further think about the code what part of it has what responsiblility
in order to better structure and organize it.

## Example program usage

For the following example Your program should produce the identical results.

1. Starting the program with command line arguments.
```console
foo@bar:~$ ./rectangle 2 8
Rectangle width=2.0 and height=8.0 has perimiter=20.0 and area=16.0.
```
2. Starting the program without command line arguments.
```console
foo@bar:~$ ./rectangle
Enter width > asdf
'asdf' is not a real number.
Enter width > -2.1
Width of the rectangle must be positive.
Enter width > 2
Enter height > -2.1
Height of the rectangle must be positive.
Enter height > 8.0
Rectangle widht=2.0 and height=8.0 has perimiter=20.0 and area=16.0.
```

### Notes
* With the inplementation provide a test suite (see [Testing Rust Code](https://doc.rust-lang.org/book/ch11-00-testing.html))
* Use proper error handling (see [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html))
* Review Cargo Target configuration (see [Configuring a target](https://doc.rust-lang.org/cargo/reference/manifest.html#configuring-a-target))

