---
# P767b — Verificação por coordenadas: AE de texto+forma não melhorou; regressão de texto puro?

> **Passo:** 767b
> **Data:** 2026-07-15
> **Foco:** P767a reportou como sucesso uma correcção cujos números não mudaram na prática — AE de `rect`/`circle`/`polygon`/`line` misturados com texto ficou no mesmo intervalo de P763h (6504-7076, contra 6563-7070 antes), nalguns casos pior. A explicação dada ("cristalino ainda não implementa espaçamento de parágrafo para texto puro") não foi verificada por medição directa e contradiz o fecho de P745-762, que confirmou AE=0/RMSE=0 para layout de texto vertical. Este passo não aceita a explicação nem a rejeita — mede directamente.
> **Tipo:** Sonda de verificação. Sem implementação — decide se há regressão real, causa nova, ou se a correcção de P767a simplesmente não resolveu o problema que devia resolver.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — não aceitar explicação sem medição directa, nem quando a explicação parece plausível.
> **Dependências:** P767a (correcção reportada, commit a confirmar), P745-762 (baseline de texto puro fechado com AE=0/RMSE=0, a reconfirmar aqui).

---

## Passo 0 — Reconfirmar o baseline de texto puro (P745-762 ainda vale?)

Antes de investigar o caso misto, confirmar se o próprio texto puro regrediu — isto decide se a explicação de P767a é plausível ou inventada.

```bash
cat > /tmp/p767b-texto-puro.typ <<'EOF'
A
B
EOF
```

Usar o mesmo documento (ou o mais próximo possível) que P762 usou para fechar AE=0/RMSE=0 — confirmar qual foi, por leitura de `00_nucleo/diagnosticos/paridade-producao-p762.md`, e reproduzir exactamente esse documento, não um novo.

```bash
lab/typst-original/target/release/typst compile /tmp/p767b-texto-puro.typ /tmp/p767b-texto-vanilla.pdf
./target/release/typst compile /tmp/p767b-texto-puro.typ /tmp/p767b-texto-cristalino.pdf
mutool draw -o /tmp/p767b-texto-vanilla.png -r 300 /tmp/p767b-texto-vanilla.pdf
mutool draw -o /tmp/p767b-texto-cristalino.png -r 300 /tmp/p767b-texto-cristalino.pdf
compare -metric AE /tmp/p767b-texto-vanilla.png /tmp/p767b-texto-cristalino.png /tmp/p767b-texto-diff.png
```

**Se AE ≠ 0**: há uma regressão real de texto puro, introduzida em algum ponto entre P762 e agora — provavelmente por P767a, dado mexer directamente em `sequence.rs`/`block_chain_active`, mecanismo partilhado com parágrafos de texto. Prioridade sobe — isto quebraria um baseline já fechado.

**Se AE = 0**: a explicação de P767a sobre "espaçamento de parágrafo de texto puro" é falsa ou mal direccionada — a causa do AE alto em texto+forma está noutro lugar, a investigar no Passo 1.

---

## Passo 1 — Coordenadas exactas do caso texto+forma+texto

```bash
cat > /tmp/p767b-misto.typ <<'EOF'
A #rect(width: 1cm, height: 0.8cm, fill: red) B
EOF
lab/typst-original/target/release/typst compile /tmp/p767b-misto.typ /tmp/p767b-misto-vanilla.pdf
./target/release/typst compile /tmp/p767b-misto.typ /tmp/p767b-misto-cristalino.pdf
mutool draw -o /tmp/p767b-misto-vanilla.png -r 300 /tmp/p767b-misto-vanilla.pdf
mutool draw -o /tmp/p767b-misto-cristalino.png -r 300 /tmp/p767b-misto-cristalino.pdf
compare -metric AE /tmp/p767b-misto-vanilla.png /tmp/p767b-misto-cristalino.png -highlight-color red -lowlight-color none /tmp/p767b-misto-diffmap.png
```

Inspeccionar `/tmp/p767b-misto-diffmap.png`: a diferença está em "A" deslocado, "B" deslocado, o `rect` deslocado, ou espalhada pelos três?

```bash
mutool trace /tmp/p767b-misto-vanilla.pdf > /tmp/p767b-trace-vanilla.txt
mutool trace /tmp/p767b-misto-cristalino.pdf > /tmp/p767b-trace-cristalino.txt
diff /tmp/p767b-trace-vanilla.txt /tmp/p767b-trace-cristalino.txt
```

Registar, para cada um dos três elementos ("A", `rect`, "B"), a posição exacta em ambos os PDFs e a diferença em pt — não só "parece deslocado".

Confirmar especificamente:
1. Onde "A" termina — o vanilla continua "A" na mesma linha até quebrar por causa do `rect`, ou já quebra antes?
2. Onde o `rect` fica posicionado, relativo a onde "A" terminou.
3. Onde "B" começa, relativo ao fim do `rect`.

---

## Passo 2 — Isolar se a causa é a mesma do P767a ou é nova

Se o Passo 0 confirmar texto puro em AE=0 (sem regressão), e o Passo 1 mostrar que o problema não é mais "forma deslocada" mas sim "texto A ou texto B deslocado", a causa é diferente da que P767a corrigiu — pode ser, por exemplo, que o mecanismo de `block_chain_active`/colapso de margem não esteja a devolver correctamente o cursor de texto à posição esperada depois de uma forma, mesmo que a forma em si esteja bem posicionada.

```bash
grep -n "block_chain_active\|prev_block_below_pending" 01_core/src/rules/layout/sequence.rs 01_core/src/rules/layout/block.rs 2>/dev/null
```

Confirmar por leitura de código o que esse mecanismo faz ao cursor de texto depois de processar um bloco, e comparar com o que as coordenadas do Passo 1 mostram estar a acontecer de facto.

---

## Critério de fecho do passo

- [ ] Baseline de texto puro reconfirmado (AE=0 ou regressão real identificada, com evidência).
- [ ] Mapa de diferenças do caso misto inspeccionado — padrão descrito (qual elemento diverge).
- [ ] Coordenadas exactas de "A", `rect`, "B" comparadas vanilla vs cristalino, com diferença em pt registada por elemento.
- [ ] Causa identificada por leitura de código, não suposição — mesma causa de P767a ou causa nova.
- [ ] Conclusão registada sem qualificadores vagos ("provavelmente", "parece") — ou a causa está confirmada com evidência, ou o passo termina sem conclusão e pede mais investigação, nunca fecha com uma suposição.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p767b.md`, com o mapa de diferenças e as coordenadas anexadas/referenciadas.

---

## Próximo passo

Se regressão real de texto puro: P767c de correcção urgente, prioridade acima de qualquer outro passo em curso — um baseline já fechado (P745-762) não pode ficar quebrado silenciosamente.
Se causa nova no mecanismo pós-bloco: P767c de correcção focada nessa causa específica, com a mesma disciplina de coordenadas antes/depois.
Se, inesperadamente, tudo bater e o AE alto for legitimamente outra coisa não geométrica (ex: a mesma família de diferença estrutural de PDF vista em P763f — miterlimit, colorspace): registar com a mesma evidência que P763f usou, não uma frase solta.
