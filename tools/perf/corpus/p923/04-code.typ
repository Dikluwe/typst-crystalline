#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
#let factorial(n) = if n <= 1 { 1 } else { n * factorial(n - 1) }
#let numbers = range(1, 101)
#let squares = numbers.map(n => n * n)
#let sum = numbers.sum()
#fib(10)
#factorial(5)
#sum
