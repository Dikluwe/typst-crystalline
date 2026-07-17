# P511 — Spec de Materialização — Math Elements Granulares

> **Passo:** 511-spec
> **Data:** 2026-06-30
> **Tipo:** Spec de materialização (gate duro ADR-0114 satisfeito pela sonda A.0)
> **Dependências:** `00_nucleo/diagnosticos/sonda-p511-math-elements-granulares.md`
> **ADRs:** ADR-0107 (paridade de linguagem), ADR-0109 (atomização), ADR-0026 (Content enum fechado)

---

## 1. Resumo Executivo

A sonda A.0 do P511 concluiu que **7/7 elementos math granulares estão ausentes** no cristalino: não existem variants em `Content`, construtores nativos, nem layout handlers. O parser aceita a sintaxe (trata os nomes como `MathIdent` + delimited), mas o eval/layout produzem texto plano em vez da semântica correta.

Esta spec define o trabalho de materialização necessário para fechar a brecha.

---

## 2. Elementos a Materializar

### 2.1 `binom(n, k)` — Coeficiente Binomial

- **Semântica:** Dois operandos empilhados entre parênteses (sem linha divisória).
- **Variant `Content`:** `MathBinom { upper: Content, lower: Content }`.
- **Construtor:** `native_binom(args)` — 2 args posicionais.
- **Registo:** `scope.define("binom", Value::Func(Func::native("binom", native_binom)))`.
- **Layout:** handler em `rules/math/layout/mod.rs` que posiciona `upper` sobre `lower` dentro de parênteses.
- **L0 necessário:** `00_nucleo/prompts/entities/elements/math_binom.md`, `00_nucleo/prompts/engine/stdlib/math_binom.md`.

### 2.2 `class("unary", x)` — Classe de Operador

- **Semântica:** Altera a classe de espaçamento do body (`unary`, `binary`, `relation`, `opening`, `closing`, `punctuation`, `large`, `vary`).
- **Variant `Content`:** `MathClass { class: MathClassKind, body: Content }`.
- **Construtor:** `native_class(args)` — 1 arg posicional string + 1 arg posicional content.
- **Registo:** scope global.
- **Layout:** handler que aplica espaçamento conforme `class` e delega layout do body.
- **L0 necessário:** `00_nucleo/prompts/entities/elements/math_class.md`, `00_nucleo/prompts/engine/stdlib/math_class.md`.

### 2.3 `limits(sum, sub: i, sup: n)` — Limites em Operadores

- **Semântica:** Força subscript/superscript em operadores grandes (`sum`, `prod`, `integral`).
- **Variant `Content`:** `MathLimits { body: Content, sub: Option<Content>, sup: Option<Content> }`.
- **Construtor:** `native_limits(args)` — body posicional, `sub`/`sup` named.
- **Registo:** scope global.
- **Layout:** handler que posiciona sub/sup diretamente abaixo/acima do body.
- **L0 necessário:** `00_nucleo/prompts/entities/elements/math_limits.md`, `00_nucleo/prompts/engine/stdlib/math_limits.md`.

### 2.4 `mid(|)` — Delimitador de Meio

- **Semântica:** Delimitador usado em conjuntos/condições; espaçamento relacional.
- **Variant `Content`:** `MathMid { body: Content }`.
- **Construtor:** `native_mid(args)` — 1 arg posicional.
- **Registo:** scope global.
- **Layout:** handler que aplica espaçamento de relação ao delimitador.
- **L0 necessário:** `00_nucleo/prompts/entities/elements/math_mid.md`, `00_nucleo/prompts/engine/stdlib/math_mid.md`.

### 2.5 `x'` — Primos

- **Estado atual:** Já funciona via `MathAttach` (primes convertidos para superscript `′`).
- **Decisão:** Não materializar `MathPrimes` como elemento granular separado neste passo. A sintaxe `$x'$` e `$x''$` já têm paridade de linguagem.
- **Ação:** Verificar regressão após P510; nenhum código novo.

