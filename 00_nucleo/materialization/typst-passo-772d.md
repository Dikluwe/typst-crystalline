---
# P772d — Verificação: a correção de P772b já resolve o `<detached>` de P772c?

> **Passo:** 772d
> **Data:** 2026-07-16
> **Foco:** P772c encontrou `<detached>` no path de erro para `#import "preview/nome:1.0.0"` (sem `@`, tratado como path relativo, erro de I/O "file not found"). P772b corrigiu `<detached>` para spans cross-file válidos que apontam para outro `FileId`. Não está confirmado se são a mesma causa (P772b já resolve) ou causas distintas (erro de I/O sem span nenhum vs span válido apontando para ficheiro errado). Verificação rápida, não é investigação nova.
> **Tipo:** Verificação. Sem implementação, a menos que confirme que ainda há um bug remanescente pequeno.
> **Tamanho:** XS.
> **Dependências:** P772b (correcção de span cross-file), P772c (achado do caso específico).

---

## Verificação

Com o código actual (já incluindo a correcção de P772b), reproduzir exactamente o caso de P772c:

```bash
cat > /tmp/p772d-test.typ <<'EOF'
#import "preview/nome:1.0.0"
EOF
lab/typst-original/target/release/typst compile /tmp/p772d-test.typ 2>&1
./target/release/typst compile /tmp/p772d-test.typ 2>&1
```

Confirmar se o cristalino ainda mostra `<detached>` no path, ou se já resolve correctamente.

---

## Critério de fecho do passo

- [ ] Caso reproduzido com o código actual (pós-P772b).
- [ ] Se já corrigido: registar que era a mesma causa, fechar sem código.
- [ ] Se ainda com `<detached>`: confirmar por leitura rápida de código se é a mesma classe de problema (span/source resolution) ou uma causa distinta (erro de I/O sem span). Se for pequeno e isolado, corrigir aqui mesmo; se maior, registar como achado para passo dedicado, não forçar correcção apressada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772d.md` (pode ser curto, dado o tamanho XS do passo).

---

## Próximo passo

Reconfirmar a lista `lacuna-inventario` restante e decidir se vale continuar a varredura sistemática ou encerrar com o que já foi coberto.
