#let counter = state("counter", 0)
#let increment() = counter.update(c => c + 1)
#let reset() = counter.update(_ => 0)
#let get-counter() = counter.get()

#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
#let factorial(n) = if n <= 1 { 1 } else { n * factorial(n - 1) }
#let gcd(a, b) = if b == 0 { a } else { gcd(b, calc.rem(a, b)) }

#let data = (
  (name: "Alice", age: 30, score: 85.5),
  (name: "Bob", age: 25, score: 92.0),
  (name: "Carol", age: 35, score: 78.25),
  (name: "David", age: 28, score: 88.75),
  (name: "Eve", age: 22, score: 95.5),
)

#let format-person(p) = [
  #p.name: #p.age anos, pontuação #p.score
]

#for person in data {
  format-person(person)
}

#let square(x) = x * x
#let cube(x) = x * x * x
#let pow(base, exp) = if exp == 0 { 1 } else { base * pow(base, exp - 1) }

#let numbers = range(1, 101)
#let evens = numbers.filter(n => calc.rem(n, 2) == 0)
#let odds = numbers.filter(n => calc.rem(n, 2) == 1)
#let sum = numbers.sum()
#let mean = sum / numbers.len()

#let mapper(f, xs) = xs.map(f)
#let filterer(p, xs) = xs.filter(p)
#let reducer(acc, f, xs) = xs.fold(acc, f)

#let result = mapper(square, numbers)
#let big = filterer(n => n > 50, numbers)
#let total = reducer(0, (a, b) => a + b, numbers)

#let closures = (
  () => 1,
  () => 2,
  () => 3,
  () => 4,
  () => 5,
)

#for c in closures {
  c()
}

#let nested(a, b) = {
  let inner(x, y) = x * y + a - b
  inner(a + b, a - b)
}

#for i in range(0, 50) {
  increment()
  let value = nested(i, i + 1)
  value
}

#get-counter()

#let compute-stats(xs) = {
  let n = xs.len()
  let mean = xs.sum() / n
  let variance = xs.map(x => calc.pow(x - mean, 2)).sum() / n
  let stddev = calc.sqrt(variance)
  (mean: mean, variance: variance, stddev: stddev)
}

#let dataset = range(1, 51)
#let stats = compute-stats(dataset)
#stats.mean
#stats.stddev

#let memoize(f) = {
  let cache = state("cache", (:))
  (x) => {
    let key = repr(x)
    let cached = cache.get().at(key, default: none)
    if cached != none {
      cached
    } else {
      let result = f(x)
      cache.update(c => { c.insert(key, result); c })
      result
    }
  }
}

#let memo-fib = memoize(fib)
#memo-fib(10)
#memo-fib(15)
#memo-fib(20)

#let grid-map(rows, cols, f) = {
  let result = ()
  for r in range(rows) {
    let row = ()
    for c in range(cols) {
      row.push(f(r, c))
    }
    result.push(row)
  }
  result
}

#let matrix = grid-map(5, 5, (r, c) => r * c)
#matrix

#let curry(f) = (a) => (b) => f(a, b)
#let add = (a, b) => a + b
#let add5 = curry(add)(5)
#add5(10)

#let pipe(value, ..fs) = {
  let result = value
  for f in fs.pos() {
    result = f(result)
  }
  result
}

#pipe(5, add5, square, (x) => x - 1)

#let validate-user(user) = {
  if user.name == none {
    return (valid: false, error: "nome ausente")
  }
  if user.age < 0 {
    return (valid: false, error: "idade inválida")
  }
  (valid: true, user: user)
}

#for i in range(0, 30) {
  validate-user((name: "user" + str(i), age: i))
}

#let counter2 = state("c2", 0)
#counter2.update(c => c + 100)
#counter2.get()

#let tree-map(t, f) = {
  if type(t) == "dictionary" and t.at("children", default: none) != none {
    let new-children = t.children.map(c => tree-map(c, f))
    f((:..t, children: new-children))
  } else {
    f(t)
  }
}

#let sample-tree = (
  value: 1,
  children: (
    (value: 2, children: ()),
    (value: 3, children: ((value: 4, children: ()),)),
  ),
)

#tree-map(sample-tree, n => (value: n.value * 2, children: n.children))

#let throttle(f, limit) = {
  let count = state("throttle", 0)
  (..args) => {
    let c = count.get()
    if c < limit {
      count.update(c + 1)
      f(..args)
    }
  }
}

#let limited-print = throttle((x) => x, 5)
#for i in range(0, 20) {
  limited-print(i)
}

#let compose(f, g) = (x) => f(g(x))
#let double = (x) => x * 2
#let inc = (x) => x + 1
#compose(double, inc)(5)

#let flatten(xs) = xs.fold((), (acc, x) => if type(x) == "array" { acc + flatten(x) } else { acc + (x,) })
#flatten((1, (2, 3), (4, (5, 6))))
