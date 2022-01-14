#include <iostream>

int n, sum;
int ans = 0;

int main(){
    std::cin >> n;
    for (int i = 0; i<n; i++){
        sum = i;
        int num = i;
        for (int j = 0; j<7; j++){
            sum += num%10;
            num /= 10;
        }
        if (sum == n){
            ans = i;
            break;
        }
    }
    std::cout << ans;
    
    return 0;
}