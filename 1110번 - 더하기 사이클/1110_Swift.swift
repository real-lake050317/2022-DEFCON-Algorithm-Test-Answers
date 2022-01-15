import Foundation

while let input = readLine() {
    let userInput = Int(input)!
    var cycle = 0
    var newNumber = 0
    newNumber = userInput
    
    while true {
        let a = newNumber / 10
        let b = newNumber % 10
        let c = a + b
        newNumber = (b*10) + (c%10)
        
        cycle += 1
        if newNumber == userInput { break }
    }
    print(cycle)
}
