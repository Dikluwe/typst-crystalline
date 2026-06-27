---

# P479 — Sonda de paridade `lab/parity/` + gaps identificados

> **Passo:** 479
> **Data:** 2026-06-27
> **Foco:** (A) Executar a suite `lab/parity/` actualizada contra o corpus de 36 ficheiros e registar diffs cristalino vs vanilla pós-P465–P477; (B) Materializar até 2 gaps de paridade S identificados pela sonda.
> **Tipo:** Sonda-first + Materialização condicional (S–M).
> **Tamanho:** S (sonda) + S–M (materialização, condicional).
> **ADR-0075 EM VIGOR** — Vanilla integration via pre-built CLI + comparação estrutural. Paridade medida por semântica/comportamento observable (ADR-0107), não por diff de bytes PDF.

---

## Contexto

O projecto está no estado mais limpo desde o início: zero DEBTs activos, Trilhas 1–4 e 7–8 completas, P295.1/P295.2 fechados. A última corrida de paridade documentada foi P206D (2026-05-08, 20/36 INCLUDE matches). Desde então, P465–P477 adicionaram funcionalidade significativa que pode ter fechado diffs antigos e, possivelmente, introduzido diffs novos.

A matriz `lab/parity/reports/latest.md` reflecte o estado de Maio 2026. Este passo actualiza-a com o estado real de Junho 2026.

---

## Sub-item A — Sonda de paridade

### A.1 — O que executar

```bash
# 1. Verificar versão do vanilla CLI (deve ser 0.14.2)
typst --version

# 2. Correr suite lab/parity completa
cd lab/parity
RUST_MIN_STACK=33554432 cargo test --test structural_parity -- --nocapture
RUST_MIN_STACK=33554432 cargo test --test consolidado_p206d -- --nocapture

# 3. Ler o relatório gerado
cat lab/parity/reports/latest.md
```

### A.2 — Matriz esperada

A sonda deve produzir uma tabela actualizada de 36 ficheiros com estado para cada:
- `✓ match` — cristalino e vanilla produzem output estruturalmente idêntico.
- `✗ diff` — divergência registada.
- `SKIP` — ficheiro excluído da comparação (pré-existente).

| Ficheiro (exemplo) | Estado P206D | Estado esperado P479 |
|-------------------|--------------|----------------------|
| `markup/headings.typ` | ✓ match | ✓ match (Trilha 1 fechada) |
| `markup/bibliography.typ` | ✗ diff | ✓ match (P468-P473) |
| `semantic/show-where.typ` | SKIP-feature | ✓ match (P417/P474) |
| `visual/gradient.typ` | ✗ diff? | ✓ match (P262-P270) |
| `code/let.typ` | SKIP-pre | SKIP-pre |

### A.3 — Critério de avaliação

Os 3 `INCLUDE-com-diff` documentados em `lab/parity/SKIPS.md` pós-P206D:
1. `equation` selector namespace — verificar se ainda diverge.
2. `cite-bibliography` — P468-P473 materializou estilos numéricos; verificar se fecha.
3. `outline-toc` TOC entries — verificar se P457 já alinha.

Se algum destes fechou, remover do SKIPS.md e actualizar contagem.

### A.4 — Corpus SKIP-feature

10 ficheiros `semantic/` estavam com SKIP-feature (selectores não suportados). Verificar se P467/P474 (`Selector::Where`, `#show regex`) permite INCLUDE alguns deles.

---

## Sub-item B — Materializar gaps S identificados

A sonda determinará o que está em diff. Este sub-item materializa até 2 gaps de magnitude S que a sonda revelar. Os candidatos prováveis (baseados no conhecimento do projecto):

### B.1 — `equation` selector namespace

Se `equation` vs `math.equation` ainda diverge, a correcção é registar `math.equation` como alias no eval:

```rust
// Em eval/mod.rs ou rules/stdlib/math.rs:
// Após registar "equation" no scope como função nativa:
// Registar também via módulo "math":
let mut math_module = Dict::new();
math_module.insert("equation", scope.get("equation").cloned().unwrap_or(Value::None));
scope.define("math", Value::Module(math_module));
```

Tamanho: XS.

### B.2 — `outline-toc` TOC entries diff

