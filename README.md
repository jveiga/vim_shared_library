# Example of creating a library that is callable in vim

## How to use
* Cargo build
* open vim
* there are to functions
* hi :echo libcall('<PATH_TO_LIB>/liblib.so', 'hi', '{"data_int":50}')
  * sends a string in vim, rust tries to parse json and add to 'data_int' value
* hn :echo libcallnr('<PATH_TO_LIB>/libmylib.so', 'hn', '123')
  * sends a string and returns the number of bytes
