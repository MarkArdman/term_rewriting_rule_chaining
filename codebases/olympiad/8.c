#include <stdio.h>

#define MAXN 626
#define INF (1 << 30)
#define abs(x) ((x) < 0 ? (-(x)) : (x))

int N, K, A, B, res, V[MAXN], SOL[MAXN], cost[MAXN][MAXN];
char C[MAXN][MAXN];
int src, dest;

int d[MAXN], from[MAXN];

void bfs(void)
{
    int i, j, k, ok = 1, x;

    for(i = src; i <= dest; i++)
        from[i] = -1, d[i] = INF;

    for(d[src] = 0, k = src; k <= dest && ok; k++)
    {
        ok = 0;
        for(i = src; i <= dest; i++)
         for(j = src; j <= dest; j++)
          if(C[i][j] > 0 && (d[i] != INF && d[j] > d[i]+cost[i][j]))
            d[j] = d[i]+cost[i][j], from[j] = i, ok = 1;
    }
    
    for(x = dest; from[x] > -1; x = from[x])
        C[from[x]][x]--, C[x][from[x]]++;
}

void solve(void)
{
    int i, j, k, t;

    src = 0, dest = N+B-A+2;

    for(i = 1; i <= N; i++)
        C[src][i] = 1;

    for(i = 1; i <= N; i++)
     for(k = N+1, j = A; j <= B; j++, k++)
        C[i][k] = 1, cost[i][k] = abs(j-V[i]), cost[k][i] = (-1)*cost[i][k];

    for(k = N+1, j = A; j <= B; j++, k++)
        C[k][dest] = 1;

    for(t = 0, i = 1; i <= N; i++)
     for(k = N+1, j = A; j <= B; k++, j++)
      if(V[i] == j && C[k][dest] > 0)
      {
            t++;
            C[src][i] = 0, C[i][src] = 1;
            C[i][k] = 0, C[k][i] = 1, C[k][dest] = 0, C[dest][k] = 1;
            break ;
      }
      
    for(i = t+1; i <= K; i++)
        bfs();

    for(i = 1; i <= N; i++)
        SOL[i] = V[i];

    for(i = 1; i <= N; i++)
     for(k = N+1, j = A; j <= B; j++, k++)
      if(C[i][k] == 0)
        SOL[i] = j, res += cost[i][k];
}

void read_data(void)
{
    int i;

    scanf("%d %d %d %d\n", &N, &K, &A, &B);

    for(i = 1; i <= N; i++)
        scanf("%d ", &V[i]);
}

void write_data(void)
{
    int i;

    printf("%d\n", res);
    for(i = 1; i < N; i++)
        printf("%d ", SOL[i]);
    printf("%d\n", SOL[N]);
}

int main(void)
{
    freopen("smen.in", "rt", stdin);
    freopen("smen.out", "wt", stdout);

    read_data();
    solve();
    write_data();

    return 0;
}