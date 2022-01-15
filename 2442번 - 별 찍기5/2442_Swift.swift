let N = Int(readLine()!)!

for i in 1...N {
    let r1 = String(repeating: " ", count: N - i)
    let r2 = String(repeating: "*", count: 2*i - 1)
    print(r1 + r2)
}
