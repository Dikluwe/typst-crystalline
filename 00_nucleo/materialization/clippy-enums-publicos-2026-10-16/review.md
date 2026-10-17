# Proposta — Tag boxed e estilos de show compartilhados

Modificar somente dois payloads públicos: `Tag::Start(Location, Box<ElementInfo>)` e `Transformation::Style(Arc<Styles>)`. Oito L0 completos e as migrações de produtores/fixtures estão em proposed/; base.json.gz e proposal.diff conservam a base e o delta. Nenhum código ou L0 ativo foi alterado.

## Evidência e custo

A medição de 4.096 pares Tag reduz bytes solicitados de 3.407.872 para 2.031.616, adicionando 4.096 alocações. Box mantém clone profundo; as travessias produtivas lidas usam o vetor temporário por empréstimo. O tipo passa de 416 para 48 bytes no ABI local.

Para show, Arc foi escolhido após comparar inline e Box nas cópias efetivas do algoritmo. ShowRule cai de 376 para 168 bytes. No controle de 128 registros 100% show-set simples, bytes solicitados acumulados caem de 12.276.960 para 5.530.624; Box exigiu 8.640 alocações contra 512 Arc. Com snapshots retidos, pico solicitado cai de 3.203.856 para 1.477.808 bytes. Clone de estilos via Arc não realoca o payload/custom.

**Contraexemplos preservados:** construir uma única coleção 100% show-set simples fica maior (1.540.096 → 2.097.152 bytes em 4.096 regras); Arc preserva capacidade sobrante de custom e adiciona contagem atômica/indireção. No controle rico de 128 registros sem snapshots antigos, o final vivo subiu de 78.064 para 156.432 bytes. Não é promessa de menor RSS/tempo em todo workload. A sonda espelha carriers/registro, sem compilar/renderizar documentos. Fontes/callers, SHA-256, comandos, resultados, variantes de 0/25/50/100% e controles ricos estão em [diagnóstico](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16/measurements.json).

## Compatibilidade e testes

Construtores Rust externos envolvem info em Box::new e Styles em Arc::new. Leitura emprestada mantém deref; quem muta estilos compartilhados migra para Arc::make_mut. Campos de ElementInfo/Styles, variantes restantes, linguagem, ordem, defaults, fases e deps não mudam. A aprovação inclui implementar os oito owners, testes T1–T6 e validar todos os targets com 12 GiB, sem swap e um job. Não inclui novos helpers públicos, supressões dos dois avisos ou release. Tekt permanece como dívida separada.

**Gate obrigatório:** a [ADR-0127 §2.1/§2.4](/repos/Antigravity/typst-crystalline/00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md) exige confirmar antes de alterar contrato público/compatibilidade: “adicionar, remover ou alterar campo público” e “qualquer mudança que, se revertida depois, quebraria código já escrito contra ela”. A proposta está pronta para revisão; não aplicar antes da resposta humana.
