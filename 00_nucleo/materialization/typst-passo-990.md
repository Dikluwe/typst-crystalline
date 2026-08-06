# Passo 990 — três achados: fração pequena com sobreposição, espaço ausente após `−` em expoente, itálico ausente em `a+b` final

**Precede este passo**: três achados independentes da auditoria (§8.5, §8.6, §8.1, 2026-08-06),
agrupados por serem todos pontuais e possivelmente rápidos — separar em passo próprio se qualquer
um revelar causa mais profunda.

---

## Parte A — espaçamento de fração pequena com sobreposição real (seção 11)

**Achado, o mais preocupante dos três**: gap acima/abaixo da barra de fração, seção 11 (física).
Vanilla consistente (~2.56pt acima / ~1.43pt abaixo). Cristalino varia de -0.79pt a 0.95pt acima,
-0.6pt a 3.12pt abaixo — **valores negativos = sobreposição real de caixa**, confirmada por
render (`ρ/ε₀`: "ρ" quase encosta no traço).

### Fase A
1. Reproduzir isoladamente as frações da seção 11 (conteúdo com símbolos gregos/subscritos,
   diferente das frações já testadas noutros passos) — confirmar se o gap varia por causa do tipo
   de conteúdo (símbolo grego com descendente, subscrito) não coberto pelas correções anteriores
   de `frac.rs` (P905/944/952/972).
2. Confirmar se a causa é extents de tinta incorretos para estes símbolos específicos (mesma
   família de erro já vista várias vezes nesta frente — P945/952/957) ou uma fórmula diferente.

### Fase B
TDD directo ou dois agentes conforme a causa revelar.

## Parte B — espaço ausente após `−` unário em expoente negativo

**Achado**: `𝑒⁻ᵗ²`, largura do traço "−" idêntica (5.99pt), mas espaço até a variável seguinte:
cristalino 0.0pt, vanilla 0.77pt.

### Fase A
1. Confirmar se isto é o mesmo mecanismo de "item espaçado" já corrigido em P903/907/967 (fallback
   de espaçamento quando um dos lados é `Text`/sinal), aplicado aqui a um contexto (expoente) que
   essas correções não cobriram.

### Fase B
TDD directo, aplicando o mesmo mecanismo já validado, ao contexto de expoente.

## Parte C — `a+b` final da seção 10 sem itálico (glifo reto em vez de itálico matemático)

**Achado**: últimas linhas de "a+b" na seção 10 — cristalino usa `a`/`b` retos, vanilla usa
`𝑎`/`𝑏` itálicos. Mesmo arquivo de fonte.

### Fase A
1. Confirmar o contexto exato deste "a+b" no documento de teste — é conteúdo dentro de um dos
   construtos de decorador já tocados nesta rodada (`⎵_⎵`, Parte A de P988)? Se sim, pode ser o
   mesmo tipo de lacuna já visto em P966 (conteúdo de função de utilizador/construto manual sem
   `apply_math_default`) — confirmar antes de tratar como achado novo e não relacionado.
2. Se for related a P966: aplicar o mesmo mecanismo. Se for causa nova: investigar como tal.

### Fase B
Conforme a causa confirmada na Fase A.

## Fase C — Revalidação conjunta

1. Medir os três achados de novo no documento de 30 secções.
2. Confirmação visual.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Fração da seção 11 sem sobreposição, gaps próximos do vanilla.
- Espaço após `−` em expoente presente.
- `a+b` final com itálico matemático correto.
- Benchmark sem regressão.
