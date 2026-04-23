# Passo 110 — Fechar DEBT-45 (`check_*_depth` integrados)

**Série**: 110 (passo pequeno; inventário decide forma).
**Precondição**: Passo 109 encerrado; `Engine<'a>` materializado;
803 L1 + 184 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0036, ADR-0037, ADR-0044 (Engine).
**ADR nova**: depende da decisão em 110.A. Se opção 3 (manter
funções livres), sem ADR. Se opção 1 ou 2 (método novo), ADR
pequena a justificar.

---

## Objectivo

Fechar DEBT-45 completamente. Do Passo 105 (auditoria):

> DEBT-45 — `check_*_depth` não chamados: 2/4 ✓ (show + call);
> 2/4 pendentes (layout adiado, html aguarda pipeline).

As 2 pendentes (provavelmente `check_layout_depth` e `check_html_depth`
ou nomes análogos) têm de ficar **chamadas nos sítios certos** ou
**documentadas como não aplicáveis** para DEBT-45 poder fechar.

A **forma** da integração (método de `Route`, método de `Engine`, ou
funções livres) fica decidida em 110.A com base no estado empírico.

---

## Decisões já tomadas

1. **Âmbito**: fechar DEBT-45 no seu critério original. Não expandir
   para "melhorar safety rails no geral".
2. **Forma**: decidida em 110.A. Opções:
   - **Método de `Route`** (tracked) — semanticamente mais limpo,
     mas restrições comemo.
   - **Método de `Engine`** (não-tracked) — agrega tudo, sem
     restrições.
   - **Funções livres** — só adicionar chamadas nos sítios certos.
3. **Gate de escopo**: se 110.A revelar que alguma check não
   é aplicável (ex: `check_html_depth` exige pipeline HTML que o
   cristalino não tem), documentar como "não aplicável no
   cristalino" e fechar DEBT-45 com nota.
4. **Âmbito estrito**: não materializar funcionalidade nova (ex:
   `check_math_depth` se existir no vanilla mas não no cristalino).
   Só integrar o que já existe.

---

## Escopo

**Dentro**:
- Leitura do estado actual de `check_call_depth`, `check_show_depth`,
  `check_layout_depth`, `check_html_depth` (ou nomes análogos).
- Integração das pendentes em sítios adequados.
- Possível refactor para método de `Route`/`Engine` se 110.A
  decidir.
- Fecho do DEBT-45 em `DEBT.md`.

**Fora**:
- `Introspector` / Candidato 2 do Passo 108.
- Formato rico de warnings.
- CLI em `04_wiring`.
- Qualquer check que não esteja já implementada como função.

---

## Sub-passos

### 110.A — Inventário agressivo

**Parte 1 — O que existe**:

1. Grep por `check_call_depth`, `check_show_depth`,
   `check_layout_depth`, `check_html_depth`, `check_math_depth`,
   `MAX_CALL_DEPTH`, `MAX_SHOW_DEPTH`, `MAX_LAYOUT_DEPTH`, etc.
   em `01_core/src/`.
2. Para cada encontrada, registar:
   - Nome exacto.
   - Ficheiro:linha de definição.
   - Assinatura.
   - Se é função livre ou método.
   - Onde é chamada hoje (grep por nome).
   - Limite que aplica.

**Parte 2 — Sítios candidatos para integração**:

Para cada check **não** chamada hoje:

1. Identificar onde faria sentido chamá-la. Exemplos prováveis:
   - `check_layout_depth` → no `Layouter` em cada entrada de nó
     composto (Grid, Page, Section).
   - `check_html_depth` → pipeline HTML se existir.
2. Verificar se o sítio candidato existe no cristalino. Se não
   (ex: pipeline HTML ausente), documentar como "não aplicável".

**Parte 3 — Estado do `check_call_depth`**:

O Passo 17 introduziu `EvalContext.depth` + `enter_call` para
controlo de profundidade de chamadas. Confirmar:

- `check_call_depth` ainda existe como função separada, ou foi
  absorvida em `EvalContext.enter_call`?
- Se existe, faz o mesmo que `EvalContext.enter_call` ou coisa
  diferente?
- Se duplica funcionalidade, candidato a consolidação.

**Parte 4 — Decisão de forma**:

