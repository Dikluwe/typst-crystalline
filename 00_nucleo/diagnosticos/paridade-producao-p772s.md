# P772s — Span por argumento em chamadas de funções nativas

> **Passo:** 772s
> **Data:** 2026-07-17
> **Commit-base:** `61b7edee7` (HEAD no início do passo).
> **Dependência:** P772p (achado original, commit `56bbaa7a4f2ef35ee0aa970f72bde8021223f552`).

---

## 1. Sonda — alcance real do problema

### 1.1 Pontos com `Span::detached()` em funções nativas (produção, exclui `#[cfg(test)]`)

```bash
for f in 01_core/src/rules/stdlib/*.rs; do
  awk '/#\[cfg\(test\)\]/{intest=1} /Span::detached\(\)/{if(!intest) print FILENAME":"NR}' "$f"
done | wc -l
```

**520** pontos de produção (contagem por linha, excluindo `mod tests`). Um
grep simples e não-filtrado (`grep -rn "Span::detached()" *.rs`) dá 520
igualmente antes das alterações deste passo — o total de 535 citado num
rascunho anterior desta sessão incluía os 16 sites de teste (não-produção)
adicionados durante a própria implementação (§2.3), não pontos de produção
adicionais.

### 1.2 Estrutura actual de `Args` e ponto onde o span do AST se perde

```bash
grep -n "struct Args\|pub fn.*positional\|pub fn.*named" 01_core/src/entities/args.rs
```

Antes deste passo, `Args` (`01_core/src/entities/args.rs`) tinha apenas
`items: Vec<Value>` e `named: IndexMap<EcoString, Value, FxBuildHasher>` —
**nenhum campo de span**. O ponto de "achatamento" é `eval_args()`
(`01_core/src/rules/eval/closures.rs`): recebe `args_node: &ast::Args`
(que tem `.span()`, o span de `(...)` completo da chamada) e produz
`Args { items, named }`, descartando esse span. A partir daí, qualquer
função nativa que precise de reportar erro de validação de argumento
não tem span nenhum para usar — só `Span::detached()`.

### 1.3 Mecanismo do vanilla

```bash
grep -n "struct Args\|struct Arg\b\|pub span" \
  lab/typst-original/crates/typst-library/src/foundations/args.rs
```

`foundations/args.rs:53-60`:

```rust
pub struct Args {
    /// The callsite span for the function. This is not the span of the
    /// argument list itself, but of the whole function call.
    pub span: Span,
    pub items: EcoVec<Arg>,
}
```

`foundations/args.rs:513-520`:

```rust
pub struct Arg {
    /// The span of the whole argument.
    pub span: Span,
    pub name: Option<Str>,
    pub value: Spanned<Value>,
}
```

O vanilla guarda **dois** níveis de span: `Args.span` (span da chamada
inteira, não da lista de argumentos — comentário explícito no código-fonte)
e `Arg.span` **por argumento individual**, dentro de uma lista unificada
`items: EcoVec<Arg>` (posicionais e nomeados juntos, cada um com o seu
span, nome opcional e valor). O cristalino separa `items: Vec<Value>` de
`named: IndexMap<...>` (decisão de Passo 17, não revista aqui) — replicar
span por-argumento exigiria ou (a) unificar `items`/`named` numa lista
`Vec<Arg>`-like como o vanilla, ou (b) adicionar `Vec<Span>` paralelo a
`items` mais `IndexMap<EcoString, (Value, Span)>` para `named` — ambas
mudanças estruturais que tocam todo ponto de construção e leitura de
`Args` em toda a `stdlib` e `eval`.

### 1.4 Custo medido de cada opção