Se o TOC do cristalino conta entradas de forma diferente do vanilla, verificar:
- `Introspector::query_kind(ElementKind::Heading)` devolve o mesmo número que vanilla `typst query heading`.
- Se outline scope filtra por `outlined: true` — verificar se cristalino filtra o mesmo.

Tamanho: XS–S.

### B.3 — Qualquer outro gap S identificado

A sonda pode revelar gaps em funcionalidades adicionadas entre P465 e P477 (Symbol, Color operators, etc.) que não tinham cobertura no corpus. Se um gap novo S é revelado, materializar.

### B.4 — Gaps M ou acima

Registar no SKIPS.md ou como DEBT, não materializar neste passo.

---

## Protocolo de actualização pós-sonda

1. Actualizar `lab/parity/reports/latest.md` com nova matriz.
2. Versionado em `lab/parity/reports/2026-06-27-passo-479.md`.
3. Actualizar `lab/parity/SKIPS.md` se algum SKIP-feature foi resolvido.
4. Actualizar sentinela `p206d_corpus_cobertura_minima` se threshold aumentou.
5. Se `INCLUDE-com-diff` passou a `match`, remover entrada do SKIPS.md.

---

## Tests

### Sub-item A

- Execução da suite existente (sem novos testes de código).
- `p479_corpus_paridade_actualizado` — sentinela nova que verifica: (a) matriz escrita sem panic; (b) cobertura INCLUDE ≥ 23 (threshold pós-P206D).

### Sub-item B (condicional à sonda)

- Testes específicos para cada gap materializado.
- Se `math.equation` alias: teste `eval_math_equation_via_module`.
- Se outline TOC: teste `query_heading_count_paridade_vanilla`.

---

## Spec L0

### Sub-item A

- `lab/parity/reports/2026-06-27-passo-479.md` — relatório versionado.
- `lab/parity/SKIPS.md` — actualizado se SKIPs removidos.

### Sub-item B (condicional)

- Ficheiro L0 relevante para cada gap materializado.
- ADR-0075 — anotação cumulativa P479 (se threshold de cobertura subiu).

---

## Critério de fecho

- [ ] `typst --version` confirma 0.14.2.
- [ ] Suite `lab/parity/` executa sem panic.
- [ ] Matriz P479 produzida em `latest.md` + `2026-06-27-passo-479.md`.
- [ ] Contagem INCLUDE ≥ 23 (threshold P206D) — preferência ≥ 28 se diffs antigos fecharam.
- [ ] 3 `INCLUDE-com-diff` P206D verificados: estado actual documentado.
- [ ] 10 SKIP-feature verificados: algum pode ser INCLUDE pós-P474?
- [ ] Até 2 gaps S materializados (se identificados pela sonda).
- [ ] `lab/parity/SKIPS.md` actualizado.
- [ ] Sentinela `p479_corpus_paridade_actualizado` verde.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] ADR-0075 anotação cumulativa P479 redigida.

---

## Cenários possíveis pós-sonda

| Resultado | Acção |
|-----------|-------|
| ≥ 30/36 INCLUDE match | Projecto em estado de paridade excelente; P480 pode ser épico Trilha 5 ou audit final |
| 23–29 INCLUDE match | Materializar gaps S identificados; continuar paridade em P480 |
| < 23 INCLUDE match | Identificar regressões; prioridade máxima para correcção |
| SKIP-feature → INCLUDE | Actualizar SKIPS.md; threshold sobe |

---

## Próximo passo

Dependendo do resultado da sonda:

- **Se paridade ≥ 30/36:** P480 inicia Épico Trilha 5 (shaping rustybuzz) ou declara o projecto em estado de consolidação.
- **Se gaps S–M identificados:** P480 materializa os seguintes da lista por prioridade.
- **Se regressão encontrada:** P480 corrige antes de qualquer nova feature.

---

## Estado pós-P478 (para referência)

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| Trilhas pendentes | 5 (épico XL), 6 (4/5) |
| P295 Footnotes | FECHADO (P304/P305) |
| ADR-0083 operadores cor | TOTALMENTE FECHADO |
| Última corrida paridade | P206D (2026-05-08, 20/36) |
| **P479** | Paridade actualizada + gaps S | 🔄 EM PREPARAÇÃO |

**Inventário de débitos: LIMPO.**
**Zero regressões conhecidas.**
**Objectivo P479: actualizar baseline de paridade para reflectir estado real pós-P465–P477.**
