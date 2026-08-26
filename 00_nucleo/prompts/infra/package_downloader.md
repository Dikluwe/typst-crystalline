# Prompt L0 — `infra/package_downloader` — Download automático de pacotes `@preview`

Hash do Código: 22057f95

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:96b4b53ee7cc95ed530231f42539707f85108de22446dd47c22f162e144cfeb9
- 00_nucleo/prompts/_nuclei/packages/downloader-contract.toml sha256:1fe7c4c5a8dff2081396620797bf0f94b051128c75fcaf306d8a691a50d4d91a

**Camada**: L3
**Criado em**: 2026-07-15 (Passo 763)
**Arquivos gerados**: `03_infra/src/package_downloader.rs` (futuro), alterações em `03_infra/src/world.rs`
**ADRs referência**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização L1/L3), ADR-0114 (sonda obrigatória)

---

## Contexto

O `SystemWorld::resolve_package` actual (P681 / `00_nucleo/prompts/infra/system-world.md`) só procura pacotes já presentes em disco. O vanilla CLI 0.15.0, quando um pacote `@preview/nome:versão` não está na cache local, descarrega-o automaticamente do registo oficial (`https://packages.typst.org`). O cristalino ainda não o faz. Este L0 define o contrato para essa funcionalidade de I/O, mantendo a lógica de rede fora de L1.

## Objetivo

Adicionar ao `SystemWorld` a capacidade de descarregar pacotes `@preview` do registo oficial quando não estiverem presentes localmente, reproduzindo o comportamento observável do vanilla CLI 0.15.0.

> **Fonte de paridade (P1031) — `file:line` do vanilla, que faltava.** As decisões abaixo
> diziam "medidas em P763" sem reproduzir comando/resultado nem apontar o código do vanilla.
> Todas se confirmam por citação literal do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-kit/src/packages.rs`:
>
> | Afirmação do L0 | Citação do vanilla |
> |---|---|
> | Registo primário `https://packages.typst.org` | `packages.rs:328-332`: *"Creates a new handle for interacting with the primary official registry at `https://packages.typst.org`."* — `Self::with_url(downloader, "https://packages.typst.org")` |
> | Mirror configurável, oficial por omissão | `packages.rs:334-342`: `pub fn with_url(…)` — *"Creates a new handle which serves packages from an alternative mirror."* |
> | Namespace `preview` | `packages.rs:325-326`: `pub const NAMESPACE: &str = "preview";` — *"The namespace from which Typst Universe serves packages."* |
> | URL do pacote `…/preview/<nome>-<versão>.tar.gz` | `packages.rs:359-364`: `format!("{}/{}/{}-{}.tar.gz", self.url, Self::NAMESPACE, spec.name, spec.version)` |
> | URL do índice `…/preview/index.json` | `packages.rs:426`: `format!("{}/{}/index.json", self.url, Self::NAMESPACE)` |
> | Data dir `$XDG_DATA_HOME/typst/packages` ou `~/.local/share/typst/packages` | `packages.rs:164-169` (doc comment + `dirs::data_dir().map(…join("typst/packages"))`) |
> | Cache dir `$XDG_CACHE_HOME/typst/packages` ou `~/.cache/typst/packages` | `packages.rs:176-181` |
> | Estrutura `{base}/<namespace>/<nome>/<versão>/` | `packages.rs:192-193`: `eco_format!("{}/{}/{}", spec.namespace, spec.name, spec.version)` |
> | `rename()` atómico a partir de temporário | `packages.rs:273-275`: `match std::fs::rename(&tempdir, &package_dir) { Ok(()) => Ok(()), Err(err) if err.kind() == ErrorKind::DirectoryNotEmpty => Ok(()), … }` |
> | Sem verificação de integridade | `packages.rs:268-272`, literal: *"This means that we do not check the integrity of an existing moved package, just like we don't check the integrity if the package directory already existed in the first place."* |
> | Namespace ≠ `preview` não é descarregado | `packages.rs:355-357`: `if spec.namespace != Self::NAMESPACE { return Err(PackageError::NotFound(spec.clone())); }` |
>
> **As mensagens de erro de §6 também se confirmam**, e são caso em que a mecânica **é** o
> observável (ADR-0108), logo paridade estrita. Fonte: `impl Display for PackageError`,
> `crates/typst-library/src/diag.rs:714-731`:
>
> ```rust
> Self::NotFound(spec) => write!(f, "package not found (searched for {spec})"),
> Self::VersionNotFound(spec, latest) => write!(f,
>     "package found, but version {} does not exist (latest is {})", spec.version, latest),
> Self::NetworkFailed(Some(err)) => write!(f, "failed to download package ({err})"),
> Self::NetworkFailed(None) => f.pad("failed to download package"),
> ```
>
> As três formas de §6 batem à letra. O vanilla tem ainda `failed to decompress package
> ({err})` e `failed to decompress package (archive malformed)` (`diag.rs:732-736`), que §6
> não regista — lacuna documentada.
>
> **Natureza**: literal para tudo. Nota: a sonda P763 continua sem comando/saída registados,
> mas a prova deixou de depender dela — o `file:line` do vanilla é reproduzível no hash
> pinado.

## Decisões arquiteturais (medidas em P763)

### 1. Registo consultado

- **Registo primário**: `https://packages.typst.org` (oficial).
- **Mirror configurável**: o ponto de extensão futuro deve aceitar um URL base alternativo, mas o valor por omissão é o registo oficial.
- O vanilla não consulta múltiplos registos; replica esse comportamento.

### 2. URLs e método de download