### 2.6 `scripts(x, sub: i, sup: j)` — Subscripts/Superscripts Explícitos

- **Semântica:** Aplica sub/superscripts a uma base sem usar `_`/`^`.
- **Variant `Content`:** Reutilizar `MathAttach { base, sub, sup }` existente.
- **Construtor:** `native_scripts(args)` — base posicional, `sub`/`sup` named.
- **Registo:** scope global.
- **Layout:** handler existente `MathAttach`.
- **L0 necessário:** `00_nucleo/prompts/engine/stdlib/math_scripts.md`.

### 2.7 `stretch(left: "|", x, right: "|")` — Delimitadores Esticáveis

- **Semântica:** Estica delimitadores para a altura do body.
- **Variant `Content`:** `MathStretch { left: Option<Content>, body: Content, right: Option<Content>, size: Option<Rel<Length>> }`.
- **Construtor:** `native_stretch(args)` — body posicional, `left`/`right`/`size` named.
- **Registo:** scope global.
- **Layout:** handler que mede altura do body e estica delimitadores (pode delegar a `MathDelimited` existente com ajuste de escala).
- **L0 necessário:** `00_nucleo/prompts/entities/elements/math_stretch.md`, `00_nucleo/prompts/engine/stdlib/math_stretch.md`.

---

## 3. Plano de Implementação

### Fase 1 — Prompts L0

Antes de qualquer código, redigir e guardar os Prompts L0:

1. `00_nucleo/prompts/entities/elements/math_binom.md`
2. `00_nucleo/prompts/entities/elements/math_class.md`
3. `00_nucleo/prompts/entities/elements/math_limits.md`
4. `00_nucleo/prompts/entities/elements/math_mid.md`
5. `00_nucleo/prompts/entities/elements/math_stretch.md`
6. `00_nucleo/prompts/engine/stdlib/math_binom.md`
7. `00_nucleo/prompts/engine/stdlib/math_class.md`
8. `00_nucleo/prompts/engine/stdlib/math_limits.md`
9. `00_nucleo/prompts/engine/stdlib/math_mid.md`
10. `00_nucleo/prompts/engine/stdlib/math_scripts.md`
11. `00_nucleo/prompts/engine/stdlib/math_stretch.md`

### Fase 2 — Entities (L1)

Adicionar variants ao enum `Content` em `01_core/src/entities/content.rs` e respetivos métodos auxiliares (`math_binom`, `math_class`, etc.).

### Fase 3 — Stdlib (L1)

Implementar construtores nativos em `01_core/src/engine/stdlib/math_elements.rs` (novo módulo) e exportá-los em `01_core/src/engine/stdlib/mod.rs`.

### Fase 4 — Registo Global

Registar cada função em `make_root_scope` (`01_core/src/engine/eval/mod.rs`).

### Fase 5 — Layout (L3/L4)

Adicionar handlers em `01_core/src/engine/math/layout/mod.rs` para cada variant.

### Fase 6 — Testes

- Testes unitários em `01_core/src/engine/stdlib/mod.rs` (padrão P308).
- Testes de layout em `01_core/src/engine/math/layout/tests.rs`.
- Corpus P490+P500: 37/37 OK (não-regressão).
- Snippets canônicos para cada elemento.

---

## 4. Critério de Aceitação

- [ ] 6 elementos novos implementados (`binom`, `class`, `limits`, `mid`, `scripts`, `stretch`); `primes` já coberto.
- [ ] Cada elemento compila e renderiza o resultado semântico esperado.
- [ ] Corpus P490+P500: 37/37 OK.
- [ ] `cargo test`: todos passam.
- [ ] `crystalline-lint .`: 0 violations.
- [ ] Prompts L0 guardados e hashes fixados.

---

## 5. Notas

- **Não-objectivos:** `MathPrimes` separado; variações avançadas de `class`; integração com `fontdb`/math font variants.
- **Paridade:** medida ao nível da linguagem (sintaxe, semântica, morfologia), não ao nível de bytes de PDF ou estrutura interna do `Content` (ADR-0107).
