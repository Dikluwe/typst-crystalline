# Prompt L0 — contrato externo da capacidade armada de `watch`
Hash do Código: bc64e243

**Camada:** L3 — teste de integração
**Ficheiro alvo exclusivo:** `03_infra/tests/p1297_watch_capability_contract.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Propriedade

Este prompt possui exclusivamente o integration test acima. O produto pertence
a `shell/watch.md`; composição e ciclo pertencem a `wiring.md`. O consumer
verifica a fronteira pública e transições de filesystem sem fixar representação.

## Medição anterior à decisão

Observar a remoção do staging depois da finalização não prova que a captura
precedeu `publish` ou `abandon`. Usar o staging como path observado distingue
as ordens: captura em `arm` conserva o estado presente antes da transição;
captura lazy posterior veria ausência nos dois instantes.

## Contrato funcional

- `abandon` consome capacidade armada, remove staging e devolve o snapshot
  anterior à transição presente → ausente;
- `publish` consome a capacidade, renomeia staging para destino, preserva os
  bytes publicados e devolve o snapshot anterior;
- abandonar nunca altera o último destino válido;
- falha de rename conserva `ErrorKind` e `raw_os_error` originais e tenta
  cleanup best effort;
- falha do cleanup nunca mascara o erro original.

As transições terminam antes da thread de espera. Timeout limita falha; não é
evidência de prontidão. Não usar sleep, retry, carga ou estímulo corretivo.

## Assinaturas e superfície

```rust
arm: fn(&[PathBuf]) -> ArmedWatch
ArmedWatch::publish:
    fn(ArmedWatch, &Path, &Path) -> io::Result<WatchSnapshot>
ArmedWatch::abandon:
    fn(ArmedWatch, &Path) -> WatchSnapshot
snapshot: fn(&[PathBuf]) -> WatchSnapshot
wait_for_change_since: fn(WatchSnapshot, Duration)
wait_for_change: fn(&[PathBuf], Duration)
```

Rustdoc JSON, com a toolchain pinada no consumer, deve ainda provar:

- `ArmedWatch` público, com estado privado e nenhum campo público;
- somente `publish` e `abandon` como métodos inherent públicos;
- `arm` como única função livre pública que referencia `ArmedWatch`;
- ausência pública de `commit_output` e `discard_output`.

Representação, campos privados, fingerprint e layout permanecem livres.

## Aceitação

Transições, controles de erro, assinaturas e superfície passam. Este contrato
não substitui o contrato do snapshot nem a suíte funcional da CLI.

## Emenda P1485 — superfície armada para bundle de diretório

### Medição anterior à decisão

Com P1485 materializado na working tree não commitada sobre
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`,
`03_infra/src/watch.rs:81-115` expõe também
`publish_directory` e `abandon_directory`, aprovados pelo
contrato de `00_nucleo/prompts/shell/watch.md`. A execução de
`rustdoc_json_pinado_fecha_superficie_publica_da_capacidade`
falhou: lista concreta `abandon, abandon_directory, publish,
publish_directory` contra lista histórica `abandon, publish`.
O teste mede a superfície pública real, não uma mudança implícita
na política de segurança.

### Decisão

A allowlist Rustdoc para `ArmedWatch` passa a conter exatamente
`abandon`, `abandon_directory`, `publish`,
`publish_directory`. As duas operações novas consomem a mesma
capacidade; não autorizam outra função livre que a referencie nem
campos públicos. A restrição histórica acima continua válida
para publicação de arquivo, e esta emenda a estende apenas para
diretório bundle. O gate de API pública foi confirmado pelo dono
para P1485; esta correção de teste não introduz API adicional.
