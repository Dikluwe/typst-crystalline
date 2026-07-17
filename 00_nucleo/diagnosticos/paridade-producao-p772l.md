# P772l — Varredura da stdlib: `typst_library::foundations::scope`

> **Passo:** 772l
> **Data:** 2026-07-16
> **Commit-base:** `61b7edee78fdae9b020e458f5989f638cbf04096` (HEAD).
> **Medido em:** 2026-07-16T22:09–22:23Z. Working tree modificado **por este
> passo** em `00_nucleo/prompts/rules/eval.md` e `01_core/src/rules/eval/{mod,
> bibliography,closures,control_flow,flow,markup,math,modules,rules,tests}.rs`
> (correcção descrita na secção 2). Sem outras alterações de código.

---

## 1. Classificação item a item (7 itens)

```
typst_library::foundations::scope::BindingKind
typst_library::foundations::scope::cannot_mutate_constant
typst_library::foundations::scope::Capturer
typst_library::foundations::scope::Deprecation
typst_library::foundations::scope::NativeScope
typst_library::foundations::scope::unknown_variable
typst_library::foundations::scope::unknown_variable_math
```

| Item | Classificação | Achado |
|---|---|---|
| `BindingKind` / `Capturer` | **Lacuna confirmada — bug real, mensagem de erro errada** | §2.2 |
| `cannot_mutate_constant` | **Lacuna confirmada — bug real, mutação silenciosa de stdlib** | §2.3 |
| `unknown_variable` | **Lacuna parcial confirmada — falta hint** | §2.4 |
| `unknown_variable_math` | **Lacuna já catalogada (P301), ainda aberta** | §2.5 |
| `Deprecation` | **Lacuna confirmada — sem aviso de símbolo deprecated** | §2.6 |
| `NativeScope` | **Diverge de propósito (ADR-0107)** — mecanismo de registo de scopes nativos é outro; sem efeito de língua observável a testar aqui | — |

Além dos 7 itens nomeados, a sonda encontrou e **corrigiu** um bug de maior
gravidade não listado no inventário por nome próprio (está coberto pela
classe `Scopes`/`enter`/`exit`, tema central deste módulo): `CodeBlock`/
`ContentBlock` não isolavam bindings de `let` — ver §2.1.

---

## 2. Evidência empírica

### 2.1 — CORRIGIDO NESTE PASSO: fuga de âmbito em `CodeBlock`/`ContentBlock`

```
#let x1 = 1
Bloco: #{ let x1 = 2; x1 }
Depois: #x1
```

Antes da correcção (commit `61b7edee7`): vanilla → `Bloco: 2` / `Depois: 1`;
cristalino → `Bloco: 2` / `Depois: 2` (o `let` interno vazava e sobrescrevia
`x1` no âmbito do chamador, permanentemente). Reproduzido também em `#if
cond { let x = .. }` (corpo do ramo é um `CodeBlock`) e `#[ #let x = ..; ..
]` (`ContentBlock`); `#while`/`#for` não tinham o problema (isolam por outra
via, `control_flow::eval_while`/`eval_for`).

Causa: `Expr::CodeBlock`/`Expr::ContentBlock` em
`01_core/src/rules/eval/mod.rs` isolavam `styles`/`show_rules` locais
(Passos 94/95, P340) mas avaliavam o corpo directamente no `scopes` do
chamador — sem `scopes.enter()`/`scopes.exit()`. Vanilla
(`typst-eval/src/code.rs:317-332`) chama `vm.scopes.enter()`/`exit()` em
ambos. A API `Scopes::enter/exit` já existe, já está especificada em
`rules/scopes.md` e já é testada (`sombra_pai_pelo_filho`,
`exit_remove_binding_filho`) — o bug estava na ausência da chamada nos dois
pontos de consumo, não na struct `Scopes`.

