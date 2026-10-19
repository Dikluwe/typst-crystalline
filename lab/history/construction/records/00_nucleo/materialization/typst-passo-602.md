---
# P602 — Distinguir `/Count` aberto/fechado nos bookmarks

> **Passo:** 602
> **Data:** 2026-07-05
> **Foco:** O `/Count` de cada entrada de bookmark no PDF indica quantos descendentes tem, e o sinal (positivo/negativo) indica se aparecem abertos ou fechados por defeito quando o PDF é aberto. Registado em P535 como "não distinguido", sem razão escrita — confirmado depois como falta de implementação, não decisão. Este passo corrige.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P535 (implementação original de bookmarks, `/Count` deixado de fora).

---

## Sonda

### Confirmar o que o vanilla faz

```bash
cat > /tmp/p602-headings.typ <<'EOF'
= Primeira Secção
== Subsecção A
== Subsecção B
= Segunda Secção
EOF
lab/typst-original/target/release/typst compile /tmp/p602-headings.typ /tmp/p602-vanilla.pdf
mutool show /tmp/p602-vanilla.pdf raw 4 2>/dev/null | grep -i count
# ou inspeccionar directamente o dicionário de outline
python3 -c "
import re
data = open('/tmp/p602-vanilla.pdf', 'rb').read()
for m in re.finditer(rb'/Count\s+(-?\d+)', data):
    print(m.group())
"
```

Confirmar se o vanilla usa `/Count` positivo (aberto por defeito) ou negativo (fechado por defeito) para entradas com filhos, e se isso depende de alguma propriedade do documento ou é sempre a mesma escolha.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p602-headings.typ /tmp/p602-cristalino.pdf
python3 -c "
import re
data = open('/tmp/p602-cristalino.pdf', 'rb').read()
for m in re.finditer(rb'/Count\s+(-?\d+)', data):
    print(m.group())
"
```

### Localizar onde os bookmarks são escritos

```bash
grep -n "/Count\|Outlines\|bookmark" 03_infra/src/export/builder.rs | head -20
```

### Critério de fecho da sonda

- [ ] Confirmado o comportamento do vanilla (sinal do `/Count`, positivo ou negativo, e em que condição).
- [ ] Confirmado o estado actual do cristalino (provavelmente sempre positivo, ou ausente).
- [ ] Localizado o ponto exacto onde `/Count` é escrito.

---

## Implementação

Cada entrada de bookmark com filhos precisa de:
- `/Count N` onde `N` é o número total de descendentes (não só filhos directos — inclui netos, etc.), com sinal positivo se a entrada aparece aberta por defeito, negativo se aparece fechada.

Implementar o sinal conforme confirmado pela sonda — se o vanilla usa sempre um valor (por exemplo, sempre aberto, positivo), replicar essa escolha; se depender de alguma propriedade do documento, replicar essa lógica.

### Critério de fecho da implementação

- [ ] `/Count` com o sinal certo, confirmado contra o vanilla.
- [ ] Testado com hierarquia de três níveis (heading dentro de heading dentro de heading), confirmando que a contagem de descendentes está certa em cada nível, não só no topo.

---

## Validação

```bash
./target/release/typst /tmp/p602-headings.typ /tmp/p602-depois.pdf
python3 -c "
import re
data = open('/tmp/p602-depois.pdf', 'rb').read()
for m in re.finditer(rb'/Count\s+(-?\d+)', data):
    print(m.group())
"
mutool show /tmp/p602-depois.pdf outline
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `/Count` com sinal e valor certos, confirmados contra o vanilla.
- [ ] Testado com hierarquia de três níveis.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p602.md`, com hash do commit.
- [ ] Listas de disparidades actualizadas.
