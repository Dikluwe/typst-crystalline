# RG1 — revisão do contrato de grupos Table

**Estado: proposta pronta; implementação NÃO autorizada por este documento.** A raiz produtiva e os L0s vigentes permanecem intactos. Aprovar este pacote autoriza instalar os candidatos, escrever os testes RED e implementar somente o contrato descrito. Não inclui commit/push do Typst, merge, release ou pin novo.

## Evidência antes da decisão

Foram executados **105 casos bilaterais, 210 invocações**: metadados, posições, papéis, erros, estilos e SVG. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [Inventário e estado anteriores](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz), [resumo inicial](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/table-row-groups-2026-10-17/summary.json), [relatório](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/table-row-groups-2026-10-17/relatorio.md). As fontes de referência consultadas foram comparadas byte a byte aos objetos Git do pin `9f2b6e8715237cb086899a42873660fe744622e8` ([hashes](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/table-row-groups-2026-10-17/reference-source-hashes.json)). O oráculo executado tem SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

- Header/footer com colspan/rowspan geram padding e papéis errados porque o Tekt mede por entradas, não ocupação.
- Headers entre células do Body são movidos ao topo; o parser também funde níveis/repeat e rejeita níveis iguais/aliases que o pin aceita. Footer fixo pode admitir células lexicamente posteriores antes da sua faixa; a rejeição atual é antecipada.
- Grupos vazios e line-only são válidos no pin; linhas não são células.
- A validação estrutural ocorre **antes** da transformação Table, mesmo com `none`, `[]`, substituição textual ou panic da Table. Cell/body ocultados continuam sem executar. As duas ordens de show-set medidas têm o mesmo erro estrutural. Os controles anteriores de conteúdo válido ocultado permanecem válidos; sua generalização para estrutura inválida foi refutada.
- Header `level`/`repeat` omitidos são desconhecidos no valor raw; explícitos aparecem em fields. `children` é conhecido inclusive vazio. Header.children.first() cru é igual à Cell crua equivalente, e func deve ser a nativa verdadeira: a promoção Auto antecipada e a func substituta do Tekt falham nesses observáveis.
- H/V ignoram y/x e perdem presença. None em índice, auto em end ou position, e ranges inválidos têm rejeições específicas no pin; o Tekt aceita vários deles.
- SVG confirmou erasure/overhang: vermelho horizontal base 82 unidades →41 com None no pin e 82→82 no Tekt; azul após None 41→0 no pin e 40→40 no Tekt. A unidade é o eixo lógico do viewBox, em documentos de 100pt sem texto. Não comparar número de paths ou bytes entre renderers. [Dados e método](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/table-row-groups-2026-10-17/svg-analysis.json).

A diferença de `tbody` omitido/expresso no controle ordinário é sintaxe de container HTML; não foi contada como defeito de ocupação. O número inicial de comparações de rows diferentes inclui essa distinção e não significa 35 defeitos funcionais.

## Contrato proposto

1. **Fonte ordenada única:** Table.children guarda Cell/Header/Footer/HLine/VLine; remover os quatro campos públicos separados. O parser conserva ordem e tipos, sem fundir/pad/posicionar.
2. **Metadata lexical:** Header ganha nível positivo próprio e máscara; Footer ganha máscara. Linhas usam x/y opcionais tipados e máscara. Omissão, auto, none e false explícitos são distintos. As funções públicas ergonômicas mantêm assinatura; a migração de seus valores raw está descrita na API.
3. **Plano por intervalos:** substituir header_rows/footer_start por row_groups reais e linhas resolvidas. Cinco tipos públicos novos e um accessor is_complete. Um único planner calcula ocupação e faixas; o código físico e o HTML consomem o resultado.
4. **Preparação causal:** com show-set aplicável resolvido, sintetizar/validar a estrutura antes da transformação Table. Plano pending não é visual pronto. A receita lê filhos lexicais. Só a Table sobrevivente realiza Cell/body uma vez, depois publica complete.
5. **Tinta:** compor linhas/bordas antes de emitir; None remove contribuição prévia no intervalo, com overhang e limites corretos. Não pintar branco nem retirar shapes/texto indiscriminadamente. Preservar o caminho Grid deste pacote.

[API pública completa e migração](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/api.md) — seis carriers existentes, cinco tipos novos, 13 constantes de máscara e is_complete; inclui remoções/substituições de campos e mudança de Auto cru. Todas as formas públicas estão nos respectivos **L0s candidatos**, não legitimadas por este documento de processo.

## Inferência Tekt e limites

Nível do grupo e kind/nível explícito da Cell são conceitos diferentes e ambos serão conservados. Para placeholders Auto de Header, escolher o papel/nível efetivo do grupo é a decisão Tekt proposta. A fonte de referência `layout/grid/resolve.rs:1565–1569` usa nível1 na ramificação de grupo completamente vazio. Não se infere intenção dessa implementação nem se exige igualdade do enum Rust. **Inferência:** herdar o grupo evita reclassificar padding para um nível não pedido; será refutada por perda do kind explícito, relação semântica demonstravelmente errada ou regressão dos controles. Isso não certifica PDF/UA/aplicabilidade de Headers.

RG1 permanece incompleto no universo Table. **RG2/table-root-fields** completa os campos/defaults gerais da Table além de children (a sonda raw-table-fields continua sentinela). Defaults settable próprios de Header/Footer/linhas exigem RG3/group-line-style-defaults. Rich functions/paints/gutters, repetição multipágina e PDF/UA continuam etapas próprias; o HTML de Grid e as três pendências math/matching também ficam abertos. Não alterar seus esperados para declarar paridade.

## L0, núcleo e revisão

- **53 L0s** candidatos com owner exclusivo: 34 têm mudança de lógica/testes/adaptação e 19 apenas o pin compartilhado. [Lista legível de owners](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/owners.md) · [Escopo por caminho e SHA](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/scope.json).
- Refinar o núcleo `00_nucleo/prompts/_nuclei/table/cell-realization.toml`, com **43 consumidores diretos**, para separar validação estrutural de receita. Pin novo `16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab`.
- Novo núcleo `00_nucleo/prompts/_nuclei/table/row-groups.toml`, dependência pinada do anterior, digest `472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade`. Não é owner nem alvo de @prompt.
- [Pins de headers previstos](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/header-pins.json); nenhuma lógica produtiva foi escrita. [Grafo candidato](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/table-row-groups-2026-10-17/graph-result.json) passou V1/V5/V15/V26 em cópia descartável; isso prova integridade/linhagem, **não implementação**. Os primeiros erros de ordem/posição dos blocos de pins foram corrigidos apenas no pacote e estão preservados.
- [R1–R20: testes e gates](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/test-plan.md). Confirmar RED funcional antes da implementação; erros de compilação não o substituem. Reexecutar build/suíte inteira/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial, sob 12 GiB, swap zero, Cargo um job e targets de mutação isolados.

## Por que exige confirmação

A [ADR-0127 §2](/repos/Antigravity/typst-crystalline/00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md) determina paragem para “adicionar, remover ou alterar campo público” e “Mudança arquitectural de fase/ordem de pipeline”, além de compatibilidade. Este pacote faz as três coisas. Clientes Rust que constroem os carriers ou leem os campos removidos precisarão migrar. A autorização genérica para corrigir o projeto não substitui esse gate específico.
