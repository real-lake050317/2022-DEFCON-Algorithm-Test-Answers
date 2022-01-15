row = []
board = []
dx = [1, 0, -1, 0]
dy = [0, 1, 0, -1]
cnt = 0
for i in range(100):
    row.append(0)
for i in range(100):
    board.append(row)


T = int(input())
for i in range(T):
    x, y = map(int, input().split())
    for j in range(x, x+10):
        for k in range(y, y+10):
            board[j][k] = 1

for i in range(100):
    for j in range(100):
        if board[i][j] == 1:
            for k in range(4):
                movex = dx[k]
                movey = dy[k]
                if board[i+movex][j+movey] == 0:
                    cnt += 1

print(cnt)