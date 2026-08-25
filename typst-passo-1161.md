# P1161 — auditar e nuclear `Symbol` multi-codepoint e `emoji.heart`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — L0 REDIGIDO; PARADO NO GATE ADR-0127`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Baseline cristalina:** commit `4ed7f6a8d`
**Gate:** ADR-0127 obrigatório antes de alterar contrato público

## 1. Objetivo

Medir a morfologia pública de símbolos Unicode multi-codepoint no vanilla,
auditar todos os consumidores cristalinos de `Symbol { ch: char }`, atualizar
os L0s donos e parar para aprovação explícita antes de alterar campos,
assinaturas ou comportamento público.

O caso sentinela é `emoji.heart`: o vanilla produz o cluster
`U+2764 U+FE0F` (`❤️`), que não cabe no `char` público atual. Este passo não
implementa a nova representação, não adiciona a entrada à tabela e não escreve
testes RED de produção.

## 2. Estado inicial ratificado

Ler integralmente antes de medir:

- `AGENTS.md`;
- `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`;
- `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md`;
- `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md`;
- `00_nucleo/prompts/entities/symbol.md`;
- `00_nucleo/prompts/compiler/stdlib/sym.md`;
- `00_nucleo/prompts/compiler/stdlib/emoji.md`;
- L0s dos consumers encontrados pela auditoria, antes de propor alterações.

Não ler nem listar `00_nucleo/context/` ou
`00_nucleo/materialization/`.

O L0 vigente fixa publicamente:

```rust
pub struct Symbol {
    pub ch: char,
    pub variants: Vec<(EcoString, char)>,
    // ...
}
```

O vanilla ratificado, medido em
`lab/typst-original/crates/typst-library/src/foundations/symbol.rs:51-126`,
guarda valores de símbolo como strings e expõe `get(&self) -> &str`. Essa
diferença de estrutura Rust ainda não decide a solução cristalina: ADR-0107
exige medir primeiro o observável da linguagem.

## 3. Proveniência obrigatória

