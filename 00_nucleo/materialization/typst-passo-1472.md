# Passo 1472 — fechar as nove rotas qualificadas restantes de `math`

**Estado:** ESCRITO — AGUARDA L0 CAUSAL E GATE ADR-0127

**Data da auditoria:** 2026-09-20T17:40:24-03:00

**Baseline de redação:** `306ca50022104619447fae2625090b8d9c7b0a35`,
tree `0f7f307e8ffda460270714cab0ae525237dc3433`, working tree P1470 preservada.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 1. Objetivo

Fechar exatamente estas nove rotas `MISSING_MEMBER`:

```text
math.bold      math.cal       math.display
math.italic    math.sans      math.scr
math.sscript   math.text      math.upright
```

Oito são projeções de estilos matemáticos já implementados. `math.text` é uma
rota de elemento textual e não é alias de `math.inline`.

## 2. Evidência anterior à decisão

O catálogo fresco encontrou as nove rotas ausentes nos quatro perfis. A fonte
pinada `typst-library/src/math/mod.rs:43-78` registra `TextElem` e, separadamente,
as funções `upright`, `bold`, `italic`, `sans`, `scr`, `cal`, `display` e
`sscript`.

No cristalino:

- `math_style.rs` já possui as oito nativas de estilo;
- o registry global ainda publica essas funções;
- `make_math_module` registra somente um subconjunto dos estilos antes do
  espelho `sym → math`;
- `Content::MathText` e seu layout já existem, mas não há constructor público
  `math.text` no módulo.

O L0 estrutural vigente registra que bindings de estilo qualificados reutilizam
a mesma nativa, sem wrapper. Ele também afirma que o scope é fechado; portanto
deve ser atualizado antes da expansão.

## 3. Contrato

### 3.1 Oito estilos

Registrar antes do espelho de símbolos as mesmas function pointers donas:

```text
bold, cal, display, italic, sans, scr, sscript, upright
```

Preservar nomes públicos, aridade, defaults `cramped`, casts, diagnósticos,
spans e morfologia já definidos em `compiler/stdlib/math_style.md`. O espelho
de símbolos nunca sobrescreve binding próprio.

Não criar uma segunda implementação, novo `MathStyleKind`, lookup dinâmico ou
despacho por string. `math.display`/`math.sscript` conservam seus defaults
diferentes; não tratá-los como funções body-only.

### 3.2 `math.text`

Sondar bilateralmente constructor, `repr`, `.func()`, positionalidade, named
args, casts de string/content/none e erros. A decisão L0 deve usar o carrier
existente somente se ele reproduzir o elemento textual do pin. Não reutilizar
`native_inline`: `inline` controla tamanho matemático; `text` constrói conteúdo
textual dentro de math.

Se o contrato exigir fields/style próprios que o carrier atual não preserva,
individualizar primeiro o Prompt L0 e consumer do elemento. É proibido esconder
o caso em `Content::Text` ou fabricar `MathText` sem os observáveis medidos.

## 4. Extras globais não pertencem a este passo

O catálogo também marca vários nomes globais de estilo como
`CRYSTALLINE_ONLY`. Sua remoção é decisão de compatibilidade e pertence ao
P1474. P1472 adiciona os membros qualificados e não remove globals no mesmo
diff; assim um RED ausente não é confundido com uma quebra independente.

## 5. Gate ADR-0127

As nove adições ampliam a superfície pública. Atualizar antes de código:

- `00_nucleo/prompts/compiler/stdlib/structural/math.md`;
- `00_nucleo/prompts/compiler/stdlib/math_style.md` apenas para afirmar as
  projeções/identidade, sem criar segundo owner;
- owner do elemento textual matemático, se a medição exigir mudança;
- prompt do registry/testes afetado.

Depois ressellar hashes, validar V5/V15/V26, commitar somente L0 e **parar para
confirmação humana**.

## 6. RED → GREEN

Para cada rota, congelar `type`, `repr`, chamada mínima, argumentos explícitos,
missing, extra, named desconhecido e cast errado. Para estilos já globais,
provar igualdade de função e igualdade semântica do conteúdo. Para
`math.text`, provar carrier, fields e layout sem usar pixels como único oráculo.

