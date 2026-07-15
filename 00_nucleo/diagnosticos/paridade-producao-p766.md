# Diagnóstico P766 — Verificação de completude: tabelas de símbolos além de `arrow`

**Data da medição:** 2026-07-15T17:00:00-03:00  
**Commit base:** `f36ca1abe2f95cffc2cf6fd2b3cf4947d2d9f846`  
**Working tree:** modificado (alterações de P765b e P766 em progresso)  
**Passo:** P766  
**Objectivo:** Medir a cobertura dos grupos de símbolos do vanilla vs cristalino, identificar lacunas desproporcionadas, cruzar com uso real no corpus e expandir os grupos com uso confirmado.

---

## 1. Levantamento no vanilla

O vanilla gera os símbolos via crate `codex` em build-time. A tabela `SYM` foi extraída de:

```text
lab/typst-original/target/release/build/codex-06e86e84ead2f1f1/out/out.rs
```

Contagem (top-level, incluindo sub-itens dos módulos `control` e `gender`):

| Métrica | Valor |
|---------|-------|
| Itens top-level em `SYM` | 1071 |
| Grupos/distinct names top-level | ~315 |
| `Symbol::Single` | 781 |
| `Symbol::Multi` | 288 |
| Módulos aninhados (`control`, `gender`) | 2 |

Top 10 grupos por número de variantes no vanilla:

| Grupo | Variant |
|-------|---------|
| `arrow` | 154 |
| `face` | 105 |
| `gt` | 33 |
| `lt` | 33 |
| `harpoon` | 32 |
| `triangle` | 27 |
| `clock` | 27 |
| `tack` | 25 |
| `eq` | 23 |
| `heart` | 23 |

---

## 2. Cobertura actual no cristalino (antes de P766)

O cristalino tinha:

| Métrica | Valor |
|---------|-------|
| Grupos implementados | 57 |
| Entradas simples (`SYM_SIMPLE`) | 60 |
| Variant do grupo `arrow` | 52 |

Grupos com cobertura >0% (antes de P766):

| Grupo | Vanilla | Cristalino | Cobertura |
|-------|---------|------------|-----------|
| `arrow` | 154 | 52 | 33.8% (subset matemático) |
| `alpha`–`omega` | 1 cada | 1 cada | 100% |
| `eq` | 23 | 2 | 8.7% |
| `gt` | 33 | 2 | 6.1% |
| `lt` | 33 | 2 | 6.1% |
| `plus` | 13 | 1 | 7.7% |
| `times` | 14 | 1 | 7.1% |
| `dot` | 9 | 1 | 11.1% |
| `subset` / `supset` | 20 cada | 1 cada | 5% |
| `union` | 15 | 1 | 6.7% |
| `integral` | 19 | 1 | 5.3% |
| ... | ... | ... | ... |

A esmagadora maioria dos grupos estava a 0%.

---

## 3. Priorização por uso real no corpus

Pesquisa por `sym.xxx` no corpus do projeto (`lab/typst-original/tests/`, `00_nucleo/testing/`, `benches/`):

```bash
grep -rn "sym\." lab/typst-original/tests/ 00_nucleo/testing/ benches/ \
  | grep -o 'sym\.[a-zA-Z0-9_.]*' | sort | uniq -c | sort -rn | head -20
```

| Uso | Ocorrências |
|-----|-------------|
| `sym.tilde` | 19 |
| `sym.integral` | 3 |
| `sym.chevron` | 3 |
| `sym.suit.heart` | 2 |
| `sym.arrow.l` | 2 |
| `sym.tack` | 1 |
| `sym.space.nobreak` | 1 |
| `sym.gt.eq.tri.not` | 1 |
| `sym.emptyset` | 1 |
| `sym.diamond.small` | 1 |
| `sym.bracket.stroked.r` | 1 |
| `sym.amp.inv` | 1 |
| `sym.plus.o` | 1 |

Os usos `sym.push` e `sym.pop` (2 ocorrências cada) não pertencem ao módulo `sym` do vanilla — são variáveis de teste, por isso foram ignorados.

---

## 4. Implementação (expansão por uso confirmado)

Foram transformados em grupos com `Symbol::with_variants`:

