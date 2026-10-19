# L0 — sequence
Hash do Código: 3ebaf35a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:a28cd4f2dd22d2bdb2cb30085596c2a758af7770fa21eb28c38dc35e904928eb



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sequence.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/sequence.rs:201`, `01_core/src/compiler/layout/sequence.rs:583`, `01_core/src/compiler/layout/sequence.rs:805`, `01_core/src/compiler/layout/sequence.rs:24`, `01_core/src/compiler/layout/sequence.rs:36`, `01_core/src/compiler/layout/sequence.rs:65`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own ordered sibling iteration, lookahead/sticky handling, item grouping, block-margin collapse and vertical fractional preflight. Dispatch each actual child through the hub, retaining boundary and page events. SpaceFromNewline has the same width/transparency/grouping/closing behavior as Space; keep its Content origin and do not normalize CJK, show/regex, Empty, Parbreak or Styled into one category.

For a finite supported region, preflight the same sibling slice without emitting items, Locations or pages: reserve fixed items/gaps, exclude fractional Block body/intrinsic height, sum finite weights and distribute nonnegative remaining height proportionally; zero weight/sum is safe and does not divide. Save/restore the private concrete Block allocation and use the re-entry guard. Do not write allocations into entities.

Recognize a fractional Block through Styled and singleton-substantive Sequence wrappers iteratively; Space, SpaceFromNewline and Empty are transparent. Ambiguous/multiple substantive children are not flattened. Ordinary finite flow retains its existing base; continuous page columns use the remaining height from the entry cursor. Only the dedicated explicit-segment entry removes the segmented-column guard and uses the same cursor-relative regional base. Cell/subframe/note contexts and nested/multiregion distribution remain fail-closed outside the source's certified paths.

### Certified inline prefixes

A supported prefix is scanned from the existing sibling slice, with scalar metric/style accumulators rather than a speculative emission engine. Plain Text/space/empty and a direct Styled/Strong/Emph Text wrapper retain their current restricted style-delta gates. Cosmetic-only wrappers do not manufacture independent fixed lines; disallowed nested/compound geometry/style cases stay opaque. Leading, font/script/tracking/edge changes and arbitrary custom channels are not implicitly accepted.

Preserve the present size-only certifications: a single reduced text wrapper; a direct outer-sized fragment paired with a larger Styled(Text) in either order; two size-only wrappers in supported decreasing/equal-above-outer cases; and the exact certified single-wrap pairs. No-wrap certification requires the real injected-width gate, not a nominal fixture size. The fixed reserve accounts for the initial top edge, actual current-line maximum/descent, the selected next ascent/leading and the paragraph-to-block boundary consistently with `00_nucleo/prompts/compiler/layout/line_commit.md`. In the above-outer decreasing/equal cases, do not double-reserve a boundary that runtime already delivers.

The certified boundary belongs to the prefix/first-fractional-Block handshake and is applied once even if trailing whitespace already closed the line. Closing early does not lose the boundary, and ordinary/rejected prefixes acquire no new displacement.

A certified single wrap requires exactly two direct size-only Styled(Text) single-word fragments, a wrap at their boundary and a complete vertical reserve that fits. Preserve equal/unequal above-outer pairs, outer/larger pairs and below/above-outer pairs in both orders. Reject multiword, additional fragment, composed delta, nested wrapper, internal/multiple wrap or insufficient reserve. A first overlong word is deferred only until the second fragment proves the complete supported form; an incomplete overlong prefix remains opaque.

Record the existing wrap/Block indices and boundary. Request the certified line-close hooks before entering the second wrapper and before terminal whitespace/Block. Reserve per-line ascent/descent/outer-leading, not the global maximum twice. When the first overlong word closes inside text emission, arm the existing one-shot so that first close uses the certified formula. For a smaller singleton, retain its own baseline/next-ascent and only outer leading; when it is second, apply the source's single baseline correction before its wrapper. The hub stores the arm, `00_nucleo/prompts/compiler/layout/line_commit.md` consumes it, and `00_nucleo/prompts/compiler/layout/sub_frame.md` isolates it.

### Grouping and flow boundaries

Preserve sticky headings, list/enum/term grouping, loose-item gap reset and enum-counter lifetime according to the current iterator, without asserting that every Parbreak resets an enum. Keep block-chain below/above collapse and terminal fractional-tail marking; do not move Block's own frame-end or Place's own anchor into a global scan. Direct private HTML markers do not create a pending material line/gap/wrap/flush; wrappers retain their existing materiality.

The certified matrix remains a bounded compatibility subset, not a full flow simulator or arbitrary fraction-parity claim. Collected structs, algorithms and exported bytes may differ from vanilla; observable content, order, pages, positions and styles may not be redefined by this documentation.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `lab/results/construction/diagnostics/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Antes de layout_parts, tentar o conjunto inteiro pelo owner paragraph_plan. Se recusado, salvar guard, suprimir planejamento de fragments durante layout_parts e restaurar; preservar toda coordenação existente e fractional preflight. Não atomizar por import/dyn.

