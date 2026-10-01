# L0 — convergência das tentativas de layout
Hash do Código: 5a8ee68a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/convergence.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência antes da decisão

`01_core/src/compiler/layout/mod.rs:1859-2035` prepara cache bibliográfico e callbacks, recria Layouter, injeta páginas conhecidas, compara os mapas e devolve Complete/Pending. O L0 vigente do hub já fixa cinco tentativas e a necessidade de convergir mesmo sem outline. Medição estrutural no commit `320c2149b`; não mede custo nem intenção vanilla.

## Contrato

Possuir a coordenação privada da tentativa: a mesma introspecção tracked, seed do Locator, cache CSL e diagnósticos iniciais em cada candidato; conhecidos de label/figura/tabela/page store transportados na mesma ordem; máximo de cinco tentativas. Aceitar somente mapas estáveis com transcript math e par.line consumidos. Uma tentativa rejeitada descarta seu transcript. Exaustão devolve Pending com requests vazios. A fachada pública mantém assinaturas; o dispatcher exaustivo Content e estado do Layouter permanecem no hub. Descoberta do primeiro style bibliográfico permanece auxiliar local à coordenação, enquanto interpretação CSL fica em `layout/bib_csl.md`.

## Aceitação

Páginas, posições, conteúdo, diagnósticos e requests de callback iguais para convergência, não convergência, cache inválido e seed injetado. Sem alteração de fase ou default. Build, testes focais e lint válidos.
