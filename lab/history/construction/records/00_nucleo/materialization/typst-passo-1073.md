# L0 — Passo 1073: Supplements de `ref` Divergem (Figura, Equação) — Achado #7 do P1031

**Gate**: `ADR-0127` — mudança de comportamento por defeito (todo `@rótulo` para
figura ou equação é afectado). **Requer confirmação do dono antes de codificar.**

**Base**: P1031, Achado #7 (citação literal confirmada no P1071): `@f`: vanilla
`Figure 1`, cristalino `Fig. 1`; `@eq`: vanilla `Equation 1`, cristalino `(1)`;
`@h`: ambos `Section 1` ✅ (já bate, não precisa de correcção). Prompt relevante
já identificado: `entities/elements/ref.md`.

---

## 1. Ler antes de codificar

Não tenho, nesta conversa, o conteúdo real de `entities/elements/ref.md`, nem de
`01_core/src/compiler/layout/references.rs` (arquivo citado no P1071 para o
código actual, linhas 242/269). Pedir:

- `00_nucleo/prompts/entities/elements/ref.md`
- `01_core/src/compiler/layout/references.rs`

## 2. Não reinventar — `@h` já funciona, entender por quê primeiro

`@h` (heading) já produz `Section 1`/`Secção 1` correctamente conforme a língua
do documento (achado do P788, citado em `compiler/layout.md` — "com numbering
(doc en) → `Section 1` ... em docs pt → `Secção 1`"). Isto indica que já existe
um mecanismo de resolução de nome localizado (`LocalName`-like) em produção,
funcionando para heading.

**Antes de desenhar qualquer coisa nova**: localizar esse mecanismo (buscar por
`LocalName` ou equivalente no código/prompts) e confirmar por que `@f`/`@eq` não
o usam — hipótese mais provável (a confirmar, não assumir): figura e equação
usam um caminho de resolução mais antigo/separado (`"Fig. "` fixo,
`CounterKey::Str` per o trecho já citado no P1071) que nunca foi migrado para o
mesmo mecanismo do heading.

Se a hipótese se confirmar, o mecanismo autorizado é **estender o uso do
mecanismo já existente** a figura/equação — não criar um segundo sistema de
localização em paralelo.

## 3. Duas correcções de forma diferente, não uma só

- **`@f`** — `Fig.` (abreviação) → `Figure`/`Figura` (nome completo,
  localizado). Mesmo formato, palavra diferente.
- **`@eq`** — `(1)` (só número entre parênteses) → `Equation 1`/`Equação 1`
  (palavra + número, sem parênteses). Formato estruturalmente diferente, não só
  troca de palavra — confirmar se `(1)` vem de um caminho de código totalmente
  distinto do de figura antes de tentar consertar os dois com a mesma mudança.

## 4. Medição antes de codificar

```
= Introdução
#figure(image("x.png"), caption: [Um gráfico]) <fig1>
$ x = y $ <eq1>
Ver @fig1 e @eq1.
```

Medir `pdftotext`/render nos dois binários, confirmar os textos exactos
produzidos para os dois `@rótulo`, em documento inglês e português (já que a
localização por língua é parte do que está em jogo, per o precedente do `@h`).

## 5. Critérios de verificação

1. `@f` em documento inglês → `Figure 1`. Em português → `Figura 1` (confirmar
   se `Figura` é mesmo o termo vanilla em pt, não presumir tradução directa).
2. `@eq` em documento inglês → `Equation 1`. Em português → termo equivalente,
   mesma confirmação.
3. `@h` continua a produzir `Section 1`/`Secção 1` — não regressão do caso que
   já funcionava (guarda explícita, mesma disciplina do P1054).
4. Se o mecanismo de `@h` foi de facto reutilizado (não duplicado) — confirmar
   isso no relatório, não só que o resultado bate.
5. `crystalline-lint .` — 0 erros.
6. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Arquivos do §1 lidos.
- Mecanismo de `@h` localizado e confirmado como reutilizável (ou, se não for,
  explicado por que não).
- §3 tratado como duas correcções distintas, não uma só.
- 6 critérios de verificação do §5 confirmados.
