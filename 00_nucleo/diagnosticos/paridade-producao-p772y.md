# P772y — Espaçamento automático por `MathClass` + `math.class(...)`

> **Passo:** 772y
> **Data:** 2026-07-17
> **Commit-base:** `2b981e7e0f2421c4952bb9b86b5f9a8ba1825014` — working tree com alterações
> não commitadas de toda a série P772f-P772y (padrão já estabelecido nesta série; ver
> `git status --porcelain` no momento da medição para a lista exacta de ficheiros).
> **Dependências:** P772w (achado original — `math.class(...)` ausente e suspeita de
> `MathClass` não chegar ao motor de layout).

---

## 1. Sonda — o espaçamento automático por classe existe no cristalino?

```bash
grep -rn "MathClass\|Class::Relation\|Class::Binary\|Class::Opening" \
  01_core/src/engine/layout/*.rs 01_core/src/engine/math/**/*.rs
```

Zero ocorrências antes deste passo — confirmado que `MathClass` só existia em
`entities/math_class.rs` e no parsing/lexing (`rules/parse/math.rs`,
`rules/lexer/math.rs`), nunca no motor de layout.

### Medição directa (`mutool trace`) — estado ANTES da implementação

```bash
cat > /tmp/p772y-spacing-test.typ <<'EOF'
$ a = b $
$ a + b $
$ (a) $
EOF
lab/typst-original/target/release/typst compile /tmp/p772y-spacing-test.typ /tmp/p772y-vanilla.pdf
./target/release/typst /tmp/p772y-spacing-test.typ /tmp/p772y-cristalino.pdf
mutool trace /tmp/p772y-vanilla.pdf > /tmp/p772y-trace-vanilla.txt
mutool trace /tmp/p772y-cristalino.pdf > /tmp/p772y-trace-cristalino.txt
```

Vanilla mostra deltas de posição x diferenciados por par de classe (medido, 11pt):
`=` → 3.056pt extra de cada lado; `+` (promovido a Binary) → 2.444pt de cada lado;
`(`/`)` → 0pt extra. Cristalino (antes desta implementação): 0pt extra em todos os
três casos — confirma espaçamento **inexistente**, não "por outro mecanismo".

### Mecanismo do vanilla (fonte)

`lab/typst-original/crates/typst-library/src/math/ir/process.rs::spacing()`
(match ordenado sobre `(l.rclass(), r.lclass())`) + `math/ir/item.rs::MathItem::
lclass()/rclass()` (idênticos a `.class()` excepto em itens Fenced) +
`math/mod.rs` — `THIN = 1/6 em`, `MEDIUM = 2/9 em`, `THICK = 5/18 em`. Promoção
Vary→Binary: `math/ir/process.rs`, condição sobre `prev.class()` ∈ {Normal,
Alphabetic, Closing, Fence}.

---

## 2. Decisão de âmbito

**Cenário confirmado**: espaçamento por classe **não existe** — cenário "maior"
da tabela de decisão do passo. Mas `entities/math_class.rs::default_math_class`
(classificação Unicode TR25 + overrides Typst) **já existia, completa e testada**,
reduzindo o âmbito de "construir classificação do zero" (XL) para "aplicar
classificação existente ao layout + tabela de espaçamento + `math.class()`" (L,
conforme o tamanho declarado no cabeçalho do passo).

**Decisão registada**: implementar neste mesmo passo. Integração não-invasiva —
classes calculadas separadamente em `layout_sequence` a partir do `Content` bruto
(antes da conversão para `MathBox`), evitando tocar nos 8 submódulos de elemento
(`attach.rs`, `root.rs`, `frac.rs`, `matrix.rs`, `cases.rs`, `delimited.rs`,
`stretchy.rs`, `assembly.rs`) — só `layout_sequence` + `hconcat` + um novo módulo
`spacing.rs` mudam.

---

## 3. Implementação

### 3.1 Dados — `Content::MathClassOverride` (Modelo D)

