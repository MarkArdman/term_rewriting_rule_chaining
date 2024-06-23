#include <stdio.h>

int N, nr;
int prim[16], put[16];
int sol[1 << 16][16];

int main()
{
	freopen("divizori.in", "rt", stdin);
	freopen("divizori.out", "wt", stdout);
	scanf("%d", &N);
	nr = 1;
	if (!(N & 1))
	{
		prim[++prim[0]] = 2;
		for (; !(N & 1); N >>= 1)
			put[1]++;
		nr *= put[1] + 1;
	}
	int i;
	for (i = 3; i * i <= N; i += 2)
		if (N % i == 0)
		{
			prim[++prim[0]] = i;
			for (; N % i == 0; N /= i)
				put[ prim[0] ]++;
			nr *= put[ prim[0] ] + 1;
		}
	if (N > 1)
	{
		prim[++prim[0]] = N;
		put[prim[0]] = 1;
		nr <<= 1;
	}
	printf("%d\n", nr);
	int j, k, NR = put[1] + 1;
	for (i = 0; i <= put[1]; i++)
		sol[i][1] = i;
	for (i = 2; i <= prim[0]; i++)
	{
		for (j = NR; j < NR * (put[i] + 1); j++)
			if ((j / NR) % 2 == 0)
			{
				for (k = 1; k < i; k++)
					sol[j][k] = sol[j % NR][k];
				sol[j][i] = j / NR;
			}
			else
			{
				for (k = 1; k < i; k++)
					sol[j][k] = sol[NR - j % NR - 1][k];
				sol[j][i] = j / NR;
			}
		NR *= put[i] + 1;
	}
	for (i = 0; i < NR; i++)
	{
		int M = 1;
		for (k = 1; k <= prim[0]; k++)
			for (j = 0; j < sol[i][k]; j++)
				M *= prim[k];
		if (i != 0) printf(" ");
		printf("%d", M);
	}
	printf("\n");
	return 0;
}