### Entrada terminal sob configuração global

O lote after-pdf inicial (after-first.log e PDF) mostra que native já escolhe as linhas optimized, mas #set ainda coincide com greedy: o Sequence exterior contém configuração opaca, recusa preparação e suprime inclusive o wrapper terminal que cobre todo o texto. Medição de sequence.rs:943/975 mostra a entrada real no último sibling. Decisão: antes de despachar o último sibling substantivo, se não há linha material pendente, permitir nova tentativa sob seus estilos; siblings finais Space/SpaceFromNewline/Empty não acrescentam conteúdo. Restaurar o guard imediatamente depois do dispatch. Não liberar fragmento que tenha texto posterior ou prefixo material; preservar scopes e preflight fracionário.

O planner do conjunto pode certificar todos os segmentos separados por Parbreak explícito antes da emissão. Esta cláusula não permite segmentar por conteúdo opaco nem alterar a iteração/preflight dos caminhos recusados.

## Refinamento de escopo integral e fronteira explícita

### Medição anterior à decisão

first-round/acceptance.json registra 14 falhas nas 397 entradas: negative/native/empty-fold/restore/plain-disabled e regressões lexicais passam a planejar antes do wrapper integral de Styles, lendo custos exteriores e usando font/leading exteriores. Fonte: paragraph_plan.rs admite default Simple antes da descida; paragraph_prepare percorre Styled restaurando a chain; sequence.rs:947-956 reabilita somente o último sibling substantivo. terminal-break conserva 3+1 em vez de 2+2 porque o último sibling é Pagebreak e o texto anterior permanece suprimido. Isto refuta a admissão antecipada; não é autorização para promover um fragmento parcial a escopo compartilhado.

### Decisão

Quando body, ou segmento completo entre Parbreaks, tem exatamente um sibling substantivo Styled (outros são Empty/Space/SpaceFromNewline), recusar a tentativa exterior e deixar style_scope entrar normalmente. Sem alterar chain/Style por antecipação, sem deduzir configuração por glyph e sem emitir metade de um conjunto recusado. Após a entrada, tentar novamente sob o escopo correto pelo dispatch existente. Preservar overrides inline parciais.

No owner Sequence, também reabilitar a tentativa quando o sibling seguinte substantivo é Pagebreak e a linha atual não tem material. Esta é uma fronteira explícita do fluxo: o sibling corrente pode fornecer o parágrafo completo antes da quebra; prefixos já materiais continuam suprimidos. Não segmentar por conteúdo opaco genérico, não alterar preflight de blocos/notas/colunas nem o despacho de pagebreak. Testar restauração/ratios desativados e repetir corpus lexical, com os mesmos inputs e expectativas.

### Reentrada antes de Parbreak medida antes de corrigir

second-round/acceptance.json registra quatro falhas: os dois controles additional-scopes/positive-first (adicionados depois da implementação, sem medição Tekt prévia) e as regressões costs/restore e overhang/restore. Positive-middle já coincide. O primeiro wrapper integral antes de Parbreak continua suprimido porque não é terminal e o guard novo só reconhece Pagebreak; o fluxo posterior entra no escopo normalmente. Parbreak é uma fronteira explícita de parágrafo com a mesma evidência de fechamento necessária. Estender a reabilitação do guard ao próximo sibling substantivo Parbreak, além de Pagebreak, sempre com current_line sem material. Não abrir tentativa antes de sibling inline ou conteúdo opaco genérico. A certificação do parágrafo continua inteira; a iteração apenas restaura possibilidade de tentativa em cada fronteira material de parágrafo. Manter inputs e expectativas dos quatro controles e repetir regressões após rebuild.

