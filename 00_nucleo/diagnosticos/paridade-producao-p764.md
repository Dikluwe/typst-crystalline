# P764 — Sonda: resolução de versão implícita (`@preview/nome` sem versão)

**Tipo**: Diagnóstico / Sonda  
**Data**: 2026-07-15  
**Passo**: 764

---

## Proveniência da medição

| Item | Valor |
|------|-------|
| HEAD cristalino | `82e84356fb0c80a58f0da21c28a5a0c476937d32` |
| Vanilla (`lab/typst-original/target/release/typst`) | `typst 0.15.0 (969087ec)` |
| Cristalino (`./target/release/typst`) | `typst 0.1.0` |
| Data/hora da sonda | `2026-07-15T14:10:39-03:00` (início) |
| Cache local do vanilla | `~/.cache/typst/packages/preview/cetz/{0.2.2,0.5.2}`, `oxifmt/{0.2.0,1.0.0}`, `tidy/0.3.0` |

## Sonda executada

### 1. Import sem versão (`#import "@preview/cetz"`)

Comando:

```bash
cat > /tmp/p764-sem-versao.typ <<'EOF'
#import "@preview/cetz": canvas
EOF
lab/typst-original/target/release/typst compile /tmp/p764-sem-versao.typ /tmp/p764-out.pdf
```

Resultado no vanilla:

```text
error: package specification is missing version
```

**Conclusão**: o vanilla CLI 0.15.0 **não aceita** import sem versão. A premissa do relatório de pendências ("o vanilla resolve para a versão mais recente") é falsa para a sintaxe de `import`. O cristalino comporta-se da mesma forma (também rejeita), pelo que não existe divergência de linguagem aqui.

### 2. Resolução implícita real no vanilla: `typst init`

O único ponto onde o vanilla resolve versão implicitamente observado é `typst init`:

```bash
lab/typst-original/target/release/typst init @preview/cetz /tmp/p764-init-test
```

Resultado:

```text
error: package @preview/cetz:0.5.2 is not a template
```

A versão foi resolvida para `0.5.2`, que é a maior entre as versões em cache local (`0.2.2`, `0.5.2`) e o índice remoto.

### 3. Ordenação semver e pré-lançamentos (namespace local)

Para testar o critério de ordenação sem depender do índice remoto, criou-se um pacote local em `~/.local/share/typst/packages/local/testpkg/` com as versões:

- `0.1.0`
- `0.2.0`
- `0.2.0-beta` (nome malformado para `PackageVersion`)

Comando:

```bash
lab/typst-original/target/release/typst init @local/testpkg /tmp/p764-local-init
```

Resultado:

```text
error: package @local/testpkg:0.2.0 is not a template
```

**Conclusões**:

- A versão escolhida foi `0.2.0` (máximo triple numérico).
- A entrada `0.2.0-beta` foi **silenciosamente ignorada**, porque o parser de `PackageVersion` rejeita sufixos.
- O vanilla não emite erro para versões malformadas na cache local.

### 4. Pré-lançamentos no namespace `@preview`

O tipo `PackageVersion` do vanilla é composto apenas por `major.minor.patch` (`u32`). O parser rejeita `"0.2.0-beta"` com:

```text
error: version number has unexpected fourth component: `beta`
```

Logo, pré-lançamentos não são representáveis no namespace `@preview`; a questão "considera-os mais recentes ou ignora?" não se coloca — simplesmente não existem.

### 5. Ponto de entrada

- `03_infra/src/world.rs:430`, `SystemWorld::resolve_package`.
- A resolução de versão implícita deve acontecer **antes** da procura em disco / download, mas actualmente o parser do cristalino também exige versão, logo não há caminho de invocação real até que o parser mude.

## Decisões registadas no L0

Todas as decisões foram registadas em `00_nucleo/prompts/infra/package_version_resolution.md` (hash `caeadbbd`):

| Decisão | Resolução |
|---|---|
| Fonte da "versão mais recente" para `@preview` | Índice remoto `https://packages.typst.org/preview/index.json` |
| Fonte para namespaces locais | Apenas data dir (não cache) |
| Ordenação semver | Ordem lexical de `major.minor.patch`; pré-lançamentos ignorados (não representáveis) |
| Erro para versão malformada na cache | Nenhum — silenciosamente ignorada |
| Cache de resolução | OnceCell em memória por instância de `SystemWorld` (decisão de dono) |

## Implicação para o projecto

A premissa de que o cristalino precisava de "resolver `@preview/nome` sem versão em imports" foi refutada. O vanilla CLI 0.15.0 também exige versão em imports. A funcionalidade de resolução implícita limita-se a `typst init` e a futuros pontos que usem `VersionlessPackageSpec`. O L0 foi escrito para esses casos, evitando introduzir uma extensão de linguagem não presente no vanilla.

## Estado do passo

- [x] Sonda executada, evidência directa registada.
- [x] Cada decisão da tabela resolvida e justificada.
- [x] L0 escrito em `00_nucleo/prompts/infra/package_version_resolution.md`, com hash `caeadbbd`.
- [x] Nenhum código L1/L2/L3 escrito neste passo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p764.md`.

## Próximo passo

P764a (implementação): função de enumeração e ordenação de versões cacheadas, ligação ao ponto de resolução de import — só depois do L0 estar fechado e, para o caso remoto, depois de P763a estar implementado. A implementação só terá efeito prático quando existir um comando ou sintaxe que invoque `VersionlessPackageSpec`.
