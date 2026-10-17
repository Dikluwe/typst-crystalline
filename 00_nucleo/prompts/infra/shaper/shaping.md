# L0 — infra/shaper/shaping
Hash do Código: 63280d13

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:3c6209895df993843d15259c7ee95195e7f0dc732ee56cf3cc553257e30298b0
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml sha256:661b7b74b7601c1b4fb11dda93fea95ee8ebcd62f9e3605507e5ce1373334808
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/shaping.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/shaper/shaping.rs:21` — `ssty_level_for_subrun`.
- `03_infra/src/shaper/shaping.rs:38` — `shaped_width`.
- `03_infra/src/shaper/shaping.rs:106` — `try_shape`.
- `03_infra/src/shaper/shaping.rs:366` — `byte_idx_to_char`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/shaping.rs`.

`00_nucleo/diagnosticos/infra-performance-followup/language-control.json` shows
that explicit ja/zh/ko settings do not affect Tekt glyph selection while a same-face
rustybuzz control does. Both local buffer sites omit language configuration.
Pinned vanilla configures language at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`.
Language selection is a style semantic input; glyph IDs are discriminating render
controls, not proof of full language parity. The cache structure is mechanics.
The repository Noto Sans fixture supplies independent ru/sr local substitutions
with different advances; controls must cover measurement and emission separately.

## Owned contract

- Own shaped_width, try_shape, UTF-8 cluster mapping, rustybuzz setup, variations, tracking, ssty and TextShaped materialization. Use sibling bidi/selection/cache helpers rather than duplicate their policies.
- Clone the cached base face before applying axes. Derived variant axes are merged with explicit style variations, explicit values winning by tag; preserve nonvariable/unknown-axis no-op behavior.
- Configure each width-measurement and emission rustybuzz buffer with the explicit TextStyle.lang when present, using its normalized language code. Leave language unset when absent, preserving the existing inferred/default behavior. Pass the same optional explicit language identity to the run-cache owner. Set direction per bidi run; preserve language, offsets, glyph ids/advances and clamped positive units_per_em. Each emitted TextShaped.text is the sub-run text and glyph clusters are UTF-8 byte offsets relative to that text, with replacement character fallback for an invalid mapping. BidiRun.byte_start is a separate original-input coordinate owned by the bidi child. Whitespace-only input takes the unchanged-Text path; traversal owns the rule against reshaping TextShaped.
- Measure multiline width as the maximum paragraph width. Emission resets x for each paragraph and advances y by the first primary's resolved top/bottom edges plus style leading (default 0.65em). This does not repair L1 vertical flow/pagination for an embedded newline.
- Preserve the existing distinct measurement/emission feature and tracking paths rather than harmonize them speculatively. In emission calculate the rounded i32 tracking increment once per candidate from resolved tracking points, positive units_per_em and style.size, using the existing conversion formula. Pass that same increment to the cache key and add it only before a following different cluster, never inside a cluster or after the last glyph of a sub-run; negative tracking remains valid.
- Math mode is decided by style.math. Consume the canonical ssty_eligible_text from font_metrics; request levels 1/2 only for eligible Script/ScriptScript outside math_text_item. TextItem keeps math family/target features and every non-ssty property but no explicit ssty feature (`None`), also represented as absence in its cache key.
- Preserve forced math script behavior independently of requesting ssty and existing default OpenType features. No family/codepoint/fixture exception, speculative feature change, vertical shaping or nonzero face-index support is introduced.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## SHY autoral — controle de glifos invisíveis

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/first-round/after/`
refuta a primeira materialização: trailing e ligature conservam os codepoints
mas ampliam a caixa de ActualText em 3.12pt e perdem o espaço na extração comum.
O caso tracking também inclui um intervalo derivado do placeholder invisível.
`first-round/final-receipt.json` conserva o estado e as falhas.
Rustybuzz 0.20.1, src/lib.rs:32-35, documenta que o tratamento padrão troca
Default_Ignorable por glifo de espaço com avanço zero; REMOVE_DEFAULT_IGNORABLES
remove esse placeholder depois do shaping. Um cluster compartilhado de f+SHY+fi
também pode dar hex vazio ao GID de espaço, bloqueando seu mapeamento posterior.
A decisão é refutável por alteração de glyphs visíveis/ligaduras no controle
com a flag documentada ou por contaminação de runs sem SHY.

### Responsabilidade

Configurar REMOVE_DEFAULT_IGNORABLES nos sub-runs cujo texto autoral
contém U+00AD ou um controle bidi especificado abaixo, em medição e emissão. Delegar a configuração a text_identity,
antes de shape; não remover SHY do texto de entrada, não filtrar por GID
ou avanço zero e não alterar shaping de runs sem SHY nem controles bidi. Os caracteres lógicos
completos continuam na ocorrência. A flag remove placeholders, não glyphs
visíveis de seleção de quebra. Tracking incide nos clusters que restam e
não acrescenta intervalo por um placeholder que não existe mais.
Cache permanece por texto completo; a política é determinística pelo texto.
Não mudar assinatura pública, fase, fallback, features OpenType ou fontes.
Texto exclusivamente invisível e seleção de quebras continuam fora do escopo.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Conservar representação/cache do tracking i32. Ao posicionar o próximo sub-run, usar width derivada pelo owner tracking, que acrescenta somente o resíduo fracionário nos pares de clusters elegíveis. Não alterar glyph IDs, texto, features ou seleção de fontes. O measured shaped_width sem tracking mantém seu contrato.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Contexto integral bidi ratificado — 2026-10-05

