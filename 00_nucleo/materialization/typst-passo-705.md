---
# P705 — `luma()` aceita percentagem (reconstrução retroactiva)

> **Passo:** 705
> **Data:** 2026-07-10 (reconstruído depois do facto — ver nota abaixo)
> **Foco:** P704 isolou que `luma(v * 1%)` falha no cristalino, que só aceita `Int` 0-255. `cetz` usa isto na mesma `palette.typ` que motivou P703/P704. Este passo estende `luma()` para aceitar percentagem, e pede explicitamente uma sondagem sobre se vale a pena generalizar um tipo de componente partilhado (`Int | Ratio`) entre `rgb()`/`luma()`/outros construtores de cor, antes de decidir o âmbito.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P704 (onde o bloqueio foi isolado).

---

## Nota sobre este ficheiro

**Este prompt foi escrito depois do passo já ter sido executado**, porque a
execução aconteceu sem o prompt correspondente ter sido criado primeiro —
uma quebra da sequência normal desta conversa (prompt → execução → relatório
→ revisão). O relatório de P705 já existe e já foi revisto; este ficheiro
existe só para que a sequência numerada fique completa e alguém no futuro
consiga encontrar "o que é que P705 devia fazer", não para alterar o que já
foi decidido ou feito. O conteúdo abaixo é reconstruído a partir do que o
relatório de P705 mostra ter sido pedido e respondido, não uma nova sonda.

**A partir de agora, todos os passos desta cadeia voltam a seguir a sequência
normal — prompt escrito primeiro, sempre.**

---

## Sonda (conforme executada)

### Confirmar o bloqueio directo

```bash
cat > /tmp/p705-luma.typ <<'EOF'
#luma(50%)
#luma(128)
EOF
lab/typst-original/target/release/typst compile /tmp/p705-luma.typ /tmp/p705-vanilla.pdf
./target/release/typst /tmp/p705-luma.typ /tmp/p705-cristalino.pdf
```

### Sondagem pedida: vale a pena generalizar `Int | Ratio`?

Medir, com `grep -rn` sobre o pacote `cetz` real em cache, se algum outro construtor de cor (`oklab`, `oklch`, `hsl`, `hsv`, `cmyk`, `linear-rgb`, ou `rgb()` com percentagens) é de facto usado, antes de decidir se vale a pena um tipo de componente partilhado entre vários construtores, ou se cada função deve ser corrigida isoladamente, como os passos anteriores (P703/P704) já fizeram.

### Confirmar o fallback silencioso do vanilla

```bash
cat > /tmp/p705-fallback.typ <<'EOF'
#luma("bad")
#luma(300)
#luma(150%)
#luma()
EOF
lab/typst-original/target/release/typst compile /tmp/p705-fallback.typ /tmp/p705-fallback-vanilla.pdf
```

Confirmar directamente se o vanilla erra ou produz um valor por defeito nestes casos — não assumir "falhar alto" como filosofia própria sem confirmar primeiro o que o vanilla realmente faz.

---

## Implementação

Estender `native_luma` para aceitar percentagem, cobrindo especificamente o caminho real usado por `cetz` (`Value::Relative` produzido por `v * 1%`, não `Value::Ratio` directo — a confirmar com testes que constroem o valor da forma como o eval realmente o produz, não por atalho).

---

## Resultado (já registado no relatório de P705)

- Sondagem: nenhum outro consumidor real usa os outros construtores — decisão de não generalizar, corrigir só `luma()`.
- Bug próprio encontrado a meio (confusão `Ratio`/`Relative`) e corrigido antes do fecho.
- Fallback silencioso do vanilla confirmado e replicado exactamente, depois de uma decisão inicial diferente ter sido revista.
- 9 testes novos, `cargo test --workspace` sem regressão, `crystalline-lint .` limpo.
- `cetz` re-testado, próximo bloqueio (`in` para `Str`/`Dict`) identificado, não é sucesso completo ainda.

---

## Critério de fecho do passo

- [x] Ver relatório `00_nucleo/diagnosticos/paridade-producao-p705.md` para os critérios de fecho já cumpridos.
- [x] Este ficheiro criado para fechar o buraco na sequência numerada.