## Direção de parágrafo distinta de direção de run — 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/direcao-alinhamento-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes, binários e treze sondas. rtl_lines põe ABC em x=20 no Tekt e x=152.949220 no vanilla pinado; rtl_inline_only e rtl_scope coincidem à esquerda. ltr_lines põe árabe em x=174.567870 no Tekt contra x=20 no vanilla. Em `cursor.rs:156–162` e `line_commit.rs:210–216`, a âncora depende do primeiro item da linha. Vanilla `flow/collect.rs:129–136` obtém trunk_from_pairs e `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados antes das linhas. Semântica observável: direção dos trechos e âncora de todas as linhas do parágrafo. Estratégia de state/preflight é mecânica interna, não cópia do vanilla. Inferência: preservar direção compartilhada evita perder a âncora em linhas só com override; refutada por controles integralmente estilizados/parbreak/wrap.

Correção interna/de paridade ADR-0127, fluxo contínuo L0 primeiro e RED→GREEN. Apenas campos privados de Layouter e helpers descendentes, nenhuma entidade ou assinatura pública/fase/default novo. Esta rodada certifica conteúdo textual estrutural (Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph); unidades desconhecidas conservam fallback anterior. Não prometer ordem bidi mista, align explícito, HTML ou todas as formas de parágrafo. As equações existentes de largura/tracking permanecem.

Responsabilidade deste owner: Consultar o resolver de paragraph para cada segmento delimitado por Parbreak no percurso incremental quando não há escopo ativo pai. Segmentar somente para resolução, sem refazer travessia de emissão ou quebrar grouping/lookahead. Restaurar escopo exterior e guardar direção da linha aberta ao fim; nested sequences herdam o escopo ativo. Se unidade desconhecida recusa, seguir fallback existente. A preparação global existente continua dona da escolha/linhas.

Aceitação: RED→GREEN em parágrafo RTL com linha LTR e inverso, escopo integral e reset por parbreak; checar checkpoints e subframes, texto/posições em CLI bilateral, build e lint sem novos findings. Slots e helper não transportam metadados ao L3; divergências de ordenação bidi ou align explícito ficam registradas separadamente.

Refinamento por medição: verification.json.gz da segunda tentativa mostra as linhas continuadas de rtl_wrap ancoradas corretamente (xMax≈200), mas a última ainda termina em xMax≈195.74; ltr_wrap corrige as linhas intermediárias para xMin=20, mas a última permanece em xMin≈68.73. Na saída do wrapper exterior não certificado (por exemplo, contendo SetPage), active=None não deve apagar pending aceito pelo filho. Guardar cauda somente quando este escopo possui direção Some; None significa ausência de prova, não reset de uma cauda que o filho certificou. O flush continua dono do reset. Conferir último run com direção oposta após uma diretiva de página.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

Guard privado bidi_scope_active distingue ausência de cursor certificado de ausência de escopo; após recusa integral, filhos não certificam um subconjunto parcial. Checkpoint e subframe conservam/suspendem/restauram este guard.

