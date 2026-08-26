# Prompt L0 — constructor e helpers de `path`
Hash do Código: 6d935eb3

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/foundations/path.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

P1141 mediu `path` como tipo chamável contextual: string resolve uma vez no
FileId do caller; `path(path)` preserva identidade; repr usa vpath absoluta.
P1154 confirmou a separação entre domínio, World e estes helpers de eval.

## Contrato

`resolve_path_value` preserva Path e resolve Str por World. `read_path_value`
materializa após obter RootedPath. `native_path` rejeita named, exige um
posicional, preserva Path e resolve Str em `current_file`; outro tipo produz
`expected path or string`. Sem path físico ou I/O direto em L1.

## Aceitação

Normalização, identidade, cast, World e retenção cross-file seguem P1141–P1154.
Mudança futura da superfície pública para no ADR-0127.
