# Passo 106 — Canal de saída do Sink (DEBT-51)

**Série**: 106 (passo único; sub-passos de inventário, ADR,
implementação e verificação).
**Precondição**: Passo 105 encerrado; auditoria DEBTs confirmou
alinhamento; 803 L1 + 174 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0036 (atomização), ADR-0037 (coesão),
ADR-0042 (Sink materializado).
**ADR nova**: ADR-00NN "Canal de saída do Sink — TrackedMut no
caller, formatação em L3" — `PROPOSTO` em 106.B, `EM VIGOR` em
106.E.

---

## Objectivo

Resolver DEBT-51. Os warnings acumulados no `Sink` do `eval`
passam a chegar ao caller (L3) e a ser demonstrados
end-to-end com pelo menos um warning real.

Forma: o caller constrói `Sink`, passa `TrackedMut<Sink>` ao
`eval`, e depois de `eval` retornar, lê `sink.into_diagnostics()`.
A assinatura de `eval` **não muda**.

Um micro-piloto em L1 emite um warning real para validar o
canal ponta-a-ponta.

---

## Decisões já tomadas

1. **Forma do canal**: `TrackedMut<Sink>` já existe na assinatura
   de `eval` (Passo 12). Não mudar. O caller passa a usar.
2. **Formato**: L1 só fornece dados (`Vec<SourceDiagnostic>`). L3
   formata como quiser. Sem formatter em L1.
3. **Âmbito**: canal + micro-piloto trivial. Um sítio em L1 onde
   `sink` já é acessível (ou acessível com propagação ≤ 1 função)
   emite um warning concreto. **Não** é o DEBT-49 completo.
4. **Escolha do micro-piloto**: decidido em 106.A com base no
   inventário. Preferência: sítio onde `sink` hoje é dead weight
   (recebido, não usado).

---

## Escopo

**Dentro**:
- `03_infra/src/` — caller do `eval` passa a construir `Sink`,
  passar `TrackedMut`, e ler warnings depois. Aplicar `println!`
  ou equivalente no stderr para os warnings.
- `01_core/src/rules/eval/` — micro-piloto: um sítio emite
  `sink.warn(...)` real.
- Testes de integração end-to-end.

**Fora**:
- DEBT-49 completo (propagação de `sink` pelas funções `eval_*`
  — 5ª aplicação da ADR-0036 fica para passo dedicado).
- `Engine<'a>`.
- Mudança à assinatura pública do `eval`.
- Formatter sofisticado (cores, JSON, SARIF). O formato é
  "`warning: <span> <message>`" simples.
- Integração com CLI externa (clap, argparse, etc.) além do
  mínimo necessário.

---

## Sub-passos

### 106.A — Inventário

**Parte 1 — Callers actuais do `eval`**:

1. Grep por `eval(` em `03_infra/src/` e em testes. Listar:
   - Ficheiro:linha.
   - Como o `sink` é construído hoje (se é).
   - O que acontece após o retorno do `eval` (se o `sink` é
     lido ou descartado).
2. Para cada caller, classificar:
   - **P (produção)**: CLI real, função exportada.
   - **T (teste)**: bloco `#[cfg(test)]`.
   - **H (helper)**: função de conveniência (`eval_for_test`).

**Parte 2 — Candidatos a micro-piloto**:

1. Grep por comentários de silenciamento em `01_core/src/rules/eval/`:
   `DEBT-49`, `silenciad`, `TODO.*warn`, `// warning`.
2. Para cada candidato, confirmar:
   - O `sink` é acessível no frame actual? (directamente ou com
     propagação de ≤ 1 função.)
3. Ranking:
   - **Preferência 1**: o sítio de DEBT-49 em `eval_set_rule`
     (propriedades não suportadas de `#set text`). É o caso de
     uso mais visível; se `sink` está acessível lá, é o piloto
     ideal.
   - **Preferência 2**: qualquer sítio onde `sink` já é recebido
     como parâmetro e o silenciamento seria um warning óbvio.
   - **Preferência 3**: um sítio trivial sem silenciamento
     existente, onde introduzir `sink.warn` serve só como prova
     de vida do canal. Último recurso.

**Parte 3 — Construção de `Sink` no caller de produção**:

1. Verificar se há pelo menos um caller em L3 (não em teste) que
   hoje constrói `Sink`. Se não há, o canal de saída não tem
   consumidor real — o micro-piloto precisa de **criar esse
   caller** (pode ser função helper em L3 que vários testes
   usam).
2. Se há caller L3, inventariar o que ele faz após eval retornar.
   Presumível: descarta tudo excepto o `Module`.

**Parte 4 — Como `TrackedMut<Sink>` é construído no caller**:

1. Grep em testes pelos padrões actuais: `comemo::track_mut`,
   `Sink::new()`, etc.
2. Confirmar que não há bloqueio técnico (proc-macros comemo
   podem exigir que `Sink` implemente certos traits que hoje
   não tem).

