# Passo 1444 — Materializar `math.primes` e preservar primes sintáticos

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)
**Data da medição:** 2026-09-18T12:18:06-03:00
**Commit medido:** `e041b66d91efcdb7c42585aee87b39527eef4fa5`
**Árvore medida:** limpa (`git status --short` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.overshell`,
`math.primes`, `math.root`, `math.round`, `math.stretch`, `math.underbrace`,
`math.underbracket`, `math.underparen` e `math.undershell`. P1443 fechou
`overshell`; portanto a próxima rota nominal é `math.primes`. `math.abs`,
deliberadamente não retomada pela sequência recente, continua residual e não
é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/attach.rs:51-66` declara `PrimesElem` público com
`count: usize` obrigatório e sintaxe dedicada por apóstrofos. `math/mod.rs:74`
registra o elemento no módulo. `math/ir/resolve.rs:531-545,582-608` conserva
primes separadamente no slot superior direito e usa U+2032/U+2033/U+2034/
U+2057 para contagens 1–4; as demais contagens chegam a `PrimesItem`.
`typst-layout/src/math/scripts.rs:73-97` posiciona essas demais contagens com
largura `prime_width * (count + 1) / 2` e avanço de meia largura.

Hashes identificador administrativo das fontes medidas:

| Fonte | identificador administrativo |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/attach.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/math/ir/item.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-layout/src/math/scripts.rs` | [dado administrativo removido] |

O binário `/usr/local/bin/typst`, identificador administrativo
[dado administrativo removido],
reporta `typst 0.15.1 (586e1bd4)` e coincide byte a byte com o build ratificado
em `lab/typst-original/target/release/typst`.

### 2.2 Observáveis públicos medidos no vanilla

- `repr(math.primes)` produz `primes`;
- `repr(math.primes(n))` produz `primes(count: n)` para 0, 1, 2, 3, 4, 5
  e 8;
- `.count`, `.fields()`, `.has("count")`, `.at("count")` e `.func()`
  conservam, respectivamente, o inteiro, `(count: n)`, `true`, o inteiro e
  `primes`;
- spread singleton preserva a chamada;
- ausência, negativo, float, string, `count:` named, named desconhecido e
  segundo positional falham com os diagnósticos medidos;
- `$ x''' $` produz `attach(base: [x], tr: primes(count: 3))`;
- `$ x'''^2 $` mantém `t: [2]` e `tr: primes(count: 3)` separados;
- `$ x^''' $` põe o carrier em `t`; `$ x^2''' $` conserva-o aninhado no
  sobrescrito.

### 2.3 Estado cristalino medido

A rota qualificada falha com `module math does not contain primes`. A sintaxe
converte 1–4 apóstrofos em texto U+2032/U+2033/U+2034/U+2057 e cinco ou mais
em repetição textual de U+2032, apagando identidade e `count`. Além disso,
sobrescrito/subscrito sintáticos são colocados em `tr`/`br`, contrariando o L0
vigente que exige equivalência com os slots `t`/`b` de `math.attach`:

- `$ x'''^2 $` vira `attach(base: [x], tr: "‴""2")`;
- `$ x^''' $` vira `attach(base: [x], tr: [])`;
- `$ x^2 $` vira `attach(base: [x], tr: [2])`, enquanto o vanilla usa `t`.

Hashes identificador administrativo dos consumers antes da proposta:

| Consumer | identificador administrativo |
|---|---|
| `01_core/src/entities/elements/math_attach.rs` | [dado administrativo removido] |
| `01_core/src/entities/content.rs` | [dado administrativo removido] |
| `01_core/src/compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/repr.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/bindings/field_access.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/attach.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/spacing.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect/locatable.rs` | [dado administrativo removido] |
| `01_core/src/compiler/layout/mod.rs` | [dado administrativo removido] |
| `03_infra/src/query_helpers.rs` | [dado administrativo removido] |

## 3. Classificação anti-deriva

- Identidade `primes`, campo/count, assinatura, `repr` e separação dos slots
  `t`/`tr` são semântica e morfologia da língua (ADR-0107).
- `PrimesItem`, o IR, a escolha de enum Rust e o algoritmo concreto de frames
  são mecânica; podem divergir desde que preservem a morfologia e a geometria
  observável.
- A mensagem primária de erro é observável; spans e o hint de remoção do named
  permanecem limitados pelo carrier diagnóstico global e não recebem crédito
  de equivalência integral neste passo.
- Medido: a fonte usa glyphs dedicados para 1–4 e composição de U+2032 fora
  desse intervalo. Inferência: a fórmula de meia largura expressa a geometria
  visual pretendida para grupos sem codepoint dedicado. Refutam a inferência
  documentação ou witness ratificado em que a contagem/ordem visual difira,
  ou em que a largura pública não siga a fórmula para a mesma fonte.
- A checagem extra da fonte refutou o enquadramento cômodo “primes já existe
  como texto”: isso preserva tinta parcial, mas perde o elemento, seu campo e
  a regra de attachment.

## 4. Decisão L0 proposta

1. Adicionar `MathPrimesElem { pub count: usize }` ao owner coeso
   `entities/elements/math_attach.rs` e `Content::MathPrimes(Arc<_>)`; não
   criar prompt compartilhado ou inferir identidade de texto.
2. Expor `count` por field access e pelos métodos `fields/has/at`; igualdade,
   hash, `ObservationEq`, `repr` e `func()` preservam a identidade pública.
3. Registrar `math.primes` com exatamente um inteiro posicional não-negativo,
   incluindo zero; preservar spread singleton e rejeitar named/excedentes.
4. `Expr::MathPrimes` produz o mesmo carrier. Em `MathAttach`, hat/underscore
   ocupam `t`/`b`; primes diretamente anexados ocupam `tr`, sem concatenação.
   A precedência/nesting do parser vigente decide casos como `x^2'''`.
5. O layout usa glyph dedicado para 1–4. Para zero e contagens acima de 4,
   preserva a largura `(count + 1) / 2` do glyph U+2032 e posiciona cada prime
   com avanço de meia largura; a implementação fica como free function em
   `math/layout/attach.rs`, com despacho magro (ADR-0109).
6. `MathPrimes` tem classe de fronteira `Normal`, não herda classe do contexto
   e não fabrica operador/diacrítico público.
7. A variante é não-locatable, sem filhos, terminal nos walkers
   introspectivos e nos helpers aproximados L3. `map_content`/`map_text`
   preservam count e identidade.
8. Parser, AST, gramática, fase eval→layout e as demais rotas P1422 não mudam.

## 5. Gate ADR-0127

A proposta adiciona struct e campo públicos, variante pública e função de
stdlib. Portanto este commit contém somente passo e L0 e **PARA**. Testes,
manifesto, contrato, código, ataques e resselo começam apenas após confirmação
humana explícita.

## 6. Execução após confirmação

1. Congelar contrato/oráculos para binding, count/fields, repr, igualdade,
   syntax slots/nesting, classe e geometria 0/1–4/>4.
2. Confirmar RED atribuível e GREEN vanilla onde aplicável.
3. Implementar carrier, binding, field surface, eval sintático e layout na
   forma B.
4. Atacar remoção de binding/identidade/count, aceitação de named/negativo,
   troca de glyph, perda de sobreposição e fusão ou inversão de slots.
5. Exigir score 1,0 em ordens direta, repetida e inversa; `Unknown` não recebe
   crédito.
6. Executar `cargo fmt --check`, `git diff --check`,
   `cargo build --workspace`, suítes focal/integral e `crystalline-lint .`,
   registrando proveniência e as falhas históricas separadamente.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18 após o commit
de proposta `d28d22d2e5db92339b63e8d805bbc9b75f53622d`. A cadeia RED→GREEN,
ataques, resselo e veredito está autorizada sem ampliar o escopo acima.

## 8. Materialização e veredito

Medição final em `2026-09-18T13:20:16-03:00`, sobre working tree não
commitado derivado do commit de proposta
`d28d22d2e5db92339b63e8d805bbc9b75f53622d`; o estado exato está descrito
por `git diff HEAD --stat` e pelos artefatos `p1444-*` deste passo.

- RED atribuível: sete erros de compilação pela ausência do carrier, variante
  e helpers congelados.
- GREEN focal: 4 passados, 0 falhados.
- Ataques: a primeira campanha matou 33/36 e revelou um ponto cego do helper
  de classe. A revisão focal passou; a segunda campanha completa matou 36/36
  em ordens direta, repetida e inversa, score 1,0 e zero `Unknown`.
- Paridade pública positiva: função qualificada, count/campo, `repr`,
  igualdade, carrier sintático, separação `t`/`tr`, nesting, classe Normal,
  não-locatabilidade e receita geométrica 0/1–4/>4 preservados.
- Nos sete negativos, status e mensagem primária coincidem com o vanilla. A
  ancoragem de span e o hint de remoção de `count:` continuam na limitação
  diagnóstica global do carrier simplificado e não recebem crédito integral.
- Gates: `cargo fmt --check`, `git diff --check`, `cargo build --workspace`
  e `crystalline-lint .` passaram; V5/V15/V26 = 0.
- Suíte integral: 5879 passados e os mesmos 8 falhados históricos, sem falha
  atribuível a P1444.

Veredito limitado: `Preserved` para o fragmento `math.primes` descrito; não
declara equivalência Typst geral, paridade integral de apresentação dos
diagnósticos nem fecha as rotas residuais de P1422.
