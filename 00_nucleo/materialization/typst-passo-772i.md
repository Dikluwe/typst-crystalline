---
# P772i — Implementar `split_header_footer`: `grid.header`/`grid.footer` como row-groups reais

> **Passo:** 772i
> **Data:** 2026-07-16
> **Foco:** P772h confirmou que `header:`/`footer:` como argumentos nomeados foi um caminho simplificado adoptado em P224 (maio 2026) por inconsistência do próprio plano da época, com o gap conhecido e registado como "graded" mas nunca retomado. P772f identificou, independentemente, o scope-out #16 (`ResolvableGridChild` sem equivalente — `Content::GridHeader`/`GridFooter` caem no braço genérico do loop de `grid()`, tratados como célula normal). As duas causas são a mesma: falta o conceito de "row-group" distinto de célula. Este passo implementa `split_header_footer` como o plano original de P224 especificava, resolvendo os dois gaps de uma vez.
> **Tipo:** Implementação directa (causa e solução já especificadas no plano original de P224; este passo materializa o que ficou por fazer).
> **Tamanho:** L — toca o loop de resolução de `grid()`, a distinção de row-groups, e potencialmente repeat-across-páginas (itens #9/#12/#14 de P772f, `Header`/`Footer`/`Repeatable<T>`).
> **ADR-0108 EM VIGOR** — confirmar a especificação exacta do vanilla antes de implementar, não assumir do plano antigo de P224 (que já provou ter uma inconsistência interna).
> **Regra 1 do handoff** — remover o argumento nomeado `header:`/`footer:` de `grid()`/`table()` faz parte desta implementação (não é opcional manter os dois caminhos).
> **Dependências:** P772h (arqueologia, commit `0af55e792` identificado como origem), P772f (itens #4, #6, #8, #9, #12, #14, #16, #20, #21 do módulo `grid::resolve`, todos decorrentes desta mesma lacuna-raiz).

---

## Sonda — especificação real do vanilla (não o plano antigo de P224)

```bash
grep -n "#\[elem(name = \"header\"\|#\[elem(name = \"footer\"" lab/typst-original/crates/typst-library/src/layout/grid/mod.rs
```

Confirmar a API exacta: `grid.header(repeat: bool, level: int, ...)[conteúdo]` como elemento-filho, não argumento nomeado da função `grid()`. Ler `ResolvableGridChild`, `Header`, `Footer`, `Repeatable<T>` (itens #16, #9, #12, #14 de P772f) no vanilla para confirmar a estrutura de dados alvo:

```bash
sed -n '427,700p' lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs
```

---

## Implementação

### 1. Remover `header:`/`footer:` como argumentos nomeados

Em `01_core/src/engine/stdlib/layout.rs` (`native_grid`/`native_table`), remover a leitura de `args.named.get("header"/"footer")`. Confirmar que `#grid(header: ...)` passa a dar erro `unexpected argument`, replicando o vanilla.

### 2. `Content::GridHeader`/`GridFooter` como row-group distinto

No loop de resolução de `grid()` (`01_core/src/engine/stdlib/layout.rs`, braço genérico `other => cells.push(...)` identificado por P772f #16), distinguir `Content::GridHeader`/`GridFooter` de células normais, agrupando-os como row-groups com `range`/`level`, conforme a estrutura do vanilla.

### 3. `split_header_footer` real

Implementar a função que separa filhos do `grid()` em `(header, footer, cells)`, conforme o plano original de P224 (secção C4) especificava — mas verificado agora contra o vanilla real, não copiado do plano sem confirmar.

### 4. Repeat-across-páginas (itens #9, #12, #14)

Avaliar se entra no âmbito deste passo ou fica scope-out consciente e nomeado (não silencioso): `GridHeaderElem`/`GridFooterElem` actualmente só renderizam uma vez (DEBT-56, conforme P772f). Se o esforço for grande, registar como scope-out explícito com decisão registada, não deixar como "mais um gap descoberto por acidente" da próxima vez.

---

## Validação

```bash
cat > /tmp/p772i-grid-header.typ <<'EOF'
#grid(
  columns: 2,
  grid.header[Nome][Idade],
  [Ana], [30],
  [Bruno], [25],
)
EOF
lab/typst-original/target/release/typst compile /tmp/p772i-grid-header.typ /tmp/p772i-vanilla.pdf
./target/release/typst compile /tmp/p772i-grid-header.typ /tmp/p772i-cristalino.pdf
mutool trace /tmp/p772i-vanilla.pdf > /tmp/p772i-trace-vanilla.txt
mutool trace /tmp/p772i-cristalino.pdf > /tmp/p772i-trace-cristalino.txt
```

Confirmar por coordenadas, não só ausência de erro. Se repeat-across-páginas estiver no âmbito, testar com um documento longo o suficiente para quebrar página, confirmando que o header se repete.

```bash
# Confirmar que header: como argumento nomeado agora dá erro, replicando o vanilla
cat > /tmp/p772i-header-nomeado.typ <<'EOF'
#grid(header: [Nome])
EOF
lab/typst-original/target/release/typst compile /tmp/p772i-header-nomeado.typ 2>&1
./target/release/typst compile /tmp/p772i-header-nomeado.typ 2>&1
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] `header:`/`footer:` removidos como argumentos nomeados; `#grid(header: ...)` dá erro consistente com o vanilla.
- [x] `split_header_footer` implementado, distinguindo row-groups no loop de resolução.
- [x] `grid.header(...)`/`grid.footer(...)` como filhos renderizam corretamente, confirmado por coordenadas.
- [x] Decisão registada sobre repeat-across-páginas — **scope-out explícito**, não implementado (requer `range`/`level`/`short_lived` + lógica de re-emissão consciente de paginação; ver relatório).
- [x] Itens #4, #6, #8, #16, #20, #21 de P772f reavaliados — só #16 passa a "mecânica-diverge"; os restantes permanecem scope-out (dependem de repeat-across-páginas).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] L0 de `grid`/`stdlib/layout` actualizado (hash `9631382f`), removendo a menção aos argumentos nomeados inventados.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772i.md`.

Achado adicional confirmado durante a implementação: `native_grid_header`/
`native_grid_footer`/`native_table_header`/`native_table_footer` só guardavam
`args.items.first()` — `grid.header[Nome][Idade]` perdia "Idade" silenciosamente.
Corrigido junto com o mecanismo de row-group (mesma correcção, mesmo commit).

---

## Próximo passo

Retomar a varredura da stdlib (`visualize::image::svg`, `foundations::scope`, `text::font::*`), em paralelo ou depois de P772j (código órfão de align+place).
