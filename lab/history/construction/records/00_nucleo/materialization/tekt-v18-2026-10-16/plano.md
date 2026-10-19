# Passo planejado — seis ranges V18

Documento tático, não Prompt L0. Não autoriza código. A validação de clippy-enums-publicos-2026-10-16 continua em execução; nenhuma fonte ou L0 será alterado até seu encerramento.

Fontes/owners lidos e hashes/UTC: read-evidence.json. O inventário final da etapa atual será a base quantitativa; conferir seu SARIF e estado antes de iniciar.

1. Medir novamente quatro ranges de cardinalidade em call_spans.rs e dois ranges de bytes no decoder de watch.rs, com HEAD, working tree, hashes e diffstat. Ler os owners atuais e os contratos de origem de spans/rotas antes de editar.
2. Candidata interna para call_spans: casar diretamente args.items.as_slice() por [], [_] e [_, _, ..], preservando has_spread, prioridade de named/bound, call/primeiro/segundo spans e fallthrough. Não trocar a ordem de validação de Attach/Mono/Script nem alterar Binom fora do necessário. Confirmar os casos de zero, um, dois e mais argumentos com/sem spread, named e bound.
3. Candidata interna para percent decoding: substituir a tabela hexadecimal manual por conversão std de byte/char para dígito radix 16, somente após medir que reconhece exatamente os mesmos 22 bytes ASCII (0–9, A–F, a–f). Testar todos os bytes e pares relevantes, UTF-8, truncamento %, NUL, backslash e dot traversal. Preservar 200/301/404 e normalização; não abrir paths de filesystem.
4. Registrar decisão após essas medições. Se a equivalência se confirmar, escrever os dois L0 primeiro e seguir ADR-0127 em fluxo contínuo (sem API/default/fase nova); se houver diferença, rever antes do código. Não renomear módulos para escapar da regra nem silenciar V18.
5. RED→GREEN, resselo restrito, Clippy zero mantido, redução esperada de seis V18 sem avisos novos, suítes de spans/rotas e workspace completo/build/Rustdoc/linhagem sob teto de 12 GiB, swap zero e um job. Gerar relatório da etapa.

## Encerramento da execução

Etapa implementada e validada em 2026-10-16. Relatório: `00_nucleo/diagnosticos/tekt-ranges-2026-10-16/relatorio.md`. Tekt 380 → 374 avisos, V18 zero, Clippy zero; suíte 8.615 aprovados/0 falhas/7 ignorados. A afirmação inicial sobre validação anterior em curso registra o momento em que este plano foi escrito; não é o estado atual. Proveniência e RAM no relatório.
