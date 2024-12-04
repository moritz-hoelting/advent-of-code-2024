#include <string.h>
#include <stdio.h>

int posToIdx(int line_length, int l, int r) {
    return l * (line_length + 1) + r;
}

int checkPosXmas(const char *input, int line_length, int xl, int xr, int ml, int mr, int al, int ar, int sl, int sr) {
    if (xl >= 0 && xr >= 0 && ml >= 0 && mr >= 0 && al >= 0 && ar >= 0 && sl >= 0 && sr >= 0) {
        return input[posToIdx(line_length, xl, xr)] == 'X' && input[posToIdx(line_length, ml, mr)] == 'M' 
            && input[posToIdx(line_length, al, ar)] == 'A' && input[posToIdx(line_length, sl, sr)] == 'S';
    }
    return 0;
}


int part1(const char input[]) {
    int line_length = (int) (strchr(input, '\n') - input);
    int line_amount = (strlen(input) + 1) / (line_length + 1);

    int found = 0;

    for (int l = 0; l < line_amount; l++) {
        for (int r = 0; r < line_length; r++) {
            // left to right
            found += checkPosXmas(input, line_length, l, r - 3, l, r - 2, l, r - 1, l, r);
            // right to left
            found += checkPosXmas(input, line_length, l, r, l, r - 1, l, r - 2, l, r - 3);
            // top to bottom
            found += checkPosXmas(input, line_length, l - 3, r, l - 2, r, l - 1, r, l, r);
            // bottom to top
            found += checkPosXmas(input, line_length, l, r, l - 1, r, l - 2, r, l - 3, r);
            // top left to bottom right
            found += checkPosXmas(input, line_length, l - 3, r - 3, l - 2, r - 2, l - 1, r - 1, l, r);
            // bottom right to top left
            found += checkPosXmas(input, line_length, l, r, l - 1, r - 1, l - 2, r - 2, l - 3, r - 3);
            // top right to bottom left
            found += checkPosXmas(input, line_length, l - 3, r, l - 2, r - 1, l - 1, r - 2, l, r - 3);
            // bottom left to top right
            found += checkPosXmas(input, line_length, l, r - 3, l - 1, r - 2, l - 2, r - 1, l - 3, r);
        }
    }

    return found;
}

int checkPosX_mas(const char *input, int line_length, int line_amount, int l, int r) {
    if (l > line_amount || r > line_length || l < 1 || r < 1) {
        return 0;
    }
    // middle is A
    return input[posToIdx(line_length, l, r)] == 'A' 
        // top left to bottom right
        && ((input[posToIdx(line_length, l - 1, r - 1)] == 'M' && input[posToIdx(line_length, l + 1, r + 1)] == 'S') 
            // bottom right to top left
            || (input[posToIdx(line_length, l - 1, r - 1)] == 'S' && input[posToIdx(line_length, l + 1, r + 1)] == 'M')) 
        // top right to bottom left
        && ((input[posToIdx(line_length, l - 1, r + 1)] == 'M' && input[posToIdx(line_length, l + 1, r - 1)] == 'S') 
            // bottom left to top right
            || (input[posToIdx(line_length, l - 1, r + 1)] == 'S' && input[posToIdx(line_length, l + 1, r - 1)] == 'M'));
}

int part2(const char input[]) {
    int line_length = (int) (strchr(input, '\n') - input);
    int line_amount = (strlen(input) + 1) / (line_length + 1);

    int found = 0;

    for (int l = 0; l < line_amount; l++) {
        for (int r = 0; r < line_length; r++) {
            found += checkPosX_mas(input, line_length, line_amount, l, r);
        }
    }

    return found;
}