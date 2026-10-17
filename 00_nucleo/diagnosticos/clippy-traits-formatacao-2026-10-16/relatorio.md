# Traits de domínio e extremo de Duration — etapa concluída

Clippy **13 → 9 avisos**, sem novos avisos: quatro nomes de domínio receberam exceções nominais prescritas nos respectivos L0; o aviso de Styles::from_iter permanece visível para correção por FromIterator. Além da manutenção, foi corrigido um defeito real de formatação de Duration no limite i128::MIN. Cinco consumers e seus L0 foram atualizados antes do código e resselados de forma restrita. [stage.diff](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/stage.diff) · [classificacao-13.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/classificacao-13.json).

## Decisões de nomes

Lexer.next retorna token/kind e produz End explicitamente; o parser controla modo/cursor. Locator.next aloca Location e mantém overflow explícito, sem fim normal Option::None. Decimal.from_str e Version.from_str são parsers Option; adicionar FromStr exige Result e um contrato de erro diferente. Preservados todos os corpos/assinaturas, rejeições, IDs, checkpoints, versão e defaults. Cada método recebeu somente expect(should_implement_trait) com razão nominal no código e no L0. Nenhum limiar, módulo/impl/crate ou outro lint foi silenciado; expectativas conferidas como erro. Não foi alegado ganho de leitura por IA ou paridade por esta comparação de interfaces Rust.

## Defeito corrigido

A sonda anterior com o consumer real produziu panic por overflow em debug e **--55s** em release para Duration::from_nanos(i128::MIN). MIN+1, MAX, zero e -1s foram controles. A causa era abs() em i128, incapaz de representar a magnitude positiva de MIN. Agora unsigned_abs() fornece u128 e os divisores positivos das mesmas unidades são convertidos a u128, preservando a decomposição e o único prefixo de sinal.

O resultado exato é **-1969226660422097589487121d13h41m55.884105728s**. A expectativa vem de divisão inteira independente de 2^127, sem f64. Constructor, campos, accessors, Neg, Default, operações aritméticas e repr Typst não foram alterados. Esta correção não encerra a política de overflow fora da formatação. [duration-before-probe.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/duration-before-probe.json) · [magnitude-calculation.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/magnitude-calculation.json) · [duration-after-probe.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/duration-after-probe.json).

## Validação

Antes da correção passaram **1682 testes de entidades e 40 de lexer**; o novo teste de extremos falhou por overflow e o gate Clippy observou os cinco métodos inicialmente sinalizados. Após a mudança, a sonda debug/release produz a string correta sem panic e todos os controles permanecem. [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/baseline-details.json).

Suíte workspace completa: **8602 aprovados, 0 falhas, 7 ignorados**, incluindo os dois novos testes de limites e nanos negativos. Build, Clippy all-targets, gate estrito de expectativas/aridade/dead_code/vec_box e lints já fechados, formato, Rustdoc com warnings como erro, quatro índices HTML, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/validation-details.json) · [strict-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/strict-details.json) · [workspace/workspace-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/workspace/workspace-result.json).

Tekt mantém **380 avisos**, {'V16': 327, 'V17': 28, 'V18': 6, 'V12': 19}, sem erros nem aumentos por regra. A auditoria/cópia proposta dos quatro L0 públicos não foi aplicada aos owners ativos. Os cinco enums grandes permanecem pendentes; tamanhos indicados pelo Clippy não são benchmark de RSS/tempo ou aprovação para alterar campos públicos.

## Proposta pública separada

Display para Decimal/Duration/Version, retirando os três métodos inerentes to_string em favor de ToString padrão; FromIterator<Style> para Styles, conservando Styles::from_iter como adapter. Os quatro L0 completos, delta e testes T1–T5 estão em [review.md](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/clippy-traits-2026-10-16/review.md), com manifesto SHA-256 **d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8**. A confirmação ADR-0127 §2.1/§2.4 foi solicitada por novos contratos públicos e migração de métodos. Até ratificação e materialização, os quatro avisos continuam abertos. Os enums não fazem parte desse pacote. Sem publicação de release.

## RAM e proveniência

Serviços pesados com **12 GiB, swap zero e CARGO_BUILD_JOBS=1**. Pico da validação **12.000 GiB**, incluindo cache; eventos max=338, OOM=0, OOM-kill=0. [validation-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/validation-result.json).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; snapshot 2026-10-16T17:06:39.456782+00:00; validação 2026-10-16T17:12:17.139773+00:00–2026-10-16T17:24:18.582974+00:00 UTC. Fontes/L0 imutáveis durante os comandos. [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/before.json.gz) e [final-state.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/final-state.json) guardam fontes/hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150 e vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, conferido pela suíte. Scripts em workers/; [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/manifest.sha256).

Restam **9 avisos Clippy** e a dívida Tekt. [lista-pendencias.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/lista-pendencias.md). A meta geral continua aberta. Sem commit, push ou release.
