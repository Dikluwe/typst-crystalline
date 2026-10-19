---
# P763 — L0 e sonda: download automático de pacotes `@preview`

> **Passo:** 763
> **Data:** 2026-07-15
> **Foco:** `relatorio-pendencias-proximos-passos.md` (secção 4.2.1) confirma que pacotes `@preview` só resolvem se já estiverem em `~/.cache/typst/packages` — o vanilla descarrega automaticamente do registo oficial quando o pacote não está cacheado, o cristalino não. Não existe prompt L0 dedicado ao downloader; `system-world.md` só menciona "download ainda não implementado (ver P-γ de P678)". Este passo escreve o L0 e a sonda que o sustenta — não avança para código.
> **Tipo:** L0 + Sonda. Sem implementação — regra de ouro do CLAUDE.md (Passo de Execução não pode instruir código L1/L2/L3 sem L0 correspondente já existente e actualizado).
> **Tamanho:** M — sonda e decisão arquitectural; a implementação fica para passo seguinte, condicionado a este L0 estar fechado.
> **ADR-0108 EM VIGOR** — medir antes de decidir. **ADR-0109 EM VIGOR** — fronteira L1/L3: rede é I/O, fica em L3, com trait em L1. **ADR-0114 EM VIGOR** — funcionalidade nunca tocada, sonda obrigatória antes de qualquer código.
> **Dependências:** nenhuma dependência técnica de passo anterior. Ordem de sequência após P762 (último passo fechado), por decisão de prioridade do relatório de pendências, não por bloqueio técnico.

---

## Contexto

O `SystemWorld::resolve_package` actual (03_infra) assume que o pacote já está em disco. Este passo não altera esse ficheiro — só confirma, por medição directa contra o vanilla, o que teria de mudar, e regista essa decisão no L0.

---

## Sonda

Isolar um pacote `@preview` pequeno e público, remover da cache local do vanilla, e observar o comportamento exacto:

```bash
# Confirmar localização real da cache usada pelo vanilla nesta máquina
lab/typst-original/target/release/typst --help | grep -i cache
strace -f -e trace=network,openat lab/typst-original/target/release/typst compile /tmp/p763-preview-test.typ /tmp/p763-out.pdf 2>&1 | grep -E "preview|cache|connect|GET"
```

```bash
# Documento de teste mínimo com um pacote @preview pequeno, público, não cacheado
cat > /tmp/p763-preview-test.typ <<'EOF'
#import "@preview/cetz:0.5.2": canvas
EOF
```

Registar, com proveniência (comando exacto, versão do vanilla, data/hora):
1. Que pedidos de rede o vanilla faz (URL, método) para descobrir e descarregar o pacote.
2. Em que estrutura de directórios grava o resultado.
3. Que erro dá com a rede desligada (`unshare -n` ou equivalente).
4. Se existe checksum/assinatura verificado antes de usar o pacote descarregado.

```bash
cargo build --release -p typst-infra 2>&1 | grep -i "package\|download\|resolve" # confirmar que não há já esboço a meio
```

---

## Decisões a registar no L0 (`00_nucleo/prompts/infra/package_downloader.md` — a criar)

| Decisão | Resolver com base em |
|---|---|
| Registo consultado (oficial / mirror configurável / ambos) | Resultado da sonda, item 1 |
| Localização(ões) de cache usadas | Resultado da sonda, item 2 |
| Concorrência/locking entre compilações simultâneas | Não observável só com um processo — decisão de dono, registada explicitamente como tal |
| Mensagens de erro exactas (pacote inexistente, falha de rede, checksum inválido, permissão negada) | Resultado da sonda, item 3, mais casos adicionais testados manualmente |
| Verificação de integridade do pacote descarregado | Resultado da sonda, item 4 |
| Ponto de entrada no código (`SystemWorld::resolve_package`, confirmar que é aqui) | Leitura directa de `03_infra/src/world.rs` |

---

## Critério de fecho do passo

- [ ] Sonda executada, evidência directa registada (não assumida).
- [ ] Cada decisão da tabela acima resolvida e justificada com a evidência da sonda, ou marcada explicitamente como "decisão de dono" quando a sonda não resolve.
- [ ] L0 escrito em `00_nucleo/prompts/infra/package_downloader.md`, com hash calculado.
- [ ] Nenhum código L1/L2/L3 escrito neste passo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763.md`, com proveniência da sonda.

---

## Próximo passo

P763a (implementação): trait de resolução em L1, cliente HTTP e cache em L3, ligação ao `SystemWorld::resolve_package`, testes cobrindo os erros confirmados pela sonda — só depois do L0 deste passo estar fechado.
