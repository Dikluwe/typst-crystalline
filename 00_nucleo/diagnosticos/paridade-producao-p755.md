# Relatório de Paridade — P755

**Passo:** 755  
**Data:** 2026-07-14  
**Foco:** Sonda da quebra de linha para scripts sem espaços (CJK, Thai) no vanilla 0.15.0 e no cristalino.  
**Hash do commit:** baa7d47a1

---

## Resumo Executivo

P755 investigou como o vanilla 0.15.0 e o cristalino tratam a quebra de linha para CJK e Thai. A principal conclusão é que o vanilla 0.15.0 **não usa `xi-unicode` puro** como as issues antigas sugerem: migrou para `icu_segmenter` 2.2.0, com um segmentador geral LSTM e um segmentador dedicado para Chinês/Japonês (CJ) que faz *tailoring* das aspas `U+201C`/`U+201D` para evitar que fiquem no início/fim de linha (problema da issue `typst/typst#1009`).

Para Thai, o vanilla usa o mesmo segmentador LSTM geral, que **não** tem dicionário de verdade: quebra em posições de carácter/sílaba, ocasionalmente cortando palavras.

O cristalino, por outro lado, ainda não tem segmentação de linha para scripts sem espaços. O `layout_word` trata cada unidade entre espaços como indivisível; CJK e Thai sem espaços são portanto palavras únicas que, quando excedem a largura, são despejadas para a linha seguinte ou cortadas na margem.

A paridade com o vanilla exige:

1. **CJK**: introduzir `icu_segmenter` (ou equivalente) e aplicar o *tailoring* de aspas para chinês/japonês.
2. **Thai**: usar o segmentador LSTM do `icu_segmenter` para obter quebras em posições de sílaba, tal como o vanilla faz hoje.

Estas são funcionalidades novas, nunca tocadas no cristalino. Recomenda-se dividir em passos separados (CJK e Thai), começando por CJK por ter impacto visual maior e exemplos de teste mais simples.

---

## Sonda

### Documentos de teste

```bash
cat > /tmp/p755-cjk.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
EOF

cat > /tmp/p755-cjk-aspas.typ <<'EOF'
#set page(width: 7em)
#set text(font: "Noto Serif CJK SC")
测试文本，"测
EOF

cat > /tmp/p755-thai.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Sans Thai", size: 12pt)
สวัสดีครับผมชื่อจอห์นยินดีที่ได้รู้จักคุณ
EOF
```

### Comportamento do vanilla 0.15.0

#### CJK — texto corrido

O vanilla quebrou o texto em várias linhas, respeitando oportunidades entre caracteres CJK:

```
测试文本，“测试
引号的位置”。这
是一段很长的中
文文字用来测试
换行的效果如何。
```

A aspa de abertura `"` não ficou no início de linha.

#### CJK — caso exacto da issue #1009

Documento `p755-cjk-aspas.typ`:

```
测试文
本，“测
```

A aspa de abertura ficou no **final** da segunda linha, não no início da terceira. O vanilla 0.15.0 evita o problema descrito na issue #1009.

#### Thai

O vanilla quebrou o Thai em três linhas:

```
สวัสดีครับผม
ชื่อจอห์นยินดีที่ได้
รู้จักคุณ
```

A segunda linha corta após `ได้`, o que não é necessariamente uma fronteira de palavra em Thai. O comportamento é consistente com segmentação por regras/sílabas, não por dicionário.

### Comportamento do cristalino

#### CJK — texto corrido

```
测试文本，
"
测试引号的位置
"
。这是一段很长的
```

O cristalino não segmenta o texto CJK. O texto entre espaços é tratado como uma única palavra; quando não cabe, é despejado para a linha seguinte. A pontuação (`，`, `"`, `。`) foi separada em linhas próprias, o que é inaceitável tipograficamente.

#### CJK — caso exacto da issue #1009

Texto extraído do PDF:

```
测
"
测
```

A aspa de abertura ficou sozinha numa linha (no limite, no início da segunda linha visível). Confirma que o cristalino não tem tratamento de kinsoku/aspas CJK.

#### Thai

O cristalino não quebrou o Thai:

```
สวัสดีครับผมชื่อจอห์ห
```

O texto foi truncado na margem direita sem qualquer quebra.

---

## Mecanismo do vanilla

