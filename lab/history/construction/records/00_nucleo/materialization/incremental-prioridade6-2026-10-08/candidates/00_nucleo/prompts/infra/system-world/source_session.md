# L0 — infra/system-world/source_session
Hash do Código: PENDENTE

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:7ec137b6996e0ea442e3cdda52ce723fd32cb67338ab363b7ff2d733458ef8fa

## Reuso sintático de watch — lote A

### Medição anterior à decisão

00_nucleo/diagnosticos/incremental-prioridade6-2026-10-08/before.json.gz,
measurement.json.gz, control-measurement.json.gz e count.json registram árvore,
HEAD, UTC, comandos e releases. Source::new reconstrói CST; World novo constrói
Source main/import a cada edição. O import mínimo já absorve recursos no eval_ms;
não atribuir toda a métrica a parsing. O contrato causal de Source/id/caminho vem
dos owners existentes; o reuso é mecânica e precisa preservar observáveis de língua.
Hipótese de benefício refutada por custo, diagnóstico, efeito, stale source,
geometria ou identificação diferente. Não inferir intenção upstream.

### Contrato proposto — gate ADR-0127 pendente

Possuir SourceSession pública opaca com new() sem I/O, campos privados e métodos
internos de comparação/retensão. Guardar apenas Source markup imutável por caminho
canônico físico e FileId corrente, validar texto integral sem Source Eq/hash como
prova. Não atribuir IDs. Manter no máximo 8 entradas LRU, 1 MiB total de texto e
512 KiB/entrada; oversize devolve parsing normal sem retenção. Uso de Mutex por
instância em L3, zero globals/unsafe/disco/clock. Parsing usa Source::new de L1.
Concorrência pode repetir trabalho; nenhum retorno usa Source de texto/id/path
não validados. Não guardar outros produtos ou erros de leitura; dados CST com
errors/warnings seguem reemissão normal pelo eval, sem cache do Sink.
Testes locais devem incluir reuso em Worlds reais, recusa por texto/id/path,
limites/evicção, transientes, erro/recuperação e concorrência. Memória real não é
limitada pelo orçamento de texto: medir RSS. Sem comprovado ganho de produto,
retirar conexão produtiva e conservar evidência negativa.

Aceitação: RED→GREEN, contagem de produção separada de tempos, três rodadas CLI
pareadas contra release P5, startup/RSS e controles de invalidação/identidade/
efeitos/Unicode/diagnóstico/assets/fontes/geométricas. Resselo apenas deste owner,
build/suítes pertinentes e lint zero erros; não fechar incrementalidade completa.
