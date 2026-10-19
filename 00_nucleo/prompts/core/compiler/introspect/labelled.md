# L0 — introspect/labelled
Hash do Código: 98bf0d4c

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/labelled.rs`.
The pure compute_labelled body at `:24` reads typed, location-aware counters.
The auto Label branch at `01_core/src/compiler/introspect.rs:2300` passes
the causal wrapper chain's language, with the explicit language argument only
as fallback. Real occurrence Start/End and generic snapshot handling remain parent-owned.
The Label descendant owns its complete walk branch and derived label handling.
Heading language text belongs to the canonical helper at
`00_nucleo/prompts/core/compiler/lang/heading_supplement.md`.

## Walk-branch refinement measurement

At the uncommitted state captured in
`lab/results/construction/diagnostics/atomizacao-introspect-heading-label-2026-09-30-before.json.gz`,
`01_core/src/compiler/introspect.rs:2280-2350` owns the Label walk branch,
while this consumer already owns location-aware resolution. The `introspect`
test filter passed 263 tests before movement. This is evidence of separable
behavior, not a new timing or bilateral vanilla measurement.

## Contract

Preserve the pure descendant free function over introspector, target Location,
Content and optional language, returning (optional resolved text, optional
figure number). Do not mutate stores or allocate identities.

For Heading, read its element-function hierarchical counter at Location and
join canonical localized supplement, a space and the number when available.
For a block Equation, read its element-function flat counter; a positive number
produces the existing compatibility text `Equação (N)`, otherwise no text.
For Figure, read its per-kind counter (default image); a positive number
produces localized figure supplement plus number and returns that figure number.
Otherwise return present empty text and no figure number. Other targets return
neither. Do not silently unwrap arbitrary Styled targets or infer a missing
numbering gate from the body's render.

This helper preserves compatibility resolved text; it is not the authoritative
numeric-reference path and must not override its precedence. It does not replace
the equation runtime supplement owner with its compatibility literal.
The Label walk branch reuses target Location, captures causal language and
publishes the derived association only when applicable.

## Derived-label walk

The Label descendant walks its target first with the causal label. For an auto
label it finds the first newly emitted real Start, resolves Heading/Equation/
Figure text with the wrapper chain language before the explicit fallback, and
publishes a derived Labelled Start/zero-hash End at that same Location only when
resolution is available. If the target has no real Start, classify it as
unreferencable. For an explicit user label, walk the body once with the label,
compare known associations before/after and classify unsupported targets when
no association appears. Preserve numeric mapping precedence; do not remove a
counter key to force legacy text. The parent match remains exhaustive and
calls this descendant statically, without a second traversal or new Location.

## Acceptance and phase boundary

Check hierarchical/flat counters at the supplied target Location, missing
values, captioned/numbered Figure paths and unsupported targets without invented
associations. Check mixed-language and forward/backward heading references
against the causal store, including an explicit synthetic language fallback.
A language present in the wrapper chain always wins over that fallback;
reference/layout/environment language is not a substitute.

A show-set that changes language only in the rendered tree is not a pre-show
introspection correction. Its parity is unresolved at this boundary and any
phase change requires its own L0 and ADR-0127 approval. Preserve current
fallbacks without promising a completed language-parity result for them.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Migrar somente a emissão Tag::Start(loc, info) para Tag::Start(loc, Box::new(info)) nos pontos já existentes. Preservar construção e valores de info, população do índice antes do move, loc, End/hash, ordem de walk e efeitos/contadores. O novo allocation é uma por Start; não clonar info, mover a população, alterar deduplicação, reserva, snapshot ou recursão. A forma pública pertence a 00_nucleo/prompts/core/entities/tag.md; este owner mantém exclusivamente seu comportamento existente.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano, manifesto SHA-256 `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`. O contrato acima passa a vigente; inclui materialização/testes T1–T6, sem release. Registro em `lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`.


## Introspecção I1 — carriers e classificações nominais (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-introspect-carriers-2026-10-17/before.json.gz
preserva UTC/status/diff HEAD --stat, fontes/L0 e 2130 hashes conferidos do
fechamento anterior. contexts-before.json identifica os três V16 vigentes.
before-citation-probe.json conserva argv, rlib/programa com SHA-256, source
freeze e saída: nos 12 casos (quatro carriers × três posições), a ordem k2,k1
da árvore sem wrapper vira vazia; citações conhecidas são perdidas. A coleta
bibliography.rs:21–76 e a conversão em :79–220 não atravessam Source/ShowRecipe/
Located/Realized. Estes são carriers internos sem syntax vanilla; a prova é
metamórfica de transparência já exigida, não comparação de representação Rust.

labelled.rs:45–80 devolve ausência para Source, embora o walk encontre o alvo
interno. Hipótese: sua projeção de compatibilidade perde o texto do mesmo
Heading/Equation/Figure no mesmo Location; o teste RED deve confirmar antes de
alterar esse helper. Seu L0 proíbe desmontar Styled arbitrário; Source não é
Styled, e a decisão anterior é preservada. populate.rs:24–44 satura futuros
ElementPayload em None. O domínio atual tem 19 variantes; as condições bool
ativas devem continuar exigidas. before-probe-result.json registra 290 casos
simbólicos e zero E0004 ao acrescentar variantes fictícias aos dois enums:
proteção estrutural ausente, sem alegar falha numérica atual nem ganho de
leitura por IA. Payloads opacos e guard booleano não provam semântica de dados.

### Responsabilidade exclusiva

Em compute_labelled, acrescentar somente Source como delegação recursiva por empréstimo ao mesmo helper, com Introspector/Location/lang idênticos. Preservar Heading, Equation block, Figure e todo walk_label. Substituir o fallback final pelo complemento nominal de 117 variantes, incluindo Equation quando o guard block recusa. Styled permanece sem unwrap, mesmo sob Source. Acrescentar um teste local com counters reais já preparados no Location fornecido, comparando os três alvos diretos a Source e controles negativos. A projeção de compatibilidade não substitui o caminho numérico autoritativo nem corrige sua política de suplemento.

### Gate e aceitação

Fluxo interno/paridade ADR-0127: sem contrato público, default, fase, I/O,
registro dinâmico ou assinatura nova. L0 precede testes e código. RED funcional
antes da implementação; não usar falhas de montagem de sonda como RED. GREEN
exige as 12 ordens de citação iguais à base, visual de Realized sem contribuições
do original, desconhecidos preservados e fast path sem chaves sem reconstrução.
Label Source deve conservar a projeção direta e o Location/lang, enquanto
Styled e outros alvos continuam com os limites do contrato vigente.

O mutante descartável deve passar de zero para dois E0004 selecionados. Os
290 controles simbólicos mantêm a classificação, salvo os dois do helper de
label que agora reconhece Source. As 17 alternativas negativas de payload
incluem flags recusadas (Equation separa block false de block true/numbering
false); Source não é neutralizado como alvo desconhecido. Notas V19 permanecem
visíveis e justificadas por owner; sem V20 novo ou supressão.

Esperado no SARIF: remover só dois V16 (labelled/populate), acrescentar dois
V19 nominais e preservar o V16 de bibliography::collect_bib_keys até I2, com
os demais diagnósticos intactos por fonte/regra/severidade/mensagem. Não tratar
variantes estáticas ainda não percorridas como terminais legítimos. Replay
permite apenas os reparos prescritos e testes. Grafo V15/V26 antes do resselo
restrito aos três pares; build, testes focais/workspace, Clippy all-targets
-D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem
estrita/reversa, source freeze e diff --check. Pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Sem commit/push/merge/release. O objetivo
global e as frentes anteriores continuam ativos; esta etapa não certifica
bibliografia ou referências universais.
