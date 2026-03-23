# PackageSpecDto — DTO de serialização para PackageSpec

**Camada**: L3 — infra
**Criado em**: 2026-03-22
**Arquivos gerados**: `03_infra/src/dto/package_spec_dto.rs`

---

## Contexto

`PackageSpec` em L1 é puro — sem `serde`, sem `unscanny`.
Para desserializar specs de pacotes de JSON/TOML e para o
`FromStr` de strings como `@preview/algo:0.1.0`, é necessário
um DTO em L3 que isola essas dependências do domínio.

Padrão DTO (ADR-0005):
- L3 desserializa para `PackageSpecDto`
- L3 converte `PackageSpecDto → PackageSpec` via `TryFrom`
- L1 nunca vê `serde` nem `unscanny`

---

## Restrições Estruturais

- `serde` e `unscanny` apenas neste módulo e nos seus testes
- `TryFrom<PackageSpecDto> for PackageSpec` — erro tipado
- `FromStr for PackageSpec` implementado aqui (usa `unscanny`)
- `FromStr for VersionlessPackageSpec` implementado aqui

---

## Tipos

```rust
use serde::{Deserialize, Serialize};
use typst_core::entities::package_spec::{
    PackageSpec, PackageVersion, VersionlessPackageSpec,
};

/// DTO para desserialização de PackageSpec de JSON/TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSpecDto {
    pub namespace: String,
    pub name:      String,
    pub version:   String,  // "0.1.0" — parse em TryFrom
}

#[derive(Debug, thiserror::Error)]
pub enum PackageSpecParseError {
    #[error("invalid version: {0}")]
    InvalidVersion(String),
    #[error("invalid format: expected @namespace/name:version")]
    InvalidFormat,
}

impl TryFrom<PackageSpecDto> for PackageSpec {
    type Error = PackageSpecParseError;

    fn try_from(dto: PackageSpecDto) -> Result<Self, Self::Error> {
        let version = dto.version.parse::<PackageVersion>()
            .map_err(|e| PackageSpecParseError::InvalidVersion(e.to_string()))?;
        Ok(PackageSpec {
            namespace: dto.namespace,
            name:      dto.name,
            version,
        })
    }
}
```

`FromStr` para `PackageSpec` e `VersionlessPackageSpec` —
parser de strings `@preview/algo:0.1.0` usando `unscanny`.
Erro: `PackageSpecParseError::InvalidFormat`.

---

## Critérios de Verificação

```
Dado PackageSpecDto { namespace: "preview", name: "algo", version: "0.1.0" }
Quando TryFrom for chamado
Então Ok(PackageSpec { namespace: "preview", name: "algo", version: 0.1.0 })

Dado PackageSpecDto { version: "não-uma-versão" }
Quando TryFrom for chamado
Então Err(PackageSpecParseError::InvalidVersion)

Dado "@preview/algo:0.1.0".parse::<PackageSpec>()
Quando bem formado
Então Ok(PackageSpec { namespace: "preview", name: "algo", version: 0.1.0 })

Dado "@preview/algo".parse::<VersionlessPackageSpec>()
Quando bem formado
Então Ok(VersionlessPackageSpec { namespace: "preview", name: "algo" })

Dado "formato-errado".parse::<PackageSpec>()
Quando mal formado
Então Err(PackageSpecParseError::InvalidFormat)

Dado que serde está em 03_infra
Quando 01_core for compilado isoladamente
Então não importa serde — isolamento confirmado
```

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação inicial — DTO pattern para PackageSpec (ADR-0005) | package_spec_dto.rs |
