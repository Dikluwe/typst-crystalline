# L0 — infra/pipeline/runtime
Hash do Código: 6bb2591b

Camada L3. Consumer produtivo único: 03_infra/src/pipeline/runtime.rs.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Compor privadamente EvalContext/Engine/Sink com World, Source.id, Route, métricas,
StyleChain inicial e stacks existentes. Contexto recebe target/features explícitos.
Helper genérico with_introspection_runtime executa somente a operação L1 fornecida,
retornando resultado e diagnostics do Sink; sem decisão semântica de elementos.
Helper de Heading chama exclusivamente a fachada pública ratificada L1.
Não antecipar State/Counter/equações nem fazer layout, I/O adicional, cache global
ou selecionar modo. Lifetime local por operação; Engine/World permanecem externos
ao L1 puro. Assinaturas deste módulo são restritas a pipeline e descendentes.

H1–H8, prova bilateral PDF/HTML e gates completos sob 12GiB/High8GiB/swap0/jobs1.
