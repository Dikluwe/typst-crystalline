# ADR — Checklist obrigatória de propagação aos quatro sub-layouts

**Data:** 2026-07-09
**Estado:** Em vigor
**Aplica-se a:** qualquer passo que corrija ou adicione um mecanismo de layout no fluxo principal (`cursor.rs`, `mod.rs`).

---

## O problema

Não existe um único algoritmo de layout neste projecto. Existem cinco caminhos de código separados que decidem como colocar conteúdo num espaço:

1. O fluxo principal (`flush_line`, `finish`, em `cursor.rs`/`mod.rs`).
2. `grid.rs`.
3. `placement.rs`.
4. `columns.rs`.
5. `boxed.rs`.

Os últimos quatro foram construídos em alturas diferentes, muitas vezes antes de um mecanismo existir no fluxo principal (RTL, por exemplo, não existia quando `grid.rs` foi escrito pela primeira vez). Quando um mecanismo é corrigido ou adicionado no fluxo principal, nada obriga a confirmar se os outros quatro precisam da mesma correcção. Não há erro de compilação nem teste a falhar — os testes desses ficheiros foram escritos antes do mecanismo existir, por isso nunca o testaram.

Isto já aconteceu três vezes com o mesmo formato:

- **P579/P580** — altura de linha calculada com `font_size_pt` estático no fluxo principal, corrigida; esquecida em `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`, descoberta só quando os testes de integração começaram a falhar.
- **P625** — alinhamento RTL corrigido no fluxo principal (P576), nunca propagado aos mesmos quatro ficheiros, descoberto por acidente dentro de um passo sobre outra coisa (P624).
- **P626** — direcção de preenchimento de colunas, um caso relacionado mas distinto, na mesma área de código.

## Regra

Qualquer passo que corrija ou adicione um mecanismo de layout no fluxo principal tem de incluir, no seu critério de fecho, uma verificação explícita dos quatro sub-layouts — não como um passo separado a fazer "depois, se sobrar tempo", mas como parte do próprio passo.

A verificação mínima, para qualquer mecanismo novo ou corrigido:

1. **Testar o mesmo conteúdo dentro de `#grid()`.**
2. **Testar o mesmo conteúdo dentro de `#box()`.**
3. **Testar o mesmo conteúdo dentro de `#place()`.**
4. **Testar o mesmo conteúdo dentro de `#set page(columns:)` ou `#columns()`.**

Se o mecanismo não se aplicar a algum destes contextos por razão arquitectural (como aconteceu com `place`/`align` em P625, onde propagar alinhamento RTL quebraria o posicionamento explícito), essa excepção fica registada com razão escrita — não descoberta mais tarde como bug.

## Como aplicar

No modelo de passo já usado neste projecto, adicionar uma secção fixa:

```markdown
## Verificação de propagação aos sub-layouts

- [ ] `#grid()` com o mesmo conteúdo/mecanismo: testado, resultado.
- [ ] `#box()` com o mesmo conteúdo/mecanismo: testado, resultado.
- [ ] `#place()` com o mesmo conteúdo/mecanismo: testado, resultado.
- [ ] Colunas com o mesmo conteúdo/mecanismo: testado, resultado.
- [ ] Excepções (se algum destes não se aplicar): razão escrita.
```

## Nota sobre a causa raiz, não só o sintoma

Esta regra trata do sintoma — verificar sempre os quatro sítios. A causa raiz (cinco implementações separadas do mesmo conceito, "colocar conteúdo num espaço") é maior, e a correcção completa seria unificar os cinco caminhos numa só função partilhada — o mesmo que já foi feito para largura de texto em P593 (`text_width`, `line_content_right`). Isso reduziria a necessidade desta checklist a zero, porque um mecanismo corrigido na função única corrigiria automaticamente todos os cinco caminhos.

Essa unificação completa é um trabalho maior, ainda não feito para todos os mecanismos (só para largura). Até lá, esta checklist é a defesa disponível.

## Ligação às regras anteriores

- **Decisão nova obrigatória** — nenhum scope-out fica aceite para sempre sem revisão.
- **Disciplina de verificação** — número, não suposição.
- **Proveniência de medição** — saber de onde veio um número.
- **Paridade de defeitos nos testes** — fonte igual dos dois lados ao comparar.
- **Esta regra** — um mecanismo corrigido num sítio não está corrigido em todo o lado até se confirmar isso nos quatro sub-layouts.
