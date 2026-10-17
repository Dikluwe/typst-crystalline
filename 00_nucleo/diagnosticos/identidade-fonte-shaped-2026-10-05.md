# Face real entre shaping e exportação — auditoria de 2026-10-05

A perda de identidade da face está confirmada num catálogo controlado. O shaping
produz GIDs da DejaVu Sans regular; o estilo preserva italic e a exportação
seleciona Oblique. Essa face não cobre o texto árabe usado na sonda. Não é
apenas um problema de bidi ou de cópia do PDF.

## Evidência e proveniência

Estado: working tree não commitada sobre
`06848cdcb5450f2c555626bf116bc7acd82bd82b`. O recibo
[initial.json.gz](identidade-fonte-shaped-2026-10-05/initial.json.gz) conserva
UTC, diff HEAD --stat completo, status/untracked, fontes, binários e fontes
Rust/L0 lidas. A sonda usa as bibliotecas debug existentes; seus hashes,
a receita de rustc, o programa independente e o resultado estão arquivados.
Não foi necessário editar código produtivo nem adicionar teste de produção
antes do gate. O [inventário](identidade-fonte-shaped-2026-10-05/inventory.json)
lista candidatos textuais, não constructors ou alterações autorizadas.

[Resultado controlado](identidade-fonte-shaped-2026-10-05/probe.txt):
- Fonte regular: cobre م e produz `[5256, 5260, 5277, 5288, 5341]`.
- Fonte Oblique: não cobre م e produz cinco GIDs zero.
- TextShaped emitido: possui exatamente os GIDs da regular, mas italic=true.
- Seleção posterior pelo estilo: escolhe Oblique. semantic_emph permanece true.

[Controles bilaterais](identidade-fonte-shaped-2026-10-05/bilateral.json):
árabe em ênfase, árabe regular e latim em ênfase, com a mesma página e família.
Registros conservam entrada, comando, hash PDF, fontes incorporadas e extração.
Oráculo: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
A extração de texto pode parecer correta mesmo com tinta errada; o raster do
relatório bidi anterior documenta a divergência visual. Não se mediu desempenho
nesta auditoria e não se afirma ganho de tempo.

## Origem histórica e alcance

P534 documenta transportar somente a família real para permitir que a
exportação incorpore a face correta, em
`00_nucleo/diagnosticos/paridade-producao-p534.md:79-80`.
Isso explica a aproximação herdada; não prova intenção de aceitar a falha.
A seleção posterior não é feita só no PDF: raster e SVG também derivam a
variante do estilo. O risco alcança fallback entre variantes e fontes distintas
com metadados iguais. A sonda demonstra o caso árabe/italic, sem alegar já ter
medido todas essas combinações. Cache de shaping já discrimina o slot real;
não corrigir o cache do shaper quando a informação se perde depois dele.

## Correção preparada e gate

O [L0 proprietário](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/layout_types.md)
propõe `font: Option<Font>` somente em TextShaped. É um clone do recurso carregado,
com Arc existente, sem I/O em L1 ou unsafe novo. None conserva o legado; o shaper
fornece Some, incluindo cache hits. A identidade deixa de depender de slots de
World e sobrevive à transformação e ao fim da sessão.

Recursos e exportadores deverão vincular face original + coordenadas efetivas
a instância/subset e ao recurso usado para os mesmos GIDs. Não apagar italic ou
marcadores semânticos para induzir seleção de uma face; não substituir família
por nome artificial. Cada consumer terá seu próprio L0 atualizado antes de código.
APIs públicas existentes e FontKey não estão autorizados a mudar neste pacote.

O L0 contém critérios RED→GREEN para cache hits, Worlds/slots distintos, faces
com metadados iguais, eixos de fontes variáveis, envelopes e camadas, além de
controles fonte→PDF/SVG/PNG e regressões de latim/math. A materialização precisa
medir estes casos; ainda não foram declarados GREEN.

[Manifesto de confirmação](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/identidade-fonte-shaped-l0-manifest-2026-10-05.json).
SHA-256 do L0 preparado: `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`.

A ADR-0127 exige paragem porque se acrescenta um campo público. Nenhum código
produtivo foi alterado nesta frente. O hash do código do L0 permanece o vigente;
seu header produtivo não foi ressellado enquanto a proposta aguarda confirmação.
V15/V26 antes da proposta não produziram resultados; isso não equivale a lint
global limpo nem valida materialização ainda inexistente.
