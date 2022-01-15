package main

import "fmt"
 
func main() {
    var n, sum int
    ans := 0
    fmt.Scanf("%d", &n)
    for i := 0; i<n; i++ {
        sum = i
        num := i
        for j := 0; j<7; j++{
            sum += num%10
            num /= 10
        }
        if (sum == n){
            ans = i
            break
        }
    }
    
    fmt.Printf("%d", ans)
}