Com base nas partes 1-3, escolher uma forma:

- **Opção A — Funções livres, só integrar**: se as funções
  estão feitas, os sítios onde chamar estão claros, e não há
  benefício em refactor, só adicionar chamadas. Sem ADR.

- **Opção B — Método de `Route`**: se `check_call_depth` faz
  sentido como operação sobre `Route` (contador da pilha de
  ficheiros). Requer ADR por ser mudança de tipo tracked —
  tracked impl tem restrições (descobertas dos passos 106-109).

- **Opção C — Método de `Engine`**: se os 4 checks são naturalmente
  operações sobre o Engine (que agrega o estado de eval).
  Sem restrições comemo. ADR pequena a documentar.

Documentar decisão em
`00_nucleo/diagnosticos/inventario-debt45-passo-110.md`:

```
Checks existentes:
  check_call_depth — <assinatura> — chamada em <N> sítios
  check_show_depth — ...
  check_layout_depth — DEFINIDA mas não chamada
  check_html_depth — AUSENTE (pipeline não existe no cristalino)

Sítios candidatos para integração:
  check_layout_depth → rules/layout/mod.rs:<linha> em layout_grid/page/...

Estado de check_call_depth vs EvalContext.enter_call:
  [análise]

Forma escolhida: A / B / C
  Razão: [...]
```

**Gate**: se 110.A revelar que **todas** as checks pendentes são
"não aplicáveis no cristalino", o passo torna-se puramente
documental — actualizar DEBT-45 para "2/4 aplicáveis, todas
integradas; 2/4 não aplicáveis por ausência de pipeline
correspondente" e fechar.

### 110.B — ADR (condicional)

Só se forma B ou C:

- **B**: ADR sobre `Route` ganhar método `check_call_depth`.
  Documentar que o método é tracked se já for, ou que não é
  (em sub-impl não-tracked).
- **C**: ADR sobre `Engine` ganhar métodos `check_*_depth`.
  Pequena — documenta o padrão (métodos de conveniência sobre
  agregador).

Se forma A, sem ADR. 110.B skipado.

### 110.C — Implementação

**Se forma A (funções livres)**:

Adicionar chamadas nos sítios identificados em 110.A. Exemplo:

```rust
// rules/layout/mod.rs — no início de layout_grid
check_layout_depth(layout_depth, grid.span())?;
layout_depth += 1;
// ... resto da função
```

A profundidade tem de vir de algum sítio. Se não há contador de
layout hoje, 110.A tem de ter identificado onde ele seria — pode
ser campo novo do Engine ou do Layouter. **Se implicar campo novo
com propagação extensiva**, sair do âmbito estrito — gate dispara.

**Se forma B (método de `Route`)**:

```rust
#[comemo::track]
impl Route {
    pub fn check_call_depth(&self, span: Span) -> SourceResult<()> {
        if self.len() > MAX_CALL_DEPTH {
            Err(...)
        } else {
            Ok(())
        }
    }
}
```

Ajustar call sites: `route.check_call_depth(span)?;`.

Atenção às 3 descobertas acumuladas sobre `#[comemo::track]`:
- `Clone` exigido na struct.
- `Option<&str>` com lifetime falha.
- `TrackedMut::reborrow_mut` em descida.

Se qualquer uma das 3 aplicar e criar fricção, reconsiderar
forma (voltar a A ou C).

**Se forma C (método de `Engine`)**:

```rust
impl Engine<'_> {
    pub fn check_call_depth(&self, span: Span) -> SourceResult<()> { ... }
    pub fn check_show_depth(&self, span: Span) -> SourceResult<()> { ... }
    pub fn check_layout_depth(&self, span: Span) -> SourceResult<()> { ... }
}
```

Call sites: `engine.check_call_depth(span)?;`.

O `Engine` hoje não tem contadores de profundidade. Se eles vivem
em `EvalContext` (caso de `check_call_depth` via `enter_call`),
os métodos do Engine delegam para lá. Ou os contadores movem para
Engine (decisão separada, pode ficar para outro passo).

### 110.D — Testes

**Novos testes mínimos** para cada check integrada:

1. Input Typst que dispara a check (ex: função que recursa 200
   vezes se `check_call_depth` é limite 64).