Novo variant `Content::MathClassOverride(Arc<MathClassOverrideElem>)`
(`entities/elements/math_class_override.rs`, `{ class: MathClass, body: Content
}`), mirror estrutural de `Content::MathCancel` — braços em `content.rs` (Debug,
constructor `Content::math_class_override`, `plain_text`, `PartialEq`,
`map_content` recursivo, `map_text` terminal), `rules/introspect.rs` (2×,
não-locatável/terminal), `rules/introspect/locatable.rs` (não-locatável),
`engine/layout/mod.rs` (fallback de math fora de contexto), `rules/eval/repr.rs`
(`class("<nome>", <repr(body)>)`), `03_infra/src/query_helpers.rs` (2×, terminal).

`entities/math_class.rs`: `MathClass` ganhou `#[derive(Hash)]`; duas novas
funções `math_class_name(MathClass) -> &str` / `parse_math_class(&str) ->
Option<MathClass>` (paridade `foundations/cast.rs` vanilla, kebab-case só para
`"glyph-part"`), com 7 novos testes incluindo round-trip sobre as 15 variantes.

### 3.2 Tabela de espaçamento — `rules/math/layout/spacing.rs` (novo, nono submódulo)

`node_math_class`, `promote_vary`, `spacing_between`, `compute_gaps` — réplica
directa do match ordenado do vanilla (THIN/MEDIUM/THICK). 26 testes unitários.

**Fora de escopo, registado**: condição "unless in script size" de cada regra
vanilla (cristalino não tem `MathSize` discreto, só `script_percent_scale_down`
contínuo ad-hoc) — espaçamento aplica-se incondicionalmente; regra "spaced
frames" (`#h()` explícito dentro de math) — sem equivalente cristalino hoje.
Simplificação: `MathIdent`/`MathText` multi-carácter classificam-se pelo
primeiro carácter (o layout trata cada nó da sequência como unidade já
layoutada, ao contrário do vanilla que resolve por-glifo) — correcto para o
caso comum (operadores/relações são nós de 1 carácter).

### 3.3 Integração em `mod.rs`

`layout_sequence` computa `gaps = spacing::compute_gaps(&filtered, style.size.
val())` e concatena via `hconcat_spaced(boxes, &gaps)`; `hconcat` passa a ser
`hconcat_spaced(boxes, &[])` (usado por `delimited.rs` — Opening/Closing já dá
0pt, logo nenhuma mudança de comportamento nesse call site). Novo arm
`Content::MathClassOverride(e) => self.layout_node(&e.body, style)`.

### 3.4 `math.class(class, body)` — `native_math_class`

`rules/stdlib/structural.rs::native_math_class` — 1º posicional `Str` (validado
via `parse_math_class`), 2º posicional `Content | Str | Symbol`. Registado no
**scope do módulo `math`** (`make_math_module`), não no scope global (diferente
de `cancel`/`accent`).

### 3.5 Pré-requisito descoberto e resolvido no mesmo passo — `rules/eval/math.rs`

Validar `math.class("relation", sym.suit.heart)` dentro de `$...$` (sintaxe
exigida pelo passo) revelou que `eval_math_expr` (`Expr::FuncCall`) só
despachava callees `Expr::MathIdent` bare — um callee namespaced
(`Expr::FieldAccess`, como `math.class`) caía em `_ => return Ok(Content::
Empty)` e desaparecia **silenciosamente** (sem erro de compilação). Medido:

```bash
$ echo '$x math.class("relation", "z") y$' > /tmp/t.typ && ./target/release/typst /tmp/t.typ /tmp/t.pdf
# antes da correcção: exit 0, PDF só com "x y" (sem erro, sem "z")
```

Corrigido com dois adicionais cirúrgicos, ambos só no caminho de callee
namespaced (não tocam o mecanismo P510 bare-ident original — `bb(x)`/`bold(x +
y)` — para não arriscar regressão):

- `eval_math_callee`: resolve `Expr::FieldAccess` recursivamente
  (`Value::Module`/`Value::Dict`); resolve `Expr::MathIdent` **directamente no
  scope** (`scopes.get`) — não delega ao `eval_expr` genérico, que trata
  `Expr::MathIdent` como "fronteira deliberada" e devolveria sempre
  `Value::None` (reproduzindo o bug original via outro caminho).