**Correcção aplicada** (sem introduzir tipo, dependência ou decisão
arquitectural nova — usa API já aprovada): `scopes.enter()` antes do corpo,
`scopes.exit()` depois, em ambos os armos. L0 actualizado
(`00_nucleo/prompts/rules/eval.md`, nova secção `§P772l` + bullets
actualizados de `Expr::CodeBlock`/`Expr::ContentBlock`); hashes
re-sincronizados via `crystalline-lint --fix-hashes .` (10 ficheiros em
`01_core/src/rules/eval/` partilham este L0). Dois testes de regressão
adicionados em `01_core/src/rules/eval/tests.rs`
(`p772l_let_dentro_de_code_block_nao_vaza_para_fora`,
`p772l_let_dentro_de_if_body_nao_vaza_para_fora`).

Verificado pós-correcção (binário recompilado) — os 4 casos batem
byte-a-byte com o vanilla: `let` em `CodeBlock` puro, em corpo de `if`, em
`ContentBlock` (`[ ]`), e não-regressão de `while`/`for`.
`cargo test --workspace`: 4172+644+... testes, 0 falhas.
`crystalline-lint .`: 0 violações (só o V7 pré-existente e não relacionado
de `package_version_resolution.md`).

### 2.2 — `BindingKind`/`Capturer` — mensagem de erro errada ao mutar variável capturada

```
#let x = 1
#let f() = { x = 2 }
#f()
```

Vanilla: `error: variables from outside the function are read-only and
cannot be modified`. Variante `#context { x = 2 }` →
`"...outside the context expression..."` (mesma família,
`Capturer::Function` vs `Capturer::Context`).

Cristalino: `error: unknown variable: x` — **mensagem enganosa**: `x`
existe (está no scope capturado pela closure), o problema é que é
só-leitura, não que está ausente. ADR-0108: aceitação no nível da língua
**inclui** mensagens de erro quando a mecânica é o observável — este é
exactamente esse caso.