### Dependências

`lab/typst-original/crates/typst-layout/Cargo.toml`:

```toml
icu_segmenter = { workspace = true }
icu_properties = { workspace = true }
icu_provider_blob = { workspace = true }
```

`lab/typst-original/Cargo.toml.original`:

```toml
icu_segmenter = { version = "2.2.0", features = ["serde"] }
```

### Código em `linebreak.rs`

Dois segmentadores são criados como `LazyLock`:

1. `SEGMENTER` — `LineSegmenter::new_lstm(LineBreakOptions::default())`, segmentador geral LSTM.
2. `CJ_SEGMENTER` — segmentador com *blob* de dados customizado (`typst_assets::icu::ICU_CJ_SEGMENT`) que sobrescreve as propriedades `LineBreak` de `U+201C` (`“`) para `OP` (`OpenPunctuation`) e `U+201D` (`”`) para `CP` (`CloseParenthesis`), evitando aspas no início/fim de linha.

Escolha em tempo de execução (`crates/typst-layout/src/inline/linebreak.rs:703`):

```rust
let segmenter = match p.config.lang {
    Some(Lang::CHINESE | Lang::JAPANESE) => CJ_SEGMENTER.as_borrowed(),
    _ => *SEGMENTER,
};
```

Ou seja:
- Chinês e japonês usam o segmentador com tailoring de aspas.
- Todas as outras linguagens (incluindo Thai, coreano, etc.) usam o segmentador LSTM geral.

### Implicações

- O vanilla não implementa kinsoku completo; apenas o tailoring mínimo de aspas para C/J.
- Thai é tratado pelo LSTM geral, não por dicionário. Isso significa que "paridade" para Thai é replicar esse comportamento LSTM, não uma solução perfeita de dicionário.
- Coreano (`ko`) usa o segmentador geral, não o CJ (embora também useja ideogramas e tenha regras de quebra específicas).

---

## Estado actual do cristalino

### `layout_word` em `01_core/src/rules/layout/cursor.rs:104`

O cristalino trata cada chamada a `layout_word(word)` como uma unidade indivisível:

```rust
pub(super) fn layout_word(&mut self, word: &str) {
    let w = self.metrics.text_width(word, self.style.size, &self.style);
    let right_margin = self.regions.current.width - self.page_config.margin;
    if self.regions.current.cursor_x.0 + w.0 > right_margin
        && self.regions.current.cursor_x.0 > self.page_config.margin
    {
        // ... tenta hyphenation se lang estiver definido ...
        self.flush_line();
    }
    self.push_text(word.into(), w);
    self.regions.current.cursor_x += w;
}
```

### `layout_text` em `01_core/src/rules/layout/text.rs:166`

O texto é dividido por espaços (`text.split(' ')`) e cada parte é passada a `layout_word`. CJK e Thai não usam espaços entre palavras, pelo que o texto inteiro vai como uma única parte.

### Consequência

- CJK: o cristalino só quebra quando a linha inteira de caracteres sem espaços excede a largura. Pontuação adjacente pode ser deixada sozinha numa linha.
- Thai: o texto tailandês não quebra e é cortado na margem.
- O `style.lang` só é usado para hyphenation ( idiomas latinos); não influencia a segmentação de linha para CJK/Thai.

---

## Avaliação de `icu_segmenter`

### Disponibilidade

```bash
cargo search icu_segmenter
# icu_segmenter = "2.2.0"
```

A versão 2.2.0 é a mesma usada pelo vanilla. O projeto cristalino já tem `icu_properties` no `Cargo.lock` como dependência transitória.

### Vantagens

- Fornece o mesmo algoritmo UAX #14 + LSTM que o vanilla usa.
- Permite replicar o tailoring de aspas para chinês/japonês.
- Resolve CJK e Thai (LSTM) com uma única dependência.

### Problemas conhecidos

