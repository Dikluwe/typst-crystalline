# Passo 965 — formalizar a distinção de gate: paragem obrigatória só para mudança de contrato/comportamento por defeito

**Precede este passo**: disclosure do executor em resposta a pergunta do dono — a "Regra de Ouro"
escrita (L0 primeiro, depois código) não distingue "parar e esperar confirmação" de "seguir em
fluxo contínuo depois de editar o L0". Na prática, ao longo de P893-964, a paragem real só
aconteceu quando havia mudança de contrato público (campo novo em `MathConstants`, método novo em
`FontMetrics`, assinatura pública) ou de comportamento por defeito do compilador (P956). Correções
L1-internas (fórmulas, mapeamentos de tabela, sem mudar contrato) seguiram em fluxo contínuo
(L0 editado primeiro, sem parar) — P957, P958, P961, P962, P963, P964.

**O dono aceitou a distinção como válida**, pedindo que deixe de ser convenção informal e vire
regra escrita.

**Pré-condição de árvore**: `git status`.

---

## Fase A — confirmar o critério com precisão, antes de formalizar

1. Enumerar, a partir dos passos já citados, o critério exato que separa os dois casos — proposta
   inicial a confirmar/refinar:
   - **Paragem obrigatória**: adicionar/remover/mudar assinatura de campo público em qualquer
     `entities/` (`MathConstants`, `GlyphVariant`, etc.); adicionar/mudar método em qualquer trait
     público (`FontMetrics`, etc.); mudar o comportamento por defeito de qualquer caminho já usado
     por utilizador final (o que P956 fez); qualquer mudança que, se revertida depois, quebraria
     compatibilidade binária/de API de outro código já escrito contra ela.
   - **Fluxo contínuo (sem paragem)**: correção de fórmula/valor dentro de uma função já existente,
     sem mudar a sua assinatura; adição/correção de entrada numa tabela de mapeamento (símbolos,
     nomes); qualquer mudança cujo "desfazer" não quebre nada fora do próprio módulo.
2. Confirmar se há casos de fronteira nos passos já executados que não se encaixam limpo em nenhum
   dos dois grupos — se houver, refinar o critério até cobrir esses casos sem ambiguidade.
3. Confirmar se este critério já está implícito em alguma ADR existente (`ADR-0114`/`0117`, sonda
   antes da spec) ou se é genuinamente uma decisão nova — varredura real de `00_nucleo/adr/`, mesmo
   protocolo de P954 (não presumir, confirmar por listagem).

## Fase B — escrever a ADR

1. Confirmar o próximo número livre por listagem real do directório (mesmo protocolo de P954/955 —
   não assumir sequência).
2. Escrever a ADR com o critério da Fase A, citando os passos onde a distinção já foi aplicada na
   prática (P893/896/906/909/915/918/922/927/937/956/959 como exemplos de paragem correta;
   P957/958/961/962/963/964 como exemplos de fluxo contínuo correto) como evidência de que o
   critério já funciona, não é só teoria.
3. Actualizar a "Regra de Ouro" existente (onde estiver escrita — `CLAUDE.md` ou equivalente) para
   referenciar esta ADR e deixar de ser ambígua sobre quando parar.

## Resultado esperado

- Critério de gate formalizado, preciso o suficiente para não exigir julgamento caso a caso.
- ADR escrita, número confirmado por varredura real.
- Regra de Ouro actualizada para referenciar o critério, não deixar a distinção implícita.
