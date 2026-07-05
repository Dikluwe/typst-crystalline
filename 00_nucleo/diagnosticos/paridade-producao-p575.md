# P575 — O código órfão de L1 era mesmo a causa da diferença de linhas?

**Status**: `CONCLUÍDO` (verificação directa no commit P569)  
**Data**: 2026-07-05  
**Scope**: teste directo do estado com código órfão de L1 ainda presente (`03_infra/src/layout_bidi.rs`, commit `3ece290f0`).

---

## 1. Pergunta

P574 atribuiu a diferença de contagem de linhas observada no documento árabe de referência — 4 linhas no relatório de P569 vs. 2 linhas no estado actual — ao código órfão de L1 descartado em P572. Essa atribuição não foi testada directamente; apenas se verificou que P568 não era a causa. Este passo executa o teste em falta.

---

## 2. Medições

### 2.1 Procedimento

Checkout para o commit P569 (antes de P572 descartar o código órfão):

```bash
git checkout 3ece290f0
cargo build --release --bin typst
./target/release/typst /tmp/p574-longo.typ /tmp/p575-com-orfao.pdf
pdftotext /tmp/p575-com-orfao.pdf -
```

Input (`/tmp/p574-longo.typ`, idêntico a `/tmp/p569-longo.typ`):

```typst
#set text(lang: "ar", size: 14pt)
هذا نص طويل باللغة العربية. يحتوي على معلومات قيمة. نأمل أن يعمل بشكل صحيح.
```

### 2.2 Resultado do teste directo

Output `pdftotext /tmp/p575-com-orfao.pdf -`:

```text
‫هذا نص طويل باللغة العربية‪.‬يحتوي على معلومات قيمة‪.‬نأمل أن‬
‫يعمل بشكل صحيح‪.‬‬

```

Contagem:

| Versão | `pdftotext` (linhas de texto) | Observação |
|--------|------------------------------|------------|
| P569 — PDF manual gerado a 2026-07-05 11:03 (`/tmp/p569-longo.pdf`) | 4 | Registado no relatório de P569. |
| P569 — rebuild do commit `3ece290f0` (`/tmp/p575-com-orfao.pdf`) | 2 | Código órfão ainda presente. |
| P574 — estado actual (`/tmp/p574-longo.pdf`) | 2 | Código órfão já removido. |

### 2.3 Verificação de palavras coladas

Em todas as três versões, as palavras árabes extraídas por `pdftotext` não estão coladas; os espaços entre palavras são preservados. O critério principal de P569 mantém-se satisfeito.

---

## 3. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---------|---------|-----------------|
| O código órfão de L1 descartado em P572 era a causa das 4 linhas em P569? | **Não confirmado** | O commit P569 (com código órfão presente) já produz 2 linhas de texto, tal como o estado actual. |
| A contagem de 4 linhas do relatório P569 é reproduzível a partir do commit P569? | **Não** | O PDF manual `/tmp/p569-longo.pdf` tem 4 linhas; o rebuild do commit `3ece290f0` tem 2 linhas. |
| A diferença de linhas afecta o critério de palavras separadas? | **Não** | Todas as versões preservam os espaços entre palavras. |

---

## 4. Inferências e riscos

1. **Inferência**: a contagem de 4 linhas no relatório de P569 reflecte o estado do working tree no momento do teste manual (2026-07-05 11:03), não o commit `3ece290f0` propriamente dito. Alguma alteração não commitada ou um passo intermédio entre o teste manual e o commit final alterou a quebra de linha.  
   **O que a refutaria**: reproduzir 4 linhas a partir do commit `3ece290f0` — não observado.

2. **Risco baixo**: a causa exacta da diferença 4 vs. 2 linhas permanece não identificada. Como não afecta a morfologia do texto (palavras separadas), não é prioritária.

3. **Risco a monitorar**: se futuros passos dependerem da reprodutibilidade exacta dos relatórios de paridade, convém registar o hash do commit e o estado do working tree usados na geração de cada PDF de referência.

---

## 5. Conclusão

A verificação directa **não confirma** a atribuição de P574. O commit P569, com o código órfão de L1 ainda presente, já produz 2 linhas de texto para o input longo — o mesmo resultado do estado actual (P574). A diferença para as 4 linhas registadas no relatório de P569 deve-se a outro factor, ainda não determinado, e provavelmente ligado ao working tree do momento do teste manual.

Regista-se a causa real como **pergunta em aberto**, sem urgência, dado que o critério principal de separação de palavras árabes permanece satisfeito.
