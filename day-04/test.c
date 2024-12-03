#include <stdio.h>
#include "lib.h"

const char sample[] = "hello, world2";

int main() {
    int failed = 0;
    int result_1 = part1(sample);
    if (result_1 == 1) {
        printf("Test 1 passed\n");
    } else {
        printf("Test 1 failed with value: %d\n", result_1);
        failed = 1;
    }
    int result_2 = part2(sample);
    if (result_2 == 2) {
        printf("Test 2 passed\n");
    } else {
        printf("Test 2 failed with value: %d\n", result_2);
        failed = 1;
    }

    return failed;
}