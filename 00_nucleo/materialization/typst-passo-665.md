---
# P665 — Reverter `text.bold`/`text.italic`, mapear internamente para `weight`/`style`

> **Passo:** 665
> **Data:** 2026-07-09
> **Foco:** P664 confirmou, com teste directo, que `#set text(bold: true)` e `#set text(italic: true)` são aceites pelo cristalino mas rejeitados pelo vanilla — divergência de linguagem introduzida desde P525, nunca reconhecida como decisão consciente. O vanilla usa `weight: "bold"` e `style: "italic"`. Este passo reverte a sintaxe de nível de linguagem, preservando o comportamento interno onde `bold`/`italic` já são usados (markup `*...*`/`_..._`, shaper), mapeando-os para `weight`/`style` em vez de os manter como propriedades próprias expostas ao utilizador.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M. Mais complexo que P662 (`variant`), porque `bold`/`italic` já têm uso interno estabelecido.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P664 (onde a divergência foi confirmada), P662 (padrão de reversão já estabelecido).

---

## Sonda

### Confirmar todos os sítios internos que usam `bold`/`italic`

```bash
grep -rn "\.bold\b\|\.italic\b\|style\.bold\|style\.italic" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v test
```

Distinguir entre:
1. Uso interno de `TextStyle.bold`/`TextStyle.italic` como campos de estado (pode manter-se, é implementação).
2. O ponto exacto onde `#set text(bold: ...)`/`#set text(italic: ...)` são parseados como argumentos nomeados (isto é o que precisa de ser removido/redireccionado).
3. Onde `*...*`/`_..._` definem `bold`/`italic` (isto é semântica de markup, não argumento nomeado — confirmar se deve manter-se como está, ou se também devia usar `weight`/`style` internamente).

### Confirmar a relação exacta entre `bold`/`italic` e `weight`/`style` no vanilla

```bash
grep -n "fn.*bold\|fn.*italic\|Strong\|Emph" lab/typst-original/crates/typst-library/src/text/mod.rs 2>/dev/null | head -20
```

Confirmar se `*...*` no vanilla define `weight: "bold"` directamente, ou se há um mecanismo intermédio (por exemplo, um `delta` de peso, não um valor absoluto — negrito dentro de negrito não deve ficar "mais que bold").

### Critério de fecho da sonda

- [ ] Todos os sítios internos que usam `bold`/`italic` mapeados.
- [ ] Confirmado como o vanilla relaciona `*...*`/`_..._` com `weight`/`style` internamente.
- [ ] Confirmado se é seguro remover `bold`/`italic` como argumentos nomeados de `#set text(...)` sem quebrar `*...*`/`_..._`.

---

## Implementação

Remover o reconhecimento de `bold`/`italic` como argumentos nomeados de `#set text(...)`. `#set text(bold: true)` passa a produzir o mesmo erro que o vanilla ("unexpected argument: bold"). Internamente, `*...*`/`_..._` continuam a funcionar, mas mapeados através de `weight`/`style` (ou do mecanismo interno que a sonda confirmar ser o correcto), não através de um campo `bold`/`italic` exposto à linguagem.

### Critério de fecho da implementação

- [ ] `#set text(bold: true)` e `#set text(italic: true)` produzem erro, igual ao vanilla.
- [ ] `*texto em negrito*` e `_texto em itálico_` continuam a funcionar visualmente sem regressão.
- [ ] `#set text(weight: "bold")` e `#set text(style: "italic")` continuam a funcionar (já confirmados por P664 como correctos).
- [ ] Negrito dentro de negrito (`*já **muito** negrito*`, se a sintaxe permitir aninhar) comporta-se de forma sensata, não "mais que bold".

---

## Validação

```bash
cat > /tmp/p665-markup.typ <<'EOF'
Texto normal, *texto em negrito*, _texto em itálico_, e *_negrito e itálico_*.
EOF
./target/release/typst /tmp/p665-markup.typ /tmp/p665.pdf
mutool draw -o /tmp/p665.png -r 150 /tmp/p665.pdf
```

Comparar visualmente com o vanilla no mesmo documento.

```bash
cat > /tmp/p665-rejeitado.typ <<'EOF'
#set text(bold: true)
Texto.
EOF
./target/release/typst /tmp/p665-rejeitado.typ /tmp/p665-r.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, uso interno mapeado.
- [ ] `bold`/`italic` como argumentos nomeados removidos, erro igual ao vanilla.
- [ ] Markup `*...*`/`_..._` sem regressão visual.
- [ ] `weight`/`style` continuam a funcionar.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p665.md`, com hash do commit.
- [ ] Nota adicionada ao histórico de P525 (onde `bold`/`italic` apareceram pela primeira vez), explicando a reversão.
