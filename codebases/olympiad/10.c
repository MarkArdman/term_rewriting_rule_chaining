#include <stdio.h>

#define MAX_N 52
#define MAX_T 106
#define MAXNODES (128*MAX_N+MAX_T)
#define min(a,b) ((a) < (b) ? (a) : (b))

int N, M, A[MAX_N], G[MAX_N][MAX_N], L[MAX_N][MAX_N];
char C[MAXNODES][MAXNODES];

int src, tmax, sum;
int maxt, Q[MAXNODES], from[MAXNODES], viz[MAXNODES];

int bfs(void)
{
    int i, inc, sf, x, y, t, p, nx, ny, nt, ok = 1, fl;

    for(i = 0; i < MAXNODES; i++)
        viz[i] = 0;
        
    Q[inc = sf = 0] = src, viz[src] = 1, from[src] = -1;

    while(inc <= sf && ok)
    {
        p = Q[inc++];
        x = p>>7, t = p&127;
        for(i = 1; i <= G[x][0] && ok; i++)
        {
            nx = G[x][i], nt = t+1;
            if(nt <= tmax)
            {
                y = (nx<<7)+nt;
                if(C[p][y] > 0 && !viz[y])
                {
                    from[y] = p, viz[y] = 1, Q[++sf] = y;
                    if(nx == 1)
                    {
                        ok = 0;
                        break ;
                    }
                }
            }

            if(ok == 0) break ;
            
            nx = G[x][i], nt = t-1;
            if(nt >= 1)
            {
                y = (nx<<7)+nt;
                if(C[p][y] > 0 && !viz[y])
                    from[y] = p, viz[y] = 1, Q[++sf] = y;
            }
        }
    }

    if(ok == 1) return 0;

    for(fl = 1<<20, x = y; from[x] > -1; x = from[x])
        fl = min(fl, C[from[x]][x]);
    for(x = y; from[x] > -1; x = from[x])
        C[from[x]][x] -= fl, C[x][from[x]] += fl;

    return fl;
}

int solve(void)
{
    int f = 0, p;

    tmax = 1;
    
    while(f < sum)
    {
        while(p = bfs())
            f += p;
        tmax++;
    }

    return tmax-2;
}

void read_data(void)
{
    int i, x, y, t, u, v, c;

    scanf("%d %d\n", &N, &M);

    for(i = 1; i <= N; i++)
        scanf("%d ", &A[i]);

    for(i = 1; i <= M; i++)
        scanf("%d %d %d\n", &u, &v, &c), L[u][v] += c, L[v][u] += c;

    for(u = 1; u <= N; u++)
     for(v = u+1; v <= N; v++)
      if(L[u][v] > 0)
        G[u][++G[u][0]] = v, G[v][++G[v][0]] = u;

    src = 0;

    for(x = 1; x <= N; x++)
        C[src][(x<<7)+1] = A[x], G[0][++G[0][0]] = x, sum += A[x];
        
    for(t = 2; t < MAX_T; t++)
     for(x = 1; x <= N; x++)
     {
        C[(x<<7)+t-1][(x<<7)+t] = 64;
        for(i = 1; i <= G[x][0]; i++)
        {
            y = G[x][i];
            C[(x<<7)+t-1][(y<<7)+t] = L[x][y];
        }
     }

    for(x = 1; x <= N; x++)
        G[x][++G[x][0]] = x;
}

void ruleaza(void)
{
    read_data();
    printf("%d\n", solve());
}

int main(void)
{
    freopen("algola.in", "rt", stdin);
    freopen("algola.out", "wt", stdout);

    ruleaza();

    return 0;
}