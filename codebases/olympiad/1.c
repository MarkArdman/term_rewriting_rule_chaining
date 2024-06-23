#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define MAX_N 3501
#define max(a, b) ((a) > (b) ? (a) : (b))
#define LSB(x) ((x) & (~((x) - 1)))

typedef struct {
    int x, y, z;
} box;

typedef int(*qcmp)(const void*, const void*);

box boxes[MAX_N];
int* a[MAX_N];
int n, t, nt;

int qcmpbox(const box* a, const box* b)
{
    return a->z - b->z;
}

void read(void)
{
    int i;
    for (i = 0; i < n; ++i) {
        scanf("%d%d%d", &boxes[i].x, &boxes[i].y, &boxes[i].z);
    }
}

int query(int x, int y)
{
    int i, j, r = 0;
    for (i = x; i; i -= LSB(i)) {
        for (j = y; j; j -= LSB(j)) {
            r = max(r, a[i][j]);
        }
    }
    return r;
}

void set(int x, int y, int v)
{
    int i, j;
    for (i = x; i < MAX_N; i += LSB(i)) {
        for (j = y; j < MAX_N; j += LSB(j)) {
            a[i][j] = max(a[i][j], v);
        }
    }
}

void dump(void)
{
    int i, j;
    for (i = 0; i < MAX_N; ++i) {
        for (j = 0; j < MAX_N; ++j) {
            printf("%2d", a[i][j]);
        }
        printf("\n");
    }
    printf("\n");
}

int solve(void)
{
    int i, v, r;
    qsort(boxes, n, sizeof(box), (qcmp)qcmpbox);
    r = 0;
    for (i = 0; i < n; ++i) {
        v = query(boxes[i].x, boxes[i].y);
        v -= nt * MAX_N;
        if (v < 0) {
            v = 0;
        }
        ++v;
        r = max(v, r);
        set(boxes[i].x, boxes[i].y, v + nt * MAX_N);
        /*dump();*/
    }
    return r;
}

void init(void)
{
    int i;
    for (i = 0; i < MAX_N; ++i) {
        a[i] = malloc(MAX_N * sizeof(int));
        memset(a[i], 0, MAX_N * sizeof(int));
    }
}

int main(void)
{
    freopen("cutii.in", "rt", stdin);
    freopen("cutii.out", "wt", stdout);

    scanf("%d%d", &n, &t);
    init();
    for (nt = 0; nt < t; ++nt) {
        read();
        printf("%d\n", solve());
    }
    return 0;
}