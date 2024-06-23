
#include <stdio.h>
#include <string.h>

#define MAXN (1 << 15)

int N, K; char x[MAXN];
int a[16][MAXN], suff[MAXN];
int tmp[1000001];

struct state { int n[2], pos; } E[MAXN], E2[MAXN];

void getfirstlvl()
{
	int i;
	memset(tmp, 0, sizeof(tmp));
	for (i = 0; i < N; i++)
		tmp[ x[i] ] = 1;
	for (i = 1; i < 1000001; i++)
		tmp[i] += tmp[i - 1];
	for (i = 0; i < N; i++)
		a[0][i] = tmp[ x[i] ];
}

int cnt[MAXN];
void countsort(int x)
{
	int i;
	memcpy(E2, E, sizeof(E));
	memset(cnt, 0, sizeof(cnt));
	for (i = 0; i < N; i++) cnt[ E2[i].n[x] ]++;
	for (i = 1; i <= N; i++) cnt[i] += cnt[i - 1];
	for (i = N - 1; i + 1; i--)
	{
		E[ cnt[E2[i].n[x]] - 1 ] = E2[i];
		cnt[E2[i].n[x]]--;
	}
}

void getnxtlvl(int K)
{
	int i, j;
	for (i = 0; i < N; i++)
	{
		E[i].n[0] = a[K][i];
		E[i].n[1] = (i + (1 << K) < N) ? a[K][i + (1 << K)] : 0;
		E[i].pos = i;
	}
	countsort(1);
	countsort(0);
	for (i = 0, j = 1; i < N; i++)
	{
		if (i)
			if (E[i - 1].n[0] != E[i].n[0] || E[i - 1].n[1] != E[i].n[1])
				j++;
		a[K + 1][ E[i].pos ] = j;
	}
}

int getlongestprefix(int i, int j)
{
	int sol = 0, step = 1 << 15, st = 15;
	for (; step; step >>= 1, st--)
		if (i + sol + step - 1 < N && j + sol + step - 1 < N)
			if (a[st][i + sol] == a[st][j + sol])
				sol += step;
	return sol;
}

int main()
{
	freopen("substr.in", "rt", stdin);
	freopen("substr.out", "wt", stdout);
	scanf("%d %d ", &N, &K);
	gets(x);
	int i;

	getfirstlvl();
	for (i = 0; i < 15; i++)
		getnxtlvl(i);
	for (i = 0; i < N; i++)
		suff[ a[15][i] - 1 ] = i;

	int MAX = -1;
	for (i = 0; i + K - 1 < N; i++)
	{
		int j = getlongestprefix(suff[i], suff[i + K - 1]);
		if (j > MAX)
			MAX = j;
	}
	printf("%d\n", MAX);
	return 0;
}