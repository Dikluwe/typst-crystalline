# Saneamento dos testes de layout e das constantes de cor

O Clippy do core continua bloqueante, mas os diagnósticos distintos observados
caíram de **1407 para 706** no mesmo comando. O módulo de testes de layout e o
owner de cobertura de fontes não têm mais diagnósticos nessa execução. O build
do workspace e a suíte completa do core passaram após os reparos.

## Estado medido

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**, preservando alterações anteriores. As medições pertencem a
essa árvore, não ao commit isolado.

`qualidade-ci-2026-10-13/before.json.gz` conserva o conteúdo inicial dos owners
de layout e cobertura, UTC, HEAD, status e `git diff HEAD --stat`.
`color-literals-before.json.gz` contém a captura posterior do owner de cor,
com a mesma proveniência, os candidatos e a prova numérica anterior à edição.
`after.json.gz` fixa o estado final; `evidence.json.gz` conserva os logs,
inventários e scripts. Arquivos avulsos seguem o ignore dos diagnósticos.

Não houve commit, push ou publicação. A rodada anterior e o escopo integral
ainda aberto estão em [Correções de produção e validação do CI](correcoes-producao-2026-10-13.md).

## Medições e decisões

O L0 de `entities/layout_types`, seção P484, define `FrameItem::Text` como
representação permanente antes do shaping em L3. Os testes de layout precisam
observá-la. Foi introduzido `expect(deprecated)` **apenas nesse módulo de
testes**, com a razão vinculada ao contrato. A expectativa também gera aviso
se deixar de ser exercida. O lint permanece ativo nos demais consumers e no
código produtivo. Isso explicita um uso previsto pelo contrato; não representa
a correção de centenas de defeitos funcionais distintos.

Foram removidos imports e helpers privados sem uso. Bindings não consumidos
passaram a nomes com underscore, preservando a avaliação que os produz.
Inicializações, condições, consultas de mapas, documentação e um alias privado
de geometria foram simplificados. Inputs, tolerâncias e observáveis permanecem.
As contagens de `assert!`, `assert_eq!` e `assert_ne!` no arquivo de layout se
mantiveram em 1022, 400 e 21; a comparação de nomes de testes não perdeu nenhum
nome único. Essa inspeção estrutural complementa a execução, não a substitui.

`p1073_ref_table_supplements_en_e_pt` tinha três atributos `#[test]`, gerando
registros duplicados. Passou a ter um, conservando todas as suas asserções. A
redução de dois registros executados não corresponde à remoção de dois casos
distintos.

As fixtures de cobertura de fontes continuam exercitando intervalos invertidos
e o domínio Unicode completo. O intervalo inválido foi escrito como
`Range { start: 8, end: 2 }`; coleções de um intervalo usam `std::iter::once`.
Não foram transformadas em iteradores descendentes nem em coleções de pontos.

No owner de conversão de cores, as 50 substituições de literais sugeridas pelo
Clippy foram compiladas separadamente como `f32`. Todas preservam exatamente os
bits dos valores originais. Só depois dessa medição o L0 foi atualizado e os
literais foram substituídos. Fórmulas, ordem das operações, arredondamento,
assinaturas e implementações transcendentais não mudaram. Trata-se de prova de
preservação mecânica, não de uma nova alegação de paridade de linguagem.

## Validação

| Verificação | Resultado |
| --- | --- |
| `cargo test --offline --locked -p typst-core --lib -- --test-threads=1` | **6682 passaram, zero falhas, 5 ignorados preexistentes** |
| `cargo build --offline --locked` | Passou |
| V1/V5/V15/V26 estritos, linter publicado pinado | Passaram |
| Gate reverso | 965 consumers, zero erros |
| Formatação e `git diff HEAD --check` | Passaram |
| Clippy do core com `-D warnings` | Continua falhando: 365 diagnósticos na lib e 677 no alvo de testes, com sobreposição |

O inventário deduplicado usa código, mensagem e spans primários. A comparação
1407 → 706 está em `clippy-comparison.json`; não soma os totais de lib e testes.
O comando foi `cargo clippy --offline --locked -p typst-core --lib --tests
--message-format=json -- -D warnings`. A medição não cobre conclusivamente as
outras camadas, cujo avanço no gate depende do core.

Os diagnósticos restantes incluem usos de APIs deprecated em outros consumers,
imports não utilizados, atributos de teste redundantes, documentação,
assertions booleanas e métodos matemáticos proibidos pelo `clippy.toml`. No
owner de cor ainda restam `f32::cos`, `f32::sin` e `f32::atan2`; sua substituição
exige uma decisão separada sobre matemática determinística e revalidação dos
observáveis, não uma alteração automática junto com os literais.

O objetivo geral permanece aberto: além do restante do Clippy, continuam as
pendências de headers/footers contextuais, tabelas/grid entre páginas, notas e
linhas longas, fragmentação/callbacks, warnings arquiteturais e validação remota
da matriz. Esta rodada não redefine prontidão de produção em torno dos checks
que já passaram.

Continuação: [matemática determinística de calc](matematica-ci-2026-10-13.md).
A nova medição preserva esta rodada como histórico e registra o reparo seguinte.
