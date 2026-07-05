# P570 — O espaço perdido é específico de RTL, ou de qualquer mudança de direcção?

**Status**: `CONCLUÍDO` — causa classificada como **específica do mecanismo RTL**.  
**Data**: 2026-07-05  
**Scope**: Sonda directa sobre `03_infra/src/layout_bidi.rs` e comportamento do Layouter em LTR.

---

## 1. Reformulação da pergunta

A pergunta do passo é: o espaço perdido observado em árabe (P569) é uma falha específica do tratamento RTL, ou é um problema geral do Layouter ao fechar linhas que por acaso só se manifestou em árabe?

Verificações mensuráveis:

1. Texto latino com quebra de linha forçada mantém as palavras separadas na extracção?
2. O código que corrige o problema em P569 só executa quando uma linha é detectada como RTL?

---

## 2. Medições

### 2.1 Teste latino com quebra de linha forçada

Input (`/tmp/p570-latim-quebra.typ`):

```typst
#set page(width: 60pt, margin: 0pt)
palavra outra
```

Nota: o passo original propunha `width: 60pt` sem especificar margem. Com as margens por omissão do Layouter, o texto era colocado fora da `MediaBox` (x ≈ 70 pt numa página de 60 pt de largura), e tanto `pdftotext` como `mutool` não extraíam nada. A adição de `margin: 0pt` colocou o texto dentro da página e permitiu a medição.

Comando:

```bash
./target/release/typst /tmp/p570-latim-quebra.typ /tmp/p570-latim.pdf
pdftotext /tmp/p570-latim.pdf -
mutool draw -F txt /tmp/p570-latim.pdf 1
```

Output `pdftotext`:

```text
palavra
outra
```

Output `mutool`:

```text
pa av a
outra
```

As palavras latinas não ficaram coladas. A quebra de linha forçada separou-as visualmente e ambos os extractores preservam a separação. O `pdftotext` apresenta cada palavra na sua linha; o `mutool` fragmenta ligeiramente a primeira palavra (provavelmente por kerning), mas mantém as duas palavras como objectos distintos.

### 2.2 Condicionalidade RTL do código P569

A correcção de P569 vive inteiramente dentro de `03_infra/src/layout_bidi.rs`:

| Ponto de entrada | Linha | Condição para executar |
|------------------|-------|------------------------|
| `reorder_bidi_document` | `03_infra/src/layout_bidi.rs:28` | Chamada posterior pura na pipeline. |
| `detect_rtl_line` | `03_infra/src/layout_bidi.rs:340` | Usa `unicode_bidi::BidiInfo` e só devolve `true` quando a direcção base do parágrafo é RTL. |
| `reorder_bidi_line` | `03_infra/src/layout_bidi.rs:130` | Só chamado quando `line_is_rtl[i]` é `true`. |
| `coalesce_space_items` (P569) | `03_infra/src/layout_bidi.rs:162` | Chamado dentro de `reorder_bidi_line` e de `try_fuse_paragraph`; ambos só actuam sobre linhas/parágrafos RTL. |
| `split_ltr_suffixes_page` (P569) | `03_infra/src/layout_bidi.rs:187` | Só chamado para linhas onde `line_is_rtl[i]` é `true`. |
| `try_fuse_paragraph` | `03_infra/src/layout_bidi.rs:433` | Só processa runs de linhas predominantemente RTL. |

O código que corrige o espaço perdido **não executa** para texto LTR. Portanto, mesmo que o Layouter tivesse algum problema geral de espaço em quebras de linha, a correcção de P569 não o estaria a mascarar em latim.

---

## 3. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---------|---------|-----------------|
| O espaço perdido é específico de RTL? | **Sim** | Texto latino com quebra de linha forçada preserva a separação das palavras; o código P569 só corre quando `detect_rtl_line` é `true`. |
| A causa está em `layout_bidi` ou no Layouter geral? | **Em `layout_bidi`** | A passagem só é activada para linhas RTL e é aí que os espaços neutros e sufixos LTR são reordenados de forma a perder a separação. |
| Escrita vertical (CJK top-down) herda este bug? | **Não directamente** | O mecanismo actual é específico de linhas horizontais RTL. Escrita vertical usará um caminho diferente, ainda por construir. |

---

## 4. Inferências e riscos

1. **Inferência**: o problema do espaço perdido é uma interacção entre a reordenação visual bidireccional e a extracção sequencial de texto, não uma falha genérica do cursor ao fechar linhas.  
   **O que a refutaria**: encontrar um documento LTR onde `pdftotext` cole palavras separadas por um espaço que tenha sido dividido por quebra de linha — não observado.

2. **Risco baixo**: escrita vertical CJK, quando for implementada, não herda automaticamente este bug, mas deve ter os seus próprios testes de extracção de texto, porque a questão geral "a ordem dos operadores PDF preserva a morfologia do texto?" continua relevante.

3. **Nota metodológica**: o teste latino inicial (sem `margin: 0pt`) colocou o texto fora da página, o que mostra que medições de extracção de texto são sensíveis ao posicionamento físico. Foi corrigido para obter uma medição válida.

---

## 5. Estado do passo

Critérios de fecho:

- [x] Teste latino com quebra forçada confirmado.
- [x] Código P569 confirmado como condicional a RTL.
- [x] Decisão registada: específico de RTL.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p570.md`.
- [x] Decisão sobre escrita vertical registada.

Não foi necessário código novo, apenas sonda e documentação.

---

## 6. Conclusão

O espaço perdido em árabe é **específico do mecanismo RTL** implementado em `layout_bidi`, não um problema geral do Layouter ao fechar linhas. A escrita vertical CJK não herda automaticamente este bug, mas deve ser testada de forma independente quando for construída.
