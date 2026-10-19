# Passo 1362 — corpos de status temporizado do `watch`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Ligar o formatter P1361 aos quatro corpos de status do ciclo `watch`, com
classificação explícita de sucesso limpo, sucesso com warnings e erro, sem
executar um processo observador durante a materialização.

## Contrato observável

L2 expõe representação fechada equivalente a:

```rust
pub enum WatchStatus {
    Compiling,
    Success(Duration),
    PartialSuccess(Duration),
    Error,
}

impl WatchStatus {
    pub fn message(&self) -> String;
}
```

Para duração literal 2.8492 s:

```text
Compiling        -> compiling ...
Success          -> compiled successfully in 2.85 s
PartialSuccess   -> compiled with warnings in 2.85 s
Error            -> compiled with errors
```

L4 emite `Compiling` em stderr antes de cada compilação do ciclo; mede com
`Instant`; depois seleciona Success se exit 0 sem warnings, PartialSuccess se
exit 0 com warnings e Error para qualquer exit não zero. `run_compile` normal
continua silencioso: somente `run_watch` ativa status. O bit de warning vem da
coleção tipada já produzida, nunca de parsing de stderr.

## Scope-out

- timestamp `[HH:MM:SS]`, cor, fullscreen e limpeza de terminal;
- posição exata do status perante diagnósticos;
- conteúdo/ordem dos diagnósticos;
- alteração de polling, dependências, arm/publish/abandon ou eviction;
- execução externa de `watch`/`w`;
- telemetria de performance ou garantia sobre duração medida.

## Owners

- `prompts/shell/duration.md` → `02_shell/src/duration.rs`;
- `prompts/shell/tests/p1361-duration.md` →
  `02_shell/tests/p1361_duration.rs`;
- `prompts/wiring.md` → `04_wiring/src/main.rs`.

## Ordem e regime

Protocolo completo: L0/preflight/passo → contrato → oráculos/mutantes → gate
1.0/selo → teste RED e implementação independentes → GREEN/gates → veredito.
Como o workspace é partilhado, declarar “executado sem atestação de
isolamento”. O manifesto deve congelar o conteúdo normativo dos Prompts por
digest canônico que exclui somente a linha mecânica `Hash do Código`; o valor
bruto antes/depois permanece registrado. Assim o resselo mecânico exigido pelo
linter não pode alterar silenciosamente a obrigação.

## Contenção de memória

- processo `watch`/`w`: proibido;
- nenhum teste `04_wiring/tests/cli.rs`;
- `CARGO_BUILD_JOBS=1`, teste exato, timeout <=120 s;
- stdout/stderr em ficheiros, máximo 4 MiB cada e 16 MiB total;
- sem suíte ampla/corpus; zero testes, timeout, OOM ou Unknown inesperado
  bloqueiam PASS.

## Gate ADR-0127

Adicionar status ao comportamento padrão de `watch` é mudança por defeito. A
instrução explícita “Escreva e implemente o próximo passo” autoriza este recorte
limitado; os eixos declarados em scope-out continuam sem autorização implícita.
