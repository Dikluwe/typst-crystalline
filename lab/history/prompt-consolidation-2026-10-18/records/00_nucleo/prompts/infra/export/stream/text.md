# L0 — stream/text
Hash do Código: 244705cd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:d6da63fc3b0282aaeb2573e92e850f807a2691c04d3578c54e61c1f4e01a51f3
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:8b7fcfaf39e60540d16868e662d48fb466fab58860385fcdae499f8c2edfe799



**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/text.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Text and glyph operators

Own font lookup by family/variant/variations and Text/TextShaped/Glyph emitters in
Compact/Verbose. Type1 retains WinAnsi fallback. CIDFont/Multifont use supplied
old-to-new GID mappings; direct mathematical glyphs use their per-font reverse map.
Paint bitmap glyphs through prepared XObjects and placement metadata, not outline
fonts. Do not perform shaping, font instancing or resource allocation.

TJ uses (nominal - x_advance) / units_per_em * 1000 and compensates x_offset with
the existing paired adjustments, rounding and units. Preserve the current y_offset
scope without introducing new operators. Verbose uses balanced q/cm/color/BT/Tr/Tf/
Tc/Tm/TJ-or-Tj/ET/Q envelopes, explicit rendering mode, faux bold, tracking and spot
colors. Compact retains its current operator form. These helpers serve both top
and local dispatchers, whose coordinate composition is owned by draw.

Coalesced-run compatibility, boundary selection and rounded inter-item TJ
compensation belong to `00_nucleo/prompts/infra/export/stream/text/runs.md`.
Declare the descendant runs module and preserve the internal reexports used
by the stream/page dispatcher. This owner retains single-item text/glyph
operators and their shared envelope helpers; do not duplicate run decisions.

## Verification

Exercise kerning, offsets, remapping, same-GID multifont identity, bitmap placement,
mode envelopes. Run boundaries and mathematical/UA1 noncoalescing are
verified through the runs owner and existing integration regressions.

## Substituição e restauração gráfica — refinamento medido

### Medição anterior à decisão