1. **Aspas CJK (issue unicode-org/icu4x#5595)**: em `icu_segmenter` 2.2.0, `U+201C`/`U+201D` ainda são classificados como `QU` (`Quotation`) em vez de `OP`/`CL` para C/J. O vanilla contorna isto com o `CJ_SEGMENTER` customizado. Replicar a paridade exige o mesmo *workaround* (blob de dados customizado ou tailoring manual).

2. **Khmer (issue unicode-org/icu4x#7218)**: versões ≤ 2.1.1 produziam oportunidades de quebra duplas em torno de espaços em texto Khmer. Corrigido no PR #7232; a versão 2.2.0 usada pelo vanilla já inclui a correção.

3. **Interpuncts CJK (issue unicode-org/icu4x#7500)**: o segmentador padrão pode deixar pontos de interpolação (`·`, `・`) no início de linha. O vanilla não trata isto explicitamente; portanto, para paridade, não precisamos de o fazer agora.

### Conclusão

`icu_segmenter` 2.2.0 é viável e é a opção que mais se aproxima do vanilla. Não dá paridade perfeita "de fábrica" por causa do tailoring de aspas, mas permite replicar exactamente o que o vanilla faz com um esforço adicional controlado.

---

## Decisão e Proposta de Divisão

Com base na sonda:

- **CJK**: o vanilla já trata correctamente (dentro do âmbito do tailoring de aspas). Paridade = replicar `icu_segmenter` + `CJ_SEGMENTER` para chinês/japonês.
- **Thai**: o vanilla quebra por LSTM (não por dicionário perfeito). Paridade = replicar o segmentador LSTM geral do `icu_segmenter`.
- **Coreano**: não foi testado directamente, mas o vanilla usa o segmentador geral. Deve ser abrangido pelo mesmo trabalho de CJK se usarmos `icu_segmenter` globalmente.

### Passos propostos

1. **P756 — Integrar `icu_segmenter` no cristalino (CJK)**
   - Adicionar `icu_segmenter` 2.2.0 a `01_core/Cargo.toml` (L1 pode usar crates puras de Unicode; o segmentador não faz I/O).
   - Implementar segmentação de linha em `layout_word`/`layout_text` para texto chinês/japonês, com tailoring de aspas.
   - Adicionar testes com os documentos desta sonda.

2. **P757 — Aplicar segmentador geral a Thai (e outros scripts complexos)**
   - Usar o mesmo `icu_segmenter` LSTM para Thai, Lao, Khmer, Myanmar.
   - Confirmar quebras de sílaba vs palavra; ajustar se necessário para bater com o vanilla.

3. **P758 (opcional) — Tailoring adicional de CJK**
   - Se necessário, estender o tailoring para interpuncts, pontuação CJK, etc. Mantém-se como *scope-out* até haver evidência de divergência mensurável.

---

## Validação

### Testes realizados

- Documentos CJK e Thai compilados com vanilla 0.15.0 e cristalino.
- PDFs inspeccionados visualmente e via `pdftotext`.
- Código do vanilla inspeccionado em `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs`.
- Código do cristalino inspeccionado em `01_core/src/rules/layout/cursor.rs` e `01_core/src/rules/layout/text.rs`.
- `cargo search icu_segmenter` confirmou disponibilidade da versão 2.2.0.

### Nenhuma mudança de código

Este passo é sonda pura. Não foram alterados ficheiros de código; portanto, `cargo test --workspace` e `crystalline-lint .` não precisam de ser reexecutados por causa deste passo. Ambos continuam a passar no estado actual do repositório (última verificação em P754).

---

## Critérios de Fecho

- [x] Comportamento real do vanilla 0.15.0 confirmado para CJK, incluindo o caso exacto da issue #1009.
- [x] Mecanismo/crate real do vanilla confirmado: `icu_segmenter` 2.2.0 com `CJ_SEGMENTER` customizado para aspas.
- [x] Comportamento real do vanilla 0.15.0 confirmado para Thai (quebra LSTM, não dicionário perfeito).
- [x] Mecanismo do vanilla para Thai confirmado: usa o segmentador geral, sem dicionário específico.
- [x] Estado actual do cristalino confirmado: `layout_word` trata palavras como indivisíveis; CJK e Thai sem espaços não quebram correctamente.
- [x] Viabilidade de `icu_segmenter` avaliada; versão 2.2.0 disponível e alinhada com o vanilla; bugs conhecidos (CJK aspas, Khmer espaços) documentados.
- [x] Âmbito real de "paridade" definido: replicar `icu_segmenter` + tailoring de aspas para C/J; replicar LSTM geral para Thai.
- [x] Proposta de divisão em passos menores (P756, P757, P758 opcional).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p755.md`.
