#include <stdio.h>
#include <string.h>

#define MAX_N 202
#define MAX_K 101
#define MOD 30103

int N, K, A, B, X[MAX_N];
int cnt[2][10][MAX_N][MAX_K];
int res;

int mod[1<<14];
int Mod_x[10][9];
int Mod_p[10];

void preproc(void)
{
    int i, j, k;
    for(i = 0; i < (1<<14); i++)
        mod[i] = i % K;
    for(i = 1; i <= 9; i++)
    {
        Mod_x[i][0] = i % K;
        for(j = i, k = 1; k <= 8; k++)
            j = j*10, Mod_x[i][k] = j % K;
    }

    for(Mod_p[0] = 1, i = 1, j = 10; i <= 8; i++)
        Mod_p[i] = j % K, j *= 10;
}

int getrest(int x, int z)
{
    int i, t, c, r;

    if(x == 0) return 0;
    
    if(z <= 8)
    {
        return Mod_x[x][z];
    }
    
    r = Mod_x[x][8];

    t = (z-8)>>3, c = (z-8)&7;

    for(i = 1; i <= t; i++)
        r = r*Mod_p[8], r = mod[r];

    r = r*Mod_p[c], r = mod[r];

    return r;
}

void solve(void)
{
    int u, v, i, j, r, c, t, rp, rnou;

    u = 0, v = 1;

    preproc();
    
    for(i = N; i >= 1; i--)
    {
        for(c = 0; c <= 9; c++)
        {
            for(j = 1; j <= (N-i+1); j++)
             for(r = 0; r < K; r++)
              if(c == X[i])
              {
                if(j == 1)
                {
                    if(r == c%K)
                        cnt[v][c][j][r] = 1;
                    continue ;
                }
                else
                {
                    rp = getrest(X[i], j-1), rnou = mod[r-rp+K];
                    cnt[v][c][j][r] = 0;
                    for(t = 0; t <= 9; t++)
                    {
                        cnt[v][c][j][r] += cnt[u][t][j-1][rnou];
                        if(cnt[v][c][j][r] >= MOD)
                            cnt[v][c][j][r] -= MOD;
                    }
                }
              }
              else
              {
                    cnt[v][c][j][r] = cnt[u][c][j][r];
              }
        }
        u ^= 1, v ^= 1;
    }

    for(c = 1; c <= 9; c++)
     for(j = A; j <= B; j++)
     {
        res += cnt[u][c][j][0];
        if(res >= MOD)
            res -= MOD;
     }
}

void read_data(void)
{
    int i;
    char sir[1024];

    scanf("%d %d %d\n", &K, &A, &B);

    scanf("%s\n", &sir), N = strlen(sir);
    for(i = 1; i <= N; i++)
        X[i] = sir[i-1]-48;
}

void write_data(void)
{
    printf("%d\n", res);
}

int main(void)
{
    freopen("diviz.in", "rt", stdin);
    freopen("diviz.out", "wt", stdout);

    read_data();
    solve();
    write_data();

    return 0;
}