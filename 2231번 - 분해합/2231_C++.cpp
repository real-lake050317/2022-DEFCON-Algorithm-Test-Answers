#include <iostream>
using namespace std;

int N, cnt = 0;

int main(){	
	cin >> N;
	for (int i = 0; i<N; i++){
		int ans = i, num = i;
		while (ans){
			num += ans%10;
			ans /= 10;
		}
		if (num == N){
			cout << i;
			return 0;
		}
 	}
 	cout << "0";
}