Medição anterior à decisão: o lote bilateral e a refutação Unicode estão em `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/{initial,probe}.json.gz`, com fontes/estado/binários. `layout_bidi.rs:73–82,227–240` redetecta base e inventa separadores; `shaper/bidi.rs:34` redetecta o fragmento; isto perde a entrada integral ratificada pelo humano. Correção interna de paridade, sem nova fase/assinatura pública externa.

Para spans certificados, usar registry proprietário local de strings auxiliares e mapas por input Arc, depois views BidiInfo emprestadas normais; não usar unsafe/self-reference. Embeddings virtuais LRE/RLE/PDF só quando dir do segmento difere da base; espaços reais e Linebreak não justificado LF (justify=true fora do certificado). Não retirar controles autorais. Resolver uma vez por input em cada registry; o pipeline paginado compartilha registry/views entre reorder e shape. Wrappers públicos existentes podem construir registry local.

Reorder consulta intervalo real da linha, intersects exatas por segment/range e níveis com regras de fim de linha; não midpoint nem espaços inventados. Text misto divide por fronteiras originais, preserva estilo e subspan. Itens não textuais e fontes desconhecidas não ganham certificado. Reorder conserva âncora física aceita por L1, largura/avanço reais; não aplicar fallback de suffix/coalescing ao contexto certificado.

Shape consulta os runs do input completo para span original e não auto-detecta direção de fragmento; manter offsets de subruns por travessia sequencial, emitir subspan exato para cada TextShaped. Chave de cache existente já discrimina direção; não pôr contexto em TextStyle. Reconciliation usa base do input quando existe em vez de qualquer run RTL. Fallback None anterior permanece. Testes RED→GREEN da refutação de 456 789, overrides/neutral, extração sem controles auxiliares e medição bilateral.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


Conservar a Font carregada no CachedRun e retorná-la com os glyphs: miss carrega a fonte uma vez, hit usa a fonte do resultado memoizado. O carrier não resolve World::font novamente depois de produzir/consultar os GIDs. Medição e contrato do cache estão no owner `00_nucleo/prompts/infra/shaper/cache.md`; sem unsafe novo ou API nova.

## Controles bidi autorais — identidade de espaço (2026-10-06)

### Medição anterior à decisão

