# L0 — introspect/convergence
Hash do Código: b1ab9a03

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/convergence.rs`.
The body of `compute_tags_hash` at `:29` hashes the complete tag slice with
Rust DefaultHasher. `01_core/src/compiler/introspect/fixpoint.rs:129` compares
successive hashes. This is an internal structural change detector, not an
implementation of the production pipeline's semantic convergence policy.

## Contract

Keep `pub fn compute_tags_hash(tags: &[Tag]) -> u64` pure and stateless.
Hash the ordered slice through its Hash implementation, including payloads,
Locations and End hashes. Equal input in the same implementation yields the
same digest, including empty input. Order is significant.

No I/O, memoization, global cache or new dependency is required.
Do not promise a collision-free comparison, a stable persisted digest across
Rust versions, or a fixed named hashing algorithm behind DefaultHasher.
A changed payload, Location, End hash or ordering is a relevant change input;
representative differing inputs must be tested, not elevated to a mathematical
guarantee that all unequal inputs have unequal hashes.

## Acceptance

Cover repeatability, empty input, equivalent slices and representative changes
in payload, Location, End hash and order. The bounded loop and its errors belong
only to `00_nucleo/prompts/compiler/introspect/fixpoint.md`.
Language convergence and upstream parity need their own observable evidence.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em 00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Migrar somente os construtores Tag::Start das fixtures deste consumer para Box::new do mesmo ElementInfo. Preservar Locations, payloads, labels, End hashes, chamadas, assertions e todos os corpos produtivos. Leituras emprestadas e hashes por valor conservam os contratos existentes; não criar algoritmo de hashing novo, aceitar novo erro ou atualizar esperados para mascarar diferença. O payload público pertence exclusivamente a 00_nucleo/prompts/entities/tag.md.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano, manifesto SHA-256 `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`. O contrato acima passa a vigente; inclui materialização/testes T1–T6, sem release. Registro em `00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16/approval-receipt.json`.
