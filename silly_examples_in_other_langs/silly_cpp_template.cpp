/******************************************************************************

Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

*******************************************************************************/
#include <iostream>


template <typename T>
class Foo {
    T myT;

    public:
    void myFunc() {
        // This compiles! Ain't that disturbing, given that T could maybe not support `+`?
        std::cout << "Hello world" << myT + 2;

    }
};

int main()
{
    return 0;
}