Antes das sondas, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
/usr/local/bin/typst --version
lab/typst-original/target/release/typst --version
```

Não usar a string de versão como prova do alvo. Registrar explicitamente o pin
`a51e02804`, os paths dos binários e que a working tree está limpa ou listar
todos os ficheiros alterados.

## 4. Medição vanilla antes da decisão

Criar sondas mínimas e medir, separadamente:

1. `type(emoji.heart)`, conteúdo visível e sequência de codepoints;
2. `repr(emoji.heart)` e modifiers públicos (`arrow`, `beat`, `black`,
   `broken`, `excl`, ao menos);
3. `symbol("♥️")`: constructor, `repr`, render em markup e math;
4. igualdade de linguagem entre:
   - `emoji.heart` e `symbol("❤️")` (mesmos codepoints);
   - duas instâncias `symbol("♥️")`;
   - `emoji.heart` e a variante que produz o mesmo cluster, se existir;
5. concatenação com string/content e interpolação em markup;
6. field access válido e inválido, incluindo mensagem e span;
7. comportamento de grapheme inválido: string vazia, dois graphemes e
   sequência ZWJ única;
8. `repr` e render de variation selectors, ZWJ, modificadores de tom de pele
   e bandeiras regionais, sem assumir que “multi-codepoint” é uma só classe;
9. warnings/depreciações de variantes, para não apagar metadados ao mudar a
   representação.

Capturar exit code, stdout/stderr e, quando houver documento, extrair texto e
inspecionar os codepoints do resultado. Números e contagens devem carregar
HEAD, estado da árvore e hora conforme a regra de proveniência.

Medição preliminar que deve ser reproduzida, não apenas copiada:

```text
vanilla: emoji.heart renderiza ❤️ (U+2764 U+FE0F)
vanilla: repr(symbol("♥️")) == symbol("♥\u{fe0f}")
vanilla: emoji.heart == symbol("❤️") resulta false
```

A última linha é semântica observável da linguagem, mas não autoriza copiar a
estrutura interna do vanilla. Classificar explicitamente cada resultado como
semântica, sintaxe, morfologia ou mecânica.

## 5. Auditoria cristalina de consumers

Inventariar com `rg`, registrando `file:line`, todos os usos de:

- `Symbol.ch`;
- `SymbolVariant` e tuplas cujo valor é `char`;
- `Symbol::new`, `with_variants` e `runtime`;
- conversões `Value::Symbol` para `Content::Text`/`MathText`;
- `repr_variants`;
- concatenação e operadores aritméticos;
- cast do constructor `symbol(...)`;
- tabelas `sym` e `emoji`;
- hashing, igualdade, introspecção e serialização;
- consumers de layout/math que assumam exatamente um scalar ou um glifo.

Separar o inventário por contrato público, semântica interna e mecânica de
render. Não fazer substituição global de `char` por `String`: cada consumer
precisa de decisão própria e L0 owner vigente.

## 6. Auditoria da fonte ratificada

Depois das sondas, confrontar com a fonte vanilla pinada, ao menos:

- `foundations/symbol.rs`: representação, `get`, variants, constructor, cast,
  igualdade e `repr`;
- tabela/fonte geradora de `emoji.heart` e seus modifiers;
- paths de eval/markup/math que consomem `Symbol::get()`.

Toda inferência deve declarar o que a refutaria. Não usar igualdade derivada do
Rust, bytes internos, escolha de `Arc` ou passos do algoritmo como critério de
paridade, salvo quando produzirem um observável da linguagem.

## 7. Decisão somente após a medição

Produzir uma tabela comparativa das representações candidatas, sem escolher
antes das secções 4–6:

- `EcoString` como valor efetivo e nas variants;
- enum escalar/cluster;
- armazenamento compartilhado para valores estáticos e runtime;
- outra forma descoberta pela auditoria.

Avaliar, no nível da língua:

1. preservação exata de grapheme/cluster e variation selectors;
2. constructor e variants runtime;
3. igualdade e hash observáveis;
4. `repr` reproduzível;
5. concatenação, markup e math;
6. modifiers e depreciações;
7. compatibilidade dos símbolos de um único codepoint existentes;
8. pureza L1 e custo de clone, sem converter performance mecânica em paridade.

Registrar a opção escolhida e as rejeitadas com evidência `file:line` que
preceda a decisão.

## 8. Atualização L0 e gate ADR-0127

Atualizar primeiro, no mínimo:

- `00_nucleo/prompts/entities/symbol.md`;
- `00_nucleo/prompts/compiler/stdlib/emoji.md`;
- `00_nucleo/prompts/compiler/stdlib/sym.md` se a representação das tabelas ou
  variants mudar;
- L0s de eval, repr, operadores, markup e math identificados pela auditoria.

Os L0s devem definir tipos, invariantes, igualdade de linguagem, conversões,
erros e plano de compatibilidade. Se a implementação for dividida, declarar
explicitamente qual passo completa cada subconjunto.

Então executar apenas a validação documental necessária e **PARAR**. Não
rodar `crystalline-lint --fix-hashes`, não alterar headers, não escrever testes
RED e não tocar em código L1 antes de o dono:

1. auditar a substância dos L0s;
2. guardar/ratificar os ficheiros;
3. autorizar explicitamente o novo contrato público conforme ADR-0127.

## 9. Entrega do P1161

Entregar ao gate:

1. matriz vanilla com comandos, resultados e proveniência;
2. inventário completo dos consumers `file:line`;
3. classificação língua versus mecânica;
4. decisão de representação e alternativas rejeitadas;
5. lista exata dos L0s atualizados;
6. impacto previsto em código e testes para o passo seguinte;
7. riscos de compatibilidade e scope-out explícito;
8. pedido de aprovação ADR-0127.

Após aprovação, escrever P1162 para testes RED e materialização do menor corte
vertical que suporte `symbol("♥️")` e `emoji.heart` sem perder a morfologia.

## 10. Resultado executado

### 10.1 Proveniência

Medição iniciada em `2026-08-25T11:31:18-03:00`, HEAD
`4ed7f6a8d9d9943b74191444e1e3c23f8f584785`, branch `Tekt`. Estado inicial
não commitado: somente `typst-passo-1161.md`. Binários medidos:
`/usr/local/bin/typst` e
`lab/typst-original/target/release/typst`; ambos imprimem a string enganosa
`typst 0.15.1 (e0e8ca4d)`, mas o alvo ratificado permanece o pin
`a51e02804` conforme AGENTS.md.

### 10.2 Matriz vanilla ratificada

| Sonda | Resultado | Classe |
|---|---|---|
| `type(emoji.heart)` | `symbol` | semântica |
| valor de `emoji.heart` | `❤️`, U+2764 U+FE0F | morfologia |
| `repr(emoji.heart)` | base + 22 variants | semântica/morfologia |
| `repr(symbol("♥️"))` | `symbol("♥\u{fe0f}")` | sintaxe observável |
| `emoji.heart == symbol("❤️")` | `false` | semântica |
| `symbol("♥️") == symbol("♥️")` | `true` | semântica |
| `emoji.heart.arrow` / `.beat` / `.excl` | `💘` / `💓` / `❣️` | morfologia |
| `"a" + emoji.heart + "b"` | `a❤️b` | semântica/morfologia |
| `symbol("👩‍💻")` | aceite; ZWJ preservado | morfologia |
| `symbol("👍🏽")` | aceite; tom preservado | morfologia |
| `symbol("🇧🇷")` | aceite; dois regionais preservados | morfologia |
| `symbol("")`, `symbol("ab")` | erro + hint “exactly one grapheme cluster” | mensagem observável |
| `emoji.heart.nope` | `unknown symbol modifier`, span no modifier | mensagem observável |

Fonte confrontada depois das sondas:
`foundations/symbol.rs:51-129,176-196,219-310,339-400`,
`foundations/value.rs:190-200,632-648`,
`foundations/ops.rs:24-39,425-439` e `symbols.rs:17-45`.

Correção da sonda preliminar: `♥️` é U+2665+FE0F, distinto do U+2764+FE0F de
`emoji.heart`; essa comparação isolada não provaria identidade. A sonda foi
repetida com `symbol("❤️")`, codepoints idênticos, e permaneceu `false`.
Inferência marcada: a igualdade distingue identidade/lista de variants, não
somente o cluster atual. Seria refutada por uma sonda onde symbols com origens
diferentes, mas mesmo texto e variants publicamente equivalentes, fossem
iguais. As comparações corrigidas sustentam a inferência para P1162; não
autorizam copiar `SymbolInner`/`Arc` do vanilla.

### 10.3 Inventário cristalino

O comando `rg` registou 156 ocorrências em 20 ficheiros L1. Owners com mudança
material prevista:

- entidade e igualdade: `entities/symbol.rs`, `entities/value.rs`;
- constructor/cast: `compiler/stdlib/foundations/cast.rs`;
- tabelas: `compiler/stdlib/sym.rs`, `compiler/stdlib/emoji.rs`;
- markup/display: `compiler/eval/mod.rs`;
- math: `compiler/eval/math.rs`, `compiler/stdlib/structural/math.rs`;
- operadores: `compiler/eval/operators/arithmetic.rs` e `join.rs`;
- representação: `compiler/eval/repr.rs`;
- modifiers: `compiler/eval/bindings/field_access.rs`;
- testes/consumers sem lógica nova: `compiler/eval/tests.rs`,
  `compiler/stdlib/mod.rs`, `entities/value.rs` e os matches que apenas
  reconhecem `Value::Symbol` em transforms/state/introspection/method dispatch.

Linhas decisivas: `entities/symbol.rs:18-151` tipa valores como `char`;
`foundations/cast.rs:442-518` valida grapheme mas usa `chars().next()`;
`eval/mod.rs:794-800`, `eval/math.rs:315-321,1395-1405,1466-1473`,
`operators/arithmetic.rs:82-107`, `operators/join.rs:30-63` e
`structural/math.rs:394` consomem apenas `s.ch`.

### 10.4 Decisão produzida pela medição

Alterar o contrato público para:

```rust
pub type SymbolVariant = (EcoString, EcoString);

