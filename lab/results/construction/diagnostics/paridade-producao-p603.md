# Paridade de Produção — P603

**Data do relatório:** 2026-07-07
**Passo:** 603
**Foco:** Re-verificar `/Count` de bookmarks isolando a árvore de `/Outlines` e usando um documento de várias páginas.

---

## Resumo executivo

P602 confirmou que o cristalino emite `/Count` com os sinais e valores correctos, mas usou uma busca cega por `/Count` em todo o PDF. Esse método não distingue:

- `/Count` da raiz `/Outlines` (bookmarks).
- `/Count` do catálogo `/Pages` (número de páginas).
- Outros `/Count` que possam aparecer noutros objectos.

Neste passo, a verificação foi refeita com um parser que:

1. Localiza o catálogo (`/Type /Catalog`) e a referência `/Outlines`.
2. Percorre **apenas** os nós da árvore de bookmarks via `/First` e `/Next`.
3. Só então extrai os valores de `/Count`.

O método foi testado com um documento de várias páginas, onde `/Pages` tem `/Count 3` (cristalino) ou `/Count 2` (vanilla) — valores distintos de qualquer `/Count` de bookmark. A conclusão de P602 mantém-se: os `/Count` da árvore `/Outlines` são `{2, -2, -1}` em ambos os casos.

---

## Proveniência

- **Hash base:** `b7393b74f8c18aa970ce32bb8e1673b7eae2d294`
- **Data/hora:** 2026-07-07T23:00-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (reconstruído em release)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `python3` para validação exploratória; parser em Rust para testes

---

## Verificação

### Documento de várias páginas

```typst
= Primeira Secção
== Subsecção A
=== Sub-sub A1
#lorem(400)
== Subsecção B
#lorem(400)
= Segunda Secção
#lorem(400)
```

### Número de páginas

| Ferramenta | Páginas |
|---|---|
| Vanilla 0.15.0 | 2 |
| Cristalino | 3 |

A diferença de paginação não é o foco de P603 e não invalida a verificação de `/Count` nos bookmarks. Importa apenas porque torna óbvia a necessidade de isolar `/Outlines`: o `/Count` de `/Pages` já não coincide por acaso com nenhum `/Count` de bookmark.

### Busca cega vs parser isolado

Busca cega em `/tmp/p603-cristalino.pdf`:

```text
/Count 3   ← /Pages
/Count -1  ← Subsecção A
/Count -2  ← Primeira Secção
/Count 2   ← /Outlines
```

Parser isolado (`outline_counts`):

```text
[-2, -1, 2]
```

O parser elimina o `/Count 3` de `/Pages` e devolve apenas os valores da árvore de bookmarks.

### Validação contra o vanilla

Aplicando o mesmo parser a `/tmp/p603-vanilla.pdf`:

```text
[-2, -1, 2]
```

Interpretação:

- `/Count 2` → raiz `/Outlines` (dois itens de topo).
- `/Count -2` → `Primeira Secção` (dois filhos directos).
- `/Count -1` → `Subsecção A` (um filho directo).

Os valores coincidem com o cristalino.

---

## Implementação do método de verificação

O parser foi adicionado como helper nos testes de integração em `03_infra/src/integration_tests.rs`:

- `parse_pdf_objects` — extrai objectos `N 0 obj << ... >> endobj` com suporte para strings literais `(...)` e hex `<...>`.
- `outline_counts` — localiza o catálogo, segue `/Outlines`, percorre a árvore via `/First`/`/Next` e extrai `/Count`.
- `assert_outline_counts` — compara os valores obtidos com os esperados.

Foram mantidos e melhorados os dois testes de P602, que deixaram de usar `String::contains` e passaram a usar o parser isolado. Foi adicionado um terceiro teste, `p603_outline_count_isolado_em_documento_multipagina`, que usa o documento acima.

Adicionou-se `regex` a `[dev-dependencies]` de `03_infra/Cargo.toml` para facilitar o parsing.

---

## Validação

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

Os testes específicos passaram:

- `p602_outline_count_sinal_negativo_para_entradas_com_filhos`
- `p602_outline_count_tres_niveis_conta_filhos_directos`
- `p603_outline_count_isolado_em_documento_multipagina`

---

## Critérios de fecho do passo

- [x] Método de verificação isola a árvore de `/Outlines`, sem misturar com `/Pages`.
- [x] Testado com documento de várias páginas.
- [x] Conclusão de P602 confirmada com o método limpo.
- [x] `cargo test --workspace` sem falhas.
- [x] `crystalline-lint .` limpo.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-603.md` — passo que originou esta verificação.
- `00_nucleo/diagnosticos/paridade-producao-p602.md` — relatório P602 cuja conclusão foi reconfirmada.
- `03_infra/src/integration_tests.rs` — helpers e testes de `/Count` isolado.
