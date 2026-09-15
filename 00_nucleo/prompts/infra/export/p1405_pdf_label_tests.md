# Prompt L0 — harness focal PDF `/Lbl`
Hash do Código: 8b0a7168

**Camada:** L3 — teste de integração, sem consumer produtivo
**Ficheiro alvo exclusivo:** `03_infra/tests/p1405_pdf_label.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição antes da decisão

P1405 introduz um envelope semântico fechado para o prefixo de outline. A
presença do papel estrutural `/Lbl` e do marked content com MCID é observável do
PDF tagueado e precisa de um ataque isolado ao exportador.

## Contrato do harness

Um documento mínimo com `SemanticKind::Label` deve produzir nó estrutural
`/Lbl` e abrir `/Lbl << /MCID 0 >> BDC` no stream verboso. O teste não fixa
offsets, bytes completos, IDs de objetos ou estrutura geral de listas.

Este L0 legitima somente o teste indicado e não autoriza comportamento
produtivo adicional.