pub struct Symbol {
    pub value: EcoString,
    pub name: EcoString,
    pub variants: Vec<SymbolVariant>,
    pub applied: Vec<EcoString>,
}
```

Cada `value` contém exatamente um extended grapheme cluster. `EcoString` foi
escolhida por preservar codepoints integralmente, ser pura em L1 e clonar
O(1). Rejeitadas: enum `Scalar | Cluster` (distinção sem correspondente na
linguagem), copiar o armazenamento static/runtime do vanilla (mecânica) e
manter `char` com sufixo separado (estados partidos e consumers truncáveis).

A tabela `sym` migra mecanicamente de `char` para `&str`. A tabela `emoji`
migra para `&str` e ganha um grupo `heart` com base e 22 variants medidas.
As restantes entradas multi-codepoint continuam scope-out explícito.

### 10.5 L0s atualizados

- `entities/symbol.md`;
- `entities/value.md`;
- `compiler/eval.md`;
- `compiler/eval/bindings/field_access.md`;
- `compiler/eval/operators/arithmetic.md`;
- `compiler/eval/operators/join.md`;
- `compiler/stdlib/foundations.md`;
- `compiler/stdlib/foundations/cast.md`;
- `compiler/stdlib/structural/math.md`;
- `compiler/stdlib/sym.md`;
- `compiler/stdlib/emoji.md`.

### 10.6 Gate

Nenhum header/hash foi ressellado; nenhum teste RED ou código L1 foi alterado.
A mudança troca campo público e payload público de `Symbol`/`SymbolVariant`,
portanto P1161 para aqui conforme ADR-0127. P1162 só pode ser escrito e
executado após aprovação explícita do dono para este contrato.

**Decisão do dono:** contrato aprovado por “Continue” em 2026-08-25. Gate
ADR-0127 aberto; P1162 autorizado.
