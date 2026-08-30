# P1162 — materializar `Symbol` multi-codepoint e `emoji.heart`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** commit `4ed7f6a8d`; vanilla ratificado `a51e02804`
**Gate:** contrato P1161 aprovado pelo dono

## Objetivo

Materializar em testes-first o contrato L0 aprovado no P1161: valor e variants
de `Symbol` como `EcoString`, preservação integral em eval/markup/math,
constructor de um grapheme e grupo público `emoji.heart` com as 22 variants
medidas.

## Ordem obrigatória

1. ressellar os hashes dos L0s aprovados;
2. escrever testes de língua para constructor, `repr`, igualdade,
   concatenação, markup/math, modifiers e erros;
3. confirmar RED contra a implementação `char`;
4. alterar entidade e consumers, sem copiar a mecânica do vanilla;
5. converter tabelas estáticas para strings e adicionar `heart`;
6. confirmar GREEN focado;
7. executar workspace, check, build, fmt, diff-check e lint;
8. atualizar este passo com proveniência e resultados;
9. parar antes de staging/commit.

## Aceitação de língua

```text
symbol("♥️") preserva U+2665 U+FE0F em repr, concatenação e markup
symbol("👩‍💻"), symbol("👍🏽"), symbol("🇧🇷") preservam o grapheme integral
symbol(""), symbol("ab") mantêm erro/hint medidos
emoji.heart == symbol("❤️") é false, apesar do cluster U+2764 U+FE0F igual
symbol("❤️") == symbol("❤️") é true
emoji.heart renderiza ❤️
emoji.heart.arrow / .beat / .excl renderizam 💘 / 💓 / ❣️
símbolos existentes de um codepoint não regridem
```

Não ampliar neste passo as restantes entradas multi-codepoint nem implementar
`Symbol::func()` genérico.

## Resultado executado

Proveniência final: `2026-08-25T11:55:45-03:00`, HEAD
`4ed7f6a8d9d9943b74191444e1e3c23f8f584785`, working tree não commitado com
36 paths (34 rastreados: 1.326 inserções e 1.003 remoções; P1161/P1162 novos).
As remoções/inserções amplas em `sym.rs`/`emoji.rs` são a conversão mecânica
das tabelas `char` para strings, auditada sem mudança dos valores existentes.

### RED → GREEN

RED inicial reproduzido:

- `symbol("♥️")` truncava VS16 e representava apenas `♥`;
- `emoji.heart` falhava como campo ausente;
- `str(emoji.heart)` falhava com `str() não suporta symbol`.

GREEN final:

- `Symbol.value` e `SymbolVariant` usam `EcoString`;
- constructor preserva exactly one grapheme cluster e mantém erro + hint;
- markup, math, operadores, join, repr e `str()` usam o cluster integral;
- `emoji.heart` contém base U+2764+FE0F e 22 variants medidas;
- `sym`/`emoji` existentes usam tabelas uniformes de string.

Correção anti-deriva durante o passo: `♥️` é U+2665+FE0F, não U+2764+FE0F.
A igualdade de identidade foi re-medida corretamente com `symbol("❤️")`,
codepoints iguais aos de `emoji.heart`, e permaneceu `false` no vanilla e no
cristalino. Os L0s e critérios foram corrigidos antes do fechamento.

### Matriz CLI final

| Expressão | Vanilla | Cristalino |
|---|---|---|
| `repr(type(emoji.heart))` | `symbol` | igual |
| `repr(symbol("♥️"))` | `symbol("♥\\u{fe0f}")` | igual |
| `emoji.heart == symbol("❤️")` | `false` | igual |
| `symbol("❤️") == symbol("❤️")` | `true` | igual |
| `str(emoji.heart)` | `❤️` | igual |
| `str(emoji.heart.arrow)` | `💘` | igual |
| `str(emoji.heart.excl)` | `❣️` | igual |
| `"a" + emoji.heart + "b"` | `a❤️b` | igual |
| `repr(symbol("👩‍💻"))` | ZWJ escapado | igual |

`repr(emoji.heart)` contém a mesma base e as mesmas 22 variants. O vanilla
aplica pretty-print multilinha e o cristalino conserva o formatter compacto
pré-existente; whitespace de pretty-print permanece risco residual de sintaxe
observável, fora do corte multi-codepoint e comum aos symbols complexos.

### Validação

```text
P1162 focado:       6 passed; 0 failed
typst-core:     5.231 passed; 0 failed
typst-infra:      846 passed; 0 failed
typst-shell:       53 passed; 0 failed
wiring/CLI:        55 passed; 0 failed
demais suites:      6 passed; 0 failed; 3 ignored
cargo check --workspace: exit 0
cargo build --workspace: exit 0
cargo fmt --all -- --check: exit 0
git diff --check: exit 0
crystalline-lint .: exit 0; zero V5
```

O lint conserva advisories históricos V16–V20 fora do gate de drift. Nenhum
staging ou commit foi realizado.
