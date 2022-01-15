package main

import (
    "fmt"
    "os"
)
 
func main() {
    var n int
    cnt := 0
    fmt.Scanf("%d", &n)
    num := n
    for {
        cnt++
        num = (num%10) * 10 + (num%10 + num/10) % 10
        if (num == n){
            fmt.Printf("%d", cnt)
            os.Exit(0)
        }
    }
}
