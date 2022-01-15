import Foundation

let num = Int(readLine()!)!

var first = 0
var second = 1
var sum = 1

for _ in 0..<num-1 {
    sum = first + second
    first = second
    second = sum
}

print(sum)
