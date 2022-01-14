N = int(input()) # 변수 N을 선언하고, 정수를 입력받습니다.
num = N # num 변수에 N을 대입합니다.
cnt = 0 # count 변수를 선언하고, 초기 값을 0으로 지정합니다.

while True: #무한반복문을 실행합니다.
    cnt += 1 # 반복문을 1회 돌때마다 count 변수에 1을 더합니다.
    num = (num%10) * 10 + (num%10 + num//10) % 10; # 문제에서 제시한 숫자 생성 규칙을 실행합니다.
    if num == N: # num 변수의 값이 N과 같다면,
        print(cnt) # 지금까지 센 count 변수를 출력합니다.
        exit() # if문을 통과하면 코드를 전체 종료합니다.