2. Verificar que o eval retorna erro de profundidade.
3. Verificar que a mensagem de erro identifica qual limite foi
   atingido.

Para checks que eram "não aplicáveis no cristalino", **sem
testes**. O DEBT é fechado por documentação.

Se forma A sem novos tipos/métodos, os testes podem viver em
`rules/layout/tests.rs` ou equivalente — onde a function vive.

Se forma B ou C, testes em `entities/route.rs` ou
`entities/engine.rs` `#[cfg(test)]`.

### 110.E — Encerramento

1. `cargo test --workspace`: ≥ linha de base + testes novos
   (provavelmente 803 + 2-4 L1 ou 184 + 2-4 L3, dependendo de
   onde vivem).
2. `crystalline-lint` zero violations.
3. `DEBT.md` actualizado:
   - DEBT-45 movido para Secção 2 (encerrados).
   - Linha "**ENCERRADO (Passo 110)** — forma X, N checks
     aplicáveis integradas, M não aplicáveis documentadas".
4. ADR promovida a `EM VIGOR` se aplicável (forma B ou C).
5. Relatório `typst-passo-110-relatorio.md`:
   - Estado antes/depois de cada check.
   - Forma escolhida e razão.
   - Sítios onde foram integradas.
   - Checks não aplicáveis e razão.
   - DEBT-45 final.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 110.A escrito.
2. Forma escolhida com razão documentada.
3. ADR criada e promovida (se B ou C).
4. Checks pendentes integradas OU documentadas como não
   aplicáveis.
5. Testes novos passam (se integração real).
6. `cargo test --workspace` passa.
7. `crystalline-lint` zero violations.
8. DEBT-45 fechado.
9. Relatório 110.E escrito.

---

## O que pode sair errado

- **Check pendente requer infraestrutura nova**. Se
  `check_layout_depth` precisa de contador de profundidade e o
  contador implica propagar mais um parâmetro por 10 funções, o
  passo sai do âmbito. Gate: se propagação > 2 funções, **parar**
  e reportar. Alternativa: adicionar contador ao Engine, mas
  isso é refactor maior — merece passo separado.
- **Forma B colide com restrições comemo**. Se `Route` já tem
  `#[comemo::track]` e adicionar método novo viola alguma
  restrição (ex: assinatura incompatível), reverter para forma
  A ou C. Registar a tentativa em relatório 110.E.
- **`check_call_depth` e `EvalContext.enter_call` duplicam
  funcionalidade**. Se 110.A revelar isto, a decisão certa é
  **consolidar**: eliminar uma das duas, manter a outra. Mas
  consolidação pode afectar testes existentes. Se afectar mais
  do que 2 testes, parar e abrir passo dedicado.
- **Todas as checks pendentes são "não aplicáveis"**. Resultado
  aceitável: DEBT-45 fecha por documentação, não por integração.
  Nenhum teste novo. Passo vira puramente documental.
- **Teste de recursão profunda demora muito ou crasha**. Limites
  típicos são 64-1024 iterações. Se o teste excede o tempo de
  timeout por estar a recursar N milhões, algo está mal — ver
  se o check está realmente implementado e a ser chamado.

---

## Notas operacionais

- O Passo 109 ressalvou explicitamente que DEBT-45 permaneceu
  pendente mesmo com Engine materializado, porque "nenhum passo
  a integrar trivialmente agora". Este é o passo dedicado.
- Se `check_call_depth` vier a ser método de `Route`, isto é
  **independente** da decisão 109 de mover `world` de
  `EvalContext` para `Engine`. `Route` permanece onde está
  (campo do Engine, tipo tracked).
- Os limites numéricos (`MAX_CALL_DEPTH` = 64 ou 256 ou outro)
  são do domínio vanilla — não alterar neste passo. Se 110.A
  revelar divergência entre cristalino e vanilla, documentar
  como "limite herdado do vanilla" ou "limite escolhido para
  cristalino" conforme aplicável.
- **Descobertas sobre comemo acumuladas continuam por
  documentar**. Se forma B escolhida e aplicar as 3 descobertas,
  este passo é oportunidade natural para fazer nota breve em
  `CLAUDE.md` — mas **não dobrar escopo**. Nota pode ser
  passo seguinte, muito pequeno.
