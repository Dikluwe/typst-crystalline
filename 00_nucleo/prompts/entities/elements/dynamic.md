# Prompt L0 — ponte object-safe `DynElement`
Hash do Código: c54d909a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

**Camada:** L1
**Ficheiro alvo:** `01_core/src/entities/elements/dynamic.rs`
**ADRs:** ADR-0026, ADR-0029, ADR-0105, ADR-0106, ADR-0108, ADR-0129.

## Medição anterior à decisão

`Element` não é object-safe por seus supertraits e métodos genéricos. O
consumer vigente define `DynElement` e um blanket impl para todo
`Element + Send + Sync + 'static`; o utilizador não implementa a ponte à mão.

## Contrato específico

- Expor bridges object-safe para texto, vazio, map, campos, kind, payload e
  resolução de settables.
- `dyn_kind` delega ao nome estável do `Element`; `dyn_eq` faz downcast ao tipo
  concreto e igualdade estrutural; tipos distintos são diferentes.
- `Arc<dyn DynElement>` preserva clone O(1); hashing continua pelo Debug do
  conteúdo, sem `dyn_hash`.
- O caminho nativo permanece estático; somente `Content::Dynamic` usa a ponte.

## Aceitação

Provar object-safety, blanket para nativo, igualdade, maps e dispatch do hub.