**Escrever** em
`00_nucleo/diagnosticos/inventario-sink-canal-passo-106.md`:

```
Callers de eval() hoje:
  03_infra/src/X.rs:N — prod/teste/helper — sink construído? sim/não — lido? sim/não
  ...

Micro-piloto eleito: <ficheiro:linha>
  Razão: <propagação, visibilidade do warning>
  Warning concreto a emitir: "<mensagem>"

Construção de TrackedMut<Sink>:
  Padrão actual: <código>
  Bloqueios técnicos: nenhum / lista
```

**Gate**: se o inventário revelar que `TrackedMut<Sink>` não pode
ser construído no caller (ex: Sink não é `Hash + Eq` para o track),
parar e reavaliar a decisão "TrackedMut usado pelo caller". Pode
ser que "tuple return" seja a única via praticável. Reportar antes
de tocar no código.

### 106.B — ADR nova

1. Criar `00_nucleo/adr/typst-adr-00NN-canal-sink-saida.md` com
   `PROPOSTO`.
2. Conteúdo:
   - **Contexto**: Sink materializado (Passo 104); warnings
     acumulam mas não chegam a L3 (DEBT-51).
   - **Decisão de canal**: `TrackedMut<Sink>` mantido; caller
     constrói `Sink`, passa mutable track, lê
     `into_diagnostics` após retorno.
   - **Decisão de formato**: L1 não formata. L3 formata
     livremente. Para este passo, formato mínimo
     `"warning: <span-repr> <message>"` no stderr.
   - **Alternativas rejeitadas**:
     - **Tuple return**: invasivo, toca todos callers.
     - **Formatter em L1**: confunde responsabilidades;
       `SourceDiagnostic` tem os campos suficientes para L3
       formatar.
   - **Relação com DEBT-49**: este passo não resolve DEBT-49;
     abre o canal. DEBT-49 é a migração dos sítios silenciados
     (propagação de `sink` via 5ª aplicação da ADR-0036).
3. Promover a `EM VIGOR` em 106.E.

### 106.C — Implementação

Ordem obrigatória.

**106.C.1 — Caller de produção (ou helper L3)**:

1. No caller eleito em 106.A, construir `Sink` antes do `eval`:
   ```rust
   use comemo::track_mut;  // confirmar path real
   use typst_core::entities::sink::Sink;
   
   let mut sink = Sink::new();
   let result = eval(
       &routines,
       tracked_world,
       traced,
       track_mut!(&mut sink),  // ou sintaxe equivalente
       tracked_route,
       &source,
   );
   ```
2. Após retorno, drenar warnings:
   ```rust
   for diag in sink.into_diagnostics() {
       eprintln!("warning: {:?} {}", diag.span, diag.message);
   }
   ```
   Nota: `{:?}` em `Span` pode não dar saída útil. Usar
   representação textual se existir (`Span::debug_string()` ou
   similar); se não, aceitar `{:?}` como formato mínimo. Formato
   sofisticado é passo futuro.
3. Ordem crítica: warnings **antes** de decidir se falhar. Se
   `result` for `Err`, imprimir warnings antes do erro.

**106.C.2 — Micro-piloto em L1**:

1. No sítio eleito em 106.A (provavelmente `eval_set_rule` linha
   do "DEBT-49"):
   ```rust
   // Antes (Passo 102):
   _ => {
       // DEBT-49: silenciado.
   }
   
   // Depois (piloto):
   _ => {
       sink.warn(SourceDiagnostic::warning(
           span,
           format!("set: propriedade '{field_name}' ainda não suportada"),
       ));
   }
   ```
2. Se `sink` não é acessível no frame, propagar por no máximo
   **1 função** (o gate do Passo 104 aplicava-se lá; aqui o
   mesmo). Se exige mais propagação, reescolher candidato em
   106.A.
3. Preservar as restantes propriedades silenciadas — só o piloto
   é activado. DEBT-49 fica com N-1 sítios pendentes.

**106.C.3 — Verificar ligação**:

Após 106.C.1 e 106.C.2, um input Typst com
`#set text(font: "Arial")` (ou o que o piloto cobrir) deve
produzir warning no stderr quando o caller L3 executa.

Teste manual antes de escrever testes automatizados: compilar,
correr com input que dispara, confirmar que stderr tem a
mensagem.

### 106.D — Testes

1. **Teste unitário do canal** (em `03_infra/tests/` ou
   `#[cfg(test)]` no caller):
   - Input Typst que dispara o micro-piloto.
   - Verificar que `sink.into_diagnostics()` devolve pelo menos
     1 `SourceDiagnostic`.
   - Verificar que a mensagem contém o nome da propriedade
     (validação literal, não fuzzy matching).

2. **Teste de dedup end-to-end**:
   - Input com a mesma directiva duas vezes
     (`#set text(font: "X")\n#set text(font: "X")`).
   - Verificar que `into_diagnostics` tem **1** entrada (dedup
     do Passo 104 funciona no pipeline real).

