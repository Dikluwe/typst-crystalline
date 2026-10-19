# L0 — PDF lexical separator emission
Hash do Código: b1ea9c03

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8

Layer: L3. Exclusive consumer: `03_infra/src/export/stream/separator.rs`.

## Evidence before decision

`03_infra/src/export/stream/draw.rs:164` emits fallback Text at page coordinates;
`:771` uses the local reflection envelope. The RED probes at
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/resultados.json`
show A B C losing real spaces. The public carrier contract in
`00_nucleo/prompts/core/entities/layout_types.md` was approved on 2026-09-28.
This is language text loss, not a requirement for identical PDF bytes.

## Contract

Own emission of one U+0020 from a causal TextSeparator. Delegate glyph/encoding
and compact/verbose envelopes to existing text helpers with the prepared font
scenario and active style. Use supplied x and PDF baseline; never add advance
to another item. Local and top-level dispatch retain their coordinate conventions.
Do not infer a separator from distance, emit glyph ink, or fuse adjacent source
fragments. Preserve grouping and resource mappings. Tests and PDF extraction
must retain lexical spaces while the adjacency control remains ABC and visible
letter positions remain unchanged.

## Espaço literal terminal em embedding LTR (2026-10-06)

### Medição anterior à decisão

`lab/results/construction/diagnostics/espaco-terminal-2026-10-06/before.json` conserva
working tree, HEAD, código/L0 e binários. O caso literal-terminal-space no
lote bidi-identidade-ocorrencia tem glifos posicionados corretamente, mas
caixa extraída de 123 termina em 121.49581pt versus 125.000002pt no oracle.
O PDF emite o carrier de espaço em 121.50342pt; separator.rs:10-14 apenas
emite U+0020, que Poppler descarta no fim do run. O vanilla pinado 9f2b6e871
usa ActualText 0020202C nesse espaço, encerrando o embedding de exportação.
`pdf-boundary-probe.json` mede que apenas ActualText 0020 não resolve, mas
0020202C conserva a caixa até 125.00142pt sem mudar glifos/avanços. A sonda
alternativa word-envelope-refutation.json refuta envolver o texto anterior
com o espaço cruzando restaurações gráficas: a extração muda de linha.
São identidade textual/geometria observáveis; não obrigação de copiar bytes
ou reproduzir todos os operadores. Hipótese refutável por perda/duplicação de
espaços, alteração da tinta ou regressão de markup/base LTR após a correção.

### Classificação e limites

Correção interna ADR-0127, sem campo/API pública/fase/flag/dependência/unsafe.
U+202C é formatação derivada **somente do PDF ActualText**, para fechar o
embedding de exportação comprovado pelo span; não conteúdo autoral, não
carrier, glifo, input bidi nem ToUnicode global. Não fabricar controle virtual
na representação autoral. L0 de paragraph-bidi-origin permanece integral.

### Responsabilidade exclusiva

O emit privado recebe origem e BidiSpan opcional explicitamente. Certificar
somente Literal com span exato U+0020 no fim de segmento Text não vazio
com prefixo anterior, dir LTR e input base RTL. Usar segmento/range originais;
None, markup/newline, outros ranges/direções e Space sem segmento Text mantêm
emissão existente. Não inferir fronteira por distância, fonte ou visualização.
Para o certificado, abrir ActualText com espaço+PDF, emitir somente o glifo
original U+0020 uma vez pelos helpers de texto com restauração gráfica
diferível, fechar EMC e só então o Q pendente. Compacto e verboso usam mesma
identidade e seus helpers respectivos. Fonte, x, baseline e avance intactos.
Nenhuma mutação do documento, clonagem de texto anterior ou reenquadramento.
Testar certificado positivo/negativos e fronteira EMC antes de Q; PDF fonte
inclui extração bruta, bbox, raster e controles top/local/markup/LTR.
