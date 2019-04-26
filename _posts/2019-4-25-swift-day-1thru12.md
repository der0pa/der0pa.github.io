---
layout: post
author: der0pa
---

100days of swift

```swift

/*
Day 1 summary
*/
var str = "Hello, playground" // vars can ge changed
var str2 = "Goodbye!"

var str3 = """
This goes \
over multiple \
lines
"""

var pi = 3.141
var awesome = true

var score = 85
var str4 = "Your score was \(score)"
var results = "The test results are here: \(str4)"

let taylor = "swift"

let album: String = "Reputation"
let year: Int = 1989
let height: Double = 1.78
let taylorRocks: Bool = true

/*
Day 2 summary
*/

// arrays
let john: String = "John Lennon"
let paul: String = "Paul McCartney"
let george: String = "George Harrison"
let ringo: String = "Ringo Star"

let beatles: [String] = [john, paul, george, ringo]

print(beatles[2])  // -> George Harrison
// Sets   unorder/random/no duplicates
let colors = Set(["red", "green", "blue"])
let colors2 = Set(["red", "green", "blue", "red", "blue"])

print(" colors set: \(colors) \n colors2 set: \(colors2)")

// Tuples fixed size/types and ordered
var name = (first: "Taylor", last: "Swift")

print(name.last, name.0)

// Tuples vs. Sets vs. Arrays most commonly used
// This is a Tuple (Int, String, String, String, Int)
let address = (house: 444
              , street: "Taylor Swift avenue"
              , city: "Nashville"
              , state: "Tennesee"
              , zip: 49355 
              )

// This is a Set:
let set = Set (["aardvard", "astronaut", "azalea"])

// This is an Array:
let pythons: [String] = ["Eric", "Graham", "John", "Michael"]

// This is a Dictionary:  
// Key:Value
let heights = [
     "Taylor Swift": 1.78,
     "Ed Sheeran": 1.73
]
// heights["Taylor Swift"]
// Default value if key in not found
let favoriteIceCream = [
    "Paul": "Chocolate",
    "Sophie": "Vanilla"
]
// favoriteIceCream["Charlotte", default: "Unknown"]
// create an empty Dictionary this way:
var teams = [String: String]() // or:
var teams2 = Dictionary<String, Int>()

// create an empty Array this way:
var results2 = [Int]() // or:
var result3 = Array<Int>()

// create an empty Set this way:
var words = Set<String>()
var nums = Set<Int>()

// Enums  limit codeing to just certain 'cases'
enum Result {
    case success
    case failure
}

let result4 = Result.failure
let result5 = Result.success

// Enums can have assciated values also:
enum Activity {
    case bored
    case running(destination: String)
    case talking(topic: String)
    case singing(volume: Int)
}
let talking = Activity.talking(topic: "football")

// Day Three summary: Operators and Conditionals
// compound assignment operators
var score2 = 95
score2 -= 5
print(score2)

var quote = "The rain in Spain falls on the "
quote += "Spaniards"
print(quote)

let firstScore = 6
let secondScore = 4
let isit = firstScore == secondScore
print(isit)

// Conditionals
let firstCard = 11
let secondCard = 10
let totalHand = firstCard + secondCard

if firstCard + secondCard == 21 {
    print("Blackjack!")
} else {
    print("You have: \(totalHand)")
}

// && 'and'  both conditions must be 'true' -> true
// || 'or'   either condition may be 'true' -> true
let age1 = 12
let age2 = 18

// if age1 > 18 || age2 > 18 {
if age1 > 18 && age2 > 18 {
    print("Both \(age1) and \(age2) are over 18")
} else {
    print("Both \(age1) and \(age2) are not over 18")
}

// Switch / case / pattern matching
let weather = "rain"

switch weather {
    case "rain":
        print("Bring an umbrella")
    case "snow":
        print("Wrap up warm")
    case "sunny":
        print("Wear sunscreen")
        fallthrough
    default:
        print("Enjoy your day!")

}

// Range operations
// 1..<5 contains 1,2,3,4
// 1...5 contains 1,2,3,4,5
// Used in switch statements
let score3 = 85

switch score3 {
    case 0..<50:
        print("You failed badly.")
    case 50..<85:
        print("You did OK.")
    default:
        print("You did GREAT!")
}

// Day 4 summary 'loops'
// switch to colab to finish 4...12 day summary

```
[colab portion 4 thru 12 days of swift](https://colab.research.google.com/drive/105057N9VnY1JSwlQSOSAr0ox7oFvlGyN)