O primeiro build SHA-256 8b223826b2a6ec44bf95a82235e5caaae57777eba27be27200e084e3ce389241
(`after/`, `after-extra/`) conserva SHY no compacto, mas no verboso a extração
coloca os substitutes em x=0/y=400. O experimento controlado de 18 streams em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/envelope-probe/`
refuta a abrangência inicial: fechar EMC antes de Q conserva caixas; fechar após
Q perde a posição no Poppler. Abrir antes/depois de cm/Tm não muda essa conclusão.
Não fechar por contagem de codepoints, nem reescrever streams procurando Q.

### Decisão interna e responsabilidade

Formatter textual conserva suas assinaturas existentes e fornece variantes
privadas com tail de restauração gráfica diferível: emitir todo o texto/ET,
retornar se há Q pendente em vez de restaurar nesse momento. Type1 considera
somente o q de faux-bold; fontes embutidas no verboso têm q próprio; bitmap
não cria tail textual. O dispatcher pede essa forma apenas ao último filho
visual de TextReplacement. Ele fecha as marcações de folha, depois ActualText,
e só então emite Q pendente. Filhos anteriores e itens normais mantêm emissão
existente. Compact/Verbose preservam seus operadores e transformações, sem
troca de modo ou heurística sobre o texto dos operadores.

É refinamento privado de paridade, sem contrato público/fase/flag nova. O owner
text possui formatação/tail, draw possui despacho e marcações de folha, e
text_replacement possui exclusivamente a codificação/BDC/EMC. Aceitação requer
extração textual na posição original em top/local e ambos os modos, coexistência
literal, scopes balanceados e raster preservado.

## Montagem Large: posição de ActualText no estado Glyph

A sonda bilateral `math-large-identidade-2026-09-30` usa o vanilla pinado e
NewCMMath com DisplayOperatorMinHeight=3000du. Envolver as três peças de `∫`
em ActualText fora dos estados gráficos devolve o caractere na extração simples,
mas Poppler mede caixa de altura zero no Verbose e coloca a ocorrência antes de
`∑` em `-layout`; o Compact também inverte a ordem. Experimento de PDF temporário
com as mesmas peças e posições, sem mudar GIDs, refuta o escopo exterior: abrir
BDC após BT da primeira peça e fechar EMC antes de ET da última recupera caixa
positiva e a ordem `∑`, `∫` em ambos os modos. Não inferir texto de GIDs.

Oferecer variantes internas dos emissores Glyph que aceitam abertura/fecho
causais fornecidos pelo dispatcher, reaproveitando integralmente seleção de
fonte, remapping de GID, cor e posições dos emissores normais. Só o ponto de
inserção de BDC/EMC muda para peças de um TextReplacement de glifos. Não
alterar o formato dos glifos comuns, o modo default, o caminho Type1 nem
acrescentar texto invisível. O helper de marcação continua pertencendo a
`text_replacement.md`; este owner só posiciona as chamadas no bloco Glyph.

## Tracking shaped — soma única e estado PDF

### Medição anterior à decisão

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/tracking-controls.json`
mede plain-verbose com desvio máximo 2.652pt contra vanilla pinado,
enquanto plain-compact tem 0.348pt. Estado, binários e comandos estão no
recibo daquela frente. O novo estado e a matriz estão em
`lab/results/construction/diagnostics/tracking-pdf-2026-09-28/initial.json` e `before/`.
`03_infra/src/shaper/shaping.rs:319-326` soma tracking aos avanços;
`03_infra/src/export/stream/text.rs:639,668` e
`03_infra/src/export/stream/text/runs.rs:107` emitem Tc novamente.
O cursor de runs considera os avanços e ajustes TJ, sem essa soma Tc.
O vanilla pinado aplica tracking nos glyphs em
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1269-1292`.
É comportamento de espaçamento medido, não requisito de operador ou bytes.
A hipótese é refutável por avanços sem tracking ou ausência do Tc adicional.

### Decisão de paridade interna

TextShaped CIDFont/Multifont carrega os avanços autoritativos já ajustados:
tracking deve vir desses avanços somente. Tanto emissão single quanto
coalescida estabelece explicitamente Tc=0 dentro do bloco BT antes de TJ,
inclusive tracking ausente/zero, para eliminar estado herdado. O zero é
o estado normativo de espaçamento adicional, não uma exceção de fonte, GID,
fixture ou largura. Não subtrair tracking dos avanços nem inferir causa
pelo texto. Text e fallback Type1, que emitem Tj sem avanços shaped,
continuam usando seu tracking existente. Bitmap continua desenho de imagem.
Sem contrato público, flag, modo padrão ou fase nova: fluxo ADR-0127.
Arredondamento dos avanços, medição de layout e política de tracking em
ligaduras são frentes independentes; esta correção não os certifica.

### Responsabilidade exclusiva e aceitação

Own helper privado push_shaped_spacing, que emite Tc=0. Chamá-lo no BT
de TextShaped CIDFont/Multifont compacto/verboso, inclusive variante de
restauração diferida. Preservar delta TJ, offsets, remapping, scopes,
transformações e o caminho bitmap. O child runs usa o mesmo helper.
Os testes deste consumer verificam a posição de cada glyph usando um
interpretador mínimo independente de Tc/TJ, com estado Tc anterior não
zero, tracking positivo/negativo/zero/ausente, CIDFont e Multifont. Verificar
também fronteiras absolutas do run fundido e Text/Type1 ainda com tracking.
Sondas PDF exigem equivalência visual compacto/verboso e preservação
lexical SHY/U+002D; progresso contra vanilla deve ser medido, sem fechar
arredondamento já divergente por normalização ou tolerância alargada.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`lab/results/construction/diagnostics/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `lab/results/construction/diagnostics/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Nos emissores shaped single/bitmap, usar resíduo fracionário derivado do owner tracking. TJ preserva o delta inteiro/rounding anterior de kerning e acrescenta somente o resíduo de tracking; serializar cinco casas quando resíduo existe. X-offset e zero-tracking conservam formato anterior. Private helper TJ pode receber TextStyle; nunca inferir tracking de GID/texto. Bitmap soma o mesmo resíduo ao cursor, sem novo operador textual. Tc=0 segue obrigatório em shaped.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Memoização local da associação — medição 2026-10-05

`first-benchmark.json` desta rodada mede +25.36ms na fixture grid320 e +14ms em rtl160/mixed160. `font_binding.rs:index` compara bytes completos por consulta; `runs.rs:verbose_run_compatible` compara Font por bytes em cada adjacência. Inferência causal, a confirmar por A/B.

Usar cache privado por tabela de recursos/exportação para positivos validados: conservar clone Arc da fonte para manter sua memória viva e index após igualdade de bytes/pedido na primeira ocorrência. Acerto exige identidade de slice (endereço e comprimento) e mesmo pedido completo. Não serializar endereço ou criar identidade pública por ponteiro. Fontes de alocações distintas consultam igualdade completa na primeira vez. Não cachear miss no binder que anexa recursos; índices positivos permanecem estáveis por append. Nenhum global ou reúso entre sessões. Comparação de merge usa identidade de slice como atalho para igualdade, mantendo comparação de bytes para fontes de alocações diferentes. Este cache não memoiza nem invalida conteúdo entre Worlds.

## Eixo real e espessamento sintético — correção interna 2026-10-05

Medição anterior à decisão: controle variable-axes-after.pdf tem fonte wght250 com hmtx e pontos glyf corretos, mas stream contém `2 Tr`/`0.800 w` pelo style.weight=700. Raster 600dpi revela tinta adicional que bbox de palavras não revela. A causa é `stream/text.rs:274`/calls shaped e `stream/text/runs.rs:105`, que reaplicam faux-bold apesar de wght efetivamente realizado. Fonte/eixos originais e semântica permanecem distintos.

Decisão de paridade visual (ADR-0107): somente para TextShaped com fonte certificada Some cuja fvar declara eixo wght, emissão verbose simples e coalescida usa preenchimento 0 Tr; não sintetizar peso por cima da instância variável já preparada. Helper privado shared no owner text verifica o eixo real somente quando o estilo pediria stroke. Manter Text/Type1, None e fontes estáticas no comportamento vigente, sem reescrever weight/bold semântico e sem nova API, fase, unsafe ou I/O adicional. Compact shaped já não aplica esse stroke. Faux-bold de fontes estáticas e outros formatos continua fora desta correção. Testar single+coalesced com weight700/wght250 em RED→GREEN e conservar None; raster e caixas bilaterais completos.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/stream/text.rs` nas linhas 9 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.