| Opção | Pontos afectados (medidos) |
|---|---|
| Span por-argumento (paridade completa com vanilla `Arg.span`) | ~1200 pontos de chamada (construção + leitura de cada argumento individualmente em toda a stdlib) — medição por amostragem de `args.items[N]`/`args.named.get` em `01_core/src/rules/stdlib/*.rs`, consistente com a estimativa já registada na era P740C e não revista desde então |
| Span por-chamada (`Args.span`, réplica do campo `Args.span` do vanilla) | 4 pontos de construção real (`eval_args`, `merge_with_args`, sink `..rest`, `math.rs`) + 16 pontos de teste (`Args { .. }` directos em `#[cfg(test)]`) + 194 de 520 pontos de erro elegíveis para usar `args.span` em vez de `Span::detached()` sem mudança de assinatura |

A opção "span por-chamada" é uma aproximação **parcial** ao vanilla — não
é span por-argumento, é o mesmo compromisso que o próprio `Args.span` do
vanilla já assume textualmente ("this is not the span of the argument
list itself, but of the whole function call" — mas o cristalino usa o
span da **lista de argumentos**, não da chamada inteira incluindo o nome
da função; uma aproximação diferente da do vanilla, mais próxima do
argumento problemático na prática, mas não idêntica ao mecanismo vanilla).

### 1.5 Elegibilidade dos 520 pontos de produção para a correção por-chamada

Script (`/tmp/scan_args_scope2.py`, análise de profundidade de chaves por
função) que determina, para cada ocorrência de `Span::detached()`, se a
função nativa envolvente tem `args: &Args` na própria assinatura:

```
TOTAL eligible=194 ineligible=326
```

`194` pontos estão em funções cuja assinatura já inclui `args: &Args`
directamente (correcção mecânica: `Span::detached()` → `args.span`, sem
mudança de assinatura). `326` estão em funções auxiliares que não recebem
`Args` directamente (tomam valores já extraídos/tipados) — corrigir esses
exigiria threading do span através de novos parâmetros em ~243 funções
auxiliares, mudança estrutural maior, fora do âmbito medido como "pequeno
e concentrado" deste passo.

Por ficheiro (só produção, `#[cfg(test)]` excluído):

```
assert.rs        eligible=6   ineligible=2
calc.rs           eligible=10  ineligible=1
collections.rs    eligible=0   ineligible=56
color.rs          eligible=0   ineligible=6
counter.rs        eligible=0   ineligible=3
eval.rs           eligible=0   ineligible=3
figure_image.rs   eligible=8   ineligible=0
foundations.rs    eligible=9   ineligible=21
gradients.rs      eligible=2   ineligible=23
label.rs          eligible=0   ineligible=4
layout.rs         eligible=79  ineligible=20
loading.rs        eligible=0   ineligible=1
math_style.rs     eligible=0   ineligible=4
mod.rs            eligible=0   ineligible=2
panic.rs          eligible=3   ineligible=0
pdf.rs            eligible=0   ineligible=1
plugin.rs         eligible=0   ineligible=1
primitives_constructors.rs eligible=0 ineligible=8
ref.rs            eligible=0   ineligible=4
shapes.rs         eligible=27  ineligible=12
structural.rs     eligible=0   ineligible=145
text.rs           eligible=42  ineligible=0
transforms.rs     eligible=8   ineligible=0
visualize.rs      eligible=0   ineligible=9
```

---

## 2. Decisão (regra 1)

O grep bruto (520 pontos de produção, disperso por 24 ficheiros) cai no
bucket "grande e disperso" da tabela de decisão do passo — a correcção
directa e completa **não** é justificável dentro deste passo (exigiria
threading de span através de ~243 funções auxiliares, mudança estrutural
maior que o âmbito "M/L" declarado no cabeçalho).

Decisão registada:

1. **Implementar a mudança de ABI** — adicionar `span: Span` a `Args`
   (réplica parcial do `Args.span` do vanilla, usando o span da lista de
   argumentos em vez do span da chamada inteira — ver §1.4). Custo medido:
   4 pontos de construção real + 16 pontos de teste. **Feito** (§2.3).
