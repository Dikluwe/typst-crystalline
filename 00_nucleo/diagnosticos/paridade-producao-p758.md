# Relatório de Paridade — P758

**Passo:** 758
**Data:** 2026-07-14
**Foco:** Investigar se o problema das aspas CJK no início de linha (residual de P756/P757) admite uma correção local, sem implementar Knuth-Plass completo.
**Hash base:** `81a2cb1f32a5045bd2633f065853cb05e00d315c`
**Hash do commit com as alterações de código:** *não aplicável — este passo não introduziu código.*
**Hash do commit final (inclui este relatório):** `e6c054113a50c79c232bffb97700c850d8eeba1b`

---

## 1. Sonda

### 1.1 Documento de teste

```typst
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
```

### 1.2 Resultado cristalino

```text
测试文本，
"测试引号的位
置"。这是一段很长的中文
文字用来测试换行的效果如
何。
```

### 1.3 Resultado vanilla 0.15.0

```text
测试文本，“测试
引号的位置”。这
是一段很长的中
文文字用来测试
换行的效果如何。
```

### 1.4 Mecanismo confirmado

O tailoring de breakpoints de P756 impede que `U+201C` (aspa esquerda) seja um ponto de quebra válido. O fragmento `"测试引号的位置"` é tratado como uma unidade indivisível. Quando essa unidade não cabe no espaço restante da linha corrente, o layout greedy move-a inteira para a linha seguinte. Como a aspa é o primeiro carácter dessa unidade, acaba no início da nova linha.

O vanilla evita este efeito porque o seu `CJ_SEGMENTER` customizado classifica `U+201C`/`U+201D` como `OP`/`CP` e o Knuth-Plass optimiza o parágrafo globalmente, permitindo que a aspa "pendure" no final da linha anterior ou que a linha anterior seja ligeiramente comprimida/esticada.

### 1.5 Teste com `linebreaks: "simple"` no vanilla

```typst
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
#set par(linebreaks: "simple")
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
```

Resultado vanilla 0.15.0 em modo `simple` (greedy):

```text
测试文本，“测试
引号的位置”。这
是一段很长的中
文文字用来测试
换行的效果如何。
```

**Achado:** o vanilla continua a posicionar a aspa correctamente **mesmo em modo greedy**. Isto demonstra que a diferença não é apenas Knuth-Plass global vs greedy local; reside no segmentador customizado do vanilla (`CJ_SEGMENTER`) e nas heurísticas associadas de kinsoku/hanging punctuation.

### 1.6 Viabilidade de uma correção local

Uma heurística local de "hanging punctuation" ou "kinsoku" que puxasse a aspa de volta para o fim da linha anterior quando um fragmento começa com `U+201C` seria:

- Ad-hoc: resolveria apenas este carácter específico, não a classe geral de pontuação proibida no início/fim de linha.
- Riscada: forçar um carácter para além da margem ou comprimir a linha anterior quebra invariantes do layout greedy actual (largura de linha, alinhamento, overflow).
- Insuficiente: o vanilla também resolve casos semelhantes para outros caracteres (`U+300C`, `U+300E`, `U+FF08`, etc.) através do segmentador customizado.

Não existe uma correção local de tamanho S-M que resolva o caso geral de forma segura.

---

## 2. Decisão

**P758 é scope-out.** Não se implementa uma correção local.

A divergência observada deve ser endereçada num passo futuro dedicado a replicar o comportamento do `CJ_SEGMENTER` customizado do vanilla (ou equivalente), incluindo a classificação de pontuação CJK como `OP`/`CP` e as heurísticas de kinsoku/hanging punctuation. Esse trabalho é substancial e independe de uma eventual implementação de Knuth-Plass completo.

---

## 3. Validação

Como não houve alterações de código, a validação limita-se a confirmar que o estado do repositório permanece consistente:

```text
cargo test --workspace
crystalline-lint .
```

(Resultados inseridos na secção 4.)

---

## 4. Estado do repositório

```text
cargo test --workspace
  4121 passed (typst-core)
   635 passed (typst-infra)
    33 passed (typst-shell)
     2 passed (typst-wiring)
    27 passed (cli integration)
     2 passed (crystalline-lint integration)
     0 failed
crystalline-lint .
  ✓ No violations found
```

---

## 5. Fecho

P758 está fechado como sonda:

- Mecanismo do problema confirmado.
- Correção local avaliada e rejeitada por ser ad-hoc e insuficiente.
- Scope-out registado: a diferença CJK residual deve ser tratada num passo focado no segmentador customizado/heurísticas CJK.
- Sem alterações de código; sem regressões.
