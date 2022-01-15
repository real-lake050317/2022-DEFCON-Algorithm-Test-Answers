package main

import "fmt"
 
func main() {
    var board[101][101] int
    var dx = [4]int{1, 0 , -1, 0}
    var dy = [4]int{0, 1, 0, -1}
    var T, x, y int
    cnt := 0
    
    fmt.Scanf("%d", &T)
    for i := 0; i<T; i++{
        fmt.Scanf("%d %d", &x, &y)
        for j := x; j<x+10; j++{
            for k := y; k<y+10; k++{
                board[j][k] = 1
            }
        }
    }
    for i := 0; i<100; i++{
        for j := 0; j<100; j++{
            if (board[i][j] == 1){
                for k := 0; k<4; k++{
                    movex := dx[k]
                    movey := dy[k]
                    if (board[i+movex][j+movey] == 0){
                        cnt++
                    }
                }
            }
        }
    }
    fmt.Printf("%d", cnt)
}
