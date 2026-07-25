# Relatório — Passo 902: `partial` — cristalino usava `∂` (U+2202), vanilla usa `𝜕` (U+1D715)

**Data:** 2026-07-25
**Commit de partida:** `b6f24e28b` (P901)

---

## Fase A — confirmar antes de corrigir

### Ponto 1 — é sempre `𝜕`, ou depende de contexto?

Confirmado **por compilação directa contra o binário vanilla real**
(`lab/typst-original/target/release/typst`, não por leitura de código só): `$ partial $`,
`$ partial x $`, `$ (partial f) / (partial x) $` produzem `unicode="𝜕"` em **todos** os casos
(`mutool trace`) — nunca `∂`. Não é dependente de contexto (posição isolada, antes de outra
variável, dentro de fracção — sempre igual).

**Achado relevante para não presumir demasiado**: a tabela de símbolos `codex` (ground truth usada
noutros passos desta frente, `sym.txt:520`) regista `partial ∂` — a variante upright, U+2202. Isto
poderia sugerir (erradamente) que a correcção certa seria noutro sítio (ex.: expandir a heurística
de itálico automático `is_math_italic_default` para incluir `∂`). Confirmado que **não é esse o
mecanismo**: comparado `alpha` (que já italiciza correctamente via `apply_math_default`/
`is_math_italic_default`, aplicado a `MathText`/`MathIdent` de 1 carácter) com `nabla`/`infty` (que
ficam **upright** nos dois binários) — `nabla`/`infty` confirmam que nem todo símbolo italiciza por
defeito, e que a exclusão de `∂` dessa heurística está correcta para os OUTROS símbolos
operator-like. `partial` é especificamente um símbolo cujo **codepoint canónico usado dentro de
modo matemático** é a variante itálica dedicada (U+1D715), não uma aplicação de itálico em cima do
codepoint upright — mesmo padrão do `dot`/`⋅` já corrigido em P894 (tabela, não heurística).

### Ponto 2 — localização da entrada actual

`01_core/src/engine/math/symbols.rs:72` (não `01_core/src/rules/math/symbols.rs`, caminho referido
na materialização — desactualizado, a estrutura de directórios mudou desde então; confirmado por
grep antes de editar): `"partial" => Some("∂")`, dentro de `ident_to_unicode` (a tabela de
prioridade 1 na resolução de identificadores em modo math, `eval/math.rs`).

### Ponto 3 — cobertura de fonte

Confirmado via `fontTools` (mesmo método de P890-893) nas 3 variantes de `NewCMMath` usadas pelo
projecto:

```
NewCMMath-Bold.otf     U+1D715 (𝜕): True -> glyph: u1D715 | U+2202 (∂): True -> glyph: partialdiff
NewCMMath-Book.otf     U+1D715 (𝜕): True -> glyph: u1D715 | U+2202 (∂): True -> glyph: partialdiff
NewCMMath-Regular.otf  U+1D715 (𝜕): True -> glyph: u1D715 | U+2202 (∂): True -> glyph: partialdiff
```

Ambos os codepoints cobertos nas 3 variantes — trocar não produz símbolo em falta.

## Fase B — Implementação (TDD directo, sem dois agentes — mapeamento de tabela)

Teste `p902_partial_converte_para_variante_italica` (`01_core/src/engine/math/symbols.rs`),
mesmo padrão dos testes já existentes para `ident_to_unicode` (`alpha_converte_para_unicode`,
`integral_converte_para_unicode`, etc.). Vermelho confirmado (`left: Some("∂") right: Some("𝜕")`)
antes da correcção.

Correcção: `"partial" => Some("\u{1D715}")`.

### Suíte completa

```
typst-core:    4736 passed; 0 failed; 3 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões.

### `crystalline-lint`

`--fix-hashes .`: 1 ficheiro (`symbols.rs`, L0 `math/symbols.md` actualizado com a entrada `partial`
documentada). `crystalline-lint .`: 0 drift, só o warning pré-existente V7 (não relacionado).

### Confirmação visual

Secções 4, 11 e 18 do `.typ` de 30 secções (as que usam `partial`) recompiladas isoladamente:

```
$ partial / (partial x) f(x,y) $
$ nabla times B = mu_0 J + mu_0 epsilon_0 (partial E) / (partial t) $
$ (partial^2 f) / (partial x partial y) $
```

`mutool trace`: 7 ocorrências de `𝜕`, 0 de `∂` — todas as 7 instâncias de `partial` nestas secções
usam agora o codepoint correcto. Confirmado visualmente no render (símbolo com o traço itálico
característico).

`.typ` de 30 secções completo: hash confirmado igual aos passos anteriores
(`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`), recompila sem regressão
(`exit=0`); `mutool trace` no PDF completo confirma 9 ocorrências de `𝜕`, 0 de `∂` em todo o
documento.

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Mapeamento de tabela sem impacto de layout, como
esperado — todas as leituras dentro da baseline estabelecida (`04-math` 155.5ms, `01-hello` 94.7ms).
Sem regressão, confirmado por disciplina mesmo sem expectativa de diferença.

## Fora de âmbito

Nenhum achado incidental novo — investigação directa, âmbito já completamente delimitado pela
materialização.