Regenerar os quatro perfis depois do GREEN. O resultado esperado é retirar
exatamente nove rotas do conjunto de 17 ausentes, sem crescimento de
`WRONG_KIND`, sem perda do espelho `sym` e sem crédito pela permanência dos
extras globais.

Executar focais, suíte completa, Clippy all-features, build, fmt, lint total,
V5/V15/V26 fatal e `git diff --check`.

## 7. Auditoria L0 e retificação (2026-09-21)

### 7.1 Revalidação da medição de §2

Sonda bilateral repetida em 2026-09-21 contra o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d` e o cristalino corrente
(`target/debug/typst`, build de working tree sobre o commit `52d3d337c`,
P1471 já integrado): as nove rotas `math.bold`/`cal`/`display`/`italic`/
`sans`/`scr`/`sscript`/`upright`/`text` continuam `module math does not
contain <nome>` no cristalino e `function` no vanilla, para todas as
nove. Confirma §2 sem deriva.

### 7.2 Achado — `math.text` não é `Content::MathText`

`math.text == text` (vanilla, medido) — é a mesma função de estilo de
texto documental (`native_text`), não um carrier matemático novo. Ver
`structural/math.md` P1472 para a sonda completa (missing/wrong-type/
none/fields, todos idênticos ao global `text`). `entities/text.md`/
`compiler/stdlib/text/constructor.md` não precisam de retificação —
`native_text` é reutilizada sem mudança.

### 7.3 Viabilidade medida (grounding, não decisão)

Os oito estilos já são globais bare em `eval/mod.rs:8226-8238`, usando
exatamente `Func::native(<nome bare>, native_<nome>)` — o mesmo padrão
de nome que `Func::PartialEq` (`entities/func.rs:384-397`, compara
nativas por nome) já usa para estabelecer igualdade sem partilha de
`Arc`. Nenhuma mudança em `entities/func.rs`, `entities/module.rs` ou
`math_style.rs` é necessária — só a inserção dos nove bindings em
`make_math_module()`.

### 7.4 L0s retificados (2 ficheiros)

`compiler/stdlib/structural/math.md` (contrato completo, owner do
consumer `make_math_module`) e `compiler/stdlib/math_style.md` (nota de
projeção — as 14 funções e seus campos `kind`/`bold`/`italic`/`cramped`
não mudam, só ganham espelho qualificado para 8 das 14). Owners não
tocados por decisão consciente: `compiler/stdlib/text/constructor.md`
(contrato de `native_text` inalterado — ver §7.2).

### 7.5 Gate ADR-0127

Hashes ressealados só nos 2 L0s (`crystalline-lint --fix-hashes .`); os
`@prompt-hash` dos `.rs` correspondentes revertidos (`git checkout --`)
antes do commit — reseal do código só junto da implementação (mesma
disciplina de P1467-P1471). `crystalline-lint --checks v15,v26 .`: zero
violações. `crystalline-lint --checks v5 .` mostra o drift esperado nos
2 ficheiros `.rs` (L0 mudou, código ainda não). Commit causal
somente-L0 a seguir; **confirmação humana explícita obrigatória antes de
qualquer código** (§5). A confirmação de P1467-P1471 não cobre esta
mudança de contrato público.

## 8. Execução segregada e integração (2026-09-21)

Confirmação humana explícita recebida em 2026-09-21, na mesma sessão,
posterior ao commit causal somente-L0 `445cf0c81`. Os três papéis
partiram desse baseline em worktrees isoladas.

- **Agente A (implementação):** commit `cbc3edf06` ("feat(math): expose
  bold/cal/display/italic/sans/scr/sscript/upright/text under math.*
  (P1472, agente A)"). Inseriu o segundo loop de estilos e o
  `dict.insert` de `text` em `make_math_module()`, ambos antes do
  espelho `sym → math`, sem ver os testes. Resolveu sozinho o caminho de
  import de `native_text` (reexport público `crate::compiler::stdlib::
  native_text`, já usado em todo o crate — ex. `eval/mod.rs:6445` — em
  vez do módulo privado `text::constructor`), decisão de implementação
  compatível com o contrato.
- **Agente B (testes):** commit `bc6126331` ("test(math): freeze nine
  qualified math.* routes red contract (P1472, agente B)"). 8 testes
  novos num módulo inline `p1472_tests` (mesmo padrão de
  `p1291_tests`/`p1283_tests` já existentes no ficheiro, sem L0 próprio),
  sem ver a implementação. RED confirmado: 5 falhas genuínas (todas por
  `"module math does not contain <nome>"`, nunca por mensagem errada) e
  3 guardas de não-regressão desenhadas para já passar no baseline
  (quatro estilos P1291, globais bare, espelho `sym`).
- **Achado do Agente B (não é correção de código, é correção de L0):**
  ao medir `native_text` diretamente (em vez de confiar nas strings
  vanilla do L0) para escrever o teste de comportamento de `math.text`,
  descobriu que a secção `structural/math.md` P1472 rotulava como
  "idêntico" ao vanilla três diagnósticos do `native_text` cristalino
  que na realidade divergem (mensagens em português, e `text(none)` erra
  em vez de produzir sequência vazia) — inferência não verificada da
  redação original, nunca medida contra o binário cristalino. Corrigido
  na integração (papel C) com uma nota explícita em `structural/math.md`
  ("Correção descoberta na integração"), sem apagar a tabela original.
  Não é regressão nem bug de P1472: `math.text` continua a reutilizar
  `native_text` bit-a-bit, qualquer que seja o seu comportamento — o
  contrato central (identidade de função) não muda; a divergência
  pré-existente pertence a `compiler/stdlib/text/constructor.md`, fora
  do escopo deste passo.

### 8.1 RED → GREEN

| Ficheiro | RED (baseline `445cf0c81`) | GREEN (após integração) |
|---|---|---|
| `compiler/stdlib/structural/math.rs` | 5 falhas genuínas (`MISSING_MEMBER`) + 3 não-regressão já GREEN | 8/8 novos, suíte completa 6067/6067 |
| `compiler/stdlib/math_style.rs` | sem novos testes (código inalterado) | 6067/6067 (suíte completa, sem regressão) |

### 8.2 Gates finais medidos (2026-09-21)

```text
cargo build --workspace                                     -> sucesso
cargo test -p typst-core --lib                                -> 6067/6067
cargo test -p typst-shell --lib                                -> 83/83
cargo test -p typst-infra --lib                                -> 995/995 (sem flake desta vez)
cargo fmt --all -- --check                                     -> sucesso
cargo clippy -p typst-core --all-targets --message-format=json
    comparado por contagem (ficheiro, mensagem) contra o
    baseline `445cf0c81` em worktree isolada                    -> zero avisos novos
crystalline-lint --checks v5,v15,v26 .                          -> 0 violações
git diff --check                                                -> sucesso
```

Não foi corrido `cargo test --workspace` completo nem a suíte de export
PNG em massa de `typst-wiring` (mesma razão de P1464/P1467-P1471); o raio
deste diff é inteiramente L1 (`01_core/src/compiler/stdlib/{structural/
math,math_style}.rs`), sem consumer L2-L4.

Sonda bilateral final (binário recompilado,
`./target/debug/typst eval "type(math.<nome>)"` para as nove rotas):
todas `function`, idêntico ao vanilla ratificado `586e1bd43fae`.

O veredito final é: `math.bold`, `math.cal`, `math.display`,
`math.italic`, `math.sans`, `math.scr`, `math.sscript`, `math.upright` e
`math.text` existem e são `function`, reutilizando exatamente as nativas
donas já materializadas (`math_style.rs` para os oito estilos,
`native_text` para `text`) — sem nova implementação, sem novo
`MathStyleKind`, sem despacho dinâmico. Os globais bare correspondentes e
o espelho `sym → math` permanecem intactos; a remoção dos extras globais
continua reservada a P1474.

