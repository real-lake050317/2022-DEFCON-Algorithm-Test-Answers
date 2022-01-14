n = int(input())
num = n
cnt = 0
while True:
    cnt += 1
    num = (num%10)*10 + (num%10+num//10)%10;
    if num == n:
        print(cnt)
        exit()