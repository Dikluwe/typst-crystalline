# Passo 979 — agrupar múltiplos runs de texto num único `BT…ET` por linha, como o vanilla (posição e sequência de operador idênticas, não bytes de arquivo)

**Precede este passo**: correção do dono — o alvo não é PDF byte-idêntico (arquivo container),
é **posição de glifo e sequência de operador idênticas**. P976 (medição de determinismo) já
confirmou que byte-idêntico de arquivo exigiria portar a serialização do `krilla` quase verbatim —
esforço desproporcionado, sem ganho real, já que o que importa é operador+posição, não a
codificação binária final (ordem de objectos, compressão, etc.).

**Residual já registado em P956 §7**: "o vanilla agrupa vários runs de texto da mesma linha num
único `BT…ET`; o cristalino emite um bloco por run — granularidade, não semântica". Este passo
fecha esse residual.

**Nota de proveniência**: P975 já achou e corrigiu um bug real desta mesma categoria (posição) —
`italics_correction` faltando no advance de glifo math singular. O achado irmão (variantes `ssty`
não aplicadas) já tem spec própria escrita, `typst-passo-977.md` — não duplicar aqui, é passo
separado já em curso.

**Pré-condição de árvore**: `git status`. Confirmar P975-978 presentes.

---

## Fase A — confirmar a regra exacta de agrupamento do vanilla

1. Ler o código do vanilla (`typst-pdf`/`krilla`) para confirmar exactamente quando funde múltiplos
   runs de texto num só `BT…ET` — candidatos: runs consecutivos na mesma linha de base, sem
   mudança de cor/fonte/transformação entre eles.
2. Confirmar se a fusão acontece só sem mudança de estado gráfico, ou se o vanilla funde mesmo com
   pequenas mudanças via `TJ` com ajustes de kerning.
3. Ler a implementação actual do cristalino (`stream.rs`, modo verboso de P956) — confirmar onde
   cada run vira o seu próprio `BT…ET`.
4. Medir o ganho esperado: contagem actual de `BT`/`ET` do cristalino vs vanilla no documento de 30
   secções, estado pós-P957-975.

## Fase A.1 — gate

Apresentar ganho medido (redução de operadores, viabilidade de diff directo) contra custo/risco.
Editar L0s, sincronizar hashes, **parar para confirmação antes da Fase B** — mudança estrutural no
exportador de produção, per `ADR-0127`.

## Fase B — Implementação (protocolo de dois agentes de P898)

1. Agente A escreve testes: runs consecutivos mesma linha/cor/fonte fundem; runs com cor/fonte
   diferente não fundem; render idêntico ao não-agrupado.
2. Agente B implementa a fusão.
3. Revisão do orquestrador — confirmar render pixel-idêntico ao estado pré-agrupamento no documento
   de 30 secções, e contagem de `BT`/`ET` mais próxima do vanilla.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Comparar sequência de operadores directamente contra o vanilla (sem heurística de
   emparelhamento).
2. `compare.py` no documento completo — confirmar posição de glifo inalterada (o agrupamento não
   deve mexer em nenhuma posição, só na estrutura de blocos).
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Runs consecutivos sem mudança de estado gráfico fundidos num único `BT…ET`, como o vanilla.
- Posição de glifo inalterada (prova via `compare.py`, não suposição).
- Sequência de operadores mais próxima da do vanilla.
- Benchmark sem regressão.
