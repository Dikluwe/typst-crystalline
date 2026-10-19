# Paridade de Produção — P602

**Data do relatório:** 2026-07-07
**Passo:** 602
**Foco:** Distinguir `/Count` aberto/fechado nos bookmarks PDF.

---

## Resumo executivo

O `/Count` de cada bookmark com filhos indica quantos descendentes são apresentados por defeito quando o PDF é aberto: sinal **positivo** = aberto, **negativo** = fechado. A sonda ao vanilla 0.15.0 mostra que:

- A raiz `/Outlines` usa `/Count N` **positivo**, onde `N` é o número de bookmarks de topo.
- Cada bookmark com filhos usa `/Count -N` **negativo**, onde `N` é o número de **filhos directos**.
- Bookmarks sem filhos não têm `/Count`.

O cristalino já emitia a árvore `/Outlines` desde P535, mas sem `/Count`. Este passo adiciona o campo com o sinal e valor correctos. A medição directa contra o vanilla confirma paridade para hierarquias de dois e três níveis.

Conclusão: disparidade `/Count` aberto/fechado está corrigida.

---

## Proveniência

- **Hash base:** `d278dfd0045091707ce93becb3b05caf50ab9fa1`
- **Data/hora:** 2026-07-07T22:54-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (reconstruído em release)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `python3` para extrair `/Count` dos bytes PDF

---

## Sonda

### Documento de três níveis

```typst
= Primeira Secção
== Subsecção A
=== Sub-sub A1
== Subsecção B
= Segunda Secção
```

### Vanilla 0.15.0

```bash
lab/typst-original/target/release/typst compile /tmp/p602-headings.typ /tmp/p602-vanilla.pdf
python3 -c "
import re
data = open('/tmp/p602-vanilla.pdf', 'rb').read()
for m in re.finditer(rb'/Count\s+(-?\d+)', data):
    print(m.group().decode())
"
```

**Resultado:**

```text
/Count 1
/Count -1
/Count -2
/Count 2
```

Interpretação:

- `/Count 2` → raiz `/Outlines` (dois itens de topo).
- `/Count -2` → `Primeira Secção` (dois filhos directos: `Subsecção A`, `Subsecção B`).
- `/Count -1` → `Subsecção A` (um filho directo: `Sub-sub A1`).
- `/Count 1` → `Sub-sub A1` (sem filhos? este `/Count` pertence ao catálogo `/Pages`; a raiz de páginas também usa `/Count`).

A separação entre `/Count` de `/Outlines` e `/Count` de `/Pages` foi confirmada por inspecção do dicionário: o único `/Count` negativo no vanilla provém de bookmarks com filhos.

### Cristalino antes da correcção

```bash
./target/release/typst /tmp/p602-headings.typ /tmp/p602-cristalino-antes.pdf
python3 -c "
import re
data = open('/tmp/p602-cristalino-antes.pdf', 'rb').read()
for m in re.finditer(rb'/Count\s+(-?\d+)', data):
    print(m.group().decode())
"
```

**Resultado:** apenas `/Count N` positivo da árvore `/Pages`; `/Outlines` não continha `/Count`.

---

## Implementação

Ficheiro alterado: `03_infra/src/export/builder.rs`.

- Adicionado campo `child_count` à struct interna `Node` usada em `emit_outlines`.
- Incrementado `child_count` sempre que um nó é adicionado como filho de outro.
- Emite `/Count -{child_count}` apenas quando `child_count > 0`.
- Emite `/Count {top_count}` positivo no dicionário raiz `/Outlines`, onde `top_count` é o número de nós sem pai.

O Prompt L0 correspondente (`00_nucleo/prompts/infra/export/builder.md`, secção §P535) foi actualizado para reflectir a semântica de `/Count`, e o `@prompt-hash` de `builder.rs` foi recalculado com `crystalline-lint --fix-hashes .` → `27be9535`.

Foram adicionados dois testes de integração em `03_infra/src/integration_tests.rs`:

- `p602_outline_count_sinal_negativo_para_entradas_com_filhos`
- `p602_outline_count_tres_niveis_conta_filhos_directos`

Durante a escrita dos testes, descobriu-se que o helper `compile_to_pdf` não transportava `extracted_headings` do introspector para o `PagedDocument`, pelo que os bookmarks não estavam a ser gerados nesse caminho de teste. O helper foi corrigido para usar `introspect_with_introspector` e popular `doc.extracted_headings`, alinhando o teste com a pipeline de produção.

---

## Validação

### Comparação directa (depois da correcção)

```bash
./target/release/typst /tmp/p602-headings.typ /tmp/p602-cristalino-depois.pdf
python3 -c "
import re
data = open('/tmp/p602-cristalino-depois.pdf', 'rb').read()
for m in re.finditer(rb'/Count\s+(-?\d+)', data):
    print(m.group().decode())
"
```

**Resultado cristalino:**

```text
/Count 1
/Count -2
/Count -1
/Count 2
```

Conjunto idêntico ao vanilla: `{1, -1, -2, 2}`. A ordem difere porque a alocação de object IDs segue o caminho de build do cristalino, mas os valores semânticos são os mesmos.

### Testes automatizados

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

O snapshot `03_infra/fixtures/p307b/reference/07-multi-feature.pdf` foi regenerado com `UPDATE_P307B_SNAPSHOTS=1` porque o documento contém headings; a alteração de `/Count` nos bookmarks modificou os bytes de referência.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - `/Count` aberto/fechado: estado alterado de **Scope-out** para **Fechado em P602**.

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - `/Count` de bookmarks aberto/fechado removido da secção "Scope-out deliberado".
  - `/Count` adicionado à secção "Corrigido ao longo desta conversa" com a razão de P602.

---

## Critérios de fecho do passo

- [x] Comportamento do vanilla confirmado por leitura directa do PDF.
- [x] `/Count` implementado no cristalino com sinal e valor correctos.
- [x] Testado com hierarquia de três níveis.
- [x] `cargo test --workspace` sem falhas.
- [x] `crystalline-lint .` limpo.
- [x] Listas de disparidades actualizadas.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-602.md` — passo que originou esta sonda.
- `00_nucleo/prompts/infra/export/builder.md` — Prompt L0 actualizado (secção §P535).
- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` — lista de decisões actualizada.
- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` — estado das disparidades actualizado.
- `03_infra/src/export/builder.rs:1064` — `emit_outlines` com `/Count`.
