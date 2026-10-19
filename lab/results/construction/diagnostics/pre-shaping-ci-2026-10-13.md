# Carrier de pré-shaping e marcação de obsolescência

`FrameItem::Text` deixou de ser marcado como deprecated. A documentação agora
descreve seu papel permanente: texto posicionado produzido por L1, convertido
em `TextShaped` pelo shaper L3 quando há fonte disponível, e preservado nos
fallbacks previstos. Nenhum campo, variante, assinatura ou algoritmo mudou.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. O diretório `pre-shaping-ci-2026-10-13/` conserva UTC,
HEAD, status, `git diff HEAD --stat`, fontes, L0, ADR e hashes em
`before.json.gz`, `pre-validation.json.gz` e `after.json.gz`. Os números
descrevem essas árvores, não o commit isolado. Logs, inventários e receitas
ficam em `evidence.json.gz`.

O inventário anterior do Clippy contém 89 diagnósticos deprecated no core:
todos se referem a `FrameItem::Text` ou aos seus campos. A anotação está em
`entities/layout_types.rs:397`; `compiler/layout/text.rs:37` é um emissor L1.
O L0 de layout_types e a ADR-0120, ambos na evolução P484, já declaram Text
permanente como carrier anterior ao shaping. A marcação P483 continuava
pedindo sua substituição mesmo nesses usos obrigatórios.

O owner `infra/shaper/traversal` manda preservar Text quando não há fonte,
a resolução falha ou o texto é whitespace. A transformação em glyphs pertence
à infraestrutura. Migrar os emissores de L1 para glyphs para eliminar o aviso
mudaria a arquitetura vigente; essa migração não faz parte desta correção.

## Correção e refutação

O L0 proprietário foi atualizado antes do código. A ADR-0120 recebeu uma
retificação do metadado histórico, preservando sua decisão de pipeline.
A correção remove a anotação deprecated e três allow(deprecated) locais
que a compensavam. A seleção de fontes, a fase de shaping e os fallbacks
permanecem iguais. A regra Clippy continua bloqueante.

`compile_probe.rs` usa a API pública para construir e ler Text sob
`#![deny(deprecated)]`, sem supressões. Antes da correção, a compilação
falhou com cinco erros de obsolescência da variante/campos; o hash da
rlib usada acompanha a captura inicial. O probe não passa por adaptação
de expectativas nem por mudança de representação.

`scope-verification.json` compara o código antes/depois descontando apenas
comentários, a anotação removida e as três supressões locais. Confirma dados
e corpos de código idênticos, além de preservação dos arquivos de emissores
e shaper inspecionados. Essa é uma prova de escopo da edição, não uma
certificação geral de paridade da linguagem.

### Limpeza da expectativa nos testes de layout

O primeiro Clippy após a correção eliminou os 89 avisos deprecated e revelou
um `unfulfilled_lint_expectations` em `compiler/layout/tests.rs:17`.
O L0 desse teste havia exigido expressamente que a expectativa se tornasse
aviso quando deixasse de ser exercida. O diagnóstico cumpriu essa função.

Esse owner foi atualizado antes de retirar a expectativa e 24 supressões
locais ligadas à mesma marcação. `layout-tests-before.json.gz` conserva o
estado anterior, e `layout-tests-scope.json` comprova que os corpos dos
testes permaneceram iguais, descontando somente esses metadados e o selo.
Nenhum cenário, assertiva, helper ou tolerância foi removido.

## Validação final

| Verificação | Resultado |
| --- | --- |
| Probe público com deny(deprecated) | RED: cinco erros; GREEN: compila e executa com sucesso |
| Clippy core, lib e testes, com `-D warnings` | 535 → 446 diagnósticos distintos; 89 deprecated removidos, nenhuma outra contagem por código mudou |
| Suíte core após a limpeza final | 6701 passaram, zero falhas, 5 ignorados preexistentes |
| Lista completa de testes | Mesmos 6706 nomes |
| Família P48 do shaper | 18 passaram; inclui fallback sem fonte e preservação textual |
| Família P845 do shaper | 5 passaram; inclui dois casos de shaping/largura com DejaVu Sans |
| Build do workspace | Passou |
| V1/V5/V15/V26 e gate reverso | Passaram; 965 consumers, zero erros |
| Formatação e diff check | Passaram |
| Arquitetura completa | Zero erros; 381 warnings; 1619 notes |

Os dois casos P845 com fonte real registram `SKIP` se a fonte não puder ser
carregada. O log com `--nocapture` não contém esse marcador, e os casos
executaram suas verificações de linhas shapeadas e largura. O SHA-256 do
arquivo DejaVu Sans usado está em `before.json.gz`. Os três outros testes
da família medem separação de parágrafos bidi, sem depender dessa fonte.

As fontes ficaram inalteradas durante a validação final. O inventário Clippy
deduplica por código, mensagem e spans primários completos
`(file, byte_start, byte_end)`, sem somar lib e testes. As 446 pendências
referem-se ao core; não certificam o inventário dos demais pacotes.
A suíte inteira do workspace e a matriz remota não foram repetidas nesta
correção de metadados.

## Limites

A mudança corrige um metadado incompatível com o uso permanente do carrier;
não implementa novas capacidades de shaping. As alegações históricas de
cobertura de P483/P484 não são reutilizadas como medições atuais. Testes que
possam retornar por fonte ausente exigem leitura do caminho executado antes
de serem usados como prova de shaping real.

As demais pendências do Clippy, os warnings arquiteturais, os defeitos
funcionais da auditoria original e a matriz remota de release permanecem
abertos. Não houve commit, push ou publicação.

Esta rodada continua [a correção de integração dos testes](cfg-test-ci-2026-10-13.md).

A medição seguinte está em [documentação e Rustdoc](rustdoc-ci-2026-10-13.md).
