# P1163 — fechar paridade observável de `Symbol` e preparar o lote para commit

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** commit `4ed7f6a8d`; vanilla ratificado `a51e02804`
**Pré-condição:** P1162 `GREEN`, com P1161/P1162 e respetivas alterações ainda
fora de staging

## Objetivo

Fechar as duas divergências observáveis que restaram da nucleação de
`Symbol`: serialização pelo comando público `typst eval` e pretty-print de
`repr` para symbols complexos. Medir primeiro contra o vanilla ratificado,
atualizar os L0s afetados antes dos testes/código e deixar o lote P1161–P1163
pronto para revisão e commit, sem iniciar ainda a auditoria de HTML.

## Proveniência obrigatória

Antes de usar qualquer número ou saída como decisão, registar:

```text
data/hora com timezone
git rev-parse HEAD
git status --short
git diff HEAD --stat
sha256sum dos dois binários medidos
```

Referências de execução:

```text
vanilla:   /usr/local/bin/typst
cristalino: ./target/debug/typst
```

Confirmar que o vanilla corresponde ao alvo ratificado `a51e02804`; não
inferir proveniência pela string `--version`. Se o estado da working tree
mudar entre medições relevantes, registar novamente hora e diff-stat.

## Ordem obrigatória

### 1. Auditar especificação vigente e código dono

Ler, sem varrer `00_nucleo/context/` ou `00_nucleo/materialization/`:

- `AGENTS.md`;
- `00_nucleo/adr/ADR-0107.md`;
- `00_nucleo/adr/ADR-0108.md`;
- `00_nucleo/adr/ADR-0127.md`;
- `00_nucleo/prompts/entities/symbol.md`;
- `00_nucleo/prompts/compiler/stdlib/foundations/repr.md`;
- `00_nucleo/prompts/shell/cli.md`;
- os L0s adicionais apontados pelos headers dos ficheiros efetivamente
  alterados.

Inspecionar os donos atuais, pelo menos:

- `01_core/src/entities/symbol.rs`;
- `01_core/src/compiler/eval/repr.rs`;
- `02_shell/src/cli.rs`;
- `04_wiring/src/main.rs`, apenas para confirmar o encadeamento existente.

Não promover a divergência observada a decisão antes desta auditoria.

### 2. Medir primeiro a superfície pública

Executar a mesma matriz nos dois binários, preservando stdout, stderr e exit
code. Cobrir `--format json`, `--format yaml` e `--format raw`, com e sem
`--pretty`, onde a combinação for aceite pelo CLI:

```typst
emoji.heart
emoji.heart.arrow
emoji.heart.excl
symbol("♥️")
symbol("👩‍💻")
repr(emoji.heart)
repr(emoji.heart.arrow)
repr(symbol("👩‍💻"))
```

Adicionar probes progressivos para descobrir, sem adivinhar:

- em que tamanho/forma o `repr` troca uma linha por múltiplas linhas;
- indentação, vírgula final, escapes e estabilidade da ordem das variants;
- se modifiers filtram ou conservam a lista no `repr`;
- quais formatos serializam `Symbol` diretamente e qual é a representação;
- se `--pretty` altera JSON/YAML, `repr`, ambos ou nenhum;
- mensagem e exit code exatos dos formatos não suportados.

Guardar no relatório apenas resultados reproduzíveis e a proveniência do
estado que os gerou.

### 3. Classificar à luz de ADR-0107/0108

Só depois da medição, classificar cada diferença:

- serialização pública e texto produzido pelo CLI são observáveis da
  linguagem/produto, incluindo erro quando esse é o contrato;
- conteúdo, escapes e forma textual de `repr` são sintaxe observável;
- a escolha interna do helper, algoritmo, alocação e estrutura de dados é
  mecânica e pode divergir;
- whitespace só pode ser descartado se a medição provar que não pertence ao
  formato observável em causa.

Marcar explicitamente toda inferência e a medição que a refutaria.

### 4. Atualizar L0 antes de escrever testes ou código

Corrigir primeiro os L0s realmente afetados. No mínimo, auditar se cabem em:

- `shell/cli.md`: formatos aceites, valor serializado, `--pretty`, erros e
  exit code de `typst eval` para `Symbol`;