### Refinamento de escopo após compilação — 2026-10-13

A primeira compilação de todos os targets identificou PageContext nos testes internos deste consumer (linhas 1018/1225/1344 do candidato, check-initial-failed.log). Restaurar seu import sob cfg(test): não remover a dependência da suíte nem reintroduzi-la na biblioteca normal. Evidência e estado: `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/`. Revalidar todos os targets e preservar nomes/assertions da suíte.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

No prefixo verboso compartilhado (inclui runs fundidos), aplicar ExtGState da cor textual dentro de q já existente antes de BT. O dispatcher isola caminhos compactos e glifos diretos. Não mudar coalescência, métricas, ActualText ou posição. O par alpha é igual em fill e stroke para faux-bold.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C051`: `03_infra/src/export/stream/text.rs:342`, `emit_shaped_pdf`, 9 argumentos contados pelo Clippy (limiar 7).
- `C052`: `03_infra/src/export/stream/text.rs:367`, `emit_shaped_pdf_deferred`, 9 argumentos contados pelo Clippy (limiar 7).
- `C053`: `03_infra/src/export/stream/text.rs:392`, `emit_shaped_pdf_inner`, 10 argumentos contados pelo Clippy (limiar 7).
- `C054`: `03_infra/src/export/stream/text.rs:533`, `emit_glyph_pdf_marked`, 9 argumentos contados pelo Clippy (limiar 7).
- `C055`: `03_infra/src/export/stream/text.rs:719`, `emit_shaped_pdf_verbose`, 9 argumentos contados pelo Clippy (limiar 7).
- `C056`: `03_infra/src/export/stream/text.rs:744`, `emit_shaped_pdf_verbose_deferred`, 9 argumentos contados pelo Clippy (limiar 7).
- `C057`: `03_infra/src/export/stream/text.rs:769`, `emit_shaped_pdf_verbose_inner`, 10 argumentos contados pelo Clippy (limiar 7).
- `C058`: `03_infra/src/export/stream/text.rs:895`, `emit_glyph_pdf_verbose_marked`, 9 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C051` — `emit_shaped_pdf`: Exceção pontual justificada

