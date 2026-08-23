# Prompt L0 — opção global `--cert` / `TYPST_CERT`
Hash do Código: PENDENTE

**Camadas:** L2/L3/L4  
**Ficheiros alvo:** `02_shell/src/cli.rs`,
`03_infra/src/package_downloader.rs`, `03_infra/src/world.rs`,
`04_wiring/src/main.rs`  
**Estado:** implementado

## Medição anterior à decisão

O vanilla declara `--cert PATH` global com env `TYPST_CERT`
(`lab/.../args.rs:73-75`) e constrói o downloader com esse certificado
(`download.rs:14-20`). O cristalino usa `ureq::AgentBuilder` apenas com proxy em
`03_infra/src/package_downloader.rs:54-62`; não há caminho de CA configurável.

**Classificação:** comportamento público de rede e segurança. Apenas aceitar a
flag seria opção decorativa e é proibido; uma sonda TLS assinada somente pela
CA fornecida refutaria tal implementação.

## Contrato

```text
typst --cert PATH <COMMAND>
TYPST_CERT=PATH typst <COMMAND>
```

1. Flag vence env; ausência preserva roots TLS normais.
2. O ficheiro é lido somente quando um downloader HTTPS é construído.
3. PEM pode conter uma ou mais CAs; conteúdo vazio, certificado inválido ou
   path ilegível falha claramente antes da request.
4. As CAs fornecidas são acrescentadas às roots normais, não substituem a
   validação TLS nem desativam hostname verification.
5. Aplica-se a todo download controlado pelo cristalino (package e índice).
6. Path/bytes do certificado não aparecem em `Debug`, diagnostics remotos ou
   `info`; `info` pode indicar somente que uma CA customizada está configurada.
7. L2 transporta `Option<PathBuf>`; L3 lê/parseia/configura TLS; L4 injeta.

## Testes e aceitação

Precedência flag/env; path ausente; PEM inválido; servidor TLS local cuja CA
somente passa quando fornecida; hostname inválido continua rejeitado; ausência
mantém comportamento anterior. Nenhum teste depende de rede pública.

## Dependência técnica

Antes do código, confirmar que a versão/feature TLS de `ureq` permite adicionar
roots PEM. Se não permitir sem desativar validação, atualizar a dependência por
decisão explícita; nunca contornar com cliente inseguro.

## Gate

Nova opção global e política TLS: confirmação obrigatória ADR-0127.
