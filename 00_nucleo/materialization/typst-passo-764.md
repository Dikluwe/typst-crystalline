---
# P764 — L0 e sonda: resolução de versão implícita (`@preview/nome` sem versão)

> **Passo:** 764
> **Data:** 2026-07-15
> **Foco:** `relatorio-pendencias-proximos-passos.md` (secção 4.2.2) confirma que `#import "@preview/nome"`, sem versão, falha no cristalino; o vanilla resolve para a versão mais recente disponível. Não existe prompt L0 dedicado. Este passo escreve o L0 e a sonda — não avança para código.
> **Tipo:** L0 + Sonda. Sem implementação — mesma regra de P763.
> **Tamanho:** S/M — decisão mais estreita que P763, mas depende dela para o caso remoto.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda obrigatória antes de código.
> **Dependências:** P763, no mínimo para a enumeração da cache local (se P763 decidir onde e como a cache é gravada). O caso "resolver localmente, entre versões já cacheadas" pode ser sondado sem esperar por P763 terminar; o caso remoto depende dele.

---

## Contexto

Este passo não decide *como descarregar* (isso é P763) — decide *qual versão escolher* quando o utilizador não especifica uma, entre as versões disponíveis (localmente já, remotamente se P763 permitir consulta ao registo).

---

## Sonda

```bash
# Preparar múltiplas versões do mesmo pacote na cache local do vanilla
ls ~/.cache/typst/packages/preview/cetz/ 2>&1

cat > /tmp/p764-sem-versao.typ <<'EOF'
#import "@preview/cetz": canvas
EOF
lab/typst-original/target/release/typst compile /tmp/p764-sem-versao.typ /tmp/p764-out.pdf
```

Registar, com proveniência:
1. Qual versão o vanilla escolhe com múltiplas versões cacheadas.
2. Se uma versão pré-lançamento (`-beta`, `-rc`) presente é considerada "mais recente" ou ignorada — testar com uma versão fabricada localmente para o efeito, se necessário.
3. Comportamento quando uma entrada de versão na cache tem nome malformado (não é semver válido).

```bash
grep -rn "resolve_package\|latest\|semver" lab/typst-original/crates/typst-kit/src/*.rs 2>/dev/null | head -30
```

Confirmar contra o código-fonte do vanilla, não contra convenção genérica de semver.

---

## Decisões a registar no L0 (`00_nucleo/prompts/infra/package_version_resolution.md` — a criar)

| Decisão | Resolver com base em |
|---|---|
| Fonte da "versão mais recente" (só local / local+remoto) | Depende da decisão de P763 sobre consulta ao registo |
| Critério de ordenação semver, incluindo pré-lançamentos | Resultado da sonda, itens 1–2 |
| Mensagem de erro para versão malformada na cache | Resultado da sonda, item 3 |
| Cache de resolução dentro da mesma compilação (evitar reconsultar o registo por cada import) | Decisão de dono se a sonda não expuser comportamento do vanilla aqui |

---

## Critério de fecho do passo

- [ ] Sonda executada, evidência directa registada.
- [ ] Cada decisão da tabela resolvida e justificada, ou marcada como decisão de dono.
- [ ] L0 escrito em `00_nucleo/prompts/infra/package_version_resolution.md`, com hash calculado.
- [ ] Nenhum código L1/L2/L3 escrito neste passo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p764.md`.

---

## Próximo passo

P764a (implementação): função de enumeração e ordenação de versões cacheadas, ligação ao ponto de resolução de import — só depois do L0 estar fechado e, para o caso remoto, depois de P763a estar implementado.
