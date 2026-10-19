# L0 — infra/system-world/source_session
Hash do Código: 90fc94f0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:7ec137b6996e0ea442e3cdda52ce723fd32cb67338ab363b7ff2d733458ef8fa

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/source_session.rs`

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

### Contrato ratificado

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


## Default público — proposta sujeita à confirmação ADR-0127

### Medição anterior à decisão

`03_infra/src/world/source_session.rs:24` declara pub fn new() -> Self sem argumentos. O inventário em `00_nucleo/diagnosticos/field-reassign-2026-10-15/clippy-inventory.json` registra new_without_default; fonte/L0/hashes atuais são preservados no pacote clippy-default-2026-10-16. SourceSession::new cria Mutex<Vec<Entry>> vazio por instância, sem parsing ou I/O. Default delega a Self::new(); os limites 8 entradas, 1 MiB total e 512 KiB por entrada, validação e isolamento dos Worlds permanecem inalterados. Isto é contrato Rust de construção, sem nova medição de paridade da linguagem.

### Contrato proposto

Adicionar somente `impl std::default::Default for SourceSession` com `fn default() -> Self` que devolve `Self::new()`. Preservar integralmente new(), sua visibilidade, campos, seus valores, todos os demais métodos e semântica. Não criar outro trait, novo modo/flag, estado compartilhado global, I/O no construtor ou mudança de fase. Nenhum caller existente precisa migrar; novo código pode usar `<SourceSession as Default>::default()` e bounds `T: Default`.

### Gate, testes e aceitação

Nenhuma materialização antes da confirmação humana do manifesto deste pacote (ADR-0127 §2.1/§2.4). Os impls são acréscimos públicos: sua remoção futura quebraria consumidores do novo contrato. Após ratificação, a cláusula só supersede proibições genéricas de nova API para estes três impls expressos; os demais contratos permanecem. Testes de uso por bound Default devem falhar por ausência de implementação antes do código e passar após delegação. Verificar valores/isolamento de contextos e sessões novas, preguiça de recursos e conservação dos controles existentes; suíte workspace, Clippy all-targets, build, Rustdoc, formato e linhagem com resselo restrito. Não alterar valores/defaults nem suprimir new_without_default.