- `compiler/stdlib/foundations/repr.md`: escolha compacta/multilinha,
  indentação, escapes, variants e modifiers;
- `entities/symbol.md`: somente se a medição revelar contrato semântico ainda
  não especificado.

Ressellar os hashes de linhagem após cada alteração de L0 e confirmar zero
`V5` antes de prosseguir.

Estas correções de paridade seguem o fluxo contínuo de ADR-0127: L0 primeiro,
RED → GREEN e revalidação, sem pausa. **Parar no gate ADR-0127**, porém, se a
solução exigir nova assinatura pública, novo campo público, mudança de
comportamento por defeito além da paridade medida, mudança de fase do pipeline
ou quebra de compatibilidade. Não criar API pública apenas para partilhar um
formatter.

### 5. Escrever testes e confirmar RED

Adicionar testes focados que expressem o resultado vanilla medido, não a sua
mecânica:

- em L1, `repr` exato de symbol simples, complexo, multi-codepoint e
  modificado, cobrindo as fronteiras compacta/multilinha encontradas;
- em L2, serialização JSON/YAML e comportamento de `--pretty` para `Symbol`;
- em wiring/CLI, pelo menos um caso direto de `typst eval emoji.heart` e um
  caso complexo, com stdout/stderr/exit code;
- casos negativos apenas para formatos que o vanilla rejeitar.

Executar os testes focados antes da implementação e registar o RED real. Se
eles já passarem, rever o corte: não fabricar uma implementação sem falha
demonstrada.

### 6. Implementar na unidade dona

Materializar o mínimo necessário para igualar a superfície medida:

- serializar `Symbol` pelo mesmo valor de linguagem observado no vanilla,
  preservando integralmente graphemes multi-codepoint;
- aplicar ao `repr` de `Symbol` as mesmas regras observáveis de
  compactação/pretty-print já legitimadas no L0;
- reutilizar abstração interna somente se a topologia permitir; não criar
  import reverso `entities → compiler`, não mover I/O para L1 e não expor
  assinatura pública por conveniência;
- manter ordem semântica das variants, identidade base+variants e modifiers
  já validados no P1162;
- não ampliar para outros tipos, para `Symbol::func()` genérico ou para HTML.

Se a arquitetura vigente não permitir corrigir o `repr` sem alterar contrato
público, documentar as opções medidas e parar no gate, sem implementar essa
parte.

### 7. Confirmar GREEN e ausência de regressão

Executar, nesta ordem:

```text
testes focados L1/L2/CLI
matriz diferencial vanilla ↔ cristalino
cargo test --workspace
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Exigir zero falhas novas e zero violações do linter. Repetir a matriz com os
binários finais gerados pelo mesmo estado registado.

### 8. Fechar relatório e parar antes do commit

Atualizar este documento com:

- tabela `expressão/formato → vanilla → cristalino → classificação`;
- RED e GREEN reproduzíveis;
- hashes dos L0s finais;
- contagens das suites com proveniência completa;
- lista exata dos paths do lote P1161–P1163;
- riscos ou diferenças remanescentes.

Auditar o diff conjunto de P1161–P1163, mas parar antes de staging/commit. O
commit só será feito mediante instrução posterior do dono. Mensagem candidata:

```text
feat(symbol): preserve multi-codepoint graphemes and emoji variants
```

## Critérios de aceitação

- `typst eval` serializa ou rejeita `Symbol` exatamente como o vanilla
  ratificado em cada formato do contrato vigente (`json|raw`);
- `repr` de symbols simples, complexos e modificados coincide no conteúdo e
  na forma textual observável;
- graphemes multi-codepoint não são truncados em nenhuma dessas superfícies;
- L0 precede testes/código e todos os hashes ficam ressellados;
- nenhum contrato público novo é introduzido sem gate ADR-0127;
- workspace, build, fmt, diff-check e `crystalline-lint` ficam GREEN;
- nenhum ficheiro é staged ou commitado neste passo.

## Próxima fila após o commit

Depois de o dono autorizar e concluir o commit do lote P1161–P1163, escrever
um passo separado para medir e nuclear a próxima superfície pendente,
começando pela auditoria feature-gated de HTML. Não misturar esse trabalho no
presente lote.

## Resultado executado

### Proveniência

Medição inicial em `2026-08-25T12:28:15-03:00` e medição final em
`2026-08-25T12:32:57-03:00`, ambas sobre HEAD
`4ed7f6a8d9d9943b74191444e1e3c23f8f584785` e working tree não commitado.
Binários finais medidos:

```text
vanilla /usr/local/bin/typst:
7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8
cristalino ./target/debug/typst:
4213b506b9d23d2310850d52e35370cfef4178530295f5b0405f1614f0d3e759
```

Estado final antes deste relatório: 40 paths rastreados alterados, 1.511
inserções e 1.008 remoções no lote acumulado; `typst-passo-1161.md`,
`typst-passo-1162.md` e `typst-passo-1163.md` permanecem untracked. A maior
parte do diff pertence à conversão mecânica das tabelas `sym`/`emoji` já
auditada no P1162.

### Medição e classificação

| Expressão/formato | Vanilla inicial | Cristalino inicial | Cristalino final | Classe |
|---|---|---|---|---|
| `emoji.heart`, JSON | `"❤️"`, exit 0 | erro `cannot serialize value`, exit 1 | igual ao vanilla | CLI observável |
| `symbol("👩‍💻")`, JSON | `"👩‍💻"`, exit 0 | mesmo erro | igual ao vanilla | semântica + CLI |
| `emoji.heart`, raw | erro `cannot print symbol`, hint, exit 1 | dizia `value` | igual ao vanilla | diagnóstico observável |
| `repr(emoji.heart)` | 23 peças multilinha | uma linha | igual ao vanilla | sintaxe |
| `repr(emoji.heart.arrow)` | `symbol("💘")` | igual | igual | sintaxe |
| `repr(symbol("👩‍💻"))` | ZWJ escapado, uma linha | igual | igual | sintaxe |
| `emoji.heart`, YAML | `❤️` + linha, exit 0 | formato rejeitado, exit 2 | não alterado | contrato público pendente |

`--pretty` não altera a serialização JSON de uma string. Raw aceita somente
string/bytes nos dois binários. A fonte ratificada confirmou `Symbol:
Serialize` como string e `pretty_array_like` com limite horizontal de 50
bytes; copiar o trait/estrutura do vanilla não foi necessário.

YAML foi separado no gate ADR-0127: o L0 P1137 já o declara scope-out e a sua
introdução requer adicionar `EvalFormat::Yaml` ao enum público. Não foi
implementado nem tratado como regressão do lote `Symbol`.

### L0 → RED → GREEN

L0s atualizados antes dos testes/código:

```text
entities/symbol.md                         @prompt-hash 45feb053
compiler/stdlib/foundations/repr.md        @prompt-hash 0158019d
shell/cli.md                               @prompt-hash f85e96a8
```

RED confirmado:

```text
L1 repr:   0 passed; 1 failed — forma compacta em vez de multilinha
L2 eval:   0 passed; 2 failed — JSON rejeitado e raw dizia `value`
CLI:       coberto após o RED unitário para fechar a superfície ponta a ponta
```

GREEN focado:

```text
L1 repr:   1 passed; 0 failed
L2 eval:   2 passed; 0 failed
CLI:       2 passed; 0 failed
```

A implementação adicionou somente mecânica privada: pretty formatting dentro
da entidade sem import reverso, braço JSON para o grapheme efetivo e nome de
tipo `symbol` no diagnóstico. Nenhuma assinatura, campo, enum ou fase pública
foi alterada no P1163.

### Validação final

```text
typst-core:             5.232 passed; 0 failed
typst-infra:              846 passed; 0 failed
typst-shell:               55 passed; 0 failed
wiring unit:                2 passed; 0 failed
wiring/CLI:                57 passed; 0 failed
lint integration:           2 passed; 0 failed
doc-tests:                  0 failed; 3 ignored
cargo check --workspace:    exit 0
cargo build --workspace:    exit 0
cargo fmt --all -- --check: exit 0
git diff --check:           exit 0
crystalline-lint .:         exit 0; zero V5
```

O lint conserva apenas advisories históricos fora do gate de drift. Nenhum
ficheiro foi staged ou commitado.