| Grupo | Variant no vanilla | Implementadas | Uso que justifica |
|-------|-------------------|---------------|-------------------|
| `tilde` | 13 | 13 (completo) | 19 ocorrências |
| `integral` | 19 | 19 (completo) | 3 ocorrências |
| `chevron` | 10 | 10 (completo) | 3 ocorrências |
| `suit` | 4 | 4 (completo) | `sym.suit.heart` |
| `tack` | 25 | 25 (completo) | `sym.tack` |
| `space` | 13 | 13 (completo) | `sym.space.nobreak` |
| `emptyset` | 7 | 7 (completo) | `sym.emptyset` |
| `bracket` | 10 | 10 (completo) | `sym.bracket.stroked.r` |
| `amp` | 2 | 2 (completo) | `sym.amp.inv` |
| `plus` | 13 | 13 (completo) | `sym.plus.o` |
| `gt` | 33 | 33 (completo) | `sym.gt.eq.tri.not` |
| `diamond` | 5 | 5 (completo) | `sym.diamond.small` |

> Nota: o grupo `arrow` do cristalino continua com 52 variantes focadas em setas matemáticas; as restantes 102 do vanilla são maioritariamente emoji (↩️, 🔜, etc.), que não têm uso identificado no corpus e foram mantidas como scope-out consciente.

### Ficheiros alterados

- `01_core/src/rules/stdlib/sym.rs` — reestruturado para `SYM_SIMPLE` + `SYM_GROUPS`; adicionadas 12 funções de variantes.
- `01_core/src/entities/math_style.rs` — não alterado por P766.
- `00_nucleo/prompts/rules/stdlib/sym.md` — L0 actualizado.

---

## 5. Validação

### Testes unitários novos

Em `01_core/src/rules/stdlib/sym.rs`:

- `sym_lookup_tilde_modifier`
- `sym_lookup_integral_modifier`
- `sym_lookup_chevron_modifier`
- `sym_lookup_suit_modifier`
- `sym_lookup_tack_modifier`
- `sym_lookup_space_modifier`
- `sym_lookup_emptyset_modifier`
- `sym_lookup_bracket_modifier`
- `sym_lookup_amp_modifier`
- `sym_lookup_plus_o`
- `sym_lookup_gt_eq_tri_not`
- `sym_lookup_diamond_small`

### Teste de integração CLI

```typst
#sym.tilde.equiv, #sym.integral.double, #sym.chevron.l, #sym.suit.heart,
#sym.tack.r, #sym.space.nobreak, #sym.emptyset.rev, #sym.bracket.l.stroked,
#sym.amp.inv, #sym.plus.o, #sym.diamond.small, #sym.gt.eq.tri.not
```

Resultado (extraído do PDF): `≅, ∬, ⟨, ♥, ⊢, , ⦰, ⟦, ⅋, ⊕, 🔹, ⋭`

(o `space.nobreak` é um espaço invisível, como esperado).

### Checks globais

- `cargo test --workspace` — verde.
- `crystalline-lint .` — zero violações (exceto V7 esperado de `package_version_resolution.md`).

---

## 6. Scope-out consciente

Grupos com cobertura 0% e **sem uso identificado** no corpus actual ficam como scope-out. Exemplos (não exaustivo):

- `face` (105 variantes, emoji)
- `harpoon` (32 variantes)
- `triangle` (27 variantes)
- `clock` (27 variantes)
- `heart` (23 variantes)
- `person` (22 variantes)
- Todo o conjunto de emoji (`cat`, `food`, `transport`, `flags`, etc.)

Se algum destes grupos for necessário para documentos futuros, deve ser introduzido num passo P767+ com a mesma metodologia (uso real → implementação).

---

## 7. Conclusão

- A tabela de cobertura vanilla vs cristalino foi construída com contagens reais.
- Foram identificados 12 grupos com uso real no corpus e implementados (completo ou com as variantes usadas).
- A cobertura de símbolos do cristalino aumentou de ~57 para ~69 grupos top-level, mantendo a filosofia de não implementar emoji/símbolos sem uso identificado.
- O mecanismo de `Symbol::with_variants` + `Symbol::modified` provou-se genérico o suficiente para absorver novos grupos sem alterar a arquitetura.
