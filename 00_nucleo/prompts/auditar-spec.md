# Workflow — `auditar-spec.md`

**Propósito**: auditar a qualidade e completude de um prompt L0 (`00_nucleo/prompts/**/*.md`).
Usado por qualquer passo que precise de avaliar se um prompt está pronto para gerar código,
ou se tem lacunas que precisam de ser fechadas primeiro.

**Não é**: um linter mecânico (isso é `crystalline-lint`, V0-V12). Este workflow avalia
**conteúdo e clareza**, coisas que uma regex não vê.

---

## Como usar

Para cada prompt a auditar, aplicar os quatro blocos abaixo, nesta ordem. Cada bloco produz
achados independentes — não parar no primeiro problema encontrado, catalogar todos.

---

## Bloco 1 — Completude estrutural

O prompt tem as 5 secções de `template-prompts.md` (Contexto, Restrições Estruturais,
Instrução, Critérios de Verificação, Resultado Esperado)?

- Aceitar vocabulário equivalente já em uso no projecto (ex.: "Propósito" ≈ "Contexto",
  "Struct"/"Interface pública" ≈ "Instrução", "Invariantes"/"Critério" ≈ "Critérios de
  Verificação") — **não exigir o nome literal**, exigir que o **papel** da secção esteja
  coberto por alguma secção do documento.
- Se nenhuma secção do documento cobre um dos 5 papéis: achado de incompletude, citar qual
  falta.

## Bloco 2 — Ambiguidade de conteúdo

Para a secção "Instrução" (ou equivalente), verificar:

1. **Toda afirmação de comportamento é verificável?** Frases como "deve funcionar
   correctamente", "trata os casos normais", "comportamento razoável" são ambíguas —
   não dizem o que "correcto"/"normal"/"razoável" significa. Achado: citar a frase, propor
   o que falta especificar.
2. **Toda palavra-chave tem definição ou é auto-evidente no domínio?** Se o prompt usa um
   termo técnico específico do Typst (ex.: "auto-scaling", "fence", "stretchy") sem o
   definir nem citar a documentação oficial, e o termo não é óbvio pelo contexto — achado.
3. **Os limites do escopo estão explícitos?** Um prompt que descreve o que uma função faz
   mas não diz o que ela **não** faz, quando existem casos de borda plausíveis não
   cobertos, é omisso. Não é preciso enumerar infinitos casos — é preciso que o leitor saiba
   se um caso não mencionado é "não coberto, não pensado" ou "não coberto, decisão
   deliberada" (scope-out).
4. **Há contradição interna?** Duas secções do mesmo prompt a afirmar coisas incompatíveis
   sobre o mesmo comportamento.

## Bloco 3 — Fundamentação (quando aplicável)

Para prompts que descrevem comportamento de uma construção da linguagem Typst (não infra-
estrutura interna do cristalino):

- Toda afirmação sobre "o que o Typst faz" devia citar a documentação oficial
  (`typst.app/docs/`) ou, na ausência de página de documentação para o caso, uma medição
  directa contra o binário vanilla com proveniência (`file:line` ou comando+resultado).
- **Qual binário é o vanilla** (armadilha medida em 2026-08-13, ver `CLAUDE.md`
  §"Referência de paridade"): o alvo é upstream/main **`586e1bd43`**, pinado. Os binários
  de referência são `lab/typst-original/target/release/typst` e `/usr/local/bin/typst`;
  ambos reportam `typst 0.15.1 (586e1bd4)` porque o build recebe o pin completo por
  `TYPST_COMMIT_SHA`. O `./target/release/typst` da raiz é o **cristalino** — usá-lo como
  oráculo é medir o cristalino contra si mesmo. Confirmar qual binário se está a correr
  antes de citar a medição; e citar o **hash pinado**, a árvore e o SHA-256 do binário,
  não uma tag nem apenas `--version`.
- Afirmações **sem nenhuma das duas fontes** — nem citação, nem medição — são achado de
  fundamentação em falta, independentemente de estarem certas ou erradas.
- **Cuidado com generalizações não suportadas pela própria citação**: se o prompt cita a
  documentação mas a conclusão vai além do que a citação diz (ex.: a citação diz "X pode
  acontecer" e o prompt conclui "X é o único caso"), isto é achado — a citação não
  fundamenta a generalização. (Precedente conhecido: `compiler/eval.md`, afirmação sobre `\`
  em math ser "apenas" quebra de linha, quando a citação de origem só dizia que fórmulas
  podem conter quebras de linha — não excluía outros usos.)
- Quando existir corpus de documentação (`00_nucleo/corpus-docs/<área>/`) para a área do
  prompt, cruzar directamente — mais rápido e mais fiável do que ir buscar à documentação
  outra vez.

## Bloco 4 — Zero referência a passo (regra vigente)

Grep no próprio prompt: `\b[Pp]asso\s*[0-9]|\bP[0-9]{2,4}[A-Za-z]?\b`. Qualquer ocorrência
fora de `Criado em`/`Histórico de Revisões` (que só podem conter data + descrição, nunca
número de passo) é achado — remeter para a regra já estabelecida (Passo 999).

---

## Output

Um achado por problema encontrado, cada um com:
- Bloco de origem (1-4).
- Localização exacta (secção, linha se aplicável).
- Descrição do problema, curta, sem propor a correcção (isto é auditoria, não fix —
  mesma disciplina dos passos de diagnóstico).

Não há classificação B1/B2/B3 aqui (isso é ADR-0084, para módulos de código) — a saída
deste workflow é só a lista de achados; a decisão de o que corrigir primeiro é do passo
que invocou a auditoria.
