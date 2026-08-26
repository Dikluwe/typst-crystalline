# Prompt L0 — comando `typst info`
Hash do Código: d2942c4d

**Camada:** L2
**Ficheiro proprietário:** `02_shell/src/info.rs`
**Estado:** INFO-1 implementado; INFO-2 permanece fora do escopo

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shell/info-projection.toml sha256:f7f63d22ab1b490c71f5e93f5fdf22da8e068dc15dfe1f9aa9cdb5fd7f95ab4d

## Medição anterior à decisão

O vanilla oferece saída humana ou serializada por `-f/--format`, com `--pretty`
para JSON (`args.rs:275-289`). O DTO inclui versão/build/plataforma/features,
configuração de fontes/packages e env relevante (`info.rs:18-42`). O cristalino
tem versão/hash compile-time e caminhos de package/fontes distribuídos entre
L2/L3, mas nenhum snapshot público consolidado.

**Classificação:** observável público; valores dependentes da máquina variam,
mas nomes de campos, tipos e origem não são mecânica descartável.

## Contrato inicial

```text
typst info
typst info --format json [--pretty]
```

DTO estável inicial:

```text
version, build.commit, build.platform.{os,arch},
features.{html,bundle}, fonts.{system,font-paths},
packages.{data-path,cache-path}, env
```

1. Saída humana é legível e pode truncar o commit; JSON nunca trunca.
2. `--pretty` sem JSON é erro de argumentos.
3. Secrets e valores de proxy com credenciais não são impressos. `env` contém
   apenas allowlist Typst/XDG e indica presença quando o valor for sensível.
4. L3 resolve paths/defaults; L2 possui DTO/serialização; L4 compõe.
5. O comando é read-only e não força download nem varredura completa de fontes.

## Escopo incompleto explícito

`P1137-C001-INFO-1` cobre human+JSON e campos acima. Outros formatos do vanilla
ficam para `P1137-C001-INFO-2`, depois de medição específica; até lá o help não
os anuncia.

## Testes e aceitação

Schema JSON, pretty/compact, commit completo, paths sob XDG injetado, ausência
de segredos, estabilidade da allowlist e saída humana contendo as mesmas
categorias sem exigir paths iguais entre máquinas.

## Gate

Novo comando e novo contrato serializado: confirmação obrigatória ADR-0127.
