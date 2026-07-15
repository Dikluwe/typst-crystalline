# P763 — Sonda: download automático de pacotes `@preview`

**Tipo**: Diagnóstico / Sonda  
**Data**: 2026-07-15  
**Passo**: 763

---

## Proveniência da medição

| Item | Valor |
|------|-------|
| HEAD cristalino | `82e84356fb0c80a58f0da21c28a5a0c476937d32` |
| Vanilla (`lab/typst-original/target/release/typst`) | `typst 0.15.0 (969087ec)` |
| Cristalino (`./target/release/typst`) | `typst 0.1.0` |
| Data/hora da sonda | `2026-07-15T14:10:39-03:00` (início) |
| Estado working tree | 176 ficheiros não commitados (todos `.md`/diagnóstico), 2 deleções em `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md` (ver `git diff HEAD --stat` em `/tmp/p763-evidencia/hora-inicio.txt`) |
| Rede | Indisponível para ICMP (`ping` 100% loss), mas HTTP/HTTPS funciona (download real observado) |

## Sonda executada

### 1. Pedidos de rede para descobrir e descarregar o pacote

Pacote de teste: `@preview/fletcher:0.5.4` (não estava em cache antes da sonda).

Comando:

```bash
cat > /tmp/p763-preview-test.typ <<'EOF'
#import "@preview/fletcher:0.5.4": diagram
EOF
strace -f -e trace=network -s 500 -o /tmp/p763-evidencia/strace2.log \
  lab/typst-original/target/release/typst compile \
  /tmp/p763-preview-test.typ /tmp/p763-out.pdf
```

Resultado:

- DNS para `packages.typst.org` → CNAME `packages-ahhgg9gsfmfdd0gf.z01.azurefd.net` → IPs `150.171.110.40` (IPv4) e `2603:1061:14:123::1` (IPv6).
- TLS handshake na porta 443 do IP resolvido.
- Download bem-sucedido; pacote gravado em `~/.cache/typst/packages/preview/fletcher/0.5.4/`.

O tráfego HTTPS é cifrado, pelo que o path exacto não é visível no `strace`, mas o código-fonte do vanilla (`lab/typst-original/crates/typst-kit/src/packages.rs:359-365`) define:

```rust
let url = format!(
    "{}/{}/{}-{}.tar.gz",
    self.url, Self::NAMESPACE, spec.name, spec.version,
);
```

com `self.url = "https://packages.typst.org"` e `NAMESPACE = "preview"`. Logo:

- **URL de download**: `GET https://packages.typst.org/preview/<nome>-<versão>.tar.gz`
- **URL do índice** (para resolução de `latest`): `GET https://packages.typst.org/preview/index.json`

### 2. Estrutura de directórios após download

```text
~/.cache/typst/packages/preview/fletcher/0.5.4/
├── LICENSE
├── README.md
├── src/
│   ├── coords.typ
│   ├── default-marks.typ
│   ├── deps.typ
│   ├── diagram.typ
│   ├── draw.typ
│   ├── edge.typ
│   ├── exports.typ
│   ├── marks.typ
│   ├── node.typ
│   ├── shapes.typ
│   └── utils.typ
└── typst.toml
```

Padrão: `{cache-dir}/{namespace}/{nome}/{versão}/{conteúdo extraído do tar.gz}`.

### 3. Erro com rede indisponível

Como `unshare -n` falhou por permissão, simulou-se falha de rede via proxy inválido:

```bash
rm -rf ~/.cache/typst/packages/preview/fletcher/0.5.4
https_proxy=http://127.0.0.1:1 \
  lab/typst-original/target/release/typst compile \
  /tmp/p763-preview-test.typ /tmp/p763-out-proxy.pdf
```

Resultado:

```text
error: failed to download package (https://packages.typst.org/preview/fletcher-0.5.4.tar.gz: Connection Failed: Connect error: Connection refused (os error 111))
```

### 4. Checksum / assinatura

O código-fonte do vanilla (`typst-kit/src/packages.rs:267-272`) indica explicitamente que **não** verifica a integridade do pacote depois de o mover para o destino final:

> "we do not check the integrity of an existing moved package, just like we don't check the integrity if the package directory already existed in the first place."

Portanto, o vanilla não valida checksum nem assinatura.

### 5. Ponto de entrada no cristalino

- `03_infra/src/world.rs:430`, `SystemWorld::resolve_package`.
- Actualmente retorna erro quando o pacote não está na cache local.
- A implementação futura deve invocar o downloader quando `spec.namespace == "preview"` e o pacote não for encontrado offline.

## Decisões registadas no L0

Todas as decisões foram registadas em `00_nucleo/prompts/infra/package_downloader.md` (hash `b6450b35`):

| Decisão | Resolução |
|---|---|
| Registo consultado | Oficial `https://packages.typst.org`; mirror configurável como extensão futura |
| Localização de cache | Data dir e cache dir conforme `system-world.md`; download grava na cache dir |
| Concorrência | Temp dir `.tmp-<versão>-<rand>` + rename atómico; sem lock explícito (decisão de dono) |
| Mensagens de erro | Replicar textos observados (rede, pacote não encontrado, versão inexistente, I/O) |
| Verificação de integridade | Não verificar checksum/assinatura (o vanilla não verifica) |
| Ponto de entrada | `SystemWorld::resolve_package` em `03_infra/src/world.rs` |

## Estado do passo

- [x] Sonda executada, evidência directa registada.
- [x] Cada decisão da tabela resolvida e justificada com a evidência da sonda.
- [x] L0 escrito em `00_nucleo/prompts/infra/package_downloader.md`, com hash `b6450b35`.
- [x] Nenhum código L1/L2/L3 escrito neste passo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763.md`.

## Próximo passo

P763a (implementação): trait de resolução em L1, cliente HTTP e cache em L3, ligação ao `SystemWorld::resolve_package`, testes cobrindo os erros confirmados pela sonda — só depois do L0 deste passo estar fechado.