- **Pacote**: `GET https://packages.typst.org/preview/<nome>-<versão>.tar.gz`
- **Índice de versões**: `GET https://packages.typst.org/preview/index.json` (usado para resolução de `latest`, ver `infra/package_version_resolution.md`)
- Método HTTP: `GET`.
- Cliente: HTTPS com TLS nativo do sistema, respeitando variáveis de proxy (`HTTPS_PROXY` / `https_proxy`), conforme implementação do vanilla (`ureq` + `env_proxy`).

### 3. Localização e estrutura de cache

- As bases de procura mantêm-se as mesmas definidas em `infra/system-world.md`:
  1. Data dir: `$XDG_DATA_HOME/typst/packages` ou `~/.local/share/typst/packages`.
  2. Cache dir: `$XDG_CACHE_HOME/typst/packages` ou `~/.cache/typst/packages`.
- O pacote descarregado é gravado em `{base}/preview/<nome>/<versão>/`, correspondendo ao conteúdo extraído do `tar.gz`.
- A prioridade de procura (data > cache) mantém-se; o download deve colocar novos pacotes na **cache dir**, nunca na data dir.

### 4. Concorrência e locking entre compilações simultâneas

- **Decisão de dono**: replicar o mecanismo do vanilla.
- O vanilla descarrega para um directório temporário `.tmp-<versão>-<rand>` dentro de `{base}/preview/<nome>/` e faz `rename()` atómico para o destino final.
- Comentário do código fonte do vanilla: "Concurrent downloads do not cause corruption".
- Não se implementa file-lock explícito nesta fase.

### 5. Verificação de integridade

- **Decisão de dono**: não verificar checksum nem assinatura do pacote descarregado.
- O vanilla não realiza verificação de integridade do arquivo descarregado (comentário em `typst-kit/src/packages.rs`: "we do not check the integrity of an existing moved package").

### 6. Mensagens de erro exactas

As mensagens devem ser claras e distinguir os casos observados na sonda:

- **Falha de rede / conexão recusada**:
  - Vanilla: `error: failed to download package (<url>: Connection Failed: Connect error: ...)`
  - Cristalino: manter equivalente em português ou alinhar ao vanilla conforme decisão futura de localização; por agora, propagar o erro do cliente HTTP.
- **Pacote inexistente (404 no nome)**:
  - Vanilla: `error: package not found (searched for @preview/<nome>:<versão>)`
- **Versão inexistente (404 na versão, mas pacote existe no índice)**:
  - Vanilla: `error: package found, but version <X> does not exist (latest is <Y>)`
- **Permissão negada / I/O ao gravar cache**:
  - Propagar mensagem do sistema operativo, prefixada com o contexto do download.

### 7. Ponto de entrada no código

- `03_infra/src/world.rs`, método `SystemWorld::resolve_package` (linha 430 em HEAD `82e84356`).
- Quando o pacote não é encontrado em nenhuma base local e `spec.namespace == "preview"`, invocar o downloader em vez de retornar imediatamente o erro de cache.
- Se ainda assim não for encontrado, manter a mensagem de erro actual ou melhorá-la com o contexto de download.

## Restrições estruturais

- Toda a lógica de rede e I/O de cache fica em L3.
- O trait L1 pertence a `contracts/package_downloader.md`; L3 implementa-o.
- Não se adicionam dependências de I/O a `01_core`.
- O `SystemWorld` continua a ser a única entidade em L3 que conhece filesystem + rede.

## CA customizada — P1137-CERT

Quando configurado por L4, `HttpPackageDownloader` guarda somente
`Option<PathBuf>` e lê o PEM ao construir o agente HTTPS, imediatamente antes
da request. Uma ou mais CAs válidas são acrescentadas a
`webpki_roots::TLS_SERVER_ROOTS`; hostname verification e as roots normais
permanecem ativas. Path ilegível, ficheiro vazio, PEM sem certificados ou DER
inválido produzem erro local sem imprimir path nem bytes. A ausência preserva
o agente `ureq` anterior. As invariantes compartilhadas estão no Núcleo Tekt
`network/custom-ca-cert.toml` pinado acima.

## Índice para `typst init` — P1137-INIT-2

`latest_version(name)` consulta `preview/index.json` com o mesmo agente TLS,
filtra entradas pelo nome e devolve a maior `PackageVersion`. Falha de rede,
leitura ou JSON inválido permanece distinguível de package ausente. O comando
`init` usa este método somente quando a versão `@preview` foi omitida.

## Critérios de verificação

```
Dado um documento que importa @preview/fletcher:0.5.4
Quando o pacote não está presente na cache local
Então SystemWorld::resolve_package descarrega o pacote para ~/.cache/typst/packages/preview/fletcher/0.5.4/
E a compilação prossegue com sucesso

Dado um documento que importa @preview/xyzdoesnotexist:1.0.0
Quando o pacote não existe no registo
Então o erro reportado indica "package not found"

Dado um documento que importa @preview/fletcher:99.99.99
Quando a versão não existe mas o pacote sim
Então o erro reportado indica a versão pedida e a versão mais recente conhecida

Dado um ambiente sem conectividade de rede (ou proxy inválido)
Quando se tenta descarregar um pacote
Então o erro reportado indica falha de ligação ao URL do pacote
```

### Capacidade ambiental da sonda TLS local — P1140.8

O teste de CA customizada usa um listener em loopback para não depender de rede
pública. Falha de `bind` especificamente por `PermissionDenied` significa que
o runner não oferece a capacidade necessária e pode encerrar essa sonda com
mensagem explícita. Qualquer outro erro de bind continua a falhar o teste.
Quando o listener é criado, handshake, cadeia, hostname, timeout e resposta
HTTP são asserções obrigatórias e nunca podem ser convertidos em skip. A sonda
completa deve passar ao menos uma vez num ambiente com loopback permitido.
