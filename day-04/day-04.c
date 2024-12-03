#include <stdio.h>
#include "lib.h"


int main(void) {
    FILE *fptr = fopen("input.txt", "r");
    char input[1000];
    fgets(input, 1000, fptr); 

    printf("Input: %s\n\n", input);

    printf("Part 1: %d\n", part1("input"));
    printf("Part 2: %d\n", part2("input"));
    return 0;
}