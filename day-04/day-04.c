#include <stdio.h>
#include <stdlib.h>
#include "lib.h"


int main(void) {
    FILE *f = fopen("input.txt", "r");
    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    printf("%lu\n", fsize);
    fseek(f, 0, SEEK_SET);

    char *input = malloc(fsize + 1);
    fread(input, fsize, 1, f);
    fclose(f);
    input[fsize] = 0;

    // printf("Input: %s\n\n", input);

    printf("Part 1: %d\n", part1(input));
    printf("Part 2: %d\n", part2(input));

    free(input);

    return 0;
}