# P1181 — fase 0: capacidade do linter corrigido

**Data:** 2026-08-25T21:02:41-03:00  
**Produto HEAD:** `00f402e875956304aa435f749a251f359287e2ba`  
**Estado inicial:** somente `typst-passo-1181.md` não rastreado; índice vazio

## Proveniência

Fonte do linter:

```text
/repos/Antigravity/tekt-linter
commit cc357d9c3bfa26a4ce1bd71c72bc9cba5b3b027c
branch codex/p0110-structural-self-lint
```

A fonte tinha somente
`00_nucleo/tekt-linter-passo-0110-fechamento-v2-v3-estrutural.md` não rastreado.
`cargo test` terminou com exit 0. Em particular, passaram os oito assessments
de bijeção/reparo que cobrem preflight integral, colisão compartilhada, falha
de metadata, rollback, falha de rollback, revalidação bidirecional e igualdade
dos planos dry/real.

O binário anterior, SHA-256
`1265a0d534274c07b9d44fb152507f1c1a9052528236876394b260fecbbfb466`, não
registrava V26. Ele foi substituído por `cargo install --path . --force`.

Binário instalado resultante:

```text
/home/dikluwe/.cargo/bin/crystalline-lint
SHA-256 b1521dbdc85d0218dee60a9710bb1b1de011c6039fa2c6adf525b1f30e78d3cf
mtime 2026-08-25T21:02:31.005622680-03:00
tamanho 18.565.592 bytes
```

## Prova no produto

`crystalline-lint --checks v15,v26 --fail-on warning .` terminou com exit 1 e
produziu 72 linhas, SHA-256
`f1feeb0620caa94210e3e6b3aade45f429d5d04729d5570dacde28e98134b70a`.
Foram diagnosticadas 24 colisões de ownership V15. Não houve V26, pois o
produto ainda não contém Núcleos Tekt.

`crystalline-lint --fix-hashes --dry-run .` terminou com exit 2, stdout vazio
e bloqueou explicitamente pelas 24 colisões antes de qualquer plano mutante.
O stderr teve SHA-256
`4b7820794f370d7a7d89e5c23e0550ea503ec731155fb998faf5bc7cb3849531`.

O total atual não substitui os 22 prompts do P1179: HEAD e ferramenta mudaram.
O inventário posterior ao gate deve partir das 24 colisões atuais e reconciliar
o manifesto histórico path a path.

## Decisão

A ferramenta cumpre a capacidade mínima para redigir a ADR-0129 e interromper
no gate. Nenhum hash, prompt, header ou código do produto foi alterado nesta
fase. A migração continua bloqueada até aprovação humana da ADR-0129.