- `eval_math_arg_value`: um argumento `Expr::Str` avalia directo para
  `Value::Str` (não `Value::Content` via `eval_math_expr`, que não tem arm para
  `Expr::Str` e cairia em `Content::Empty`).

Detalhe completo em `00_nucleo/prompts/engine/eval.md` §P772y.

### 3.6 Dois gaps maiores descobertos, medidos e explicitamente **fora de escopo**

Ambos pré-existentes, confirmados **sem qualquer relação com `math.class`**
(reproduzidos com símbolos/conteúdo triviais alheios a este passo):

1. **Bare `MathIdent` não resolve variável do utilizador em modo math**:
   ```bash
   $ echo '#let loves = math.class("relation", sym.suit.heart); $x loves y$' > /tmp/t.typ
   # renderiza "loves" como 5 glifos literais (l,o,v,e,s), não o Content vinculado
   ```
   `Expr::MathIdent` (arm principal, não o callee) só resolve símbolos Unicode
   e operadores `math`; qualquer outro nome vira sempre `Content::MathIdent
   (name)` (texto), mesmo vinculado a `Content`/`Value::Symbol` no scope.

2. **`#expr` em modo math não faz splice** (drop silencioso, sem erro):
   ```bash
   $ echo '$x #sym.suit.heart y$' > /tmp/t.typ && ./target/release/typst /tmp/t.typ /tmp/t.pdf
   # exit 0, PDF só com "x y" — símbolo desaparece
   $ echo '#let hc = $diamond.filled$; $x #hc y$' > /tmp/t.typ && ...
   # mesmo resultado — não é específico de Value::Symbol
   ```

3. **Glifo `♥` (U+2665) não renderiza em nenhum contexto** (nem fora de math):
   ```bash
   $ echo '$sym.suit.heart$' > /tmp/t.typ && ./target/release/typst /tmp/t.typ /tmp/t.pdf
   # mutool trace: página sem NENHUM fill_text — gap de cobertura de fonte/glifo,
   # não uma questão de spacing/MathClass
   ```

Estes três são candidatos a passo(s) dedicado(s) — âmbito maior (resolução geral
de identificadores/expressões em modo math; cobertura de glifos Unicode fora do
alfabeto latino/dígitos/operadores comuns), cada um a medir e decidir por si.
**Não** implementados aqui — validação de `math.class()` feita por caminho
equivalente que não depende deles (ver §4).

---

## 4. Validação

### 4.1 Tabela de espaçamento (sem `math.class`)

Medido pós-implementação, mesmo ficheiro `/tmp/p772y-spacing-test.typ` do §1:

| Par | Delta cristalino (11pt) | Extra sobre `adv` | Vanilla esperado | Match |
|---|---:|---:|---:|:-:|
| `a`→`=` | 8.082pt | 3.055pt | THICK = 3.0556pt | ✅ |
| `=`→`b` | 9.106pt | 3.056pt | THICK = 3.0556pt | ✅ |
| `a`→`+` (promovido Binary) | 7.471pt | 2.444pt | MEDIUM = 2.4444pt | ✅ |
| `+`→`b` | 8.495pt | 2.445pt | MEDIUM = 2.4444pt | ✅ |
| `(`→`a` | 3.278pt | 0pt | 0pt | ✅ |
| `a`→`)` | 5.027pt | 0pt | 0pt | ✅ |

Todos os 6 deltas batem com o vanilla à precisão de milésimas de ponto (fonte
fallback do sistema usada em ambos, glifos diferentes de `adv` mas a mesma
lógica de espaçamento aplicada).

### 4.2 `math.class(...)` — via caminho equivalente (`"z"` em vez de `sym.suit.heart`)

```bash
$ cat > /tmp/p772y-class-test3.typ <<'EOF'
$ x math.class("relation", "z") y $
$ x = y $
$ x z y $
EOF
$ ./target/release/typst /tmp/p772y-class-test3.typ /tmp/p772y-class3-cristalino.pdf
$ mutool trace /tmp/p772y-class3-cristalino.pdf
```

