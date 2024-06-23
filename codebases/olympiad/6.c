#include <stdio.h>
#include <stdlib.h>

#define MAXN (1 << 10)
#define abs(x) ((x) < 0 ? (-(x)) : (x))

typedef struct point { int x, y, p, ind; } point;

int N, T, total, cnt, last, lastind;

point X, Y;
point A[MAXN], over[MAXN], under[MAXN], V[MAXN];

int cross(point A, point B, point C)
{
    return (B.x-A.x)*(C.y-A.y)-(C.x-A.x)*(B.y-A.y) > 0 ? 1 : -1;
}

int cmp_sort(const void *a, const void *b)
{
    point u = *(point*)a, v = *(point*)b;
    if(u.x == v.x)
        return u.y-v.y;
    return u.x-v.x;
}

int cmp(point x, point y)
{
    if( cross(X, x, y) == -1 )
        return 1;
    return (-1);
}

point AUX[MAXN];

void my_qsort(int st, int dr, point V[MAXN])
{
    int i, m, l = st, r = dr;

    if(st >= dr) return ;
    
    m = rand() % (dr-st+1) + st;

    for(i = st; i <= dr; i++)
     if(i != m)
     {
        if(cmp(V[i], V[m]) == -1)
            AUX[l++] = V[i];
        else
            AUX[r--] = V[i];
     }
    for(AUX[l] = V[m], i = st; i <= dr; i++)
        V[i] = AUX[i];

    if(st < l) my_qsort(st, l-1, V);
    if(l < dr) my_qsort(l+1, dr, V);
}

void solve(void)
{
    int i, j, k, n1, n2, P1, P2;

    srand(time(0));
    
    for(i = 1; i < N; i++)
    {
        X = A[i], Y = A[i+1], n1 = n2 = P1 = P2 = 0;
        
        for(j = 1; j <= N; j++)
         if(j != i && j != i+1)
         {
            if( cross(X, A[j], Y) == -1 ) // deasupra
                over[n1++] = A[j], P1 += A[j].p;
            else
                under[n2++] = A[j], P2 += A[j].p;
         }

        my_qsort(0, n1-1, over), my_qsort(0, n2-1, under);
        
        if( abs(P1-P2) <= T )
        {
            if(Y.ind == lastind) last++;
            cnt++;
        }

        for(k = 0, j = 0; j < n1; j++)
        {
            P1 -= over[j].p, P2 += j > 0 ? over[j-1].p : Y.p;
            while(cross(X, under[k], over[j]) == -1 && k < n2)
                P1 += under[k].p, P2 -= under[k].p, k++;
            if( abs(P1-P2) <= T )
            {
                if(over[j].ind == lastind) last++;
                cnt++;
            }
        }

        for(k = P1 = P2 = 0, j = 1; j < n2; j++)
            P1 += under[j].p;
        while( cross(X, over[k], under[0]) == -1 && k < n1 )
            P1 += over[k].p, k++;
        P1 += Y.p, P2 = total-P1-A[i].p-under[0].p;

        if( abs(P1-P2) <= T )
        {
            if(under[0].ind == lastind) last++;
            cnt++;
        }
        for(j = 1; j < n2; j++)
        {
            P1 -= under[j].p, P2 += under[j-1].p;
            while( cross(X, over[k], under[j]) == -1 && k < n1)
                P1 += over[k].p, P2 -= over[k].p, k++;
            if( abs(P1-P2) <= T )
            {
                if(under[j].ind == lastind) last++;
                cnt++;
            }
        }
    }
}

void read_data(void)
{
    int i;

    scanf("%d\n%d\n", &N, &T);

    for(i = 1; i <= N; i++)
        scanf("%d ", &A[i].p), total += A[i].p;

    for(i = 1; i <= N; i++)
        scanf("%d %d\n", &A[i].x, &A[i].y), A[i].ind = i;

    A[0].x = -16001, qsort(A, N+1, sizeof(A[0]), cmp_sort);
    lastind = A[N].ind;
}

void write_data(void)
{
    printf("%d\n", (cnt+last) >> 1);
}

int main(void)
{
    freopen("gold.in", "rt", stdin);
    freopen("gold.out", "wt", stdout);

    read_data();
    solve();
    write_data();

    return 0;
}