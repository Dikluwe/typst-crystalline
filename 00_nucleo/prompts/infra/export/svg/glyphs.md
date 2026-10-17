# L0 — svg/glyphs
Hash do Código: 7790632c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/glyphs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

## Consumer evidence

`03_infra/src/export/svg/glyphs.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font context and glyph definitions

Own GlyphFontRequest/SvgGlyphFontContext, immutable request normalization/lookup,
GlyphDefs, outline extraction/cache/path builder, direct/shaped glyph painting and
the existing HTML textual writer. Follow the pinned glyph-font-context nucleus;
its shared resolution invariants are not redeclared by the façade.

Use the resolved FontKey's supplied bytes for glyph_id, verifying the key exists
in the collection. Deduplicate definitions by font identity and GID. Apply size /
units_per_em, position and Y inversion once; preserve shaped advances/offsets,
fill/alpha and supplied styles without reshaping or moving the layout cursor.
Keep OutlineBuilder methods as trait methods and current visibility.
Absent/ambiguous mapping, invalid face, zero units_per_em or missing outline remains
Unknown with the existing fallback, not a fabricated glyph. Fontless wrappers do
not synthesize text/outlines or probe incidental/system fonts or network.

## Verification

Exercise reordered fonts, same-GID distinct fonts, mathematical assemblies,
fill/alpha, transforms and missing/invalid/fontless contexts.


## Simplificações locais de Clippy — 2026-10-15

### Medição anterior à decisão

`03_infra/src/export/svg/glyphs.rs:319` declara mutável a coordenada y, que não recebe nova atribuição.
`00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/before.json.gz`
conserva fontes, owners, HEAD, working tree não commitado, UTC e diffstat;
selected-red.json identifica os diagnósticos do compilador antes da edição.
São expressões Rust equivalentes, sem nova medição de paridade upstream.

### Responsabilidade e aceitação

Manter y imutável no escritor de glifos. Preservar seu valor, offsets, avanços, inversão de Y, preenchimento e identidade das definições.

Manutenção interna em fluxo contínuo ADR-0127: conservar contratos públicos,
defaults e fases. Não adicionar supressões nem testes que espelhem sintaxe.
O gate dos avisos selecionados deve passar a GREEN; validar a suíte existente,
build, Clippy all-targets, formato, documentação e linhagem com resselo restrito.


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

- `C066`: `03_infra/src/export/svg/glyphs.rs:286`, `render_text_shaped`, 9 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C066` — `render_text_shaped`: Correção interna prescrita

O helper SVG de contornos ignora _text; o caminho HTML textual é escolhido antes no dispatcher e usa outro writer. Remover somente o argumento morto; os oito inputs restantes conservam face efetiva, glyph definitions e escala, com exceção pontual residual.

Remover somente _text do leaf SVG de contornos e seu argumento no dispatcher. Após passar de nove para oito inputs, autorizar expect local para os oito inputs efetivos.

Aceitação: O caminho HTML continua emitindo o texto original antes do leaf; o SVG conserva glyph identity/escala/offsets e definições. Não remover texto do FrameItem ou da API.

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


## Aridade interna — materialização de seis casos, 2026-10-16

### Medição anterior à decisão

As fontes, chamadores, cláusulas vigentes, spans e inventário de 28 avisos foram conferidos antes da decisão. `00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/before.json.gz` guarda fontes/L0 completos, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A auditoria anterior classifica estes casos como correção; não são exceções diretas. O RED deve observar as aridades atuais antes de alterações; controles funcionais existentes serão corridos antes/depois. É refatoração de representação privada, sem nova paridade ou ganho de performance medido.

### Responsabilidade exclusiva e aceitação

C066: retirar somente _text do helper pub(super) de contornos, mantendo oito inputs e o corpo atual. Depois do cleanup, usar #[expect(clippy::too_many_arguments, reason="C066: Leaf SVG usa oito inputs efetivos de contorno, face, escala e definições; texto HTML segue writer separado")]. Não retirar texto do writer HTML, da entidade ou das APIs públicas; manter upem==0, fallback, glyph IDs, defs, escala e avanços.

Fluxo contínuo ADR-0127: nenhuma API externamente pública, default do produto, fase, dependência ou semântica é alterada. Tipos/constructors novos ficam internos; contratos públicos existentes permanecem. Este adendo concretiza somente os seis casos nominais; substitui proibições genéricas de supressão apenas para as duas expectativas residuais condicionadas C047/C066 após remover _text. As outras cinco aridades a corrigir continuam visíveis. Não elevar limiar ou silenciar no módulo/impl/crate. L0 precede código e resselo restrito. Confirmar preservação das assinaturas externamente públicas, campos/valores transportados, controles PNG/SVG/PDF, precedência/diagnósticos/recursos/MCIDs e ausência de mutações globais. Usar os testes existentes sem relaxar fixtures/esperados; não escrever testes que espelhem os carriers privados. Revalidar suíte workspace completa, build, Clippy all-targets e unfulfilled_lint_expectations, Rustdoc, fmt e linhagem estrita/reversa, mantendo 12 GiB, swap zero e um job.
