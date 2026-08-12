# Passo 1020 — Fechar: marcador de nota de rodapé — formato e `numbering` ignorado

**Tipo**: Investigar → classificar gate → corrigir se aprovado. Dois eixos, per o próprio
achado do P1016 — tratar como uma decisão só, não fragmentar em dois passos, mas resolver
os dois juntos aqui.
**Medido em P1016** (Parte 3, secção 3.6b): vanilla usa número em superscript sem
delimitadores (`A¹ B²`); cristalino usa `[N]` literal (`A[1] B[2]`).
`FootnoteElem::numbering` existe como campo mas é ignorado por `compiler/layout/
footnote.rs` — `#set footnote(numbering: "*")` não tem efeito.
**Pré-condição**: `git status` limpo. Independente dos Passos 1018/1019.

---

## Fase A — Confirmar os dois problemas por leitura, antes de desenhar

1. `compiler/layout/footnote.rs` — confirmar por `file:line` que `numbering` é lido do
   `FootnoteElem` mas nunca usado no render do marcador (achado do P1016, reconfirmar
   directamente, não aceitar por citação).
2. Localizar o mecanismo de superscript já existente no cristalino (usado para
   sobrescrito matemático, ou outro) — é reaproveitável para o marcador de nota, ou o
   marcador de nota precisa de mecanismo próprio? Não presumir.
3. Confirmar a regra vanilla exacta: `numbering` default é o quê, e como o superscript é
   aplicado (estilo de texto, ou glifo específico)?

## Fase B — Classificar o gate

Mudança de output visual em qualquer documento com notas de rodapé → categoria 2/3,
ADR-0127. L0 antes de código.

```
Dado #footnote[nota] sem #set footnote(numbering:) definido
Quando renderizado
Então marcador aparece em superscript, formato per regra vanilla confirmada na Fase A
  (confirmar se há ou não delimitador visível — não presumir "sem colchetes" sozinho)

Dado #set footnote(numbering: "*") definido
Quando renderizado
Então o padrão de numbering é aplicado (hoje: ignorado, deve passar a funcionar)
```

Não-regressão: `p552_footnote_counter_avanca_em_set_page_columns` e todos os testes de
posicionamento/numeração de nota do Passo 1016 — a numeração em si (já fechada) não pode
mudar, só a forma como é desenhada.

## Fase C — Implementar (só após gate) e validar

```
crystalline-lint .
cargo test --workspace
```

Decalque visual explícito contra o vanilla (render, não só `pdftotext`) — é mudança de
aparência, texto extraído pode não bastar para confirmar.

---

## Resultado esperado

Marcador de nota em superscript per vanilla, `numbering` a ter efeito. Numeração (P1016)
inalterada — só a forma de desenho muda.