Causa: `Scopes::get_mut` (`01_core/src/rules/scopes.rs`) não consulta
`captured` (comentário do próprio código, P715: "não é um caso medido/
alcançado — devolve `None`, tratado como unknown variable"). `access()`
(`01_core/src/rules/eval/bindings.rs:409-417`) trata qualquer `get_mut ==
None` uniformemente como "unknown variable".

**Por que não foi corrigido directamente**: replicar a mensagem exacta do
vanilla exige distinguir *por que* a closure capturou o scope
(`Capturer::Function` vs `Capturer::Context`) — o que é exactamente o par
`BindingKind`/`Capturer` que este item do inventário nomeia, e que
`entities/scope.md` (L0 vigente) **explicitamente adia**: "Campos
adicionais do original (kind, span, category, deprecation) são adicionados
quando Value real migrar — não antecipar (ADR-0017)". `Value` real já
migrou (confirmado: `Value::Int`/`Str`/`Bool` existem e são usados em todo
o código); a condição de adiamento do ADR-0017 já está satisfeita, mas
adicionar o campo é uma alteração de schema em `Binding` — precisa de L0
actualizado (`entities/scope.md` + `rules/scopes.md`) e confirmação humana
antes de código (Regra de Ouro), não um "achado" implementável directamente
num M.

### 2.3 — `cannot_mutate_constant` — mutação de nome da stdlib sucede silenciosamente

```
#{ calc = 5 }
#calc
```

Vanilla: `error: cannot mutate a constant: calc`, exit 1.

Cristalino: **exit 0, sem erro nenhum** — pior que "mensagem errada": não há
mensagem nenhuma. `calc` (ou `image`, ou qualquer nome da stdlib) fica
reatribuído para o resto do âmbito, silenciosamente. Confirmado também com
`image`. Efeito medido: `#{ calc = 5 }` seguido de `#calc` no mesmo
documento compila e mostra `5` em vez do módulo `calc` — corrompe o uso
subsequente de `calc.pi`, `calc.max`, etc., sem qualquer diagnóstico.

Causa: o cristalino não distingue bindings "constantes" (stdlib, seedados
uma vez) de bindings normais — todos vivem como `Binding` comum no mesmo
`Scope`/`IndexMap`, sem metadado de imutabilidade. `Scopes::get_mut` só
pesquisa `top`/`scopes` (nunca `base`), então "mutar algo da stdlib" na
prática está a mutar/criar uma entrada normal no âmbito local — sem
protecção.

**Por que não foi corrigido directamente**: exige o mesmo campo de schema
que §2.2 (`BindingKind`/constante) mais lógica para semear os bindings da
`Library` como imutáveis. Mesma barreira de L0 — arquitectura nova, não um
fix pontual.

**Severidade**: alta. Blast radius: qualquer documento com uma variável de
utilizador cujo nome colide com um símbolo da stdlib (`table`, `list`,
`grid`, `image`, `text`, `calc`, etc., usado como nome de `let`/loop-var
depois reatribuído) corrompe silenciosamente esse nome pelo resto do
documento, com resultados visuais divergentes sem qualquer erro.

### 2.4 — `unknown_variable` — falta hint de subtracção

```
#foo-bar
```

Vanilla: `unknown variable: foo-bar` + `hint: if you meant to use
subtraction, try adding spaces around the minus sign: foo - bar`.

Cristalino: `unknown variable: foo-bar`, **sem hint**. Mensagem base
idêntica; falta só o hint.

Este item **é o comportamento actualmente especificado** em
`00_nucleo/prompts/rules/eval.md` linha 1191 (referência a P715): "…como
'unknown variable' pelo caller" — o L0 vigente já documenta a mensagem
simples sem hint como o estado aceite. Adicionar o hint exige actualizar o
L0 primeiro (mudança de contrato documentado, não bug vs L0 já aprovado —
ao contrário de §2.1). A infra-estrutura para o hint já existe
(`SourceDiagnostic::with_hint`, usada em `missing_key` no mesmo ficheiro) —
o custo de implementação é baixo assim que o L0 for actualizado.

### 2.5 — `unknown_variable_math` — já catalogado em P301, continua aberto

```
#let myvar123 = 5
$ myvar123 $
```

Vanilla: substitui pelo valor — mostra `5`.
Cristalino: mostra `myvar123` (o identificador, literal).

Também: `$ foobarbaz $` (identificador nunca definido) — vanilla `error:
unknown variable: foobarbaz` + 2 hints (letras separadas / aspas), exit 1;
cristalino aceita silenciosamente, exit 0.

Causa: `eval_math_expr` (`01_core/src/rules/eval/math.rs:90-103`) só resolve
`Expr::MathIdent` por (1) símbolo Unicode/grego, (2) operador matemático via
`lookup_math_op` (scope `math`, 42 operadores P299/P301); qualquer outro
identificador — variável de utilizador real ou erro de digitação — cai no
mesmo fallback `Content::MathIdent` sem nunca consultar `scopes.get_in_math`
(que no vanilla, `typst-eval/src/math.rs:45-56`, é chamado
incondicionalmente para *todo* `MathIdent`).

**Isto não é um achado novo desta sonda.** `00_nucleo/diagnosticos/
diagnostico-auto-lookup-math-passo-301.md` §A.5 já mediu e classificou `$x$
(variável)` como "continua MathIdent (lookup não encontra)" — uma decisão
consciente de scope-out, não um esquecimento, registada como frente futura.
P772l apenas reconfirma que essa frente continua aberta e mede o impacto
mais concretamente (`myvar123` com valor perdido; identificador inexistente
sem erro).

**Por que não foi corrigido directamente**: replicar `get_in_math` exige
decidir a fronteira entre "letra única sempre simbólica" (não observado
directamente aqui, mas plausível dado `$r$` → glifo itálico mesmo com `r`
definido no vanilla) vs "identificador multi-carácter sempre via scope" —
uma regra de língua que precisa de medição própria e L0 novo, não inferível
com segurança dos dois casos medidos aqui. Fica como debt explicitamente
religado ao já existente em P301.

### 2.6 — `Deprecation` — símbolos deprecated não avisam

```
#sym.gt.tri
```

Vanilla: `warning: gt.tri is deprecated, use gt.closed instead` (exit 0,
mas com aviso visível ao utilizador).
Cristalino: exit 0, sem aviso nenhum.

Mecanismo vanilla (`typst-library/src/symbols.rs`): a tabela `codex`
carrega uma mensagem de depreciação por símbolo; `Scope::bind` marca o
`Binding` com `Deprecation`; leitura via `read_checked` emite o warning no
sink. Uso estreito no vanilla (1 call site, dados vindos da crate externa
`codex`). Mesma barreira de schema que §2.2/§2.3 (`Binding` sem campo
`deprecation`). Severidade baixa (é warning, não altera o output visual),
mas confirmado real.

---

## 3. Por que os itens restantes não foram implementados directamente

§2.2, §2.3, §2.5 e §2.6 partilham a mesma causa estrutural: o `Binding` do
cristalino (`01_core/src/entities/scope.rs`) tem **só** `value` — os campos
`kind`/`span`/`category`/`deprecation` do original foram explicitamente
adiados por ADR-0017 ("não antecipar", até `Value` migrar). Fechar estes
quatro achados em conjunto é uma única mudança de schema bem definida, não
quatro patches independentes — mas é uma mudança de schema, e o L0 vigente
de `entities/scope.md`/`rules/scopes.md` não a cobre. Pela Regra de Ouro do
`CLAUDE.md`, requer L0 redigido e hash confirmado pelo humano antes de
código.

§2.1 (fuga de âmbito) foi diferente e **foi corrigido**: usava API já
aprovada e testada (`Scopes::enter/exit`), sem mudança de schema — dentro do
mandato de "implementação directa para achados confirmados" deste passo.

---

## 4. Decisão

1. **Correcção aplicada**: fuga de âmbito em `CodeBlock`/`ContentBlock`
   (§2.1) — código, L0, testes e hashes actualizados; validado.
2. **Recomenda-se um novo passo dedicado** para a mudança de schema de
   `Binding` (`kind: BindingKind`, `deprecation: Option<Deprecation>`),
   cobrindo em conjunto: mensagem correcta para mutação de captura
   (§2.2), `cannot_mutate_constant` para bindings da stdlib (§2.3, **maior
   severidade — corrupção silenciosa**), e avisos de depreciação (§2.6).
3. **Recomenda-se abrir/retomar a frente já registada em P301** para
   `get_in_math` real (substituição de variável + erro de identificador
   desconhecido em modo matemático) — §2.5.
4. Hint de subtracção em `unknown_variable` (§2.4) é o item de menor custo
   — mas ainda depende de actualizar o L0 vigente (que hoje documenta
   explicitamente "sem hint" como comportamento aceite) antes de código.

---

## 5. Validação

```
cargo test --workspace   # 4172 (typst-core) + 644 (typst-infra) + 33 + 2 + 29 + 2 passaram, 0 falhas
crystalline-lint .       # 0 violações (1 warning V7 pré-existente, não relacionado)
```

## Critério de fecho do passo (`typst-passo-772l.md`)

- [x] Os 7 itens de `foundations::scope` classificados item a item.
- [x] Casos de shadowing/escopo testados com saída comparada directamente
      (`pdftotext`/valor resolvido, não só exit code).
- [x] Bugs reais corrigidos com teste comparando valor resolvido — fuga de
      âmbito em `CodeBlock`/`ContentBlock` (§2.1). Os restantes 4 achados
      confirmados (§2.2/2.3/2.5/2.6) requerem L0 novo (Regra de Ouro) —
      registados na secção 3-4, não implementados neste passo.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772l.md`.