`00_nucleo/diagnosticos/controles-bidi-pdf-2026-10-06/before.json` registra
HEAD, working tree, fontes de código/L0 e binário anterior. As sondas
`bidi-contexto-rtl_literal_embedding-cmap-before.json` e
`bidi-contexto-rtl_forced_mark-cmap-before.json` mostram CID 0001 associado
respectivamente a U+202C e U+200F: espaços posteriores extraem controles e
fundem palavras. A extração bilateral com fontes e comandos está no lote
`00_nucleo/diagnosticos/block-align-2026-10-06/verification.json.gz`.
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:992-997`, pin
9f2b6e8715237cb086899a42873660fe744622e8, remove placeholders Default_Ignorable
após shaping. `03_infra/src/export/fonts.rs:307-328` mantém controles na
substring global por GID; `03_infra/src/export/builder/fonts.rs:523-535`
retém o primeiro mapeamento. São observáveis de texto e geometria, não
paridade de bytes ou obrigação de copiar o algoritmo upstream.
Hipótese refutável por persistência da colisão após remover placeholders e
controles apenas do mapa visual. Não inferir intenção histórica.

### Decisão interna e limites

Correção interna de paridade ADR-0127, sem campo/assinatura pública, fase,
flag, default de produto ou unsafe novo. As invariantes compartilhadas pertencem ao núcleo bidi-visual-identity pinado acima. SHY conserva envelope ActualText existente por ocorrência.
Controles bidi não ganham envelope ActualText: o vanilla não os copia como
identidade dos espaços nestas sondas. Aceitação exige espaços/letras separados,
clusters Unicode preservados e controle visual; extratores podem inserir seus
próprios marcadores de direção, logo texto bruto não prova identidade autoral.

Este owner delega a configuração ao owner text_identity nos dois pontos de buffer, medição e emissão; cache conserva texto autoral completo.

### Resultado vazio válido versus falha

A primeira extração após remover placeholders, preservada em
`00_nucleo/diagnosticos/controles-bidi-pdf-2026-10-06/first-verification.json.gz`,
mostra RLM residual no CID 000D. `shaping.rs:389-393` devolve None quando
items está vazio, e `traversal.rs:98-109` conserva o Text como fallback.
Para entrada não vazia composta exclusivamente do conjunto bidi, ao fim do
shaping bem-sucedido retornar Some(Vec vazio). Falhas anteriores de fonte,
resolução de contexto ou buffer conservam None; demais entradas vazias ou
não certificadas conservam política existente. Não fabricar glifo/envelope.

A saída antecipada por paragraphs sem runs também precisa distinguir o
resultado vazio válido: com primária legível e texto não vazio exclusivamente
bidi, retornar Some(Vec vazio). A inspeção first-raster.json refuta o fechamento
apenas do final da função: LRI voltou como Text e desenhou notdef. Sem primária,
sem contexto resolvido ou para outros textos, conservar fallback anterior.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/shaper/shaping.rs` nas linhas [113] apresenta os avisos selecionados.
`00_nucleo/diagnosticos/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

try_shape é fachada pub(super) sem contexto usada apenas pela suíte. Compilar essa fachada somente em cfg(test), conservando assinatura e delegação integral a try_shape_context. try_shape_context, shaped_width e todos os caminhos de produção permanecem compilados normalmente. Não limitar os controles de SHY/bidi/fontes ou mudar None versus resultado vazio válido.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C117`: `03_infra/src/shaper/shaping.rs:124`, `try_shape_context`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C117` — `try_shape_context`: Exceção pontual justificada

Owner do shaping recebe dois caches com lifetimes próprios, serviços e dados da ocorrência, mais BidiSpan causal e view resolvida compartilhada. O L0 proíbe redetectar direção do fragmento ou pôr contexto em TextStyle; manter a injeção explícita.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C117: Owner do shaping recebe dois caches com lifetimes próprios, serviços e dados da ocorrência, mais BidiSpan causal e view resolvida compartilhada")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C117. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## V16 — projeção nominal de MathSize para ssty (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-ssty-2026-10-17/before.json.gz conserva UTC,
status, git diff HEAD --stat, fontes/L0 e 2128 hashes conferidos do fechamento
anterior. O SARIF em 00_nucleo/diagnosticos/import-syntax-2026-10-17/validation-architecture.log
registra 117 V16, incluindo font_metrics/math.rs:337 e shaper/shaping.rs:28.
MathSize em 01_core/src/entities/layout_types.rs:147 possui Display, Text,
Script e ScriptScript. math.rs:330–338 e shaping.rs:21–30 retornam Some(1/2)
para Script/ScriptScript; o wildcard devolve None para os dois tamanhos atuais
restantes e ocultaria uma variante futura. Os L0 vigentes já exigem esses
níveis e gates de math/TextItem/elegibilidade; não há decisão linguística nova.
before-probe-result.json conserva fontes/hashes, rustc/argv/UTC e 224 controles
com resultados atuais. A sonda extrai funções e declaração real de MathSize,
modelando somente os três campos de TextStyle lidos; no mutante descartável,
a variante TektSstyFuture é aceita em ambos os matches (zero E0004). O RED é
essa falta de proteção estrutural; controles funcionais atuais passam.

É mecânica de manutenção Rust: enum crescente precisa de decisão explícita;
não é medição nova de paridade nem de velocidade de leitura por IA. Os níveis
ssty continuam o contrato de linguagem/render anterior. A inferência de que o
reparo conserva os quatro casos atuais é refutada por qualquer controle,
replay textual ou regressão divergente. Não substituir uma decisão semântica
por enumeração cosmética: Display/Text são ausência intencional de pedido,
justificada no núcleo pinado; Script/ScriptScript mantêm níveis distintos.

### Responsabilidade exclusiva

Em ssty_level_for_subrun, conservar assinatura/guard e predicado canônico; importar MathSize no use privado de layout_types existente, usar os nomes MathSize nas quatro variantes e substituir somente o wildcard por MathSize::Display | MathSize::Text => None. Não alterar shape, medição/emissão, caches ou transporte.

A política compartilhada está em 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml,
pinada neste owner. Fluxo contínuo ADR-0127: nenhuma mudança de contrato público,
comportamento padrão ou fase; campos e enum público permanecem byte a byte.

### Aceitação

Primeiro registrar o RED estrutural e os controles antes do código. Depois,
a mesma sonda com variante futura precisa produzir exatamente dois E0004 nos
matches auditados; controles 224/224 idênticos. O modelo parcial de TextStyle
não prova shaping integral: conferir também os testes reais existentes de
font_metrics e integração ssty/TextItem, a suíte workspace e os gates finais.
Não adicionar testes que apenas reescrevam o match; o guard de crescimento é
a nova propriedade verificada. Replay do código permite somente o reparo
nominal prescrito e o hash do header. SARIF: retirar precisamente os dois V16,
adicionar duas notas V19 dos grupos Display/Text com motivo e cobertura dos
dois tamanhos; todos os outros diagnósticos devem permanecer por fonte, regra,
severidade e mensagem. Sem V20 novo, supressão ou alteração de configuração.
Grafo V15/V26 antes de resselo restrito aos dois pares; build, fmt, Clippy
all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
V1/V5/V15/V26, reverse lineage, suíte completa, diff --check e integridade.
Comandos pesados sequenciais sob MemoryMax12GiB/High10GiB/swap0/jobs1; sem
commit/push/merge/release. O objetivo global continua além deste lote.
