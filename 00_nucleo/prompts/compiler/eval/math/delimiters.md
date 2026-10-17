# L0 — compiler/eval/math/delimiters
Hash do Código: 5b1abb80

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/delimiters.rs`. No HEAD `e754ab364a221dbe344852178e84bd42b02c463e` e árvore limpa, `01_core/src/compiler/eval/math.rs:1555-1671` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contrato

Interpretar delimitadores e opções de sintaxe vetorial com os erros existentes. Preservar assinaturas, semântica da linguagem e diagnósticos de `00_nucleo/prompts/compiler/eval/math.md`; não alterar fase do pipeline, defaults ou compatibilidade. A extração é interna a L1, sem I/O nem estado global mutável.

## Aceitação

A suíte L1 e os testes de math passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

## Exaustividade nominal Value — V16, 2026-10-16

### Medição anterior à decisão

M1 em 01_core/src/compiler/eval/math/delimiters.rs:19, M2 em 01_core/src/compiler/eval/math/delimiters.rs:28. O inventário desta árvore não commitada (HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`) e os L0 vigentes foram lidos antes da prescrição. before.json.gz em `00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16/` conserva UTC, status, git diff HEAD --stat e hashes integrais. Value possui 39 variantes; os braços fallback medidos são internos e hoje uniformes. São avisos estruturais de evolução Rust, não prova de falha funcional ou de melhor leitura por IA. Não inferir intenção upstream.

### Decisão do owner e aceitação

M1 parse_delim_val aceita Str/Symbol, None e Array. O complemento nominal das outras 35 variantes atuais retorna None no mesmo braço; M2 parse_delim_char aceita Str/Symbol e None, e o complemento das outras 36 variantes retorna None. Preservar aliases de strings, pares Unicode, primeiro caractere de Symbol/string e todos os defaults de arrays (primeiro inválido/vazio vira NUL; segundo ausente/inválido repete o primeiro; elementos excedentes ignorados).

Substituir somente o wildcard selecionado pelo complemento nominal das variantes atuais; compartilhar o corpo existente como grupo de alternativas equivalentes. V19 informativo é legítimo aqui por consequência/efeito idênticos e deve permanecer visível. Não duplicar corpos, criar helper de fachada, usar binding catchall, isenção ou mudança na configuração para esconder V16. Payloads não utilizados podem manter padrões internos `_` dentro de uma variante nominal. Assinaturas públicas, carriers, fase, defaults e diagnósticos não mudam.

Antes da implementação, escrever controles co-localizados para os casos aceitos/rejeitados e a morfologia/presença indicadas. Os controles de preservação devem passar antes/depois; o RED é V16 no fonte atual e o teste de evolução em cópia do workspace real: adicionar uma variante sentinela em Value deve ser barrado em cada decisão selecionada (hoje o fallback a absorve). GREEN exige E0004 nesses mesmos pontos, zero V16 selecionado, somente novos V19 equivalentes e nenhum outro diagnóstico novo. Os controles não afirmam cobertura de execução das 39 variantes nem paridade vanilla global.

Resselo direto/reverso apenas deste owner e consumer; revalidar build, workspace completo, Clippy all-targets com warnings/expectativas não cumpridas como erro, fmt, Rustdoc estrito, V1/V5/V15/V26 e linhagem reversa. Executar comandos pesados sequencialmente sob 12 GiB, swap zero, Cargo um job. Guardar comandos, fontes/hashes e UTC; preservar código alheio à etapa.
