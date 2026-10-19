
// === b2_text.typ ===
= Lorem Ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.

Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?

At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga.

Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae.

Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

= Outra Secção

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

- Item um da lista
- Item dois da lista
- Item três da lista com *ênfase* e **forte**

#quote(attribution: "Autor Desconhecido")[
  Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
  eiusmod tempor incididunt ut labore et dolore magna aliqua.
]

#figure(
  caption: [Uma figura exemplo.],
  [Conteúdo da figura.],
)

#table(
  columns: (1fr, 1fr, 1fr),
  [A], [B], [C],
  [1], [2], [3],
  [x], [y], [z],
)


= Secção Adicional

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent commodo cursus magna, vel scelerisque nisl consectetur et. Nullam quis risus eget urna mollis ornare vel eu leo. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec id elit non mi porta gravida at eget metus.

Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Maecenas faucibus mollis interdum. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Cras mattis consectetur purus sit amet fermentum.

#heading[Outro Título]

Sed posuere consectetur est at lobortis. Aenean lacinia bibendum nulla sed consectetur. Donec ullamcorper nulla non metus auctor fringilla. Vestibulum id ligula porta felis euismod semper.

#set text(font: "Linux Libertine", size: 11pt)
#set par(justify: true, leading: 0.65em)

Este parágrafo usa configuração de texto e parágrafo. O justify activo força o lexer a processar palavras, espaços e pontuação de forma mais variada. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

#align(center)[
  *Texto centrado* com _ênfase_ e `código inline`.
]

#enum(
  [Primeiro item numerado],
  [Segundo item numerado],
  [Terceiro item numerado],
)

#terms[
  / Termo A: Definição do termo A com algum texto explicativo.
  / Termo B: Definição do termo B com mais texto explicativo.
  / Termo C: Definição do termo C.
]

#align(right)[
  #link("https://example.com")[Um link de exemplo]
]

= Conclusão

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

// === b3_math.typ ===
Math denso: $x$, $y$, $z$, $alpha$, $beta$, $gamma$, $sum_(i=0)^n i^2$, $integral_0^1 f(x) dif x$, $lim_(x -> 0) sin(x)/x$.

$ a x^2 + b x + c = 0 quad "com" quad x = (-b plus.minus sqrt(b^2 - 4 a c)) / (2 a) $

$ vec(a, b, c) dot vec(x, y, z) = a x + b y + c z $

$ sum_(n=1)^oo 1/n^2 = pi^2/6 $

$ integral_(-oo)^oo e^(-x^2) dif x = sqrt(pi) $

$ "let" f(x) = cases(x^2 "se" x >= 0, -x "se" x < 0) $

$ frac(partial f, partial x) + frac(partial f, partial y) = nabla f $

$ underbrace(a + b + c, "soma") = overbrace(d + e, "total") $

$ x_1, x_2, dots, x_n in RR "tal que" norm(x) < epsilon $

$ bigcup_(i in I) A_i subseteq bigcap_(j in J) B_j $

$ forall epsilon > 0 quad exists delta > 0 quad abs(x - a) < delta => abs(f(x) - L) < epsilon $

$ mat(1, 2, 3; 4, 5, 6; 7, 8, 9) vec(x, y, z) = vec(1, 2, 3) $

$ sqrt(x^2 + y^2), root(3, x), x!, n choose k, angle.l u, v angle.r $

$ script(A), script(B), frak(C), bb(D), tt(E), sf(F) $

$ a/(b+c) + d/(e+f) = (a(e+f) + d(b+c))/((b+c)(e+f)) $

$ floor(x), ceil(x), abs(x), norm(v), hat(i), bar(x), tilde(y) $

$ sum product integral union intersection $

$ sin cos tan arcsin arccos arctan sinh cosh tanh log ln exp $

$ alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi pi rho sigma tau upsilon phi chi psi omega $

$ Alpha Beta Gamma Delta Epsilon Zeta Eta Theta Iota Kappa Lambda Mu Nu Xi Pi Rho Sigma Tau Upsilon Phi Chi Psi Omega $


Mais equações:

