# Integração dos módulos de teste e Clippy

A correção posterior da marcação de obsolescência de Text está em
[Carrier de pré-shaping e marcação de obsolescência](pre-shaping-ci-2026-10-13.md).
Os números abaixo conservam o estado desta rodada de integração de testes.

Foram retirados atributos `cfg(test)` duplicados nas declarações de módulos
de teste em oito arquivos pais. Cada arquivo de teste conserva seu atributo
interno `#![cfg(test)]`, que exclui o módulo inteiro do build de produção.
Os 42 arquivos de teste, seus L0, corpos, expectativas, nomes e atributos
individuais permanecem idênticos à captura inicial desta rodada.

## Proveniência e medição

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**, preservando alterações preexistentes. Em
`cfg-test-ci-2026-10-13/`, `before.json.gz` registra UTC, HEAD, status,
`git diff HEAD --stat`, fontes, L0 e os locais de cada duplicação. Os
recibos seguintes preservam o estado intermediário e o estado final;
`evidence.json.gz` conserva logs, inventários, scripts e comparações.

O Clippy em Rust 1.92.0 reproduziu 42 erros `duplicated_attributes` antes
da alteração. Em todos, o atributo interno repetia a condição da declaração
no pai. Os L0 dos testes foram lidos e mantêm obrigações de linguagem,
morfologia e geometria que não dependem da duplicação do atributo.

## Correção final

Os L0 dos oito pais foram atualizados antes do código, mantendo os testes
condicionais através do atributo interno de cada filho. A alteração remove
somente os atributos externos correspondentes, preservando nomes, caminhos,
atributos `path`, visibilidade e corpos de implementação. Não muda API,
default, dependências, comportamento ou fase de pipeline; segue o fluxo
contínuo do ADR-0127. O resselo ficou restrito aos oito pares proprietários.

A comparação `scope-final.json` verifica os 84 arquivos de teste/L0
restaurados byte a byte e, nos oito pais, verifica que a diferença é apenas
a retirada das condições redundantes e o selo do prompt. O inventário de
testes antes/depois verifica os nomes completos, não apenas a contagem.

### Compatibilidade com o linter arquitetural

A tentativa inicial retirou a condição interna. A suíte Rust e o build
passaram, mas o linter arquitetural publicado passou a emitir 36 V2 em
helpers de arquivos que já eram exclusivos de teste. Essa tentativa não
foi aceita como solução final.

A fonte pinada do linter, `03_infra/rs_parser.rs:130-133,906-923`, detecta
`cfg(test)` no arquivo analisado, sem herdar a condição da declaração no
pai; `01_core/rules/test_file.rs` usa esse resultado para V2. Conservar a
condição interna permite ao compilador e ao linter reconhecer a mesma
exclusão de produção. Não foi criada exceção de V2, supressão global nem
mudança no pin do CI.

`refinement-before.json.gz` e `architecture.sarif` conservam a tentativa
rejeitada. `pre-validation-final.json.gz` e `architecture-final.sarif`
identificam o estado final. Os resultados intermediários não substituem
a validação final.

## Validação final

| Verificação | Resultado |
| --- | --- |
| Clippy core, lib e testes, com `-D warnings` | 577 → 535 diagnósticos distintos; continua falhando nas pendências restantes |
| Atributos duplicados | 42 → 0; nenhuma outra contagem por código mudou |
| Lista completa de testes | Mesmos 6706 nomes, sem remoções ou adições |
| Suíte core | 6701 passaram, zero falhas, 5 ignorados preexistentes |
| Build do workspace sem cfg(test) | Passou |
| Gates V1/V5/V15/V26 e reverso | Passaram; 965 consumers, zero erros |
| Formatação e diff check | Passaram |
| Linter arquitetural completo | Zero erros; 381 warnings; 1619 notes |

As fontes permaneceram inalteradas durante a validação final. O inventário
Clippy deduplica código, mensagem e spans primários completos
`(file, byte_start, byte_end)`, sem somar lib e testes. Os números pertencem
ao estado capturado, não ao commit isolado. A suíte inteira do workspace e
a matriz remota não foram repetidas nesta correção de integração de testes.

## Pendências preservadas

Esta correção trata uma causa de falha do CI. Permanecem o restante do
Clippy, os warnings arquiteturais, as lacunas funcionais da auditoria
original e a validação remota da matriz de release. Nenhum teste foi
apagado ou marcado como ignorado para obter aprovação. Não houve commit,
push ou publicação.

Esta rodada segue [as correções de componentes e igualdade de cor](limites-cor-2026-10-13.md).
