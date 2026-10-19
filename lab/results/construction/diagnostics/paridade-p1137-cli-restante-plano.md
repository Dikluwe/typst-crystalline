# P1137 — plano documental do CLI restante

**Data:** 2026-08-23  
**Baseline:** vanilla ratificado `a51e02804`  
**Estado da matriz:** 10 `MATCH` após o fechamento de `P1137-C-001`.

## L0 preparados

| Capacidade | Prompt L0 | Dependência principal | Entrega |
|---|---|---|---|
| `fonts` | `prompts/shell/fonts-command.md` | descoberta L3 já existente | **implementada** |
| `completions` | `prompts/shell/completions.md` | `clap_complete` em L2 | **implementada** |
| `info` | `prompts/shell/info.md` | snapshot allowlisted L2/L3 | **INFO-1 implementado**; INFO-2 futuro |
| `--cert` | `prompts/shell/custom-ca-cert.md` | roots TLS no `ureq` | **implementada** |
| `init` | `prompts/shell/init.md` | packages + template seguro | **INIT-1 e INIT-2 implementados** |
| `watch`/`w` | `prompts/shell/watch.md` | dependências do World + watcher | **WATCH-1/WATCH-2 implementados** |

## Ordem recomendada

1. ~~`fonts`~~: implementada.
2. ~~`completions`~~: implementada a partir da árvore clap real.
3. ~~`info` INFO-1~~: implementado com saída humana/JSON e snapshot read-only allowlisted.
4. ~~`--cert`~~: implementada no TLS real, preservando roots e hostname verification.
5. ~~`init` INIT-1/INIT-2~~: implementado com resolução e escrita transacional.
6. ~~`watch` WATCH-1/2~~: implementado com inventário transitivo,
   recompilação seletiva e publicação atómica.

O plano está concluído. Limitações deliberadamente fora do denominador inicial
(como INFO-2, expansão do HTML e comparadores geométricos/visuais profundos)
seguem como trabalho futuro independente e não reabrem `P1137-C-001`.

## Gates

Todos alteram contrato público e permanecem atrás da confirmação obrigatória
da ADR-0127. Cada prompt deve ser confirmado e ressellado antes do primeiro
teste RED da capacidade correspondente. Entregas faseadas mantêm estado
`PARTIAL` explicitamente até o passo nomeado que fecha o subconjunto.

Não anunciar comando ou opção no help antes de existir comportamento real.
