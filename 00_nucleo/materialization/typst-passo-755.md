---
# P755 — Sonda: quebra de linha CJK e Thai

> **Passo:** 755
> **Data:** 2026-07-14
> **Foco:** Quebra de linha para scripts sem espaços (CJK, Thai) está confirmada como ausente desde o início desta conversa, nunca investigada a fundo. Pesquisa externa revelou uma issue aberta no repositório do Typst (`typst/typst#1009`, 2023) a dizer que o vanilla usa UAX#14 "puro" via `xi-unicode`, sem regras de kinsoku específicas de CJK — o que pode significar que o próprio vanilla tem uma lacuna conhecida aqui. Thai é um problema diferente: exige segmentação por dicionário, não regras de carácter. Este passo é só sonda: confirmar o estado real do vanilla 0.15.0 (não uma issue antiga), antes de decidir o que "paridade" exige.
> **Tipo:** Sonda directa. Sem implementação neste passo.
> **Tamanho:** M para a sonda; a implementação, se avançar, é provavelmente L, e pode dividir-se em duas frentes distintas (CJK vs Thai).
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — funcionalidade nova, nunca tocada; sonda obrigatória.
> **Dependências:** nenhuma directa; primeira investigação desta área.

---

## Sonda

### Confirmar o comportamento real do vanilla 0.15.0 com CJK

```bash
cat > /tmp/p755-cjk.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
EOF
lab/typst-original/target/release/typst compile /tmp/p755-cjk.typ /tmp/p755-cjk-vanilla.pdf
mutool draw -o /tmp/p755-cjk-vanilla.png -r 150 /tmp/p755-cjk-vanilla.pdf
```

Inspeccionar visualmente onde as quebras de linha acontecem. Confirmar especificamente o caso do exemplo da issue #1009 — aspas de abertura no início de uma linha (proibido em tipografia chinesa correcta) — para confirmar se esta versão do vanilla já corrige isso ou ainda tem o problema descrito na issue.

```bash
cat > /tmp/p755-cjk-aspas.typ <<'EOF'
#set page(width: 5em + 2em, margin: (x: 1em))
#set text(font: "Noto Serif CJK SC")
测试文本，"测
EOF
lab/typst-original/target/release/typst compile /tmp/p755-cjk-aspas.typ /tmp/p755-cjk-aspas-vanilla.pdf
mutool draw -o /tmp/p755-cjk-aspas-vanilla.png -r 150 /tmp/p755-cjk-aspas-vanilla.pdf
```

Este é o caso exacto da issue #1009 — confirmar se a aspa de abertura (`"`) acaba no início da segunda linha (errado, segundo a tipografia chinesa) ou se o vanilla já evita isso.

### Confirmar a crate/mecanismo real usado pelo vanilla hoje

```bash
grep -n "xi.unicode\|icu_segmenter\|linebreak" lab/typst-original/Cargo.toml lab/typst-original/crates/typst-layout/Cargo.toml 2>/dev/null
grep -rn "fn linebreak\|LineBreak\|kinsoku" lab/typst-original/crates/typst-layout/src/inline/*.rs 2>/dev/null | head -30
```

Confirmar se ainda é `xi-unicode` puro, ou se o vanilla já migrou para algo com tailoring CJK entretanto.

### Confirmar o comportamento com Thai

```bash
cat > /tmp/p755-thai.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Sans Thai", size: 12pt)
สวัสดีครับผมชื่อจอห์นยินดีที่ได้รู้จักคุณ
EOF
lab/typst-original/target/release/typst compile /tmp/p755-thai.typ /tmp/p755-thai-vanilla.pdf
mutool draw -o /tmp/p755-thai-vanilla.png -r 150 /tmp/p755-thai-vanilla.pdf
```

Confirmar se o texto tailandês quebra em posições de palavra sensatas, ou se não quebra de todo (uma linha inteira, ou corta em posições absurdas a meio de palavras).

### Confirmar o mecanismo real do vanilla para Thai

```bash
grep -rn "thai\|Thai\|dictionary.*break" lab/typst-original/crates/typst-layout/src/inline/*.rs lab/typst-original/Cargo.toml 2>/dev/null | head -20
```

Confirmar se o vanilla usa alguma dependência de dicionário para Thai, ou se simplesmente não quebra Thai correctamente também (isto mudaria drasticamente o âmbito de "paridade" a atingir).

### Confirmar o estado actual do cristalino, para os dois casos

```bash
./target/release/typst /tmp/p755-cjk.typ /tmp/p755-cjk-cristalino.pdf
mutool draw -o /tmp/p755-cjk-cristalino.png -r 150 /tmp/p755-cjk-cristalino.pdf
./target/release/typst /tmp/p755-thai.typ /tmp/p755-thai-cristalino.pdf
mutool draw -o /tmp/p755-thai-cristalino.png -r 150 /tmp/p755-thai-cristalino.pdf
```

Confirmar exactamente como o cristalino trata hoje texto CJK e Thai — quebra em todo o carácter sem regra nenhuma? Não quebra de todo?

### Confirmar a crate `icu_segmenter`, e os bugs conhecidos que a pesquisa externa encontrou

```bash
cargo search icu_segmenter 2>/dev/null | head -5
```

Confirmar se `icu_segmenter` seria uma opção viável para o cristalino, tendo em conta os bugs já conhecidos (issues do `icu4x`, encontrados na pesquisa externa, sobre Khmer e aspas CJK) — usar esta crate não dá paridade automática e perfeita, mesmo que resolva o essencial.

### Critério de fecho da sonda

- [ ] Comportamento real do vanilla 0.15.0 confirmado para CJK, incluindo o caso exacto da issue #1009 (aspas no início de linha).
- [ ] Mecanismo/crate real do vanilla confirmado (ainda `xi-unicode` puro, ou algo mais).
- [ ] Comportamento real do vanilla 0.15.0 confirmado para Thai.
- [ ] Mecanismo do vanilla para Thai confirmado (dicionário, ou ausência de tratamento).
- [ ] Estado actual do cristalino confirmado para os dois casos.
- [ ] Viabilidade de `icu_segmenter` avaliada, com os seus bugs conhecidos considerados.

---

## Decisão

Este passo não implementa nada. Com base no que a sonda confirmar:

- Se o vanilla já tiver CJK correcto (kinsoku incluído): "paridade" significa replicar esse comportamento, tarefa bem definida.
- Se o vanilla ainda tiver a lacuna da issue #1009: "paridade" pode significar replicar UAX#14 puro (o que o vanilla realmente faz hoje), não a versão "perfeita" com kinsoku — decisão a registar explicitamente, não assumida.
- Para Thai: se o vanilla não tratar correctamente, o âmbito muda radicalmente (não há "paridade" a atingir para além do que o vanilla já faz).

Produzir uma proposta de divisão em passos menores, seguindo o padrão já usado para outras funcionalidades grandes (P614, P628, P678, P696).

---

## Critério de fecho do passo

- [ ] Sonda completa, com evidência directa (testes reais, não pesquisa externa sozinha).
- [ ] Estado real do vanilla 0.15.0 confirmado para CJK e Thai, distinto do que issues antigas possam sugerir.
- [ ] Âmbito real de "paridade" definido para cada caso.
- [ ] Proposta de divisão em passos menores.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p755.md`.
