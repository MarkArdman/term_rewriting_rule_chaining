#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define MAX_N (1 << 10)

FILE *fin, *fout;

int n;
int val[MAX_N][MAX_N];
int vals[MAX_N][MAX_N];

void read(void)
{
    fscanf(fin, "%d", &n);
}

void solve(void)
{
    int i, j, valt;
    /* i length, j n elem */
    /* vals[i][j] = sum(val[i][1], val[i][j])*/
    memset(val, 0, sizeof(val));
    memset(vals, 0, sizeof(vals));
    for (i = 1; i <= n; ++i) {
        val[1][i] = i;
        val[2][i] = i * i;
        vals[1][i] = vals[1][i - 1] + i; 
        vals[2][i] = vals[2][i - 1] + i * i; 
    }
    for (i = 3; i <= n; ++i) {
        valt = 0;
        for (j = 2; j <= n; ++j) {
            val[i][j] = 0;
            val[i][j] += val[i - 2][j - 1];
            val[i][j] += val[i - 1][j - 1];

            valt += vals[i - 2][j - 2] + val[i - 2][j - 2];
            val[i][j] += valt + vals[i - 1][j - 2];
            valt = valt % 1000000;
            val[i][j] = val[i][j] % 1000000;
            vals[i][j] = vals[i][j - 1] + val[i][j];
        }
    }
/*    for (i = 1; i <= n; ++i) {
        for (j = 1; j <= n; ++j) {
            fprintf(fout, "%d ", val[i][j]);
        }
        fprintf(fout, "\n");
    }*/
    fprintf(fout, "%d\n", val[n][n]);
}

int main(void)
{
    fin = fopen("sir23.in", "rt");
    fout = fopen("sir23.out", "wt");
    read();
    solve();
    fclose(fin);
    fclose(fout);
    return 0;
}