2. **Corrigir os 194 pontos elegíveis** (funções com `args: &Args` já na
   assinatura) — correcção puramente mecânica (`Span::detached()` →
   `args.span`), sem mudança de assinatura, sem risco de regressão de
   tipo. **Feito** (§2.4), incluindo o caso motivador original de P772p
   (`native_image`/`figure_image.rs`).
3. **Registar como débito técnico priorizado** os 326 pontos restantes
   (243 funções auxiliares sem `Args` na assinatura) — corrigir estes
   exige threading de span por parâmetro adicional em cada função
   auxiliar, uma mudança estrutural maior. **Não implementado aqui.**
4. **Span por-argumento individual** (paridade completa com vanilla
   `Arg.span`, ~1200 pontos) — mantido como débito técnico priorizado,
   consistente com a estimativa já registada na era P740C.

---

## 3. Implementação

### 3.1 L0 actualizado antes do código

`00_nucleo/prompts/entities/args.md` — novo campo `span: Span` na struct,
com racional completo (span da lista de argumentos, não da chamada
inteira; `Span::detached()` para `Args` sintéticos; paridade parcial
com o vanilla, span por-argumento registado como débito). Novo critério
de verificação (`eval_args(...) numa chamada real → span = span da lista
de argumentos`). Entrada em "Histórico de Revisões".

### 3.2 Mudança de ABI — `Args.span`

`01_core/src/entities/args.rs`: campo `pub span: Span` adicionado à
struct. `Args::positional()` usa `Span::detached()` (construções
sintéticas/internas sem chamada real — spread, `.with()`, testes).

### 3.3 Pontos de construção real corrigidos (4)

- `eval_args()` (`01_core/src/rules/eval/closures.rs`) — ponto principal:
  `args_node.span()` (span de `(...)` da chamada real no AST).
- `merge_with_args()` (mesma ficheiro) — usa o span da chamada final
  (`new.span`, capturado antes de `new` ser consumida).
- Sink de argumentos (`..rest`) dentro de `apply_closure` — usa
  `args.span` (span da chamada corrente).
- `math.rs` (chamadas de função em modo matemático) — `call.args().span()`.

### 3.4 Correcção mecânica dos 194 pontos elegíveis

Script (`/tmp/scan_args_scope2.py`) identificou as 194 linhas exactas
(função com `args: &Args` na assinatura) e aplicou substituição
`Span::detached()` → `args.span` **apenas nessas linhas** (não um
find-replace cego por ficheiro — 326 outras ocorrências nos mesmos
ficheiros foram deliberadamente preservadas por não terem `args` em
escopo directo). Por ficheiro: `assert.rs` 6, `calc.rs` 10,
`figure_image.rs` 8, `foundations.rs` 9, `gradients.rs` 2, `layout.rs`
79, `panic.rs` 3, `shapes.rs` 27, `text.rs` 42, `transforms.rs` 8 — total
194.

Caso motivador de P772p (`native_image`, `figure_image.rs`): os 8 pontos
de erro da função (named arg inesperado, path ausente/inválido, leitura
de ficheiro falhada, SVG não suportado, formato desconhecido, fit
inválido) passaram todos de `Span::detached()` para `args.span`.

### 3.5 Testes — 16 pontos de construção directa de `Args {}` em `#[cfg(test)]`

Adicionar o campo obrigatório quebrou compilação de 16 construções
directas de `Args { items, named }` (sem usar `Args::positional()`) em
`#[cfg(test)]`, previamente não contadas no grep de produção (§1.1
exclui-as por desenho). Todas corrigidas com `span: Span::detached()`
(construções sintéticas de teste, sem chamada real correspondente no
documento) — `01_core/src/rules/eval/closures.rs` (1),
`01_core/src/rules/stdlib/structural.rs` (13),
`01_core/src/rules/stdlib/collections.rs` (1),
`01_core/src/rules/stdlib/mod.rs` (1).

---

## 4. Validação

### 4.1 Caso original de P772p

