---
# P587 — Causa exacta da quebra de linha prematura em RTL

> **Passo:** 587
> **Data:** 2026-07-05
> **Foco:** P586 encontrou que o cristalino quebra a linha `الكتاب 42 على الطاولة` antes do necessário, e atribuiu isto a "largura útil", sem escolher entre fonte, espaçamento, ou margem. Fazendo a conta com os próprios números da tabela de P586: a caixa de linha do cristalino (276pt para três palavras) é muito mais estreita do que a do vanilla (451,2pt para quatro), enquanto a largura de cada palavra, somada, é quase igual dos dois lados — a diferença de "42" é inferior a 3 pontos. Isto aponta para o cálculo da largura disponível na decisão de quebra, não para a fonte. Este passo confirma isso com `file:line`, não com suposição.
> **Tipo:** Sonda directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Três causas possíveis ao mesmo tempo não é uma conclusão — é a falta de uma.
> **Dependências:** P586 (dados brutos já medidos, usados aqui para a conta).

---

## A conta já feita, para referência

Dados da tabela de P586:

| | Cristalino | Vanilla |
|---|---|---|
| Largura da caixa de linha (`###FLOW###`) | 276,00 pt (3 palavras) | 451,20 pt (4 palavras) |
| Soma das larguras das palavras | 256,00 pt (على+42+الكتاب) | 421,20 pt (todas as 4) |
| Espaço ocupado só pelas 3 palavras comuns | على(72)+42(40)+الكتاب(144) = 256 | على(72)+42(37.2)+الكتاب(144) = 253,2 |

A diferença entre os dois lados nas três palavras comuns é de 2,8 pontos (só o "42"). Não é isto que explica uma diferença de 175 pontos na largura da caixa de linha inteira.

---

## Sonda

### Confirmar a largura disponível usada na decisão de quebra

```bash
grep -n "right_margin\|width - self.page_config.margin\|available_width" 01_core/src/engine/layout/cursor.rs | head -20
```

Perguntas, com `file:line`:

1. Que valor de largura disponível é calculado no momento em que o cristalino decide que "الطاولة" não cabe? Confirmar o número exacto, não uma variável com nome.
2. Esse valor bate certo com a largura da página menos as margens (595,28 − 2×70,87 = 453,54, próximo do valor real do vanilla, 451,20)? Ou é mais pequeno?
3. Se for mais pequeno: onde é que a diferença entra — é subtraída duas vezes a mesma margem, é usada uma margem maior do que a configurada, ou outra coisa?

### Teste directo para confirmar

```bash
cat > /tmp/p587-margem.typ <<'EOF'
#set page(margin: 0pt)
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p587-margem.typ /tmp/p587.pdf
pdftotext -tsv /tmp/p587.pdf -
```

Com margem zero, a largura disponível devia ser a largura da página inteira (595,28pt) — muito mais do que suficiente para as quatro palavras (soma ≈424pt). Se mesmo assim quebrar, confirma que o problema não é o cálculo da margem em si, mas outra coisa a seguir na cadeia (por exemplo, um valor por defeito que não está a ler a margem configurada).

### Critério de fecho

- [ ] Valor exacto da largura disponível na decisão de quebra, confirmado com `file:line`, não com nome de variável.
- [ ] Comparado com a largura esperada (página menos margens).
- [ ] Teste de margem zero corrido, confirmando ou eliminando a hipótese de duplicação de margem.
- [ ] Causa localizada com precisão — não "fonte, espaçamento, ou margem", uma resposta só.

---

## Decisão

Depois de confirmada a causa: corrigir, ou registar por escrito porque a diferença é aceitável, com o número exacto ao lado, não com a frase "variações acumuladas".

---

## Critério de fecho do passo

- [ ] Causa exacta confirmada com `file:line`.
- [ ] Teste de margem zero corrido.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p587.md`, com hash do commit.
