
int main(void) {
    int x = 5;
    foo();

    int * ptr_to_a = baz();
    bar();
    printf("Hi %d", *ptr_to_a);
    free(ptr_to_a);


}

int foo() {
    int y = 6;
}

int bar() {
    int y = 8;
    int z = 9;
}

int * baz() {
    int * ptr_to_a = malloc(sizeof(int));
    *ptr_to_a = 90;
    return ptr_to_a;
}