$ "Bernoulli": p(x) = sum_(k=0)^n (n "choose" k) x^k (1-x)^(n-k) $

$ "Taylor": f(x) = sum_(n=0)^oo (f^((n))(a))/(n!) (x-a)^n $

$ "Fourier": f(x) = a_0/2 + sum_(n=1)^oo (a_n cos(n x) + b_n sin(n x)) $

$ "Euler": e^(i pi) + 1 = 0 $

$ "Cauchy-Schwarz": angle.l u, v angle.r^2 <= norm(u)^2 norm(v)^2 $

$ "Gaussian": integral_(-oo)^oo e^(-x^2/2) dif x = sqrt(2 pi) $

$ "Gamma": Gamma(z) = integral_0^oo t^(z-1) e^(-t) dif t $

$ "Beta": B(x, y) = integral_0^1 t^(x-1) (1-t)^(y-1) dif t $

$ "Laplace": cal(L){f(t)} = integral_0^oo e^(-s t) f(t) dif t $

$ "Delta": delta(x) = { 0 "se" x != 0, oo "se" x = 0 } $

$ "Maxwell": nabla dot E = rho/epsilon_0, quad nabla times B = mu_0 J + mu_0 epsilon_0 (partial E)/(partial t) $

$ "Schrödinger": i hbar (partial Psi)/(partial t) = hat(H) Psi $

$ "Einstein": E = m c^2 $

$ "Planck": E = h nu $

$ "De Broglie": lambda = h/p $

$ "Riemann": zeta(s) = sum_(n=1)^oo 1/n^s $

$ "Möbius": mu(n) = cases(0 "se" n "quadrado", (-1)^k "se" n "produto de" k "primos distintos") $

// === b4_code.typ ===
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

// === b5_utf8.typ ===
UTF-8 multibyte: 🎉 🚀 💯 🔥 🌟 ✨ 💻 📊 📝 ✅ ❌ ⚠️ ⭐ 🎨 🎭

Japonês: こんにちは世界。これはベンチマーク用のテキストです。日本語の文字は複数バイトで構成されており、スキャナーの境界処理をテストします。

Chinês: 你好，世界。这是一个用于基准测试的文本。中文字符由多个字节组成，用于测试扫描器的边界处理。

Coreano: 안녕하세요 세계입니다. 이것은 벤치마크용 텍스트입니다. 한글 문자는 여러 바이트로 구성됩니다.

Emoji e acentuação misturada: café ☕, naïve résumé, piñata señor, 日本のコーヒー 🍵, 中国茶 🍵, 한국 커피 ☕.

Matemática com unicode: ∀x ∈ ℝ, ∃y ∈ ℝ : y = x² + 1. ∑ₙ₌₁^∞ 1/n² = π²/6. ∫₀^∞ e⁻ˣ dx = 1.

Símbolos diversos: ← ↑ → ↓ ↔ ↕ ⇐ ⇑ ⇒ ⇓ ⇔ ⇕ ≠ ≤ ≥ ± × ÷ ∞ ∂ √ ≈ ≔ ∈ ∉ ⊂ ⊆ ∪ ∩ ∅ ∧ ∨ ¬ ∀ ∃ ∴ ∵

Setas e geometria: ▲ ▼ ◀ ▶ ◆ ◇ ● ○ ■ □ ★ ☆ ♠ ♥ ♦ ♣

Letras acentuadas: à á â ã ä å æ ç è é ê ë ì í î ï ñ ò ó ô õ ö ø ù ú û ü ý þ ÿ

Grego: α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω Α Β Γ Δ Ε Ζ Η Θ Ι Κ Λ Μ Ν Ξ Ο Π Ρ Σ Τ Υ Φ Χ Ψ Ω

Cirílico: абвгдеёжзийклмнопрстуфхцчшщъыьэюя АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ

Emoji sequências: 👨‍👩‍👧‍👦 🏳️‍🌈 🎅🏽 👩🏻‍💻 🏃🏿‍♀️

Final com caracteres de controle visuais: 	 tabulado 
 newline e retorno 
.
