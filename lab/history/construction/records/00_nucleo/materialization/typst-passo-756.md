---
# P756 — Integrar `icu_segmenter` para CJK (confirmar fronteira L1/L3 primeiro)

> **Passo:** 756
> **Data:** 2026-07-14
> **Foco:** P755 propôs `icu_segmenter` em L1, afirmando "não faz I/O" sem confirmar directamente. Antes de adicionar a dependência, confirmar se a funcionalidade `compiled_data` embute mesmo as tabelas Unicode e o modelo LSTM como constantes estáticas (L1-seguro, sem I/O em tempo de execução), ou se exige carregar dados externos (I/O real, exigindo L3, como todas as outras dependências de dados externos já tratadas nesta conversa — WASM, `fontdb`, `hayagriva`).
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0109** (fronteira L1/L3) — verificar antes de decidir a camada.
> **Dependências:** P755 (sonda completa, mecanismo do vanilla confirmado — `icu_segmenter 2.2.0` com `CJ_SEGMENTER` customizado para aspas).

---

## Sonda mínima

### Confirmar directamente se `compiled_data` é I/O-livre

```bash
cargo tree -p icu_segmenter --features compiled_data 2>/dev/null | head -20
grep -rn "include_bytes\|std::fs\|File::open" $(find ~/.cargo -path "*icu_segmenter*/src" -type d 2>/dev/null) 2>/dev/null | head -20
```

Confirmar se os dados vêm de `include_bytes!`/dados estáticos compilados no binário (equivalente a uma constante, sem I/O), ou se há qualquer leitura de ficheiro em tempo de execução.

### Confirmar o mecanismo exacto do `CJ_SEGMENTER` customizado do vanilla

```bash
grep -n "ICU_CJ_SEGMENT\|BufferProvider\|DataProvider" lab/typst-original/crates/typst-layout/src/inline/linebreak.rs lab/typst-original/crates/typst-assets/src/*.rs 2>/dev/null | head -20
```

Confirmar se o blob de dados customizado (`typst_assets::icu::ICU_CJ_SEGMENT`) também é embutido estaticamente (mesmo padrão de `compiled_data`), ou se é carregado de outra forma.

### Critério de fecho da sonda mínima

- [ ] Confirmado directamente (não assumido) se `icu_segmenter` com `compiled_data` é I/O-livre.
- [ ] Confirmado o mecanismo do blob customizado de aspas CJ.
- [ ] Decisão de camada (L1 ou L3) tomada com base na confirmação, não em suposição.

---

## Implementação

### Se confirmado I/O-livre: L1

Adicionar `icu_segmenter` a `01_core/Cargo.toml`, seguindo o mecanismo confirmado pela sonda.

### Se exigir I/O: L3, com contrato em L1

Seguir o mesmo padrão já usado para WASM (P696-700) — trait em L1, implementação concreta em L3.

### Segmentação de linha para chinês/japonês

Implementar o mesmo mecanismo do vanilla: segmentador geral para a maioria dos casos, segmentador com tailoring de aspas (`U+201C`→OP, `U+201D`→CL) quando `lang` for `zh`/`ja`, seguindo `linebreak.rs:703` do vanilla como referência exacta.

Reaproveitar/estender `layout_word`/`layout_text` (`01_core/src/engine/layout/cursor.rs:104`, `text.rs:166`) para não tratar texto CJK como uma palavra indivisível.

### Critério de fecho da implementação

- [ ] Segmentação de linha funciona para chinês/japonês, testada com os documentos de P755.
- [ ] Tailoring de aspas replicado, testado com o caso exacto da issue #1009.
- [ ] Texto latino com espaços (mecanismo já existente) sem regressão.

---

## Validação

```bash
cat > /tmp/p756-cjk.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
EOF
./target/release/typst /tmp/p756-cjk.typ /tmp/p756-depois.pdf
mutool draw -o /tmp/p756-depois.png -r 150 /tmp/p756-depois.pdf
```

Comparar visualmente com o resultado do vanilla já obtido em P755.

```bash
cat > /tmp/p756-cjk-aspas.typ <<'EOF'
#set page(width: 7em)
#set text(font: "Noto Serif CJK SC")
测试文本，"测
EOF
./target/release/typst /tmp/p756-cjk-aspas.typ /tmp/p756-aspas-depois.pdf
```

Confirmar que a aspa de abertura não fica no início de linha.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, fronteira L1/L3 confirmada, não assumida.
- [ ] Camada correcta escolhida com base na confirmação.
- [ ] Segmentação CJK implementada e testada, incluindo o caso das aspas.
- [ ] Texto latino sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p756.md`, com hash do commit.
