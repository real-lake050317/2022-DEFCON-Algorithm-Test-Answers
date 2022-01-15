package main

import "fmt"
 
func main() {
    var arr = [50]int{0, 1}
    var n int
    fmt.Scanf("%d", &n)
    for i := 2; i<=n; i++ {
        arr[i] = arr[i-1] + arr[i-2]
    }
    fmt.Printf("%d", arr[n])
}