Medição complementar antes da decisão: `pipeline-module.log` imprime a árvore realmente emitida: Sequence começa por SetPage, seguido do Styled textual; a tentativa Located isolada não resolveu o RED. SetPage é uma fronteira de configuração, não uma folha do parágrafo. Ao início e após SetPage direto, certificar somente até a próxima fronteira SetPage/Parbreak, usando a região já instalada. Não ignorar objetos opacos nem retirar o guard de recusa integral. O primeiro candidato e ambas as sondas RED permanecem arquivados.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-fourth refuta Source como singleton da região segmentada de colunas: classificar body antes de reconhecer Styled/Sequence, mantendo FractionalRegionBasis::CursorRemaining e prefixos existentes.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Escopo textual e cauda fracionária — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/scope-tail-before/results.json.gz`
registra HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/status/diffstat e hashes de fontes/binários contra vanilla pinado9f2b6e871.
Os lotes block-par-spacing-after e block-par-extra-after anteriores já
refutam escopo por consenso de runs: um wrapper18 em Lead More exige margem
21.6pt, enquanto dois wrappers18 adjacentes mantêm o escopo exterior.
sequence.rs:196–245 fecha prefixos na chain exterior; paragraph.rs:12–90 já
resolve direção pelo ancestral lexical comum. Fonte vanilla flow/collect.rs:
129–145 resolve estilos comuns antes de compor linhas e espaçamento.
A cauda TAIL top10/bottom0 após Block1fr ocupa página2 contra página1 no pin:
cursor.rs:127–135 rejeita igualdade com o limite embora a cauda esteja
certificada. Cauda Styled18 fica10.52pt acima; fractional_preflight.rs:170–211
mede nós individualmente e só corrige a altura de Text direto. Linebreak
explícito também excede a região. A nova sonda inclui wraps, fonte8/18,
Strong, wrappers irmãos, Align e colunas. Isso é geometria e paginação de
linguagem; não exige estrutura ou bytes iguais. before-edit.json.gz em
scope-tail conserva os consumers/owners anteriores.

### Classificação

Correção interna de paridade ADR-0127: L0 primeiro, RED→GREEN e revalidação.
Sem API pública, nova entidade, fase eval/layout ou default novo. Refutar
por wrappers irmãos, preservação da ordem, última linha, corpos opacos,
colunas e ausência de efeitos/estado vazando da medição. Não copiar defeito
upstream nem declarar paginação universal para cauda maior que a região.

### Responsabilidade exclusiva

Na fronteira Block/Align, fechar o segmento textual lexicalmente comum
pelo owner paragraph antes de despachar a unidade, preservando a chain
exterior. Segmentos terminam em nós opacos; wrappers inline não fecham por
si. Não repetir fechamento do handshake fracionário legado certificado.
Para novo prefixo geométrico certificado pela medição de paragraph, usar o
fechamento lexical uma vez e deixar Block consumir a cadeia de margens.
Preservar os hooks dos prefixos legados já certificados, direção/bidi,
agrupamento, transições e restauração da alocação. A antiga restrição a
prefixos escalares permanece para a rota legada; a nova medição isolada
positiva é autorizada para prefixo textual anteriormente recusado.


### Refutadores de extensão e primeira região

scope-tail-after-first/results.json.gz confirma26/28 documentos. Os dois
restantes são caudas com cinco linhas que excedem a primeira região; reservar
altura integral deixa o início32/17pt acima. O primeiro fragmento deve ser
medido sob a capacidade disponível, usando a mesma admissão de viúvas/órfãs,
e somente sua altura causal entra na reserva anterior ao1fr. Próximas regiões
preservam sua capacidade normal; não somar margem terminal de parágrafo.

scope-tail-refutators/results.json.gz mede os três controles antigos que
exigiam caudas desalinhadas. Três fragmentos numa linha agora coincidem
integralmente. Multiword21 e par40/8 com fill ainda divergem em entrelinha:
a medição isolada reproduz essa geometria defeituosa, não a certifica.
O novo fallback de prefixo geométrico exige uma única baseline textual;
prefixos multiline conservam os certificados escalares anteriores ou a rota
opaca. Caudas multiline continuam sob medição integral/primeiro fragmento.
Não alterar as asserções opacas desses dois prefixos para declará-los verdes.
O aninhamento parcial OUTER18 + Styled8 contendo Block também continua
incorreto no pin; um segmento local que entrou com linha exterior aberta
não prova o escopo do parágrafo inteiro e não recebe novo fechamento.


### Predicado da fronteira ordinária

A validação scope-tail/validation/followup-clippy-inventory.json registra
nonminimal_bool no predicado ordinary_boundary. Expressar a ausência de
prefixo no índice atual como is_none_or(prefix.block_index != part_index),
equivalente à negação de is_some_and(prefix.block_index == part_index).
Preservar exatamente a admissão de Block/Align e a posse do segmento;
não altera fluxo, margem, contrato público ou fase do pipeline.


## Prefixo multiline e aninhamento parcial — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/stat/UTC, fontes e binários contra o pin vanilla
9f2b6e8715237cb086899a42873660fe744622e8. Multiword21 gera duas páginas
contra uma; Long/More estão 6.5/13pt abaixo. O prefixo40/8 com fill gera duas
páginas; More está39.906pt abaixo. Nested OUTER18/INNER8 tem BODY12pt acima
e página auto12pt menor. sequence/fractional_preflight.rs:151–171 recusa
multiline; word_layout.rs:49 fecha dentro do estilo local; line_commit.rs:
398–480 usa esse estilo para floor/leading/top previsto. sequence.rs:239
recusa fechar segmento parcial; Block sem cadeia anterior usa spacing8pt.
Vanilla flow/collect.rs:129–145,179–214 resolve trunk dos filhos, spacing e
leading comuns; cada frame contribui sua geometria. Esses são observáveis
textuais, geométricos e de paginação da linguagem (ADR-0107), não obrigação
de copiar buffers, algoritmos ou bytes. A intenção histórica não é inferida.

### Decisão e limite

Correção interna de paridade ADR-0127 em fluxo contínuo: sem entidade,
trait/assinatura pública, nova flag/default ou fase. O núcleo text-segment-scope
pinado define as invariantes compartilhadas. Aceitação RED→GREEN dos três
casos, controles de fontes, cores, margens, colunas, restauração, checkpoint
e subframes, mais suíte completa e linhagem. A nova rota só amplia prefixos
multiline textuais positivamente certificados; não declara paginação universal,
bidi/floats nem conteúdo opaco. Preservar os certificados escalares anteriores.

### Responsabilidade exclusiva

Instalar/restaurar cadeia dona na entrada/saída lexical de Sequence; entrada com linha já aberta herda dono exterior. Antes de Block/Align em segmento parcial, fechar sob esse dono exterior. Após fronteira opaca reiniciar dono na chain local. Ativar geometric_text_lines só durante prefixo multiline identificado pelo runtime; fechar uma vez e restaurar antes do Block. Rotas legacy e single-line anteriores ficam fora desse modo.

Durante geometric_text_lines, a cadeia comum já certificada não é substituída por um wrapper filho que inicia com linha vazia; isso preserva o trunk também em corpos Sequence dos wrappers.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada,
status/diffstat/UTC e2099 fontes/L0s. before.json.gz contém92 pares;
extra-before.json.gz contém26; boundary-before.json.gz contém10 e
repeat-before.json.gz contém4. Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3,
binário9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt inicial2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095.
Em160pt/margem8/font12, hanging10 mantém a primeira linha x8 mas move
continuações x8→18; negativo-5 exige x3. RTL mantém primeira borda direita152
mas continuações vão para142. O Tekt ignora o comprimento. MLA tem default
18pt (1.5em), set ambiente3em não o troca, show-set3em exige36pt; IEEE/Plain
não adquirem recuo de hanging por esse show-set. bibliography.where(style:"mla")
não casa na sonda vanilla: conservar a decisão do seletor, não forçar override.

Block/Box com body apenas inline ignoram hanging10; Par explícito dentro
deles e Block com Parbreak aplicam. Heading/Raw/List/Quote simples não o
aplicam. Declaração parcial, zero, repetida e escopo vazio fecham fronteiras:
FIRST/MIDDLE/LAST em5.168/27.464/49.76, contra todos5.168 no Tekt. A primeira
sonda lexical usou colchetes literais: foi refutada, corrigida para #[...] e
preservada como extra-before-initial; não é evidência de escopo Styled.
Valores extremos geram tinta recortada pelo extrator: não equiparar ausência
em pdftotext a apagamento de Content. Fonte parcial24pt não muda o em do
comprimento compartilhado12pt, como partial-font registra.

Fontes pinadas: typst-realize/src/lib.rs:1061–1085,1190–1219 distinguem
fragmento/Par e interrupção; typst-layout/src/flow/collect.rs:72–134 escolhe
fluxo, inline/mod.rs:174,216–221 e collect.rs:141–147 resolvem forma;
inline/line.rs:496–503 posiciona; rules.rs:510–523 faz inset/compensação CSL.
typst-library/src/model/bibliography.rs:112–127,367 determina o default e
precedência. Isso mede língua/geometria; não exige a estrutura Rust vanilla
nem atribui intenção histórica. Refutação por fronteira perdida, em de run,
recuo duplo, reset em página, fallback greedy imposto, cópia perdida, callbacks
reexecutados ou regressões dos controles impede fechamento.

### Classificação e limites

Correção interna/de paridade ADR-0127, fluxo contínuo: sem campo ou assinatura
pública, flag/default externo novo, fase ou ordem de realização diferente.
A nova forma inicial/continuação supersede somente a limitação correspondente
de largura constante; RTL latino certificado segue as condições do núcleo,
sem BidiCursor ativo/tracking e sem inventar BidiSpan. Os demais limites
anteriores continuam. Os valores/cópia fora da página requerem inspeção dos
frames ou SVG, não igualdade de extração recortada. Não antecipar R3-T1.

### Responsabilidade exclusiva deste owner

Preparar escopo compartilhado completo antes de tentar seleção ou percorrer filhos; contexto já ativo é herdado por sequências parciais. Fronteiras explícitas reabrem escopo pelo canal existente, sem modificar preflight fracionário/bidi ou deduzir escopo por consenso de runs.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.
