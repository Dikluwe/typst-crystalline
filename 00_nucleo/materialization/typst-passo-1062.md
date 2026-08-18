# L0 — Passo 1062: Resolução de Nomes de Arquivo Duplicados em `00_nucleo/prompts/`

**Gate**: `ADR-0127` — não é mudança de comportamento do compilador, mas é mudança
de convenção documental que afecta ~300 prompts e potencialmente 83 arquivos.
**Requer confirmação do dono antes de executar qualquer renomeação em massa.**

**Base**: inventário do P1060 (`typst-passo-1060-relatorio.md`, Parte 2) — 350
arquivos `.md`, 35 nomes colidindo, 83 arquivos envolvidos (23.7% do total).

---

## 1. Recapitulação dos dados (P1060)

- 1 nome com 7 ocorrências: `mod.md` (7 arquivos).
- 8 nomes com 3 ocorrências: `layout.md`, `heading.md`, `footnote.md`,
  `bibliography.md`, `table.md`, `label.md`, `color.md`, `_comum.md` (24 arquivos).
- 26 nomes com 2 ocorrências (52 arquivos).

## 2. Recomendação (a confirmar, não decisão unilateral)

**Opção (b)** — manter os nomes, exigir caminho completo em toda referência entre
prompts. Razões:

1. **`mod.md` é estrutural, não acidental.** As 7 ocorrências espelham a convenção
   do próprio Rust — todo submódulo chama-se `mod.rs`. Renomear os `.md`
   correspondentes (`entities-mod.md`, `compiler-lexer-mod.md`, etc.) quebraria a
   correspondência 1:1 entre nome de prompt e nome de arquivo Rust que o projecto já
   valoriza explicitamente (citado no próprio relatório do P1060, coluna "Arquivos
   afectados" da Opção b: "Preserva correspondência 1:1 de nomes entre especificação
   e módulo Rust").
2. **Já existe precedente no projecto para a disciplina de referência completa.** A
   regra do P1054 ("repetir citação completa em cada ocorrência reutilizada, não
   remissão interna curta — resiliência a fatiamento futuro") resolve um problema
   estrutural análogo com o mesmo princípio: preferir explicitação a atalho.
3. **Custo da Opção (a)**: 83 arquivos `.md` renomeados + anotação `//! @prompt` em
   cada arquivo `.rs` correspondente actualizada + risco de quebrar o rasto de
   proveniência que o `crystalline-lint` verifica (não confirmado se o linter lê
   `@prompt` para validação cruzada — a confirmar antes de escolher, ver §4).

Esta recomendação não é definitiva — é o ponto de partida para a confirmação do
dono, não uma execução automática.

## 3. Mecanismo autorizado, condicional à escolha de (b)

Se (b) for confirmada, este passo não altera nenhum arquivo `.md` ou `.rs`. Só
formaliza a regra:

- Adicionar a `00_nucleo/prompts/_convencoes.md` (ou arquivo equivalente de regras
  gerais do projecto, a localizar — não presumir que existe sem verificar) uma
  secção explícita: "referências a outro prompt L0, em qualquer arquivo `.md`,
  usam sempre o caminho completo a partir de `00_nucleo/prompts/` (ex.:
  `compiler/layout.md`, nunca só `layout.md`) — há 35 nomes de arquivo repetidos
  no corpus (P1060), e um nome sozinho é ambíguo."
- Verificar se o `crystalline-lint` (ferramenta do `tekt-linter`) já tem uma regra
  que audita menções a `.md` sem caminho completo dentro de outros `.md`. Se não
  tiver, isto é um candidato a nova regra V do linter — passo separado, não
  incluído aqui.

## 4. Se a escolha for (a) em vez de (b)

Confirmar antes de executar:

1. O `crystalline-lint` lê a anotação `//! @prompt <caminho>` nos arquivos `.rs`
   para validação cruzada (proveniência)? Se sim, a renomeação exige actualizar
   as ~300 anotações na mesma operação, não só os 83 arquivos `.md`.
2. Qual convenção de nome adoptar para o prefixo de camada — `compiler-layout.md`,
   `compiler_layout.md`, ou reestruturar em subpastas mais uma vez (já existe
   estrutura de pastas; o problema é nomes de folha repetidos entre pastas
   diferentes, não falta de pastas)?

Não implementar (a) sem resposta às duas perguntas acima.

## 5. Critério de conclusão

- Decisão registada explicitamente (a) ou (b), com justificação, no relatório deste
  passo.
- Se (b): regra de convenção escrita e localizada correctamente no corpus de
  prompts.
- Se (a): lista exacta dos 83 arquivos renomeados + confirmação de que todas as
  anotações `@prompt` correspondentes foram actualizadas (grep de verificação,
  zero referências ao nome antigo restantes).
- Nenhuma alteração de comportamento do compilador em qualquer dos dois casos —
  isto é só documentação/convenção.

---

## Nota — Caso 1 do P1061 continua pendente, separado deste passo

`Parágrafo → Heading` (P1059 Caso 1) não foi resolvido no P1061 (scope-out
explícito, depende de ler `compiler/layout/heading.rs` real). Não incluído aqui
porque é um passo de código, não de convenção documental — merece o seu próprio
L0 quando `heading.rs` estiver disponível.