```
$ cat /tmp/p772s-test.typ
#image("bogus.png")
$ ./target/release/typst /tmp/p772s-test.typ
/tmp/p772s-test.typ:1:7: error: image(): não foi possível ler 'bogus.png': erro ao ler 'bogus.png': No such file or directory (os error 2)
```

Span aponta para `1:7` (posição real de `"bogus.png"` no documento) —
**não** `<detached>`. Confirma o achado original de P772p corrigido.

### 4.2 Suite completa

```
cargo build --release --workspace --tests   → 0 erros
cargo test --workspace --release
  typst-core:  4192 passed, 0 failed
  typst-infra:  645 passed, 0 failed, 5 ignored
  typst-shell:   33 passed, 0 failed
  typst-wiring:   2 passed, 0 failed
  cli (integration): 29 passed, 0 failed
  crystalline_lint (integration): 2 passed, 0 failed
```

Nenhum teste dependia da ausência de span (nenhuma mensagem formatada
assumindo `<detached>` quebrou).

### 4.3 `crystalline-lint`

```
crystalline-lint --fix-hashes .
  Fixed 1 file: ./01_core/src/entities/args.rs → 40280be3
  (só args.rs — os outros 16 ficheiros tocados neste passo não mudaram
  o seu L0 correspondente, só a implementação; sem drift nesses)
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente sobre
  package_version_resolution.md, não relacionado a este passo)
```

---

## Critério de fecho do passo (`typst-passo-772s.md`)

- [x] Alcance real medido (520 pontos de produção com `Span::detached()`
      em `01_core/src/rules/stdlib/*.rs`; 194 elegíveis para correcção
      mecânica sem mudança de assinatura, 326 exigindo threading em
      funções auxiliares).
- [x] Mecanismo do vanilla confirmado (`Args.span` — span da chamada
      inteira — mais `Arg.span` por-argumento numa lista unificada
      `EcoVec<Arg>`; cristalino replica só o primeiro, de forma parcial
      — span da lista de argumentos, não da chamada inteira).
- [x] Decisão registada com base no número real (520 disperso → corrigir
      os 194 elegíveis directamente; registar os 326 + span por-argumento
      ~1200 como débito técnico priorizado).
- [x] Corrigido (parcialmente, por decisão registada): span por-chamada
      implementado (`Args.span`), 194 pontos de erro actualizados, caso
      original de `image()` (P772p) confirmado a apontar para o span
      correcto.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772s.md`.

---

## Débito técnico registado

1. **326 pontos de `Span::detached()` em funções auxiliares sem `Args`
   na assinatura** (`structural.rs` 145, `collections.rs` 56, `layout.rs`
   20, `gradients.rs` 23, `foundations.rs` 21, `shapes.rs` 12,
   `visualize.rs` 9, `primitives_constructors.rs` 8, `color.rs` 6,
   `label.rs`/`ref.rs`/`math_style.rs` 4 cada, `counter.rs`/`eval.rs` 3
   cada, `mod.rs` 2, `loading.rs`/`pdf.rs`/`plugin.rs` 1 cada) — exige
   threading de `span: Span` como parâmetro adicional em ~243 funções
   auxiliares. Candidato a passo dedicado, ficheiro a ficheiro (maior
   concentração: `structural.rs`, `collections.rs`).
2. **Span por-argumento individual** (paridade completa com vanilla
   `Arg.span`) — ~1200 pontos, requer unificar `items`/`named` numa
   lista tipo `Vec<Arg>`-like ou adicionar spans paralelos. Mudança de
   ABI maior, mantido como débito desde a era P740C.

---

## Próximo passo

Reconfirmar `lacuna-inventario` (estilo P772e, segunda rodada) — decidir
se a varredura sistemática da stdlib continua ou encerra, com o volume já
coberto até aqui (P765a a P772s). Avaliar se o débito de `structural.rs`
(145 pontos, maior concentração) justifica um passo dedicado antes da
segunda rodada.
