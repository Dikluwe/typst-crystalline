# Prompt L0 — elemento `strong`
Hash do Código: 891600f6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

**Camada:** L1
**Ficheiro alvo:** `01_core/src/entities/elements/strong.rs`
**ADRs:** ADR-0026, ADR-0105, ADR-0107, ADR-0109, ADR-0129.

## Medição anterior à decisão

O consumer representa `*bold*` como variante semântica própria com um único
`body`. É transparente para plain text/vazio, recursa em maps e expõe somente o
campo `body`. O bold de render pertence ao layout.

## Contrato

- `StrongElem::new(body)` preserva o conteúdo.
- `plain_text` e `is_empty` delegam ao body.
- `map_content`/`map_text` reconstroem `Content::strong`.
- `get_field("body")` devolve o Content; demais campos retornam `None`.

## Aceitação

Preservar distinção morfológica de strong, transparência textual e recursão.

## P1339 — payload da ocorrência própria

### Medição anterior à decisão

No antecedente HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`,
`entities/elements/strong.rs:38-69` implementa Element sem override de
to_payload; o default de `entities/elements/mod.rs:173` devolve None.
Vanilla ratificado `a51e02804`, `model/strong.rs:21`, declara Locatable e
Tagged. A sonda congelada `diagnosticos/p1339-where-integration-probe-runs.json`
registra query/body/counter de strong e a proveniência integral.

### Decisão

Implementar o método já existente `Element::to_payload` com
`Some(ElementPayload::NativeElement)`, autorizado em
`diagnosticos/p1339-where-payload-approval.json`. A unidade representa este
StrongElem, mesmo com body vazio; não o estilo dos descendentes. Não mudar
body, kind, campos, assinatura/trait, mapas ou métodos de render. A identidade
da função e o snapshot da ocorrência permanecem nos respectivos owners de
compiler, não nesta entidade. Preservar todos os métodos anteriores.

Aceitação: payload presente no Strong próprio; transformação de filhos
preserva a ocorrência sem criar wrappers adicionais; plain_text e leitura
de body permanecem iguais. Os testes linguísticos de query/counter são
integração obrigatória e não são substituídos pela construção unit.
