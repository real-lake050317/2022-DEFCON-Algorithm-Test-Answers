let scan = readLine()!
let input = Int(scan)!
let inputDigit = String(input).count
var output = input-(9*inputDigit) > 0 ? input-(9*inputDigit) : 1

while output < input {
    var instance = output
    var outputDigitNum = 0
    while instance != 0 {
        outputDigitNum += instance % 10
        instance /= 10
    }
    if output + outputDigitNum == input {
        break
    }
    output += 1
}
print(output < input ? output : "0")