3. **Teste de ausência**:
   - Input sem directivas que disparem.
   - Verificar que `sink.is_empty()` após eval.

4. **Teste de formato** (mínimo):
   - Capturar stderr (se viável no harness de teste) ou chamar
     a função de formatação directamente.
   - Verificar que a saída contém "warning:" e a mensagem.

### 106.E — Encerramento

1. Grep: `into_diagnostics` usado em pelo menos um sítio em L3
   (não só em testes do próprio Sink).
2. `cargo test --workspace`: ≥ linha de base + testes novos
   (803 + ~4 = ~807 L1 ou distribuído por L1/L3 conforme onde
   os testes ficam).
3. `crystalline-lint` zero violations.
4. ADR promovida a `EM VIGOR`.
5. **DEBT-51 marcado como ENCERRADO (Passo 106)** em DEBT.md.
   Mover para Secção 2.
6. **DEBT-49 actualizado**: uma propriedade migrada (o
   micro-piloto). Texto actualizado para reflectir "N-1 sítios
   pendentes"; se N é pequeno (ex: 4 propriedades), o DEBT pode
   ficar muito perto de fechar. Decidir em 106.E se merece
   sub-classificação ou se se mantém aberto com nota.
7. Relatório `typst-passo-106-relatorio.md` com:
   - Caller eleito + razão.
   - Micro-piloto eleito + mensagem literal escolhida.
   - Exemplo end-to-end: input Typst → stderr esperado.
   - Estado final DEBT-49 (N restantes).
   - Lacuna remanescente se aplicável (ex: formato rico adiado).

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 106.A escrito.
2. ADR-00NN criada e promovida.
3. Um caller em L3 (produção ou helper) constrói `Sink`,
   passa `TrackedMut`, lê warnings após eval, imprime.
4. Um micro-piloto em L1 emite warning real.
5. Testes de integração passam (canal, dedup, ausência,
   formato).
6. `cargo test --workspace` passa.
7. `crystalline-lint` zero violations.
8. DEBT-51 fechado.
9. DEBT-49 actualizado (−1 sítio).
10. Relatório 106.E escrito.

---

## O que pode sair errado

- **`TrackedMut<Sink>` não construível**. Gate em 106.A. Se a
  proc-macro comemo exigir traits que Sink não tem, parar.
  Alternativa: redesenhar com tuple return (fora do âmbito).
- **`Span` não é imprimível de forma útil**. `Span` pode ser um
  índice opaco; imprimir `{:?}` dá algo tipo `Span(42)` sem
  contexto de ficheiro/linha. Neste passo, aceitar — formato
  rico requer resolver `Span` para linha/coluna via `Source`,
  que é trabalho extra. Registar como limitação em 106.E.
- **Escolha do piloto subóptima**. Se o único candidato decente
  não é DEBT-49 mas um sítio menos visível, o passo perde
  impacto demonstrativo. Aceitar e registar — o objectivo é
  provar o canal, não provar qualquer caso particular.
- **Warnings em testes que passavam silenciosamente**. Activar
  o piloto pode fazer testes existentes passarem a produzir
  warnings. Se algum teste assere `sink.is_empty()` após eval
  com input que agora dispara, falha. Detectar em 106.D e
  actualizar os testes.
- **Ordem warnings vs erros**. Se o caller imprime warnings
  só quando `result.is_ok()`, perde warnings em casos de erro.
  Ordem crítica em 106.C.1: warnings **sempre**, erro depois.
- **Duplo drenar**. `into_diagnostics` consome `self`. Se o
  caller tenta ler duas vezes (ex: log + teste), fica com
  `Sink` morto. O padrão correcto é drenar uma vez e usar o
  `Vec` resultante; se for preciso duas vistas, clonar o `Vec`.
- **DEBT-49 quase fechado acidentalmente**. Se N (sítios
  silenciados) é pequeno e o piloto paga um, pode restar
  apenas 2-3. Isto não é problema — apenas nota que DEBT-49
  pode ser fechado num passo muito pequeno a seguir. Registar
  no relatório.

---

## Notas operacionais

- Este passo não materializa `Engine<'a>`. Os 9 parâmetros de
  `eval` permanecem.
- Não abrir caminho para formatter em L1. Se surgir tentação,
  resistir — `SourceDiagnostic` já tem span + message + hints,
  que é suficiente. Formatter é trabalho futuro de L3.
- Se a CLI real já existe e tem caminho sofisticado de erro,
  este passo não deve reescrever esse caminho. Só adiciona um
  `for diag in sink.into_diagnostics() { eprintln!(...) }` no
  sítio certo.
- O nome oficial de `Span::debug_string()` ou equivalente pode
  variar. Confirmar em 106.A; se não há método útil, usar
  `{:?}` e aceitar o resultado feio.
