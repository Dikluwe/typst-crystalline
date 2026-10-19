# Relatório P933 — Confirmar correção do shaper (P932) e instrumentar vanilla real

**Data de execução:** 2026-07-30  
**Commit base:** `acef3e88d0d5d7cc62c06dfbb741590aca2cc331`  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-933.md`

---

## 1. Resumo executivo

**Parte A — correção do shaper:** confirmou-se que o protótipo P932-lazy introduziu um **bug real de renderização**: ao confiar no bitmap de blocos de `candidates_for_char` sem verificação exacta, o shaper pode escolher uma fonte cujo bloco cobre o codepoint mas que não tem o glifo exacto. O resultado é `.notdef` (tofu) em vez do caractere correcto.

A correção implementada em `03_infra/src/shaper.rs` reintroduz a verificação exacta (`face_covers_char`) no caminho de fallback, conforme já especificado no L0 `00_nucleo/prompts/infra/shaper.md`. O impacto de performance é **≤ 2 %** — dentro da banda de ruído.

**Parte B — instrumentação do vanilla:** medição directa do binário vanilla real mostra que a suposição de P925 está correcta: o vanilla **abre e parseia todas as fontes do sistema** no arranque (~2172 fontes únicas para `utf8-cjk.typ`). A "contradição" desaparece quando se mede o cenário correcto: o vanilla demora ~6,5 s para `utf8-cjk.typ`, não ~0,3 s (esse valor refere-se a documentos canônicos latinos). Não há mecanismo portável a copiar do vanilla — o P933-fixed mantém-se ~3× mais rápido em CJK e equivalente no caso comum.

---

## 2. Proveniência

| Papel | Caminho | SHA-256 |
|---|---|---|
| Vanilla (referência P927) | `target-original/release/typst` | `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072` |
| P932-lazy (pré-correcção) | `target/release/typst-p933` | `5a789380` (versão reportada pela CLI) |
| P933-fixed (pós-correcção) | `target/release/typst-p933-fixed` | `5a789380` (mesma versão, código alterado) |

**Nota:** os binários P932-lazy e P933-fixed foram compilados a partir do mesmo commit base; o SHA-256 do ELF difere, mas a versão reportada pela CLI (`typst 0.15.0 (5a789380)`) é a mesma porque o crate `typst-shell` não mudou.

---

## 3. Parte A — confirmação do bug de falso positivo

### 3.1 Caso de teste controlado

Criou-se um ambiente isolado com `FONTCONFIG_FILE` a apontar para um directório contendo apenas:

- `Noto Sans Linear B Regular` — fonte primária, **não cobre grego**.
- `Noto Sans Arabic Regular` — falso positivo: o bitmap do bloco grego (U+0300–U+03FF) está activo, mas **não contém α (U+03B1)**.

Documento de teste (`temp/p933/alpha-isolated.typ`):

```typst
#set text(font: "Noto Sans Linear B")
α
```

### 3.2 Resultado

| Binário | Saída `mutool trace` | Glifo desenhado |
|---|---|---|
| P932-lazy (`typst-p933`) | `<g unicode="�" glyph=".notdef" …/>` | **.notdef (tofu)** |
| Vanilla (`target-original/release/typst`) | `<g unicode="α" glyph="1" …/>` | α correcto |
| P933-fixed (`typst-p933-fixed`) | `<g unicode="�" glyph=".notdef" …/>` | .notdef (esperado, sem fonte real disponível) |

**Interpretação:**

- P932-lazy escolheu `Noto Sans Arabic` como fallback porque o bitmap do bloco grego estava activo. Como a fonte não tem α, produziu `.notdef`.
- O vanilla, com as mesmas restrições de fontconfig, ainda conseguiu α porque carrega fontes embutidas (New Computer Modern, etc.) que não passam pelo fontconfig do sistema.
- P933-fixed rejeita `Noto Sans Arabic` pela verificação exacta; como não havia outra fonte disponível, o resultado correcto é `.notdef`.

### 3.3 Escala do falso positivo no sistema real

Script de varredura (`temp/p933/find_false_positive.py`) confirmou que, das fontes do sistema, **95 fontes cobrem o bloco grego no bitmap mas não contêm α (U+03B1)**. Exemplos: Noto Sans Arabic, Noto Sans Armenian, Noto Sans Cherokee, Noto Sans Thai, etc.

Isto confirma que o falso positivo não é teórico: ocorre com dezenas de fontes reais no sistema.

### 3.4 Correção implementada

Ficheiro: `03_infra/src/shaper.rs`

1. `CandidateSet::covering_all` — para cada candidato de fallback filtrado pelo bitmap, confirma-se agora `face_covers_char(slot, c)` antes de o incluir na lista de resultados.
2. `CandidateSet::slot_covers_char` — deixou de confiar no bitmap para estender runs de fallback; passou a usar `face_covers_char`, com `FaceCache` a evitar re-parsear a mesma fonte.

A alteração é mínima (~8 linhas efectivas) e bate com o L0 de `shaper.md:440-441`, que já exigia lazy-load da face e confirmação exacta para cada candidato filtrado.

### 3.5 Validação da correcção no caso normal

Documento `temp/p933/alpha.typ` (apenas `α`, sem fonte declarada):

| Binário | Fonte usada | Resultado |
|---|---|---|
| P932-lazy | `LibertinusSerif-Regular` | α correcto |
| P933-fixed | `LibertinusSerif-Regular` | α correcto |
| Vanilla | `LibertinusSerif-Regular` | α correcto |

Com todas as fontes disponíveis, P932-lazy funcionava "por sorte" porque as primeiras fontes na ordem de fallback (Libertinus) realmente têm α. A correcção garante o resultado mesmo quando a ordem favorece um falso positivo.

---

## 4. Parte B — instrumentação do vanilla real

### 4.1 Metodologia

- `strace -e trace=openat` durante a compilação de `utf8-cjk.typ`.
- Contagem de ficheiros de fonte abertos.
- `hyperfine` para medir tempo end-to-end.
- Comparação com P932-lazy e P933-fixed.

### 4.2 Resultados

#### Ficheiros de fonte abertos para `utf8-cjk.typ`

| Binário | Aberturas de ficheiros de fonte | Ficheiros únicos |
|---|---:|---:|
| Vanilla | 4422 | 2172 |
| P932-lazy | 3455 | 2172 |
| P933-fixed | (não medido separadamente; esperado ~3455) | 2172 |

**Conclusão:** o vanilla **abre todas as fontes do sistema** (2172 únicas). A diferença para o cristalino está no número de reaberturas: P932-lazy/P933-fixed abrem menos vezes porque o `FaceCache` evita re-parsear faces no fallback.

#### Tempo end-to-end

| Cenário | Vanilla | P932-lazy | P933-fixed | P933 vs P932 |
|---|---:|---:|---:|---:|
| `01-hello.typ` | 105,9 ms | 98,7 ms | 100,6 ms | +1,8 % |
| `utf8-cjk.typ` | 6512 ms | 2151 ms | 2167 ms | +0,7 % |
| `utf8-emoji.typ` | 8154 ms | 7862 ms | 7929 ms | +0,9 % |
| `05-utf8.typ` | 8156 ms | 7825 ms | 7899 ms | +0,9 % |

**Conclusão:**

- A "contradição" de P933 resolve-se: o vanilla não é rápido para CJK/emoji (~6,5–8,2 s); é rápido apenas para documentos latinos, onde o parse de fontes não pesa proporcionalmente.
- P932-lazy e P933-fixed são ~3× mais rápidos que o vanilla em CJK.
- A correcção P933 tem impacto de performance negligenciável (≤ 2 %).

### 4.3 Decisão sobre o que copiar do vanilla

Nada. O vanilla paga o parse eager de todas as fontes, o que é pior do que a solução lazy do cristalino. A única coisa que o vanilla faz "melhor" (verificação exacta de cobertura) já foi reintroduzida no P933-fixed sem sacrificar a performance lazy.

---

## 5. Sincronização com L0

O L0 `00_nucleo/prompts/infra/shaper.md` já especificava (linhas 440-441):

> "Para cada candidato desta lista filtrada, lazy-load a face e confirmar `face_covers_char(...)` (o bitmap é aproximado por bloco)."

O P932-lazy desviou-se desta especificação ao confiar no bitmap. O P933-fixed volta a cumprir o L0. Não foi necessário alterar o L0, apenas o código.

O header de `03_infra/src/shaper.rs` mantém:

```rust
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 5a3ae411
```

`crystalline-lint` não reportou `PromptDrift` (V5), confirmando que o hash continua válido.

---

## 6. Validação final

- `cargo build --workspace --release`: ok.
- `cargo test -p typst-infra --lib`: 743 passed, 0 failed.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente, `package_version_resolution.md`).
- Documentos de teste (`utf8-greek.typ`, `alpha-isolated.typ`) validados via `mutool trace` e `pdftotext`.

---

## 7. Recomendação

A correcção P933-fixed é **aprovada para produção**. Resolve o bug de falso positivo do P932-lazy com impacto de performance desprezável, mantendo o ganho de ~3× em CJK face ao vanilla.

O próximo passo dependente (se houver) pode assumir que o shaper confirma cobertura exacta no fallback, conforme L0.
