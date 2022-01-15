package main

import "fmt"
 
func main() {
    var n int
    fmt.Scanf("%d", &n)
    for i := 1; i <= n; i++{
        for j := 0; j<n-i; j++{
            fmt.Print(" ")
        }
        for j := 0; j<2*i-1; j++{
            fmt.Print("*")
        }
        fmt.Println()
    }
}