| Linha | `x`→meio | meio→`y` |
|---|---:|---:|
| `x math.class("relation","z") y` | 8.445pt (3.055 extra) | 7.72pt (3.056 extra) |
| `x = y` (referência relação real) | 8.445pt (3.055 extra) | 9.106pt→88.418-79.312 (3.056 extra) |
| `x z y` (sem override — baseline) | 5.39pt (**0 extra**) | 4.664pt (**0 extra**) |

`math.class("relation", "z")` produz exactamente o mesmo delta que `=`
(coincide ao milésimo de ponto), e difere claramente do delta de `z` sem
override (0 extra, Alphabetic-Alphabetic). **Confirma o critério de validação
do passo** — "o espaçamento ao redor de `loves` bate com o de uma relação
real (`=`), não com o espaçamento por omissão do símbolo" — usando `"z"` como
substituto de `sym.suit.heart` (que não renderiza por gap de fonte não
relacionado, §3.6.3).

`math.class("bad-name", "z")` → `Err "class(): 'bad-name' não é uma MathClass
reconhecida"` (validação do nome confirmada).

### 4.3 Suites de teste e lint

```
cargo build --workspace --release     → 0 erros
cargo test --workspace --release      → 4246+647+33+2+29+2 = 4959 passed, 0 failed
crystalline-lint . --fix-hashes       → 16 ficheiros re-hashed
crystalline-lint .                    → 0 drift warnings (só o V7 pré-existente
                                          de package_version_resolution.md, não
                                          relacionado)
```

Ajuste de 1 teste pré-existente: `p299_math_module_total_42_operadores`
esperava `scope().len() == 43` (42 vanilla + `equation` pós-P299); passa a
`44` (+ `class`, P772y) — não uma regressão, uma actualização de contagem
esperada dado que o módulo `math` ganhou um novo membro legítimo.

Sem regressão nos testes de math já existentes: 124 testes em
`rules::math::layout` (incluindo P299-301, P765b, P772w) continuam verdes.

---

## 5. Critério de fecho do passo

- [x] Estado real do espaçamento automático por classe confirmado — **não
      existia** (confirmado por sonda de código e medição de PDF).
- [x] Decisão de âmbito registada com evidência (§2).
- [x] Tabela de espaçamento por classe confirmada contra o vanilla (§4.1),
      aplicada no motor de layout matemático (§3.3).
- [x] `math.class(...)` implementado e testado com override real — validado
      via `"z"` (equivalente funcional a `sym.suit.heart`, que não renderiza
      por gap de fonte não relacionado a este passo, registado em §3.6.3).
- [x] Sem regressão em testes de math existentes (§4.3).
- [x] `cargo test --workspace` verde (4959 passed, 0 failed).
- [x] `crystalline-lint .` zero violações (excepto V7 pré-existente,
      não relacionado).
- [x] L0 de layout matemático atualizado antes do fecho (`spacing.md` novo,
      `_comum.md`/`math-class.md`/`structural.md`/`eval.md` atualizados,
      `elements/math_class_override.md` novo — todos com hash real via
      `--fix-hashes`).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772y.md`
      (este ficheiro).

---

## 6. Próximo passo

Três débitos novos, medidos e registados neste passo (§3.6), candidatos a
passo(s) dedicado(s) — cada um precisa de sonda e decisão de âmbito próprias
antes de implementação (ADR-0108):

1. Resolução de variável do utilizador para `MathIdent` bare em modo math
   (`#let x = ...; $... x ...$` com `x` multi-letra e não-operador).
2. Splice de `#expr` dentro de `$...$` (símbolos, conteúdo, variáveis —
   actualmente produz página sem o conteúdo interpolado, sem erro).
3. Cobertura de glifos Unicode fora de latim/dígitos/operadores comuns na
   fonte fallback (confirmado com `♥` U+2665; extensão a outros símbolos por
   verificar).

Nenhum dos três bloqueia o fecho de P772y (a feature `math.class()` +
espaçamento por classe está implementada, testada e validada por caminho
equivalente). Se a série P765a-P772y estiver a ser encerrada: restam
`image::pdf` e fallback de fontes matemáticas como débitos de P772w, mais os
três acima.
