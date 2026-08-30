# P1189 — separar `label_kind` e nuclear a semântica de referência inválida

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`
**Dependências:** P1188 GREEN
**Classe ADR-0127:** linhagem/norma interna; enum e mensagens públicas inalterados

## Objetivo e classificação

Resolver:

```text
compiler/layout_references.md
├── 01_core/src/compiler/layout/references.rs
└── 01_core/src/entities/label_kind.rs
```

`references.rs` resolve refs, links, numbering e mensagens (hash `c329926a`);
`label_kind.rs` define `UnreferencableKind` e seu teste (hash `ed5de277`). A
classificação e sua tradução em diagnóstico são obrigação compartilhada entre
o tipo de domínio e o consumer de layout; criar Núcleo, preservando owners
distintos. Baseline condicionado: V15=18, V26=0, V5=409.

## L0 e Núcleo primeiro

1. Criar `_nuclei/references/unreferencable-label.toml` com claims atômicas:
   labels existentes mas não referenciáveis não são “inexistentes”; `Text`,
   `Raw`, `EquationWithoutNumbering` e `Other` permanecem distintos; o layout
   traduz cada classe para o observável aplicável sem inventar nova classe.
2. Manter `compiler/layout_references.md` como owner exclusivo de
   `references.rs`. Conservar resolução, precedência, supplements, links e
   mensagens; substituir a definição/ownership do enum por pin ao Núcleo e
   referência ao owner da entidade.
3. Criar `entities/label_kind.md` como owner exclusivo de `label_kind.rs`,
   especificando o enum público fechado, derives e distinção das quatro
   variantes. Não incluir algoritmo de layout nem textos completos de erro.
   Pinar o mesmo Núcleo.
4. Claims do trait `Introspector` pertencem ao seu owner vigente; não ampliar
   este lote nem repointar terceiros ao Núcleo.

Seguir Núcleo→pins completos→metadata de código→hash integral dos prompts→
headers. Qualquer V26 interrompe o lote antes de reparos adicionais.

## GREEN e testes

Exigir V15 18→17, V26=0, V5 esperado 409→407, ausência focal e exatamente
dois consumidores do Núcleo. Dry-run bloqueado por 17 V15, zero writes.
Sources idênticos fora da linhagem; `references.rs` mantém path e
`label_kind.rs` reponta para `entities/label_kind.md`.

Executar `cargo test -p typst-core entities::label_kind`, filtros focais de
layout/ref e, se vazios, suíte `typst-core`; depois `cargo build`, diff limpo e
índice vazio. Não alterar enum público, mensagens, precedência ou layout.

Fechar em
`00_nucleo/diagnosticos/typst-p1189-saneamento-reference-label-owners.md`, com
hashes, digest/pins do Núcleo, V26 e proveniência.

## Horizonte após P1189

Se os cinco lotes fecharem sem deriva, V15 terá caído de 22 para 17 e V5 de
417 para aproximadamente 407. O próximo passo deve regressar ao inventário
P1182 e escolher outro grupo classe A, sem reutilizar estes Núcleos por mera
semelhança nominal.
