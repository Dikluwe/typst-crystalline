# Passo 1312 — cast de caminho de read, separando a dívida CSV

## Medição antes da escolha

P1310 e P1311 estão concluídos e não commitados. A próxima coorte histórica
P1309 é loader-path-cast, read/csv. A leitura da fonte ratificada refutou a
homogeneidade: read recebe PathOrStr; CSV recebe DataSource e aceita Bytes.
Medição fresca em `00_nucleo/diagnosticos/p1312-measurement.json`, SHA-256
`d986f593b9d8ca8358d3bf8fc5303eade65bd47382dec0424c3cd3d9d54e07c1`,
com argv, fontes, UTC, binários e working tree no HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`. Baseline completo:
`00_nucleo/diagnosticos/p1312-baseline.json`, SHA-256
`0597c75b13da999990886b32577b563bab330d8d1be4e5bf85e97ff0f03588c5`.

P1312 resolve somente read: `read(42)` deve dar
`expected path or string, found integer`, com origem em 42, em vez de erro
português com int e span detached. A seleção histórica continua imutável;
registrar a retificação no relatório, sem declarar a coorte inteira fechada.

## Escopo e L0

Owner único `01_core/src/compiler/stdlib/loading.rs`; L0
`00_nucleo/prompts/compiler/stdlib/loading.md`, amendment P1312 anterior ao Rust.
Corrigir tipo longo e origem da primeira ocorrência posicional, inclusive
With/Args, preservando origem detached legítima. Path/Str e todos os outros
validadores continuam como estavam. CSV permanece byte-comportamentalmente
igual; não aproveitar o helper compartilhado para aplicar a mensagem de read.
Symbol preserva rejeição, com delta de formatter explícito; não é paridade.
Não mudar encoding, nomes ausentes, excesso, I/O, registro, entidades ou dispatch.

ADR-0127: correção interna em fluxo contínuo com L0-first e RED→GREEN.
Não é uma nova assinatura pública nem autorização para CSV/Bytes ou outras coortes.

## Execução

Regime A/B da skill tekt-materializacao-segregada: root escreve intenção/L0,
testes locais e implementação; testador em contexto novo constrói corpus sem
ler o candidato; revisor distinto valida as políticas antes do patch e verifica
o resultado depois sem editar solução/oráculo. Filesystem compartilhado:
executado sem atestação de isolamento técnico. Sem selo completo ou score de mutação.

1. Baseline inclui P1310/P1311. Target RAM próprio, preservando binários anteriores.
2. L0 integral lido e atualizado primeiro; ownership/núcleos e resselo prévio.
3. Testador mede bilateralmente alvos/controles nos quatro perfis e congela
   expectativas, inclusive política Symbol, antes do patch. Revisor emite GO.
4. Testes locais no owner, RED real; implementação mínima, GREEN e build.
5. Comparação congelada normal/repeat/reverse, full stdout/stderr/exit e
   diagnóstico causal. Timeout/crash/falta de observável = Unknown bloqueante.
6. Replays P1311/P1310/P1308; separar novos deltas esperados de anteriores e
   controles que permanecem divergentes do vanilla. Não editar oráculos antigos.
7. Build e testes workspace release, fmt/check, crystalline-lint, diff/check,
   hashes e escopo. Veredito independente e relatório substantivo em
   `00_nucleo/diagnosticos/p1312-final-report.md`.

Até duas revisões focais por causa, registrando custo e ganho; sem ganho,
reexaminar escopo/observabilidade antes de repetir o corpus ou alterar política.
Escritas: este passo, owner/L0, diagnósticos p1312-* e temporários dedicados.
Sem stage, commit, push, limpeza histórica ou redação de passo 1313.
