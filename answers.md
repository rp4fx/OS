The reason why rust does not like a modification of a global variable as such 
is because the global variable may go out of scope and cause a dynamic memory
hole. Therefore, it could lead to memory problems. Rust, being a very safe language,
would try to prevent this from happening which is why an error is printed out.