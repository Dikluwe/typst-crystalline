---
# P589 — Fonte neutra para testes de algoritmo + re-classificar comparações passadas

> **Passo:** 589
> **Data:** 2026-07-05
> **Foco:** Aplicar a ADR de paridade de definições por defeito. Escolher uma fonte disponível nos dois ambientes (cristalino e vanilla), criar o helper que a aplica automaticamente em testes de algoritmo, e rever os documentos de teste já usados na sequência RTL (P563 a P588) para confirmar quais eram testes de algoritmo que deviam ter usado fonte neutra, e não usaram.
> **Tipo:** Implementação + Revisão.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Aplica-se a ADR de paridade de definições por defeito.
> **Dependências:** P588 (onde o problema apareceu), a ADR escrita depois de P588.

---

## Parte 1 — Escolher a fonte neutra

### Sonda

```bash
fc-list : family | sort -u > /tmp/p589-fontes-sistema.txt
lab/typst-original/target/release/typst fonts 2>/dev/null | sort -u > /tmp/p589-fontes-vanilla.txt
./target/release/typst fonts 2>/dev/null | sort -u > /tmp/p589-fontes-cristalino.txt
comm -12 /tmp/p589-fontes-vanilla.txt /tmp/p589-fontes-cristalino.txt
```

Confirmar quais fontes estão disponíveis nos dois lados. Preferir uma fonte sem os problemas já encontrados nesta sequência (não `FreeSerif`, dado o bug de decomposição de acentos de P558; confirmar se `DejaVu Sans` ou `Liberation Sans` servem, já usadas noutros testes desta sequência sem problemas registados).

### Decisão

Escolher uma fonte e documentar a razão em `00_nucleo/testing/fontes-padrao-teste.md`, conforme a ADR pede.

### Critério de fecho da Parte 1

- [ ] Fonte neutra escolhida, disponível nos dois ambientes, sem problemas conhecidos.
- [ ] Documentado em `00_nucleo/testing/fontes-padrao-teste.md`.

---

## Parte 2 — Helper de teste

Criar uma função ou macro que insira automaticamente `#set text(font: "<fonte neutra>")` no início de qualquer documento de teste de algoritmo, para não depender de cada pessoa escrever isto à mão.

```rust
// Esboço, a confirmar contra o padrão de testes já usado no projecto:
fn documento_algoritmo(conteudo: &str) -> String {
    format!("#set text(font: \"{}\")\n{}", FONTE_NEUTRA_TESTE, conteudo)
}
```

### Critério de fecho da Parte 2

- [ ] Helper criado, testado com um caso simples.
- [ ] Usado em pelo menos um teste novo, para confirmar que funciona antes de aplicar em massa.

---

## Parte 3 — Rever os documentos de teste da sequência RTL

Os documentos de teste usados em P563, P564, P566, P567, P569, P574, P577, P586, P587, P588 eram testes de algoritmo (comparavam posições entre cristalino e vanilla) mas não forçavam fonte neutra — usaram o que cada lado tinha por defeito, ou uma fonte específica (`DejaVu Sans` nalguns, nenhuma explícita noutros).

### Verificação

Para cada documento de teste já usado, confirmar se especificava fonte explícita, e se essa fonte estava disponível e idêntica nos dois ambientes.

```bash
grep -l "set text" /tmp/p56*.typ /tmp/p57*.typ /tmp/p58*.typ 2>/dev/null
```

### Decisão

Onde a fonte já era explícita e igual nos dois lados (por exemplo, `DejaVu Sans`, usada em vários testes desta sequência): as medições já feitas continuam válidas, não é preciso repetir.

Onde a fonte não era explícita, ou era `dir: rtl, lang: "ar"` sem fonte definida (o caso do documento de referência de P563 em diante, que dependia da fonte por defeito de cada lado): as medições feitas com esse documento precisam de nota — a diferença de 2,8/0,46 pontos encontrada em P588 já explica a maior parte do que restava, mas outras medições anteriores da mesma sequência (P566, P567) podem ter a mesma fonte de ruído por trás de números que pareciam bugs de algoritmo.

### Critério de fecho da Parte 3

- [ ] Documentos de teste da sequência RTL revistos, um a um.
- [ ] Para cada um: confirmado se usava fonte neutra ou não.
- [ ] Onde não usava: decidido se vale a pena repetir a medição com fonte neutra, para separar de vez o que era bug de algoritmo do que era diferença de fonte.

---

## Critério de fecho do passo

- [ ] Parte 1: fonte neutra escolhida e documentada.
- [ ] Parte 2: helper criado e testado.
- [ ] Parte 3: documentos de teste da sequência RTL revistos, com decisão sobre quais precisam de nova medição.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p589.md`, com hash do commit.
- [ ] `00_nucleo/testing/fontes-padrao-teste.md` criado, seguindo a estrutura pedida na ADR.