Emissor PDF Compact recebe texto autoral para fallback Type1, glyphs e face efetiva separados, estilo, posição PDF, escala e cenário de recursos. A aridade preserva identidade e evita reshaping; wrapper restaura o tail normalmente.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C051: Emissor PDF Compact recebe texto autoral para fallback Type1, glyphs e face efetiva separados, estilo, posição PDF, escala e cenário de recursos")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C052` — `emit_shaped_pdf_deferred`: Exceção pontual justificada

Variante Compact diferida possui os mesmos inputs causais e retorna o Q pendente para fechar ActualText antes da restauração gráfica. Esse protocolo explícito está prescrito no L0.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C052: Variante Compact diferida possui os mesmos inputs causais e retorna o Q pendente para fechar ActualText antes da restauração gráfica")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C053` — `emit_shaped_pdf_inner`: Exceção pontual justificada

Núcleo Compact acrescenta restore à operação shaped; o controle distingue tail normal de diferido sem duplicar emissão. FontScenario e fonte efetiva não são substituíveis por nome, nem texto é derivável de GIDs.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C053: Núcleo Compact acrescenta restore à operação shaped; o controle distingue tail normal de diferido sem duplicar emissão")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C054` — `emit_glyph_pdf_marked`: Exceção pontual justificada

Emissor de glifo marcado recebe abertura e fecho causais de ActualText independentemente. Uma ocorrência Large pode abrir na primeira peça e fechar na última; não deduzir esses limites pelo GID ou stream.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C054: Emissor de glifo marcado recebe abertura e fecho causais de ActualText independentemente")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C055` — `emit_shaped_pdf_verbose`: Exceção pontual justificada

Emissor Verbose preserva a mesma identidade shaped, posição PDF, escala e recursos com envelope específico. A diferença de formato pertence a este owner e não autoriza reconstruir texto ou fonte.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C055: Emissor Verbose preserva a mesma identidade shaped, posição PDF, escala e recursos com envelope específico")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C056` — `emit_shaped_pdf_verbose_deferred`: Exceção pontual justificada

Variante Verbose diferida transporta os mesmos inputs e informa restauração Q pendente. O L0 exige manter ActualText e restauração nesta ordem.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C056: Variante Verbose diferida transporta os mesmos inputs e informa restauração Q pendente")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C057` — `emit_shaped_pdf_verbose_inner`: Exceção pontual justificada

Núcleo Verbose recebe restore explicitamente para compartilhar a implementação normal/diferida; os demais inputs são os operandos reais do texto e seu cenário de fontes.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C057: Núcleo Verbose recebe restore explicitamente para compartilhar a implementação normal/diferida; os demais inputs são os operandos reais do texto e seu cenário de fontes")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C058` — `emit_glyph_pdf_verbose_marked`: Exceção pontual justificada

Variante Verbose do glifo marcado conserva abertura/fecho de ocorrência Large dentro de BT/ET e o cenário nominal/remapeado. Reduzir a aridade por inferência perderia o vínculo causal.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C058: Variante Verbose do glifo marcado conserva abertura/fecho de ocorrência Large dentro de BT/ET e o cenário nominal/remapeado")], somente nesta função.

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


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/export/stream/text.rs:25 (C050).
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Usar o alias público existente export::FontKey no parâmetro fonts de font_index_for_style, com expansão idêntica ao tipo anterior. Remover apenas os imports de FontList/FontVariant/FontVariations tornados inúteis; preservar a seleção por família/variante/eixos, fallback, estilo e todas as operações PDF.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/export/stream/text.rs:1073 (clippy::double_ended_iterator_last). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

No observador rendered_positions de testes, selecionar o último Tc com next_back no iterador duplo de linhas filtradas. O filtro strip_suffix é puro; conservar parse, fallback zero, cálculo independente de posições e todos os esperados TJ. Não alterar o emissor.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C051, C052, C053, C054, C055, C056, C057, C058. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.
