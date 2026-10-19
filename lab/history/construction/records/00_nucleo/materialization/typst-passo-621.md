---
# P621 — `tracking` com árabe e sânscrito (devanágari)

> **Passo:** 621
> **Data:** 2026-07-05
> **Foco:** P593 identificou que a combinação de `tracking` com texto árabe nunca foi testada, apesar da consolidação de largura (`text_width`) ter corrigido a inconsistência entre versões com e sem tracking. Este passo testa isso directamente, e estende o teste ao sânscrito (devanágari), que tem exigências de forma de escrita diferentes do árabe — consoantes conjuntas e reordenação de sinais vocálicos (mátras), não ligação contínua de letras.
> **Tipo:** Verificação directa + correcção, se necessário.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P593 (onde o gap foi identificado), P591 (shaping com forma de escrita aplicada), P581 (onde devanágari foi testado como texto simples, sem tracking).

---

## Contexto

`tracking` adiciona espaço extra entre caracteres. Para scripts onde as letras se ligam ou mudam de forma consoante a vizinha (árabe), ou onde caracteres se reorganizam visualmente (devanágari, onde uma vogal pode aparecer visualmente antes da consoante a que pertence foneticamente), adicionar espaço entre "caracteres" pode ou quebrar a ligação/forma esperada, ou não ter efeito nenhum se o tracking for aplicado ao nível errado da cascata (por exemplo, entre glifos finais em vez de entre caracteres lógicos).

---

## Verificação — Árabe

### Confirmar o comportamento do vanilla

```bash
cat > /tmp/p621-arabe-tracking.typ <<'EOF'
#set text(lang: "ar", font: "DejaVu Sans", size: 24pt)
مرحبا بالعالم

#set text(tracking: 3pt)
مرحبا بالعالم
EOF
lab/typst-original/target/release/typst compile /tmp/p621-arabe-tracking.typ /tmp/p621-arabe-vanilla.pdf
mutool draw -o /tmp/p621-arabe-vanilla.png -r 150 /tmp/p621-arabe-vanilla.pdf
```

Confirmar visualmente: o vanilla aplica o tracking mantendo as letras ligadas dentro de cada palavra (o espaço extra só entre palavras/caracteres separados), ou quebra a ligação, inserindo espaço até dentro de uma palavra?

### Confirmar o cristalino

```bash
./target/release/typst /tmp/p621-arabe-tracking.typ /tmp/p621-arabe-cristalino.pdf
mutool draw -o /tmp/p621-arabe-cristalino.png -r 150 /tmp/p621-arabe-cristalino.pdf
```

Comparar as duas imagens directamente.

### Critério de fecho — Árabe

- [ ] Comportamento do vanilla confirmado visualmente.
- [ ] Comportamento do cristalino confirmado visualmente.
- [ ] Se divergirem: localizada a causa, com `file:line`.

---

## Verificação — Sânscrito (devanágari)

### Confirmar o comportamento do vanilla

```bash
cat > /tmp/p621-sanscrito-tracking.typ <<'EOF'
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 24pt)
नमस्ते संसार

#set text(tracking: 3pt)
नमस्ते संसार
EOF
lab/typst-original/target/release/typst compile /tmp/p621-sanscrito-tracking.typ /tmp/p621-sanscrito-vanilla.pdf
mutool draw -o /tmp/p621-sanscrito-vanilla.png -r 150 /tmp/p621-sanscrito-vanilla.pdf
```

`नमस्ते` (namaste) contém uma consoante conjunta (`स्ते`, "st" + "e") — bom caso de teste porque a forma conjunta muda visualmente se for separada por tracking.

### Confirmar o cristalino

```bash
./target/release/typst /tmp/p621-sanscrito-tracking.typ /tmp/p621-sanscrito-cristalino.pdf
mutool draw -o /tmp/p621-sanscrito-cristalino.png -r 150 /tmp/p621-sanscrito-cristalino.pdf
```

Comparar directamente. Confirmar se a consoante conjunta se mantém visualmente unida com tracking activo, nos dois lados.

### Critério de fecho — Sânscrito

- [ ] Comportamento do vanilla confirmado visualmente, com atenção à consoante conjunta.
- [ ] Comportamento do cristalino confirmado visualmente.
- [ ] Se divergirem: localizada a causa, com `file:line`.
- [ ] Confirmar separadamente se o devanágari, sem tracking nenhum, já passa pelo mesmo mecanismo de `advance_shaped` criado em P591 para o árabe — ou se usa `advance` sem forma de escrita aplicada (o que seria um gap mais antigo, anterior a este passo).

---

## Decisão

Para cada script, um de três resultados:

1. **Cristalino bate com o vanilla:** confirmado, sem código.
2. **Cristalino diverge, causa localizada:** corrigir, seguindo o mesmo padrão já estabelecido (não misturar as duas correcções, árabe e sânscrito, num só commit se as causas forem diferentes).
3. **Cristalino diverge, causa não localizável neste passo:** registar como item novo, com o que já se sabe, para passo dedicado.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Árabe com tracking testado e resolvido (bate com vanilla, ou corrigido, ou registado).
- [ ] Sânscrito com tracking testado e resolvido, da mesma forma.
- [ ] Confirmado se devanágari usa `advance_shaped` (forma de escrita aplicada) ou só `advance` (sem forma), independentemente do tracking.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p621.md`, com hash do commit e as imagens ou descrição suficiente.
- [ ] Lista de disparidades actualizada — último item da lista original de quatro "incertos".
