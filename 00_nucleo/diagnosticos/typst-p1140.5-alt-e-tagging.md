# Diagnóstico P1140.5 — `math.equation.alt` e tagging

**Data/hora:** 2026-08-24T09:30:36-03:00  
**HEAD:** `ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`  
**Estado:** working tree não commitada; `git diff HEAD --stat` antes da medição
L0: 51 ficheiros, 1549 inserções e 112 remoções  
**Vanilla:** pin `a51e02804`, binário
`lab/typst-original/target/release/typst`

## Contrato de linguagem medido

- omitido: `equation(body: [x])`;
- string: `equation(alt: "x squared", body: [x^2])`;
- `none`: `equation(alt: none, body: [x])`;
- vazio: `equation(alt: "", body: [x])`;
- content: `expected string or none, found content`;
- integer: `expected string or none, found integer`;
- `fields()` preserva `alt` explícito antes de `body`;
- `query(math.equation)` realiza defaults e set-rule: ausência/none aparecem
  null, vazio permanece vazio, e string herdada aparece no elemento consultado.

O cristalino falhou no primeiro named com `unexpected argument: alt`.

## Artefato acessível medido

Vanilla padrão:

- `pdfinfo`: `Tagged: yes`, PDF 1.7;
- catálogo contém `/StructTreeRoot` e `/MarkInfo<</Marked true...>>`;
- equação gera `/StructElem/S/Formula`;
- alt não vazio gera `/Alt(...)` com Unicode preservado;
- `alt: ""` gera `/Alt()`;
- ausência e `none` não geram `/Alt`.

`--no-pdf-tags` produziu `Tagged: no`. `--pdf-standard ua-1` aceitou a equação
com alt e rejeitou ausência, none e vazio com `PDF/UA-1 error: missing alt
text`. Logo tagging padrão e validação UA são eixos relacionados, mas não
idênticos.

Cristalino padrão, com documento mínimo:

- `pdfinfo`: `Tagged: no`, PDF 1.7;
- nenhum `StructTreeRoot`, `StructElem`, `Formula`, `Alt` ou `MarkInfo` foi
  encontrado;
- varredura do exportador não encontrou BDC/EMC ou estrutura semântica.

## Classificação

`alt` é semântica da linguagem e acessibilidade observável (ADR-0107). Sua
representação Rust e o caminho de transporte podem divergir. O consumer PDF
tagueado exige nova fase real de pipeline, logo cai no gate ADR-0127.

## Decisão de faseamento

P1140.5 implementará contrato de linguagem, query realizada e transporte por
`FrameItem::Semantic(Formula)` até a fronteira de exportação, visualmente
transparente. Não declarará PDF tagueado. P1140.6 será o dono de MCIDs,
ParentTree, StructTreeRoot, padrão on/off e validação PDF/UA; ADR-0126 impede
misturar isso com StreamMode Verbose/Compact.

## Adendo P1140.6 — tagging implementado

**Medição:** 2026-08-24T10:37:11-03:00, working tree não commitada sobre
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`; diff final acumulado: 89
ficheiros, 3248 inserções, 307 remoções.

O cristalino agora gera PDF tagueado por defeito e aceita
`--no-pdf-tags`. Quatro artefatos reais (Verbose/Compact × tags on/off)
foram validados: `pdfinfo` reportou `yes/no` conforme a configuração e
`qpdf --check` não encontrou erros. `mutool show` confirmou
StructTreeRoot/ParentTree alcançáveis. Formula preserva alt Unicode, vazio e
ausente, com MCID determinístico e sem duplicar desenho.

O estado anterior `Tagged: no` acima permanece como medição histórica de
P1140.5. P1140.6 não faz alegação PDF/UA: os demais papéis estruturais do
documento continuam fora de escopo.
