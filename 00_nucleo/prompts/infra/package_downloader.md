# Prompt L0 — `infra/package_downloader` — Download automático de pacotes `@preview`

Hash do Código: b6450b35

**Camada**: L3
**Criado em**: 2026-07-15 (Passo 763)
**Arquivos gerados**: `03_infra/src/package_downloader.rs` (futuro), alterações em `03_infra/src/world.rs`
**ADRs referência**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização L1/L3), ADR-0114 (sonda obrigatória)

---

## Contexto

O `SystemWorld::resolve_package` actual (P681 / `00_nucleo/prompts/infra/system-world.md`) só procura pacotes já presentes em disco. O vanilla CLI 0.15.0, quando um pacote `@preview/nome:versão` não está na cache local, descarrega-o automaticamente do registo oficial (`https://packages.typst.org`). O cristalino ainda não o faz. Este L0 define o contrato para essa funcionalidade de I/O, mantendo a lógica de rede fora de L1.

## Objetivo

Adicionar ao `SystemWorld` a capacidade de descarregar pacotes `@preview` do registo oficial quando não estiverem presentes localmente, reproduzindo o comportamento observável do vanilla CLI 0.15.0.

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
- L1 define apenas um trait mínimo (`PackageResolver` ou nome equivalente) com assinatura pura; L3 implementa-o.
- Não se adicionam dependências de I/O a `01_core`.
- O `SystemWorld` continua a ser a única entidade em L3 que conhece filesystem + rede.

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
