# Passo 980 — Fase A: oráculo de paridade de operador como caminho separado (PARADO no gate, ADR-0127)

**Data:** 2026-08-05 · **Estado da árvore:** HEAD = `266f1e841` (P979),
working tree limpa à entrada. Nenhum código escrito; nenhum L0 editado
(o único artefacto L0 novo seria um prompt órfão nesta fase — o rascunho
completo segue abaixo para o dono guardar; ver nota de P969 sobre órfãos
V7).

## Fase A.1 — onde vive

**`03_infra/src/export/oracle.rs`** (módulo novo, L3). A transformação é
uma **função pura sobre a string do content stream já construída** pelo
modo verbose normal — `fn collapse_trivial_tj(content: &str) -> String` —
aplicada **só** no caminho do oráculo, no ponto onde o builder finaliza o
conteúdo da página (antes da compressão). O caminho normal não toca
nada disto: a função só é chamada quando a flag está activa.

Porque pós-transformação de string e não um modo novo de emissão: o
formato verbose é nosso e exacto (entries `<XXXX> int `), logo o parse é
seguro; e a prova de "saída principal inalterada" fica estrutural — o
código de emissão não muda de todo.

## Fase A.2 — activação

Flag nova **`--oracle-pdf`** (nome do passo), marcada no help como
ferramenta de diagnóstico: `02_shell/src/cli.rs` (campo em `Args` +
`RunIntent.oracle_pdf: bool`, mesmo padrão de `--compact`) → L4
(`main.rs`) chama uma entrada nova **`compile_to_pdf_bytes_oracle`**
(L3, `pipeline.rs`) — **aditiva**, sem mudar assinaturas existentes; por
dentro chama a impl partilhada com `oracle: true`, que só activa a
pós-transformação. Sem efeito em PNG/SVG (como `--compact`).

## Fase A.3 — escopo desta versão

**Só o PDF ajustado.** O relatório de paridade (proporções `Tr 2`,
`BDC`/`EMC`, balanceamento `q`/`cm`/`Q` — o desenho do P975 original)
fica para P981+. Razão: a transformação `Tj`/`TJ` é pequena e testável
isoladamente; o relatório é outra fatia de trabalho com as suas próprias
decisões de formato.

## Medição de base (Fase A, sobre `typst-math-comprehensive-test.pdf` e
`lorem`, estado pós-P979)

| doc | lado | TJ | TJ colapsáveis (todos os ajustes 0) | Tj |
|---|---|---|---|---|
| 30 secções | cristalino | 889 | **626** | 404 |
| 30 secções | vanilla | 148 | 3 | 1817 |
| lorem | cristalino | 36 | 0 | 0 |
| lorem | vanilla | 36 | 1 | 0 |

Após a transformação (30 secções): Tj ≈ 1030, TJ ≈ 263 → proporção Tj
≈ **79.7%** (hoje 31.2%; vanilla 92.5%). Em prosa os dois já convergem em
blocos (36 = 36, ambos TJ — o ajuste não muda nada aí, e não deve).

## Rascunho do L0 — `00_nucleo/prompts/infra/export/oracle.md` (para o dono guardar)

```markdown
# export/oracle — oráculo de paridade de operador (P980)

**Data:** 2026-08-05 · **Camada:** L3 · **Passo:** 980 (gate confirmado
pelo dono em ___)

## Propósito

Caminho de diagnóstico **separado** do exportador de produção: recebe o
content stream já construído pelo modo verbose normal e aplica
transformações de paridade de operador que **não** pertencem à saída
principal (não mudam posições nem têm benefício visível fora de
comparação). Activo só com a flag CLI `--oracle-pdf`.

## Regras do módulo

1. `03_infra/src/export/oracle.rs`; funções puras sobre a string do
   content stream; zero impacto no caminho normal (a saída sem a flag é
   bit-a-bit a mesma — guardado por teste).
2. Activação: `RunIntent.oracle_pdf` (L2) → `compile_to_pdf_bytes_oracle`
   (L3, aditiva) → aplica `collapse_trivial_tj` antes da compressão da
   página.

## Transformação 1 — `collapse_trivial_tj`

Um array `[ … ] TJ` cujos ajustes são **todos inteiros zero** (entries
`<XXXX> 0` e ajustes de fronteira `0`) é semanticamente um `Tj` puro:
reescreve-se como `<XXXXYYYY…> Tj` (hex concatenado). Arrays com qualquer
ajuste ≠ 0 ficam intocados. Números são **parseados** como inteiros (um
`-0` conta como zero). Paridade: o vanilla usa `Tj` quando não há
ajustes (medido: 92.5% dos blocos de texto do documento canónico).
```

## Gate

Parado per ADR-0127 (superfície nova de CLI + entrada pública nova em
L3). À confirmação: Fase B com TDD directo (`collapse_trivial_tj` puro
primeiro, RED→GREEN), prova de bit-identidade da saída principal, Fase C
(proporções medidas + compare.py + benchmark do caminho normal, que deve
ficar exactamente 1.00×).
