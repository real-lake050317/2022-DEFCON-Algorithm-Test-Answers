#include <iostream>

int N;
int cnt = 0; //빈 변수 N과 0이 대입된 변수 cnt를 선언합니다.

int main(void){
	std::cin >> N; //비어있는 N 변수에 입력값을 대입합니다.
	int num = N; //num 변수를 선언하고, N값을 대입합니다.
	while (true){ 
		cnt++; //무한반복문을 1회 돌때마다 cnt변수의 값을 1 증가시킵니다.
		num = (num%10) * 10 + (num%10+num/10) % 10; //문제에서 제시한 숫자 변환 규칙을 시행합니다.
		if (num == N){ //num변수와 N 변수의 값이 같다면,
			std::cout << cnt; //지금까지 센 cnt 변수를 출력합니다.
			return 0; //0 값을 반환하며 코드를 종료합니다.
		}
	}
}