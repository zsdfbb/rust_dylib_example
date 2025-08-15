#include <stdio.h>
#include "myclib.h"

int c_add(int a, int b) {
    return a + b;
}

void c_greet(const char *s) {
    if (s) {
        printf("[myclib] Hello from C: %s\n", s);
    } else {
        printf("[myclib] Hello from C: (null)\n");
    }
}
