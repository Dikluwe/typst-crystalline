# Prompt L0 — infra/export/stream/shape — pintura e primitivas geométricas
Hash do Código: 9042e78a

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream/shape.rs`
**Criado em**: 2026-09-26
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0129

## Medição anterior à decisão

Working tree não commitada sobre HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`:
`03_infra/src/export/stream.rs:359-970` contém texto e glifos;
`:971-1070,1830-1990` contém pintura e geometria;
`:1104-1182` contém montagem de página; `:1370-1829,1991-2329`
contém dispatch, marcação e desenho recursivo. O diagnóstico de atomização
registra horário e estado da árvore dessa observação antes da extração.

## Decisão e contrato proprietário

Extração interna estática para esta unidade, preservando a interface do hub.

- Mantém juntas as primitivas de path, RoundedRect, fill, stroke e line cap.
  Não subdividir shape primitives em owners adicionais nesta execução.
- Consome ShapeKind<Pt> e Corners<Pt> já resolvidos em L1. RoundedRect limita
  raios pela geometria disponível sem resolver Length ou apagar contexto relativo.
- Conserva PathItem, regras de preenchimento, curvas, retângulos e elipses.
- LineCap Butt omite 0 J por compatibilidade; Round emite 1 J, Square 2 J.
  Cor None conserva string vazia no prefixo da linha.
- Processo conserva rg/RG; spot usa Separation por identidade completa do
  colorant e tint quantizado a u8. O nome do recurso é determinístico.
- Paint Gradient procura Pattern pelo DedupKey e bbox: gradientes spot usam
  bbox do shape como no walker de recursos; processo conserva a chave vigente.
  Tiling e recurso ausente conservam os fallbacks existentes.
- Não cria recursos PDF nem escolhe política de subsetting ou compressão.

## Verificação

Testes P1133 exercitam os quatro cantos; P1431 verifica line cap nos dois
caminhos; P1483 verifica spot, tint e Pattern; snapshots P307b verificam a
extração. O teste local RoundedRect é movido com a primitiva.
