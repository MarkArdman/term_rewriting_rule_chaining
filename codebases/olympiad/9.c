#include <stdio.h>
#include <string.h>

#define MAXN 528

typedef long long llong;

int N, uz[16];
llong B[MAXN][MAXN], S[16], V[16];

int L1, L2, C1, C2;

llong sum(int l1, int c1, int l2, int c2)
{
    return B[l2][c2]-B[l2][c1-1]-B[l1-1][c2]+B[l1-1][c1-1];
}

void baga(void)
{
    int i, j, l1, l2, c1, c2, st, dr, m, r, p;
    int viz[16];
    llong val;

    for(l1 = 1; l1 < N; l1++)
    {
        // caut c1
        st = 1, dr = N-1, r = 0;
        while(st <= dr)
        {
            m = (st+dr) >> 1;
            if( sum(1, 1, l1, m) >= V[1] )
                r = m, dr = m-1;
            else
                st = m+1;
        }
        
        if( r == 0 || sum(1, 1, l1, r) != V[1] )
            continue ;
        c1 = r;

        // caut c2
        st = c1+1, dr = N-1, r = 0;
        while(st <= dr)
        {
            m = (st+dr) >> 1;
            while(st <= dr)
            {
                m = (st+dr) >> 1;
                if( sum(1, c1+1, l1, m) >= V[2] )
                    r = m, dr = m-1;
                else
                    st = m+1;
            }
        }

        if( r == 0 || sum(1, c1+1, l1, r) != V[2] )
            continue ;
        c2 = r;
        
        if( sum(1, c2+1, l1, N) != V[3] )
            continue ;

        // caut l2

        for(i = 1; i <= 9; i++)
         if(!uz[i])
         {
            for(p = 1; p <= 9; p++)
                viz[p] = uz[p];

            viz[i] = 1, val = S[i];
            st = l1+1, dr = N-1, r = 0;
            while(st <= dr)
            {
                m = (st+dr) >> 1;
                if( sum(l1+1, 1, m, c1) >= val )
                    r = m, dr = m-1;
                else
                    st = m+1;
            }
            l2 = r;
            if( r == 0 || sum(l1+1, 1, l2, c1) != val )
                continue ;

            val = sum(l1+1, c1+1, l2, c2);
            for(j = 1; j <= 9; j++)
             if(!viz[j] && S[j] == val) { viz[j] = 1; break ; }

            val = sum(l1+1, c2+1, l2, N);
            for(j = 1; j <= 9; j++)
             if(!viz[j] && S[j] == val) { viz[j] = 1; break ; }

            val = sum(l2+1, 1, N, c1);
            for(j = 1; j <= 9; j++)
             if(!viz[j] && S[j] == val) { viz[j] = 1; break ; }

            val = sum(l2+1, c1+1, N, c2);
            for(j = 1; j <= 9; j++)
             if(!viz[j] && S[j] == val) { viz[j] = 1; break ; }

            val = sum(l2+1, c2+1, N, N);
            for(j = 1; j <= 9; j++)
             if(!viz[j] && S[j] == val) { viz[j] = 1; break ; }

            for(j = 1; j <= 9; j++)
             if(viz[j] == 0)
                break ;

            if(j == 10) // am solutie
            {
                if((l1 < L1) ||
                (l1 == L1 && c1 < C1) ||
                (l1 == L1 && c1 == C1 && l2 < L2) ||
                (l1 == L1 && c1 == C1 && l2 == L2 && c2 < C2)
                )
                L1 = l1, L2 = l2, C1 = c1, C2 = c2;
            }
         }
    }
}

void solve(void)
{
    int i, j, k, p;

    L1 = 1024;
    
    for(i = 1; i <= 9; i++)
     for(j = 1; j <= 9; j++)
      for(k = 1; k <= 9; k++)
       if(i != j && i != k && j != k)
       {
            for(p = 1; p <= 9; p++)
                uz[p] = 0;
            V[1] = S[i], V[2] = S[j], V[3] = S[k];
            uz[i] = uz[j] = uz[k] = 1;
            baga();
       }
}


void read_data(void)
{
    int i, j, x;
    
    scanf("%d\n", &N);

    for(i = 1; i <= 9; i++)
        scanf("%lld ", &S[i]);


    for(i = 1; i <= N; i++)
     for(j = 1; j <= N; j++)
     {
        scanf("%d ", &x);
        B[i][j] = (llong)x+B[i-1][j]+B[i][j-1]-B[i-1][j-1];
     }
}

void write_data(void)
{
    printf("%d %d %d %d\n", L1, L2, C1, C2);
}

int main(void)
{
    freopen("zone.in", "rt", stdin);
    freopen("zone.out", "wt", stdout);

    read_data();
    solve();
    write_data();

    return 0;
}