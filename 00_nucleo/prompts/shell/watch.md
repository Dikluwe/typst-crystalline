# Prompt L0 — observação de filesystem do `typst watch`
Hash do Código: 49fbb038

**Camada:** L3
**Ficheiro alvo exclusivo:** `03_infra/src/watch.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Propriedade e fronteira

Este Prompt possui exclusivamente `03_infra/src/watch.rs`. Parsing e intent do
comando pertencem ao L0 de L2; inventário de dependências pertence ao owner de
`SystemWorld`; composição e ordem do ciclo pertencem a `wiring.md`; observação
externa do binário pertence a `wiring/tests/cli.md`.

L3 concentra todo o acesso ao filesystem, relógio de polling, fingerprint e
finalização do staging. Nenhum desses mecanismos atravessa para L1 ou L4.

## Medição anterior à decisão

No baseline congelado de P1297, HEAD
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`,
`03_infra/src/watch.rs:42-45` captura fingerprints imediatamente e
`wait_for_change_since` em `:49-59` não recaptura. Em
`04_wiring/src/main.rs:95-101`, a compilação termina, as dependências são
normalizadas e o snapshot é capturado antes dos dois ramos; `:114-115`
descarta staging no erro e `:119-122` espera com o snapshot capturado. A ordem
produtiva desejada já existe, mas L4 ainda pode expressá-la como chamadas
públicas independentes.

A campanha P1296 mediu que observar posteriormente a remoção não discrimina a
ordem: MO1 pode remover e capturar antes do próximo poll de 50 ms. A remoção
vista pelo pai não prova que o snapshot a precedeu. A tentativa FIFO seguinte
expirou no primeiro controle durante recompilação por asset após 20 s.

Classificação ADR-0107/0108: snapshot, rename, remoção, polling e ordem de
chamadas são mecânica; aqui essa mecânica é o observável causal do processo,
porque uma recuperação anterior à captura pode tornar-se o baseline e não
provocar nova compilação. Inferência: tornar finalização uma operação que
consome uma capacidade já armada impede L4 de publicar ou abandonar antes da
captura. Refutador: a capacidade guardar paths para captura lazy, algum helper
cru de finalização continuar chamável por L4, a inversão mínima sobreviver ao
teste de integração, ocorrer recaptura na espera ou um controle positivo regredir.

## Decisão P1297/R1 — capacidade de ciclo já armado

L3 expõe contrato público equivalente a:

```rust
pub struct WatchSnapshot { /* campos privados */ }
pub struct ArmedWatch { /* contém somente um WatchSnapshot já capturado */ }

pub fn snapshot(paths: &[PathBuf]) -> WatchSnapshot;
pub fn arm(paths: &[PathBuf]) -> ArmedWatch;

impl ArmedWatch {
    pub fn publish(
        self,
        staging: &Path,
        destination: &Path,
    ) -> io::Result<WatchSnapshot>;

    pub fn abandon(self, staging: &Path) -> WatchSnapshot;
}

pub fn wait_for_change_since(snapshot: WatchSnapshot, interval: Duration);
pub fn wait_for_change(paths: &[PathBuf], interval: Duration);
```

### Armamento e identidade do baseline

1. `arm(paths)` captura imediatamente e exatamente uma vez as fingerprints de
   todos os paths e constrói o `WatchSnapshot` completo antes de retornar.
2. `ArmedWatch` armazena somente esse `WatchSnapshot` já materializado. Não
   armazena paths crus, closures ou qualquer representação que permita captura
   lazy durante `publish` ou `abandon`.
3. `WatchSnapshot` permanece opaco fora de L3: paths e fingerprints são campos
   privados, sem getters ou mutação externa do baseline.
4. Ausência de path é estado legítimo, permitindo detectar criação posterior;
   alteração, criação, remoção e conteúdo diferente com mesmo tamanho
   continuam observáveis.

### Finalização consumidora

1. `publish(self, staging, destination)` consome a capacidade e move o staging
   concluído ao destino por uma única operação de rename atômico. Em sucesso,
   devolve por movimento exatamente o `WatchSnapshot` contido na capacidade.
2. Se o rename falhar, `publish` tenta limpar o staging em best-effort e
   devolve o erro original do rename. Falha de cleanup nunca substitui, mascara
   ou converte esse erro.
3. `abandon(self, staging)` consome a capacidade, descarta o staging em
   best-effort, nunca toca no destino válido e devolve por movimento exatamente
   o `WatchSnapshot` contido na capacidade.
4. Helpers crus de rename/commit e remoção/discard são detalhes privados do
   módulo. Não são API pública e não podem ser chamados por L4 para contornar
   `ArmedWatch`.

### Compatibilidade das APIs de snapshot e espera

`WatchSnapshot`, `snapshot` e `wait_for_change_since` permanecem públicos para
compatibilidade com o contrato P1295 congelado. Essa preservação é compatível
com R1 porque `snapshot` sozinho não pode publicar nem descartar: toda
finalização chamável por L4 exige consumir `ArmedWatch`.

`wait_for_change_since` continua consumindo o snapshot recebido, compara apenas
fingerprints atuais por polling e nunca recaptura, substitui ou reinterpreta o
baseline. `wait_for_change(paths, interval)` também permanece como API de
compatibilidade e equivale semanticamente a uma captura imediata seguida da
espera; não participa da finalização produtiva do ciclo L4.

## Aceitação própria do owner L3

- `arm` congela `Some(fingerprint)` antes de `abandon` remover um path que é ao
  mesmo tempo observado e staging; a espera com o snapshot devolvido detecta
  `Some -> None`;
- o mesmo princípio vale para o desaparecimento do staging após `publish`;
- `publish` preserva rename atômico, cleanup best-effort e o erro original;
- `abandon` preserva o destino e não propaga falha de cleanup;
- ambas as finalizações devolvem o mesmo baseline capturado por `arm`;
- `wait_for_change_since` não recaptura, e as APIs P1295 preservadas continuam
  funcionais;
- timeout serve somente como limite de falha do runner; duração, sleep,
  repetição, carga ou chance de agendamento não são sinal de prontidão.

Aceitação é a transição de filesystem já congelada e observável. Igualdade
estrutural Rust, algoritmo de hash e número de polls permanecem mecânica.

## Contratos de integração

As propriedades de snapshot pertencem a
`infra/tests/p1295_watch_contract.md`; o consumo de `ArmedWatch` pertence a
`infra/tests/p1297_watch_capability_contract.md`. Cada prompt possui
exclusivamente seu integration test e não altera o ownership deste módulo.
