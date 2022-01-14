N = int(input())
num = N
cnt = 0

while True:
    cnt += 1
    num = (num%10)   * 10 + (num%10 + num//10) % 10;
    if num == N:
        print(cnt)
        exit()